#!/usr/bin/env bash
set -uo pipefail
LOG="edr_bpf_collect_$(date +%Y%m%d_%H%M%S).log"
exec > >(tee -a "$LOG") 2>&1

hdr(){ echo -e "\n===== $* ====="; }

hdr "BASIC SYSTEM INFO"
echo "Time:            $(date -Is)"
echo "Kernel:          $(uname -a)"
echo "Arch:            $(uname -m)"
echo "User:            $(id -u) ($(id -un))"
echo "Repo root:       $PWD"

hdr "KERNEL EBPF CAPABILITIES"
echo -n "BTF:             "; ls -l /sys/kernel/btf/vmlinux 2>/dev/null || echo "missing"
echo -n "unpriv bpf:      "; cat /proc/sys/kernel/unprivileged_bpf_disabled 2>/dev/null || echo "n/a"
echo -n "kptr_restrict:   "; cat /proc/sys/kernel/kptr_restrict 2>/dev/null || echo "n/a"
echo -n "lockdown:        "; cat /sys/kernel/security/lockdown 2>/dev/null || echo "[none]"
echo "cgroup2 mount:   $(mount | grep ' type cgroup2 ' || true)"
echo "bpf fs mount:    $(mount | grep ' type bpf ' || true)"
echo "tracefs mount:   $(mount | egrep 'tracefs|debugfs' || true)"
echo -n "ulimit -l:       "; ulimit -l 2>/dev/null || echo "n/a"

hdr "TOOLING VERSIONS"
echo -n "cargo: "; cargo -V 2>/dev/null || true
echo -n "rustc: "; rustc -V 2>/dev/null || true
echo -n "clang: "; clang --version 2>/dev/null | head -n1 || true
echo -n "llc:   "; llc --version 2>/dev/null | head -n1 || true
echo -n "bpftool: "; (bpftool version 2>/dev/null || bpftool -V 2>/dev/null || which bpftool || echo "not found")
echo -n "getcap: "; (which getcap 2>/dev/null || echo "not found")
[ -x ./target/debug/forensic_hooks ] && echo "binary caps: $(getcap ./target/debug/forensic_hooks 2>/dev/null || echo 'n/a')" || echo "binary caps: n/a"

hdr "ENV VARS (effective)"
env | egrep '^(EDR_BPF_DIR|EDR_BPF_FILE_OBJ|EDR_BPF_NET_OBJ|RUST_LOG)=' || echo "(none set)"

hdr "HEADERS & vmlinux.h"
echo -n "/usr/include/bpf/bpf_helpers.h: "; [ -f /usr/include/bpf/bpf_helpers.h ] && echo "present" || echo "missing"
echo -n "vmlinux.h in src/ebpf: "; [ -f src/ebpf/vmlinux.h ] && ls -lh src/ebpf/vmlinux.h || echo "missing"
[ -f src/ebpf/vmlinux.h ] && head -n 3 src/ebpf/vmlinux.h || true

hdr "OBJECT FILES (existence + size + mtime)"
ls -lh src/ebpf/*.bpf.o 2>/dev/null || echo "no *.bpf.o in src/ebpf"

hdr "OBJECT INSPECTION — file_access_monitor"
[ -f src/ebpf/file_access_monitor.bpf.o ] && {
  echo "--- sections ---"
  llvm-objdump -h src/ebpf/file_access_monitor.bpf.o 2>/dev/null || readelf -S src/ebpf/file_access_monitor.bpf.o 2>/dev/null
  echo "--- symbols (maps) ---"
  llvm-objdump -t src/ebpf/file_access_monitor.bpf.o 2>/dev/null | egrep '\.maps| events|EVENTS' || true
} || echo "skip (missing)"

hdr "OBJECT INSPECTION — net_flow"
[ -f src/ebpf/net_flow.bpf.o ] && {
  echo "--- sections ---"
  llvm-objdump -h src/ebpf/net_flow.bpf.o 2>/dev/null || readelf -S src/ebpf/net_flow.bpf.o 2>/dev/null
  echo "--- symbols (maps) ---"
  llvm-objdump -t src/ebpf/net_flow.bpf.o 2>/dev/null | egrep '\.maps|net_events|last_fd_map' || true
} || echo "skip (missing)"

hdr "SOURCE REFERENCES TO .bpf.o (include_bytes / hardcoded paths)"
grep -R --line-number -E 'include_bytes_aligned!\(.+\.bpf\.o|/opt/edr-ebpf/.+\.bpf\.o' src 2>/dev/null || echo "none"

hdr "TRACEFS/BPFFS CONTENTS (may need root)"
bpftool map show 2>/dev/null | head -n 50 || echo "map show: permission denied or empty"
bpftool prog show 2>/dev/null | head -n 50 || echo "prog show: permission denied or empty"

hdr "DMESG TAIL (verifier output if any) (may need root)"
dmesg | tail -n 120 2>/dev/null || echo "dmesg not readable"

hdr "OPTIONAL: QUICK LOAD TEST (set TEST_LOAD=1 to enable)"
if [ "${TEST_LOAD:-0}" = "1" ]; then
  echo "(Trying to load/attach file_access_monitor; requires root & tracefs)"
  bpftool prog load src/ebpf/file_access_monitor.bpf.o /sys/fs/bpf/_test_fam 2>&1 | sed 's/^/    /'
  bpftool perf attach pinned /sys/fs/bpf/_test_fam tracepoint syscalls:sys_enter_openat 2>&1 | sed 's/^/    /'
fi

echo -e "\n===== DONE ====="
echo "Log saved to: $LOG"
