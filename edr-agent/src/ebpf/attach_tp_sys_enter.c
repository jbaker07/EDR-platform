#include <signal.h>
#include <stdio.h>
#include <errno.h>
#include <string.h>
#include <unistd.h>
#include <sys/resource.h>
#include <sys/statfs.h>
#include <linux/magic.h>
#include <bpf/libbpf.h>
#include "raw_syscalls_catcher.skel.h"

static volatile sig_atomic_t stopf;
static void on_sig(int sig){ (void)sig; stopf = 1; }

static int bump_memlock(void){
  struct rlimit r = { RLIM_INFINITY, RLIM_INFINITY };
  return setrlimit(RLIMIT_MEMLOCK, &r);
}
static int ensure_bpffs(void){
  struct statfs s = {0};
  if (statfs("/sys/fs/bpf",&s)==0 && (unsigned)s.f_type==BPF_FS_MAGIC) return 0;
  fprintf(stderr,"bpffs not mounted at /sys/fs/bpf\n");
  return -1;
}

int main(void){
  libbpf_set_strict_mode(LIBBPF_STRICT_ALL);
  signal(SIGINT,on_sig); signal(SIGTERM,on_sig);
  if (ensure_bpffs()){ return 1; }
  if (bump_memlock()){ perror("setrlimit MEMLOCK"); return 2; }

  struct raw_syscalls_catcher_bpf *skel = raw_syscalls_catcher_bpf__open_and_load();
  if (!skel){ fprintf(stderr,"open_and_load failed\n"); return 3; }

  int err = raw_syscalls_catcher_bpf__attach(skel);
  if (err){ fprintf(stderr,"attach failed: %s (%d)\n", strerror(-err), -err); return 4; }

  (void)system("mkdir -p /sys/fs/bpf/edr /sys/fs/bpf/edr/maps");

  int perr = bpf_link__pin(skel->links.tp_sys_enter, "/sys/fs/bpf/edr/tp_sys_enter.link");
  if (perr == 0){
    puts("Pinned link at /sys/fs/bpf/edr/tp_sys_enter.link");
  } else if (perr == -EPERM){
    fprintf(stderr,"WARN: kernel forbids pinning links (EPERM); running unpinned.\n");
  } else {
    fprintf(stderr,"pin link failed: %s (%d)\n", strerror(-perr), -perr);
  }

  (void)bpf_object__pin_maps(skel->obj, "/sys/fs/bpf/edr/maps");

  puts("OK: attached. Ctrl-C to exit.");
  while (!stopf) sleep(1);
  return 0;
}
