#include <linux/bpf.h>
#include <linux/ptrace.h>
#include <bpf/bpf_helpers.h>

char LICENSE[] SEC("license") = "GPL";

// Tracepoint: syscalls:sys_enter_openat
struct sys_enter_openat_args {
    __u64 unused;
    __u64 dfd;
    const char *filename;
    __u64 flags;
    __u64 mode;
};

// You must match the name exactly as declared in the kernel
SEC("tracepoint/syscalls/sys_enter_openat")
int trace_usb_open(struct sys_enter_openat_args *ctx) {
    char fname[256] = {};
    bpf_probe_read_user_str(fname, sizeof(fname), ctx->filename);

    if (__builtin_memcmp(fname, "/dev/bus/usb", 14) == 0) {
        bpf_printk("🔌 USB access detected: %s\n", fname);
    }

    return 0;
}
