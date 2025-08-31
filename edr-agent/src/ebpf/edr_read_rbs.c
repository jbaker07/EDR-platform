// edr_read_rbs.c
// Open every pinned ring buffer under /sys/fs/bpf/edr and print events with a common header.

#define _GNU_SOURCE
#include <stdio.h>
#include <stdint.h>
#include <errno.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>

#include <bpf/libbpf.h>
#include <bpf/bpf.h>
#include <linux/bpf.h>

#ifndef PATH_MAX
#define PATH_MAX 4096
#endif

// Minimal, shared event header we've been using
struct __attribute__((packed)) ev_min {
    uint64_t ts_ns;
    uint32_t type;
    uint32_t nr;
    uint32_t pid;
    uint32_t tgid;
    uint32_t uid;
    int32_t  fd;
};

static const char *pin_root = "/sys/fs/bpf/edr";
static struct ring_buffer *rb = NULL;

static void ts_to_str(uint64_t ns, char *buf, size_t n) {
    struct timespec ts = { .tv_sec = (time_t)(ns / 1000000000ULL),
                           .tv_nsec = (long)(ns % 1000000000ULL) };
    struct tm tm;
    localtime_r(&ts.tv_sec, &tm);
    snprintf(buf, n, "%02d:%02d:%02d.%03ld",
             tm.tm_hour, tm.tm_min, tm.tm_sec, ts.tv_nsec / 1000000L);
}

static int on_sample(void *ctx, void *data, size_t size) {
    if (size < sizeof(struct ev_min)) {
        fprintf(stdout, "[rb] len=%zu bytes\n", size);
        fflush(stdout);
        return 0;
    }
    struct ev_min *e = (struct ev_min *)data;
    char tbuf[64]; ts_to_str(e->ts_ns, tbuf, sizeof tbuf);

    fprintf(stdout, "%s  type=0x%02x  nr=%u  pid=%u tgid=%u uid=%u fd=%d\n",
            tbuf, e->type, e->nr, e->pid, e->tgid, e->uid, e->fd);
    fflush(stdout);
    return 0;
}

static int map_is_ringbuf_fd(int fd) {
    struct bpf_map_info info = {};
    __u32 sz = sizeof(info);
    if (bpf_obj_get_info_by_fd(fd, &info, &sz) < 0) return 0;
    return info.type == BPF_MAP_TYPE_RINGBUF;
}

static int add_ringbuf_from_path(const char *path) {
    int fd = bpf_obj_get(path);
    if (fd < 0) return -errno;
    if (!map_is_ringbuf_fd(fd)) { close(fd); return -EINVAL; }

    int err = 0;
    if (!rb) {
        rb = ring_buffer__new(fd, on_sample, NULL, NULL);
        if (!rb) { err = -errno; close(fd); return err; }
    } else {
        err = ring_buffer__add(rb, fd, on_sample, NULL);
        if (err) { close(fd); return err; }
    }
    // ring_buffer takes ownership of fd (will close on destroy)
    fprintf(stderr, "[reader] listening on %s\n", path);
    return 0;
}

int main(void) {
    libbpf_set_strict_mode(LIBBPF_STRICT_ALL);

    // Walk pin_root and try to add every ringbuf map
    DIR *d = opendir(pin_root);
    if (!d) {
        fprintf(stderr, "open %s failed: %s\n", pin_root, strerror(errno));
        return 2;
    }
    struct dirent *de;
    int found = 0;
    char p[PATH_MAX];
    while ((de = readdir(d))) {
        if (!strcmp(de->d_name, ".") || !strcmp(de->d_name, "..")) continue;
        // Only consider top-level pins (maps usually here)
        snprintf(p, sizeof(p), "%s/%s", pin_root, de->d_name);

        struct stat st;
        if (lstat(p, &st) || !S_ISREG(st.st_mode)) continue;

        if (add_ringbuf_from_path(p) == 0) found++;
    }
    closedir(d);

    if (!found) {
        fprintf(stderr, "No pinned ring buffers found in %s\n", pin_root);
        return 1;
    }

    fprintf(stderr, "Listening… Ctrl-C to stop.\n");
    while (1) {
        int r = ring_buffer__poll(rb, 250 /*ms*/);
        if (r < 0) { fprintf(stderr, "poll err: %d\n", r); break; }
    }
    ring_buffer__free(rb);
    return 0;
}
