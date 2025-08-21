// ebpf/usb_monitor_ebpf.c
// CO-RE tracepoint for sys_enter_openat to spot /dev/bus/usb access.
// Avoid UAPI <linux/*> includes to fix asm/types.h errors.

#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
#include <bpf/bpf_core_read.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

static __always_inline int handle_openat(struct trace_event_raw_sys_enter *ctx)
{
    const char *user_path = (const char *)ctx->args[1]; // openat(dfd, pathname, flags, mode)
    if (!user_path)
        return 0;

    char path[128] = {};
    int n = bpf_probe_read_user_str(path, sizeof(path), user_path);
    if (n <= 0)
        return 0;

    const char prefix[] = "/dev/bus/usb";
    if (__builtin_memcmp(path, prefix, sizeof(prefix) - 1) == 0) {
        bpf_printk("usb_monitor: access to %s\n", path);
    }
    return 0;
}

SEC("tracepoint/syscalls/sys_enter_openat")
int tp_enter_openat(struct trace_event_raw_sys_enter *ctx)
{
    return handle_openat(ctx);
}

// Optional: catch openat2 on newer kernels; harmless if not present at attach time.
SEC("tracepoint/syscalls/sys_enter_openat2")
int tp_enter_openat2(struct trace_event_raw_sys_enter *ctx)
{
    return handle_openat(ctx);
}
