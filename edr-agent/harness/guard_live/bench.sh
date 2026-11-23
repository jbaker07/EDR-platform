#!/bin/bash
# Connect overhead of the guards. Alternates three states so drift on a busy
# host hits all of them alike: no guard loaded, every guard enforcing, and
# loaded with only the socket guard off.
# usage: bench.sh <edr_guard.bpf.o> <guard_live binary> <output dir> <cycles>
set -u
OBJ=$1
BIN=$2
OUT=$3
CYCLES=$4
mkdir -p "$OUT"
mountpoint -q /sys/fs/bpf || mount -t bpf bpf /sys/fs/bpf
mountpoint -q /sys/kernel/security || mount -t securityfs securityfs /sys/kernel/security
export EDR_BPF_GUARD_OBJ=$OBJ EDR_GUARD_SCOPE_CGROUP=/sys/fs/cgroup EDR_GUARD_OFF_FILE=/tmp/bench_off

start() {
    rm -f /tmp/bench_off
    env "$@" "$BIN" run 300 > "$OUT/bench_guard.log" 2>&1 &
    GUARD=$!
    until grep -q "\[run\] ready" "$OUT/bench_guard.log"; do sleep 0.1; done
}
stop() {
    touch /tmp/bench_off
    until grep -q "self protection turned off" "$OUT/bench_guard.log"; do sleep 0.1; done
    kill -TERM $GUARD
    wait $GUARD
}
# Prints "median/min" in ns per connect over 9 rounds of 10,000.
measure() { "$BIN" bench 10000 9 | awk '/median/ {print $3 "/" $5}'; }

echo "cycle none enforce socket_off (ns per connect, median/min of 9 rounds of 10,000)"
for c in $(seq 1 "$CYCLES"); do
    none=$(measure)
    start EDR_GUARD_MODE=enforce
    on=$(measure)
    stop
    start EDR_GUARD_MODE=enforce EDR_GUARD_SOCKET_MODE=off
    off=$(measure)
    stop
    echo "$c $none $on $off"
done
