#!/bin/bash
# Live scenarios for src/guard. Run as root in a privileged container started
# with --pid=host (the kernel's pids are host pids), on a kernel with the bpf
# LSM active. Every guard is scoped to this container's cgroup, so the other
# containers on the same kernel are never judged, and docker rm -f from the
# host can always remove this one.
# usage: scenarios.sh <edr_guard.bpf.o> <guard_live binary> <output dir>
set -u
OBJ=$1
BIN=$2
OUT=$3
W=/tmp/guard_scn
mkdir -p "$OUT" $W/spoof /opt/guardtest
cd $W

mountpoint -q /sys/fs/bpf || mount -t bpf bpf /sys/fs/bpf
mountpoint -q /sys/kernel/tracing || mount -t tracefs tracefs /sys/kernel/tracing
mountpoint -q /sys/kernel/security || mount -t securityfs securityfs /sys/kernel/security

export EDR_BPF_GUARD_OBJ=$OBJ
export EDR_GUARD_SCOPE_CGROUP=/sys/fs/cgroup
export EDR_GUARD_OFF_FILE=$W/off

step() { echo; echo "=== $* ($(date +%T))"; }

# Waits up to $2 seconds for pattern $1 in file $3.
wait_for() {
    local end=$((SECONDS + $2))
    until grep -q -- "$1" "$3"; do
        [ $SECONDS -ge $end ] && { echo "timed out waiting for '$1' in $3"; return 1; }
        sleep 0.1
    done
}

# start_guard <label> <seconds> [VAR=value ...]: starts the harness, waits until it is ready.
start_guard() {
    local label=$1 secs=$2
    shift 2
    rm -f $W/off
    env "$@" "$BIN" run "$secs" > "$OUT/$label.log" 2>&1 &
    GUARD=$!
    wait_for "\[run\] \(ready\|not started\)" 15 "$OUT/$label.log"
    grep -E "^\[(run|setup)\]" "$OUT/$label.log"
}

# stop_guard <label>: throws the off switch, then stops the harness with SIGTERM.
stop_guard() {
    local label=$1
    touch $W/off
    wait_for "self protection turned off\|\[run\] stopped" 5 "$OUT/$label.log" >/dev/null 2>&1
    kill -TERM $GUARD 2>/dev/null
    wait $GUARD 2>/dev/null
    grep -E "^\[run\] stop" "$OUT/$label.log"
    echo "guard programs still loaded: $(bpftool prog show 2>/dev/null | grep -c 'name guard_')"
}

events() { grep "^\[event\]" "$OUT/$1.log" | sed 's/ tags=.*//' ; }

cat > server.py <<'EOF'
import os, socket, sys
path = sys.argv[1]
if os.path.exists(path):
    os.unlink(path)
s = socket.socket(socket.AF_UNIX)
s.bind(path)
s.listen(64)
while True:
    c, _ = s.accept()
    c.recv(4096)
    c.sendall(b"HTTP/1.0 200 OK\r\nContent-Length: 2\r\n\r\nOK")
    c.close()
EOF

# Connects to a unix socket. Optional second arg sets comm with prctl first.
cat > client.py <<'EOF'
import ctypes, socket, sys
if len(sys.argv) > 2:
    ctypes.CDLL(None).prctl(15, sys.argv[2].encode(), 0, 0, 0)  # PR_SET_NAME
comm = open("/proc/self/comm").read().strip()
s = socket.socket(socket.AF_UNIX)
try:
    s.connect(sys.argv[1])
except OSError as e:
    print(f"client comm={comm}: connect({sys.argv[1]}) failed: {e}")
    sys.exit(1)
s.sendall(b"GET /_ping HTTP/1.0\r\n\r\n")
status = s.recv(100).split(b"\r\n")[0].decode()
print(f"client comm={comm}: connected, got {status}")
EOF

# memfd_create, copy /bin/true into it, then fexecve (execveat with AT_EMPTY_PATH).
cat > memfd_exec.py <<'EOF'
import os, sys
fd = os.memfd_create("payload", 0)
with open("/bin/true", "rb") as f:
    os.write(fd, f.read())
try:
    os.execve(fd, ["true"], {})
except OSError as e:
    print(f"memfd exec failed: {e}")
    sys.exit(1)
EOF

# Same fexecve, from a file that was on disk and is now deleted. Not a memfd.
cat > deleted_exec.py <<'EOF'
import os, shutil, sys
shutil.copy("/bin/true", "/tmp/guard_scn/gone")
fd = os.open("/tmp/guard_scn/gone", os.O_RDONLY)
os.unlink("/tmp/guard_scn/gone")
try:
    os.execve(fd, ["true"], {})
except OSError as e:
    print(f"deleted file exec failed: {e}")
    sys.exit(1)
EOF

cat > ptrace_try.py <<'EOF'
import ctypes, os, sys
libc = ctypes.CDLL(None, use_errno=True)
libc.ptrace.argtypes = [ctypes.c_long, ctypes.c_long, ctypes.c_void_p, ctypes.c_void_p]
libc.ptrace.restype = ctypes.c_long
req = {"seize": 0x4206, "attach": 0x10}[sys.argv[1]]
pid = int(sys.argv[2])
if libc.ptrace(req, pid, None, None) != 0:
    e = ctypes.get_errno()
    print(f"ptrace({sys.argv[1]}, {pid}) failed: [Errno {e}] {os.strerror(e)}")
    sys.exit(1)
print(f"ptrace({sys.argv[1]}, {pid}) succeeded (exiting detaches)")
EOF

cat > procmem.py <<'EOF'
import os, sys
pid = sys.argv[1]
print(f"readlink /proc/{pid}/exe -> {os.readlink(f'/proc/{pid}/exe')}")
try:
    open(f"/proc/{pid}/mem", "rb").close()
    print(f"open /proc/{pid}/mem succeeded")
except OSError as e:
    print(f"open /proc/{pid}/mem failed: {e}")
EOF

step "kernel and LSMs"
uname -r
echo "active LSMs: $(cat /sys/kernel/security/lsm)"
echo "pid namespace inode: $(stat -L -c %i /proc/self/ns/pid) (host is 4026531836)"
echo "cgroup: $(cat /proc/self/cgroup), id of /sys/fs/cgroup: $(stat -c %i /sys/fs/cgroup)"

step "setup: runtime socket, test clients"
python3 server.py /var/run/docker.sock & SRV1=$!
python3 server.py $W/app.sock & SRV2=$!
until [ -S /var/run/docker.sock ] && [ -S $W/app.sock ]; do sleep 0.1; done
ls -l /var/run/docker.sock
# The "real client found on the host": a copy of curl installed as /usr/local/bin/docker.
cp /usr/bin/curl /usr/local/bin/docker
# A client added through EDR_GUARD_SOCKET_CLIENTS.
cp /usr/bin/curl /opt/guardtest/allowed-client
# The spoof: python3 copied to a file named docker, so comm is "docker".
cp "$(readlink -f /usr/bin/python3)" $W/spoof/docker
ln -sf /var/run/docker.sock $W/innocent.sock
for f in /usr/local/bin/docker /opt/guardtest/allowed-client /usr/bin/curl $W/spoof/docker "$(readlink -f /usr/bin/python3)"; do
    echo "$(stat -c 'dev=%d ino=%i' "$f") $f"
done
PING="http://localhost/_ping"

# ---------------------------------------------------------------- (1) monitor
step "(1) monitor mode (the default): allowed and reported as would_block"
start_guard monitor 90 EDR_GUARD_SOCKET_CLIENTS=/opt/guardtest/allowed-client
echo "--- curl --unix-socket /var/run/docker.sock"
curl -s -m 3 --unix-socket /var/run/docker.sock $PING; echo " (curl exit $?)"
echo "--- python3 client"
python3 client.py /var/run/docker.sock
echo "--- memfd exec"
python3 memfd_exec.py; echo "memfd_exec.py exit $?"
echo "--- self: ptrace seize, /proc/<agent>/mem"
python3 ptrace_try.py seize $GUARD
python3 procmem.py $GUARD
sleep 1
echo "--- SIGTERM to the agent: allowed in monitor, the harness stops gracefully"
kill -TERM $GUARD; echo "kill -TERM exit $?"
wait $GUARD
grep -E "^\[run\] stop" "$OUT/monitor.log"
echo "--- decisions the agent reported"
events monitor
echo "--- one full TrustEvent"
grep -m1 -A6 "TrustEvent\] Module=Some(\"edr_guard\")" "$OUT/monitor.log"
grep -m3 "^\[sweep\]" "$OUT/monitor.log"

# ---------------------------------------------------------------- (2)-(4) enforce
step "(2) enforce: runtime socket guard"
start_guard enforce 120 EDR_GUARD_MODE=enforce EDR_GUARD_SOCKET_CLIENTS=/opt/guardtest/allowed-client
AGENT=$GUARD
echo "--- curl (not a runtime client)"
curl -sS -m 3 --unix-socket /var/run/docker.sock $PING; echo " (curl exit $?)"
echo "--- python3 client"
python3 client.py /var/run/docker.sock
echo "--- spoof 1: python3 copied to a file named docker (comm=docker)"
$W/spoof/docker client.py /var/run/docker.sock
echo "--- spoof 2: python3 renamed with prctl(PR_SET_NAME, docker)"
python3 client.py /var/run/docker.sock docker
echo "--- symlink to the socket, and a relative path from /run"
curl -sS -m 3 --unix-socket $W/innocent.sock $PING; echo " (curl exit $?)"
(cd /run && python3 $W/client.py docker.sock)
echo "--- allowlisted client: /usr/local/bin/docker (found by name, allowed by inode)"
/usr/local/bin/docker -s -m 3 --unix-socket /var/run/docker.sock $PING; echo " (exit $?)"
echo "--- allowlisted client: /opt/guardtest/allowed-client (EDR_GUARD_SOCKET_CLIENTS)"
/opt/guardtest/allowed-client -s -m 3 --unix-socket /var/run/docker.sock $PING; echo " (exit $?)"
echo "--- a socket no guard watches"
python3 client.py $W/app.sock
echo "--- out of scope: the same curl from a child cgroup is not judged"
mkdir -p /sys/fs/cgroup/guard_outside
sh -c 'echo $$ > /sys/fs/cgroup/guard_outside/cgroup.procs && exec curl -s -m 3 --unix-socket /var/run/docker.sock http://localhost/_ping'; echo " (curl exit $?)"

step "(3) enforce: memfd exec guard"
python3 memfd_exec.py; echo "memfd_exec.py exit $?"
echo "--- exec of a deleted file that is not a memfd (allowed by design)"
python3 deleted_exec.py; echo "deleted_exec.py exit $?"
echo "--- a real file named memfd:fake (has a link, allowed)"
cp /bin/true "$W/memfd:fake" && "$W/memfd:fake"; echo "exit $?"
echo "--- a normal exec"
/bin/true; echo "/bin/true exit $?"

step "(4) enforce: self protection, agent pid $AGENT"
for sig in KILL TERM STOP INT HUP; do
    kill -$sig $AGENT; echo "kill -$sig exit $?"
done
kill -0 $AGENT; echo "kill -0 (alive check) exit $?"
python3 ptrace_try.py attach $AGENT
python3 ptrace_try.py seize $AGENT
python3 procmem.py $AGENT
echo "--- bpftool against the agent's own objects"
bpftool map update name guard_config key 2 0 0 0 value 0 0 0 0 0 0 0 0; echo "bpftool map update exit $?"
bpftool prog show name guard_task_kill; echo "bpftool prog show exit $?"
bpftool link show 2>&1 | grep -c "Operation not permitted" | sed 's/^/bpftool link show errors: /'
echo "--- still alive after all of that: $(kill -0 $AGENT 2>/dev/null && echo yes || echo no)"
sleep 1
echo "--- decisions the agent reported"
events enforce
grep "suppressed\|dropped" "$OUT/enforce.log" | head -3
echo "--- off switch: touch $W/off, then SIGTERM"
stop_guard enforce
grep "self protection turned off" "$OUT/enforce.log"

# ---------------------------------------------------------------- (5) off
step "(5) EDR_GUARD_MODE=off: nothing loaded, nothing blocked"
start_guard off 30 EDR_GUARD_MODE=off
curl -s -m 3 --unix-socket /var/run/docker.sock $PING; echo " (curl exit $?)"
python3 memfd_exec.py; echo "memfd_exec.py exit $?"
echo "guard programs loaded: $(bpftool prog show 2>/dev/null | grep -c 'name guard_')"

step "(5b) per guard off: EDR_GUARD_MODE=enforce EDR_GUARD_SOCKET_MODE=off EDR_GUARD_SELF_MODE=off"
start_guard partial 60 EDR_GUARD_MODE=enforce EDR_GUARD_SOCKET_MODE=off EDR_GUARD_SELF_MODE=off
curl -s -m 3 --unix-socket /var/run/docker.sock $PING; echo " (curl exit $?, socket guard off)"
python3 memfd_exec.py; echo "memfd_exec.py exit $? (memfd guard still enforcing)"
kill -TERM $GUARD; echo "kill -TERM exit $? (self protection off)"
wait $GUARD
grep -E "^\[run\] stop" "$OUT/partial.log"
events partial

# ---------------------------------------------------------------- (6) overhead
step "(6) overhead: 10,000 connect() calls to a socket no guard watches, 7 rounds each"
for pass in 1 2; do
    echo "--- pass $pass, no guard loaded"
    "$BIN" bench 10000 7 | tail -1
    start_guard bench_on_$pass 120 EDR_GUARD_MODE=enforce >/dev/null
    echo "--- pass $pass, all guards enforcing"
    "$BIN" bench 10000 7 | tail -1
    stop_guard bench_on_$pass >/dev/null
    start_guard bench_off_$pass 120 EDR_GUARD_MODE=enforce EDR_GUARD_SOCKET_MODE=off >/dev/null
    echo "--- pass $pass, loaded, socket guard off (others enforcing)"
    "$BIN" bench 10000 7 | tail -1
    stop_guard bench_off_$pass >/dev/null
done

step "cleanup"
kill $SRV1 $SRV2 2>/dev/null
rmdir /sys/fs/cgroup/guard_outside 2>/dev/null
rm -f /var/run/docker.sock /usr/local/bin/docker
echo "guard programs still loaded: $(bpftool prog show 2>/dev/null | grep -c 'name guard_')"
echo "done"
