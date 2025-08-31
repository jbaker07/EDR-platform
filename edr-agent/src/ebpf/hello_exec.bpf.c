#include "include/vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>
char LICENSE[] SEC("license") = "Dual BSD/GPL";
SEC("tracepoint/sched/sched_process_exec")
int on_exec(void *ctx) { return 0; }
