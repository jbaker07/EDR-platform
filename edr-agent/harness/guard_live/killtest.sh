#!/bin/bash
# Container side of the "never unkillable" check. Starts the guards with
# self protection enforcing and waits. The host side then tries kill -9 from
# inside the container (refused) and docker rm -f from outside (must work,
# because runc runs outside the scoped cgroup).
# usage: killtest.sh <edr_guard.bpf.o> <guard_live binary>
mountpoint -q /sys/fs/bpf || mount -t bpf bpf /sys/fs/bpf
mountpoint -q /sys/kernel/security || mount -t securityfs securityfs /sys/kernel/security
export EDR_BPF_GUARD_OBJ=$1 EDR_GUARD_SCOPE_CGROUP=/sys/fs/cgroup EDR_GUARD_MODE=enforce
"$2" run 300 &
echo "agent pid $!"
wait
