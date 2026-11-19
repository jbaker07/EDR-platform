#!/bin/bash
# Live scenarios for suspicious_ipc. Run as root in the host pid namespace,
# for example a container started with --privileged --pid=host.
# usage: scenarios.sh <suspicious_ipc.bpf.o> <ipc_live binary> <output dir>
set -u
OBJ=$1
BIN=$2
OUT=$3
mkdir -p "$OUT" /tmp/ipc_scn/bin
cd /tmp/ipc_scn

mountpoint -q /sys/fs/bpf || mount -t bpf bpf /sys/fs/bpf
mountpoint -q /sys/kernel/tracing || mount -t tracefs tracefs /sys/kernel/tracing

step() { echo; echo "=== $* ($(date +%T))"; }

# Waits up to $2 seconds for pattern $1 to show up in file $3.
wait_for() {
    local end=$((SECONDS + $2))
    until grep -q -- "$1" "$3"; do
        [ $SECONDS -ge $end ] && { echo "timed out waiting for '$1' in $3"; return 1; }
        sleep 0.2
    done
}

# Minimal unix socket HTTP server: answers every request with 200 OK.
cat > server.py <<'EOF'
import os, socket, sys
path = sys.argv[1]
if os.path.exists(path):
    os.unlink(path)
s = socket.socket(socket.AF_UNIX)
s.bind(path)
s.listen(16)
while True:
    c, _ = s.accept()
    c.recv(4096)
    c.sendall(b"HTTP/1.0 200 OK\r\nContent-Length: 2\r\n\r\nOK")
    c.close()
EOF

# Connects to a unix socket path. Optional second arg sets comm with prctl first.
cat > client.py <<'EOF'
import ctypes, socket, sys
if len(sys.argv) > 2:
    ctypes.CDLL(None).prctl(15, sys.argv[2].encode(), 0, 0, 0)  # PR_SET_NAME
s = socket.socket(socket.AF_UNIX)
s.connect(sys.argv[1])
s.sendall(b"GET /_ping HTTP/1.0\r\n\r\n")
print("client got:", s.recv(100).split(b"\r\n")[0].decode())
EOF

# Attaches to a pid with PTRACE_SEIZE and holds the attachment.
cat > seize.py <<'EOF'
import ctypes, os, sys, time
libc = ctypes.CDLL(None, use_errno=True)
libc.ptrace.argtypes = [ctypes.c_long, ctypes.c_long, ctypes.c_void_p, ctypes.c_void_p]
libc.ptrace.restype = ctypes.c_long
PTRACE_SEIZE = 0x4206
pid = int(sys.argv[1])
if libc.ptrace(PTRACE_SEIZE, pid, None, None) != 0:
    raise OSError(ctypes.get_errno(), "PTRACE_SEIZE failed")
print(f"seize.py pid {os.getpid()} seized {pid}", flush=True)
time.sleep(int(sys.argv[2]))
EOF

step "kernel"
uname -r

step "(1) listeners: /var/run/docker.sock (watched) and /tmp/ipc_scn/app.sock (not watched)"
python3 server.py /var/run/docker.sock & SRV1=$!
python3 server.py /tmp/ipc_scn/app.sock & SRV2=$!
until [ -S /var/run/docker.sock ] && [ -S /tmp/ipc_scn/app.sock ]; do sleep 0.1; done
ls -l /var/run/docker.sock /tmp/ipc_scn/app.sock

step "start ipc_live watch, ptrace and agent modes"
RUST_LOG=info "$BIN" watch "$OBJ" 120 > "$OUT/watch.out" 2>&1 & WATCH=$!
RUST_LOG=info "$BIN" ptrace 120 > "$OUT/ptrace.out" 2>&1 & PTRACE=$!
EDR_BPF_IPC_OBJ="$OBJ" RUST_LOG=info "$BIN" agent 120 > "$OUT/agent.out" 2>&1 & AGENT=$!
# Counts loaded copies of the program that have a link, so are attached.
attached_copies() {
    local n=0 id
    for id in $(bpftool prog show name trace_runtime_socket_connect 2>/dev/null | grep -o '^[0-9]*'); do
        bpftool link show 2>/dev/null | grep -qw "prog $id" && n=$((n + 1))
    done
    echo $n
}
# watch and agent each load their own copy. Both attached means both are live.
end=$((SECONDS + 30))
until [ "$(attached_copies)" -ge 2 ] || [ $SECONDS -ge $end ]; do sleep 0.2; done
echo "attached copies of trace_runtime_socket_connect: $(attached_copies)"
bpftool link show | grep -B1 -A2 perf_event
# The watch list as the kernel holds it, decoded from one copy of the map.
map_id=$(bpftool map show name watched_sockets | grep -o '^[0-9]*' | head -1)
bpftool -j map dump id "$map_id" | python3 -c '
import json, sys
for e in json.load(sys.stdin):
    key = bytes(int(b, 16) for b in e["key"])
    print("watched_sockets key:", key.rstrip(b"\0").decode(), "(%d bytes)" % len(key))'

step "(2) python3 and curl connect to /var/run/docker.sock: expect FLAGGED"
python3 client.py /var/run/docker.sock
curl -s --unix-socket /var/run/docker.sock http://localhost/_ping; echo " <- curl"

step "(3) a binary named docker connects: expect ALLOWED"
cp /usr/bin/curl bin/docker
bin/docker -s --unix-socket /var/run/docker.sock http://localhost/_ping; echo " <- bin/docker (a curl copy)"

step "(3b) python3 renames itself to docker with prctl: ALLOWED, the comm weakness"
python3 client.py /var/run/docker.sock docker

step "(4) connect to unwatched /tmp/ipc_scn/app.sock and to an abstract socket: expect nothing"
python3 client.py /tmp/ipc_scn/app.sock
python3 -c 'import socket; s=socket.socket(socket.AF_UNIX)
try: s.connect("\0docker.sock")
except OSError as e: print("abstract connect:", e)'

step "(4b) known gap: symlink and relative path to the watched socket: not reported"
ln -sf /var/run/docker.sock /tmp/ipc_scn/dsock
python3 client.py /tmp/ipc_scn/dsock
(cd /var/run && python3 /tmp/ipc_scn/client.py docker.sock)

step "(5) ptrace: python3 PTRACE_SEIZE and a renamed strace copy (expect FLAGGED), plain strace (expect ALLOWED)"
sleep 600 & T1=$!
sleep 600 & T2=$!
sleep 600 & T3=$!
python3 seize.py $T1 60 & SEIZE=$!
cp "$(command -v strace)" bin/notstrace
bin/notstrace -p $T2 -o /dev/null & NOTSTRACE=$!
strace -p $T3 -o /dev/null & STRACE=$!
echo "traced: $T1 $T2 $T3  tracers: seize.py=$SEIZE notstrace=$NOTSTRACE strace=$STRACE"
for pid in $T1 $T2 $T3; do echo "pid $pid $(grep TracerPid /proc/$pid/status)"; done

step "wait for the harness to see everything"
wait_for "FLAGGED pid=.* comm=curl" 15 "$OUT/watch.out"
wait_for "ALLOWED (Docker CLI) pid=.* comm=docker .*exe=/usr/bin/python" 15 "$OUT/watch.out"
wait_for "tracer_pid=$STRACE" 15 "$OUT/ptrace.out"
wait_for "tracer_pid=$SEIZE" 15 "$OUT/ptrace.out"
wait_for "suspicious_ipc_ptrace_attach.*pid=$NOTSTRACE " 15 "$OUT/agent.out"
wait_for "suspicious_ipc_ptrace_attach.*pid=$SEIZE " 15 "$OUT/agent.out"
wait_for "suspicious_ipc_runtime_socket" 15 "$OUT/agent.out"
# Three more agent cycles with the tracers still attached, to show each pair
# is reported once and not every cycle.
last=$(grep -o '^\[agent\] cycle=[0-9]* outputs' "$OUT/agent.out" | tail -1 | grep -o '[0-9]*')
wait_for "^\[agent\] cycle=$((last + 3)) outputs" 10 "$OUT/agent.out"

step "stop"
kill $WATCH $PTRACE $AGENT $SEIZE $NOTSTRACE $STRACE $T1 $T2 $T3 $SRV1 $SRV2 2>/dev/null
wait 2>/dev/null
rm -f /var/run/docker.sock
echo "outputs in $OUT"
