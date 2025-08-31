#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <errno.h>
#include <unistd.h>
#include <time.h>
#include <limits.h>
#include <stdlib.h>
#include <bpf/libbpf.h>
#include <bpf/bpf.h>

struct __attribute__((packed)) ev_min {
    uint64_t ts_ns;
    uint32_t type;
    uint32_t nr;
    uint32_t pid;
    uint32_t tgid;
    uint32_t uid;
    int32_t  fd;
};

static const char* type_str(uint32_t t){
    switch(t){
        case 0x0a: return "OPEN/OPENAT";
        case 0x12: return "CLOSE";
        case 0x15: return "CONNECT";
        case 0x1e: return "EXECVE";
        default:   return "OTHER";
    }
}

static const char* nr_str(uint32_t nr){
    switch(nr){
        case 56:  return "openat";
        case 57:  return "close";
        case 203: return "connect";
        case 221: return "execve";
        default:  return "?";
    }
}

static void ts_to_str(uint64_t ns, char *out, size_t n){
    struct timespec ts = { .tv_sec = (time_t)(ns/1000000000ULL), .tv_nsec = (long)(ns%1000000000ULL) };
    struct tm tm; localtime_r(&ts.tv_sec, &tm);
    snprintf(out, n, "%02d:%02d:%02d.%03ld", tm.tm_hour, tm.tm_min, tm.tm_sec, ts.tv_nsec/1000000L);
}

static int read_one_comm(pid_t id, char *buf, size_t n){
    char p[64]; snprintf(p,sizeof(p),"/proc/%d/comm", id);
    FILE *f = fopen(p,"r");
    if(!f) return -1;
    if(!fgets(buf,(int)n,f)) { fclose(f); return -1; }
    size_t L=strlen(buf); if(L && buf[L-1]=='\n') buf[L-1]=0;
    fclose(f); return 0;
}

static void read_comm(uint32_t pid, uint32_t tgid, char *buf, size_t n){
    if(read_one_comm(tgid, buf, n) == 0) return;
    if(read_one_comm(pid,  buf, n) == 0) return;
    strncpy(buf,"?",n); buf[n-1]=0;
}

static void read_fd_path(uint32_t tgid, int fd, char *buf, size_t n){
    if(fd < 0){ buf[0]=0; return; }
    char link[64]; snprintf(link,sizeof(link),"/proc/%u/fd/%d",tgid,fd);
    ssize_t r = readlink(link, buf, n-1);
    if(r < 0){ buf[0]=0; return; }
    buf[r]=0;
}

static int on_sample(void *ctx, void *data, size_t size){
    (void)ctx;
    if(size < sizeof(struct ev_min)) return 0;
    struct ev_min *e = (struct ev_min*)data;

    // Hide open/close noise unless SHOW_OPEN_CLOSE=1
    static int hide_open_close = -1;
    if (hide_open_close == -1) hide_open_close = (getenv("SHOW_OPEN_CLOSE") == NULL);
    if (hide_open_close && (e->nr == 56 || e->nr == 57)) return 0;

    char tbuf[32]; ts_to_str(e->ts_ns, tbuf, sizeof tbuf);
    char comm[64]; read_comm(e->pid, e->tgid, comm, sizeof comm);
    char path[PATH_MAX]; read_fd_path(e->tgid, e->fd, path, sizeof path);

    printf("%s  type=%s(0x%02x)  nr=%s(%u)  pid=%u tgid=%u uid=%u fd=%d  comm=%s\n",
           tbuf, type_str(e->type), e->type, nr_str(e->nr), e->nr,
           e->pid, e->tgid, e->uid, e->fd, comm);
    if(path[0]) printf("    path=%s\n", path);
    fflush(stdout);
    return 0;
}

static int open_map_fd(const char *arg){
    const char *candidates[] = {
        arg,
        "/sys/fs/bpf/edr/edr_events_rb",
        "/sys/fs/bpf/edr/maps/edr_events_rb",
        "/sys/fs/bpf/edr_events_rb",
        NULL
    };
    for (int i=0; candidates[i]; i++){
        if(!candidates[i]) continue;
        int fd = bpf_obj_get(candidates[i]);
        if (fd >= 0) return fd;
    }
    return -1;
}

int main(int argc, char **argv){
    const char *arg = (argc > 1) ? argv[1] : NULL;
    int map_fd = open_map_fd(arg);
    if (map_fd < 0) {
        fprintf(stderr,"Unable to open ringbuf map (try passing its path). Last error: %s\n", strerror(errno));
        fprintf(stderr,"Examples:\n  sudo %s /sys/fs/bpf/edr/edr_events_rb\n  sudo %s /sys/fs/bpf/edr/maps/edr_events_rb\n", argv[0], argv[0]);
        return 1;
    }

    struct ring_buffer *rb = ring_buffer__new(map_fd, on_sample, NULL, NULL);
    if (!rb) { fprintf(stderr, "ring_buffer__new failed: %s\n", strerror(errno)); close(map_fd); return 2; }

    puts("Listening… Ctrl-C to stop.");
    for (;;) {
        int r = ring_buffer__poll(rb, 500);
        if (r == -EINTR) break;
        if (r < 0) { fprintf(stderr, "poll err: %d\n", r); break; }
    }
    ring_buffer__free(rb);
    close(map_fd);
    return 0;
}
