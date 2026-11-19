#!/bin/bash
# harness/dns_live/run_live.sh
# Live test for the DNS tunnel monitor. Runs inside a privileged Linux
# container with the edr-agent tree at /src, for example:
#   docker run --rm --privileged --sysctl net.ipv6.conf.all.disable_ipv6=0 \
#     -v "$PWD":/src -v edr-cargo-registry:/usr/local/cargo/registry \
#     -v edr-target-dns:/target -e CARGO_TARGET_DIR=/target \
#     edr-dns-build:local bash /src/harness/dns_live/run_live.sh
# Needs clang, libbpf headers (libbpf-dev), cargo and python3 in the image.
# Build output goes to /target so nothing new lands in the source tree.

set -u
BUILD_DIR=/target/dns_live
HARNESS_DIR=/src/harness/dns_live
mkdir -p $BUILD_DIR

case "$(uname -m)" in
  x86_64) TARGET_ARCH=x86 ;;
  aarch64|arm64) TARGET_ARCH=arm64 ;;
  *) echo "unsupported arch $(uname -m)"; exit 1 ;;
esac

echo "== kernel"
uname -r
ls -l /sys/kernel/btf/vmlinux

echo "== mount bpffs and tracefs"
mountpoint -q /sys/fs/bpf || mount -t bpf bpf /sys/fs/bpf
mountpoint -q /sys/kernel/tracing || mount -t tracefs tracefs /sys/kernel/tracing
mount | grep -E " /sys/fs/bpf | /sys/kernel/tracing "

echo "== compile dns_tunnel_monitor.bpf.c with the src/ebpf/Makefile flags"
cd /src/src/ebpf || exit 1
clang -O2 -g -Wall -Werror -target bpf -D__TARGET_ARCH_$TARGET_ARCH -I. -I/usr/include/bpf \
  -c dns_tunnel_monitor.bpf.c -o $BUILD_DIR/dns_tunnel_monitor.bpf.o || exit 1
ls -l $BUILD_DIR/dns_tunnel_monitor.bpf.o

echo "== build harness"
cd $HARNESS_DIR || exit 1
cargo build --release --locked 2>&1 | tail -3
DNS_LIVE=$CARGO_TARGET_DIR/release/dns_live
[ -x $DNS_LIVE ] || exit 1

echo "== CO-RE check against the 5.15 types (relocate and verify, no attach)"
clang -g -O2 -target bpf -I /src/src/ebpf -c btf_5_15.c -o $BUILD_DIR/btf_5_15.o || exit 1
$DNS_LIVE --check-btf $BUILD_DIR/btf_5_15.o $BUILD_DIR/dns_tunnel_monitor.bpf.o

echo "== CO-RE check against the running kernel's BTF"
$DNS_LIVE --check-btf /sys/kernel/btf/vmlinux $BUILD_DIR/dns_tunnel_monitor.bpf.o

echo "== live run"
LIVE_LOG=$BUILD_DIR/live.log
$DNS_LIVE $BUILD_DIR/dns_tunnel_monitor.bpf.o --stop-on end-of-test.invalid --timeout 90 > $LIVE_LOG 2>&1 &
HARNESS_PID=$!
WAITED=0
until grep -q -E "^(READY|ATTACH FAILED)" $LIVE_LOG; do
  WAITED=$((WAITED + 1))
  if [ $WAITED -gt 100 ]; then echo "harness never became ready"; cat $LIVE_LOG; exit 1; fi
  sleep 0.1
done
grep -q "^READY" $LIVE_LOG || { cat $LIVE_LOG; exit 1; }
python3 $HARNESS_DIR/traffic.py
wait $HARNESS_PID
echo "== harness output"
cat $LIVE_LOG
