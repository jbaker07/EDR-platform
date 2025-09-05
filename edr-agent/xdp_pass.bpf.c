#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
SEC("xdp")
int xdp_pass_main(struct xdp_md *ctx) { return XDP_PASS; }
char LICENSE[] SEC("license") = "GPL";
