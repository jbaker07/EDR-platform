// harness/dns_live/btf_5_15.c
// Turns the repo's 5.15 vmlinux.h back into a small BTF blob that holds the
// types the DNS probes read. dns_live --check-btf relocates the probes
// against it, which shows the 5.15 path (iov, no ITER_UBUF) resolves.
// Build: clang -g -O2 -target bpf -I ../../src/ebpf -c btf_5_15.c -o btf_5_15.o
#include "vmlinux.h"

struct iov_iter btf_iov_iter;
enum iter_type btf_iter_type;
struct iovec btf_iovec;
struct msghdr btf_msghdr;
struct sock btf_sock;
struct sockaddr_in btf_sockaddr_in;
struct sockaddr_in6 btf_sockaddr_in6;
struct pt_regs btf_pt_regs;
