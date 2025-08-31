#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <sys/stat.h>
#include <bpf/libbpf.h>
#include <bpf/bpf.h>

static void ensure_dir(const char *p) { mkdir(p, 0755); }

int main(void) {
    const char *obj_path = "wx_exec.bpf.o";
    const char *pin_dir  = "/sys/fs/bpf/edr";
    const char *rb_pin   = "/sys/fs/bpf/edr/wx_events_rb";

    ensure_dir("/sys/fs/bpf");
    ensure_dir(pin_dir);

    struct bpf_object *obj = NULL;
    struct bpf_program *prog;
    struct bpf_link *link;
    int err;

    obj = bpf_object__open_file(obj_path, NULL);
    if (libbpf_get_error(obj)) {
        fprintf(stderr, "open %s: %s\n", obj_path, strerror(-libbpf_get_error(obj)));
        return 1;
    }

    if ((err = bpf_object__load(obj))) {
        fprintf(stderr, "load %s: %s\n", obj_path, strerror(-err));
        return 1;
    }

    /* Attach all programs (libbpf infers type from SEC("...")) */
    bpf_object__for_each_program(prog, obj) {
        link = bpf_program__attach(prog);
        if (libbpf_get_error(link)) {
            err = -libbpf_get_error(link);
            const char *sec = bpf_program__section_name(prog);
            fprintf(stderr, "attach %s failed: %s\n", sec ? sec : "?", strerror(-err));
            continue;
        }
        /* Pin each link under /sys/fs/bpf/edr/ using its section name */
        char path[512];
        const char *sec = bpf_program__section_name(prog);
        snprintf(path, sizeof(path), "%s/wx_%s.link", pin_dir, sec ? sec : "prog");
        /* sanitize slashes for pin path */
        for (char *p = path; *p; ++p) if (*p == '/') *p = '_';
        err = bpf_link__pin(link, path);
        if (err) {
            fprintf(stderr, "pin link %s failed: %s\n", path, strerror(-err));
        } else {
            printf("Pinned link: %s\n", path);
        }
    }

    /* Pin the first ring buffer map we find so a reader can open it */
    struct bpf_map *m;
    bool pinned_rb = false;
    bpf_object__for_each_map(m, obj) {
        if (bpf_map__type(m) == BPF_MAP_TYPE_RINGBUF) {
            if ((err = bpf_map__pin(m, rb_pin))) {
                fprintf(stderr, "pin ringbuf %s failed: %s\n", rb_pin, strerror(-err));
            } else {
                printf("Pinned ringbuf: %s\n", rb_pin);
                pinned_rb = true;
            }
            break;
        }
    }

    if (!pinned_rb) {
        fprintf(stderr, "No ringbuf map found in %s\n", obj_path);
        return 2;
    }

    puts("wx_exec attached and pinned. You can now run your ringbuf reader.");
    return 0;
}
