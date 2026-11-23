// harness/guard_live/btf_5_15.c
// Turns the repo's 5.15 vmlinux.h back into a small BTF blob that holds the
// types the guard programs read. guard_live check-btf relocates the programs
// against it, which shows every field they use resolves on 5.15.
// Build: clang -g -O2 -target bpf -I ../../src/ebpf -c btf_5_15.c -o btf_5_15.o
#include "vmlinux.h"

struct unix_sock btf_unix_sock;
struct unix_address btf_unix_address;
struct sockaddr_un btf_sockaddr_un;
struct linux_binprm btf_linux_binprm;
struct file btf_file;
struct inode btf_inode;
struct super_block btf_super_block;
struct dentry btf_dentry;
struct task_struct btf_task_struct;
struct mm_struct btf_mm_struct;
union bpf_attr btf_bpf_attr;
