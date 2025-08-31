#include <bpf/bpf.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>
#include <unistd.h>
#include <bpf/libbpf.h>

static int on_sample(void *ctx, void *data, size_t size) {
    (void)ctx;
    printf("event size=%zu\n", size);
    size_t n = size < 64 ? size : 64;
    const unsigned char *p = data;
    for (size_t i = 0; i < n; i++) {
        if (i % 16 == 0) printf("  ");
        printf("%02x ", p[i]);
        if (i % 16 == 15 || i == n - 1) printf("\n");
    }
    fflush(stdout);
    return 0;
}

int main(void) {
    int map_fd = bpf_obj_get("/sys/fs/bpf/edr/maps/edr_events_rb");
    if (map_fd < 0) { perror("bpf_obj_get edr_events_rb"); return 1; }

    struct ring_buffer *rb = ring_buffer__new(map_fd, on_sample, NULL, NULL);
    if (!rb) { fprintf(stderr, "ring_buffer__new failed: %s\n", strerror(errno)); return 2; }

    puts("Waiting for events... (run `ls`, `cat`, `curl` etc in another shell)");
    while (1) {
        int r = ring_buffer__poll(rb, 1000);
        if (r == -EINTR) break;
        if (r < 0) { fprintf(stderr, "poll err: %d\n", r); break; }
    }
    ring_buffer__free(rb);
    close(map_fd);
    return 0;
}
