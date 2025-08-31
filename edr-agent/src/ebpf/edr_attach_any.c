// edr_attach_any.c
// Load + attach a CO-RE .bpf.o and pin maps & links under /sys/fs/bpf/edr.
// - Reuses a shared ringbuf "edr_events_rb" if already pinned
// - bpffs-safe pinning: never put '.' in any path component
// - If any link can't be pinned, stay alive to hold it open (until SIGINT)

#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <libgen.h>
#include <sys/stat.h>
#include <sys/resource.h>
#include <sys/types.h>
#include <signal.h>
#include <unistd.h>
#include <fcntl.h>
#include <dirent.h>
#include <limits.h>

#include <bpf/libbpf.h>
#include <bpf/bpf.h>
#include <linux/bpf.h>

static const char *DEF_PIN_ROOT = "/sys/fs/bpf/edr";
static volatile sig_atomic_t stop = 0;

static void on_sigint(int sig) { (void)sig; stop = 1; }

/* --------------------------- util --------------------------- */

static int ensure_dir(const char *path) {
    if (!path || !*path) return -EINVAL;

    struct stat st;
    if (stat(path, &st) == 0) return S_ISDIR(st.st_mode) ? 0 : -ENOTDIR;

    // Recursively create like `mkdir -p`
    char tmp[PATH_MAX];
    size_t len = strnlen(path, sizeof(tmp) - 1);
    memcpy(tmp, path, len);
    tmp[len] = '\0';

    // Skip leading slashes
    char *p = tmp;
    while (*p == '/') p++;

    for (; *p; p++) {
        if (*p == '/') {
            *p = '\0';
            if (tmp[0] && mkdir(tmp, 0755) && errno != EEXIST) return -errno;
            *p = '/';
        }
    }
    if (mkdir(tmp, 0755) && errno != EEXIST) return -errno;
    return 0;
}

static int try_open_pinned_map(const char *path) {
    int fd = bpf_obj_get(path);
    return (fd < 0) ? -errno : fd;
}

static void set_memlock_unlimited(void) {
    struct rlimit rl = { RLIM_INFINITY, RLIM_INFINITY };
    setrlimit(RLIMIT_MEMLOCK, &rl);
}

// keep only [A-Za-z0-9_-] in a bpffs component; replace others (incl. '.') with '_'
static void sanitize_component(const char *in, char *out, size_t n) {
    if (!n) return;
    size_t j = 0;
    for (size_t i = 0; in && in[i] && j + 1 < n; i++) {
        char c = in[i];
        if ((c >= '0' && c <= '9') ||
            (c >= 'A' && c <= 'Z') ||
            (c >= 'a' && c <= 'z') ||
            c == '_' || c == '-') {
            out[j++] = c;
        } else {
            out[j++] = '_';
        }
    }
    if (j == 0) {
        const char *d = "unnamed";
        while (*d && j + 1 < n) out[j++] = *d++;
    }
    out[j] = '\0';
}

// extract basename, strip typical eBPF suffixes (.bpf.o / .bpf / .o), then sanitize
static const char *obj_base_sanitized(const char *path, char *buf, size_t n) {
    char tmp[PATH_MAX];
    strncpy(tmp, path, sizeof(tmp));
    tmp[sizeof(tmp) - 1] = '\0';
    char *b = basename(tmp);

    size_t L = strlen(b);
    while (L > 0) {
        if (L > 6 && !strcmp(b + L - 6, ".bpf.o")) { L -= 6; continue; }
        if (L > 4 && !strcmp(b + L - 4, ".bpf"))   { L -= 4; continue; }
        if (L > 2 && !strcmp(b + L - 2, ".o"))     { L -= 2; continue; }
        break;
    }

    char core[PATH_MAX];
    if (L >= sizeof(core)) L = sizeof(core) - 1;
    memcpy(core, b, L);
    core[L] = '\0';

    sanitize_component(core, buf, n);
    return buf;
}

/* --------------------------- main --------------------------- */

int main(int argc, char **argv) {
    const char *pin_root = DEF_PIN_ROOT;
    const char *obj_path = NULL;

    // args: [-p /sys/fs/bpf/edr] file.bpf.o
    for (int i = 1; i < argc; i++) {
        if (!strcmp(argv[i], "-p") && i + 1 < argc) { pin_root = argv[++i]; continue; }
        obj_path = argv[i];
    }
    if (!obj_path) {
        fprintf(stderr, "Usage: %s [-p PIN_ROOT] <file.bpf.o>\n", argv[0]);
        return 2;
    }

    signal(SIGINT, on_sigint);
    signal(SIGTERM, on_sigint);

    set_memlock_unlimited();
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
    libbpf_set_print(NULL); // set a printer for libbpf debug if needed

    if (ensure_dir(pin_root) < 0) {
        fprintf(stderr, "Failed to ensure pin root %s: %s\n", pin_root, strerror(errno));
        return 2;
    }

    // Prepare a per-object subdir for links (dot-less and sanitized)
    char objdir[256];
    obj_base_sanitized(obj_path, objdir, sizeof(objdir));

    char link_dir[PATH_MAX];
    snprintf(link_dir, sizeof(link_dir), "%s/%s", pin_root, objdir);
    if (ensure_dir(link_dir) < 0) {
        fprintf(stderr, "Failed to ensure link dir %s: %s\n", link_dir, strerror(errno));
        return 2;
    }

    // Attempt to reuse a shared ringbuf if pinned
    char rb_path[PATH_MAX];
    snprintf(rb_path, sizeof(rb_path), "%s/%s", pin_root, "edr_events_rb");
    int shared_rb_fd = try_open_pinned_map(rb_path);
    if (shared_rb_fd < 0) shared_rb_fd = -1; // not fatal

    struct bpf_object *obj = NULL;
    struct bpf_program *prog;
    struct bpf_map *map;

    obj = bpf_object__open_file(obj_path, NULL);
    if (!obj) {
        fprintf(stderr, "open_file failed: %s\n", strerror(errno));
        if (shared_rb_fd >= 0) close(shared_rb_fd);
        return 2;
    }

    // Pre-load: reuse shared ringbuf for maps named "events" or "edr_events_rb"
    bpf_object__for_each_map(map, obj) {
        const char *mname = bpf_map__name(map);
        enum bpf_map_type mtype = bpf_map__type(map);
        if (shared_rb_fd >= 0 && mtype == BPF_MAP_TYPE_RINGBUF) {
            if (mname && (!strcmp(mname, "events") || !strcmp(mname, "edr_events_rb"))) {
                if (bpf_map__reuse_fd(map, shared_rb_fd) == 0) {
                    fprintf(stdout, "[attach] Reused shared ringbuf for map '%s'\n", mname);
                }
            }
        }
    }

    /* ---- fixup: auto-size perf_event_array to #CPUs (pre-load) ---- */
    {
        int ncpu = libbpf_num_possible_cpus();
        if (ncpu < 1) ncpu = 1;

        bpf_object__for_each_map(map, obj) {
            const char *mname = bpf_map__name(map);
            enum bpf_map_type mtype = bpf_map__type(map);

            if (mtype == BPF_MAP_TYPE_PERF_EVENT_ARRAY) {
                int me = bpf_map__max_entries(map);
                if (me <= 0 || me < ncpu) {
                    if (bpf_map__set_max_entries(map, ncpu) == 0) {
                        fprintf(stdout, "[fixup] perf_event_array '%s' max_entries %d -> %d\n",
                                mname ? mname : "events", me, ncpu);
                    } else {
                        fprintf(stderr, "[fixup] failed to size perf_event_array '%s'\n",
                                mname ? mname : "events");
                    }
                }
            }
        }
    }

    if (bpf_object__load(obj)) {
        fprintf(stderr, "load failed: %s\n", strerror(errno));
        if (shared_rb_fd >= 0) close(shared_rb_fd);
        return 2;
    }

    // Pin maps (best-effort) directly under pin_root by map name
    bpf_object__for_each_map(map, obj) {
        const char *mname = bpf_map__name(map);
        if (!mname || !*mname) continue;

        char mcomp[NAME_MAX];
        sanitize_component(mname, mcomp, sizeof(mcomp));

        char mpath[PATH_MAX];
        snprintf(mpath, sizeof(mpath), "%s/%s", pin_root, mcomp);

        // If already pinned, skip quietly
        if (bpf_obj_get(mpath) >= 0) continue;

        if (bpf_map__pin(map, mpath) == 0) {
            fprintf(stdout, "[pin] map %s -> %s\n", mname, mpath);
        } else {
            // Optional: fprintf(stderr, "[pin] map %s failed: %s\n", mname, strerror(errno));
        }
    }

    // Attach all programs and pin links with dot-less filenames
    int attached = 0;
    int unpinned_links = 0;

    bpf_object__for_each_program(prog, obj) {
        const char *pname_raw = bpf_program__name(prog);

        struct bpf_link *lnk = bpf_program__attach(prog);
        if (!lnk) {
            fprintf(stderr, "[attach] FAIL %s: %s\n",
                    pname_raw ? pname_raw : "(null)", strerror(errno));
            continue;
        }
        attached++;

        char pname_safe[NAME_MAX];
        sanitize_component(pname_raw ? pname_raw : "prog", pname_safe, sizeof(pname_safe));

        char lpath[PATH_MAX];
        snprintf(lpath, sizeof(lpath), "%s/%s_link", link_dir, pname_safe);

        if (bpf_link__pin(lnk, lpath) == 0) {
            fprintf(stdout, "[pin] link %s\n", lpath);
        } else {
            fprintf(stderr, "[pin] link failed %s: %s (keeping link alive in this process)\n",
                    lpath, strerror(errno));
            unpinned_links++;
            // keep lnk referenced by this process for its lifetime (no destroy)
        }
    }

    if (shared_rb_fd >= 0) close(shared_rb_fd);

    if (attached == 0) {
        fprintf(stderr, "No programs attached from %s\n", obj_path);
        return 1;
    }

    fprintf(stdout, "[ok] %d program(s) attached from %s\n", attached, obj_path);

    // If any link couldn't be pinned, keep the process alive so links persist.
    if (unpinned_links > 0) {
        fprintf(stdout, "[hold] %d unpinned link(s); staying alive (Ctrl-C to exit)\n", unpinned_links);
        while (!stop) pause();
    } else {
        // brief delay so stdout flushes in piped runs; pinned links survive regardless
        sleep(1);
    }
    return 0;
}
