#!/bin/bash
# Overhead of the runtime shields: exec, open and connect latency with no agent, with
# the agent but no shield programs (sensors and guard only), then with the shield
# programs and 0, 100 and 10,000 shield rows of each kind. Run as root on a
# throwaway Linux machine whose kernel has bpf in its LSM list:
#
#   sudo bash scripts/ci/shield_bench.sh <dir with the .bpf.o files and edr_attach_any> <agent binary>
#
# The rows are real signed shields (a throwaway test key), posted through the API.
# None of them matches what the benchmark does, so the numbers are the cost of the
# hook and the map lookup on every call, the price every program on the host pays.
#   exec  posix_spawn of a copy of /bin/true, then waitpid
#   open  open and close of one file
#   udp   connect() of a UDP socket, which only runs the connect hook and a route lookup
#   tcp   connect() to a listener on 127.0.0.1, the full handshake
# Each figure is the median of ROUNDS rounds. Results go to $OUT/bench.tsv.
set -u
OBJ_DIR=$(cd "$1" && pwd)
AGENT_BIN=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
REPO=$(cd "$(dirname "$0")/../.." && pwd)
OUT=${OUT:-$PWD/bench-out}
ROUNDS=${ROUNDS:-9}
API=http://127.0.0.1:8080
PIN=/sys/fs/bpf/edr
SPIN=/sys/fs/bpf/edr_shield
B=/tmp/shieldbench
mkdir -p "$OUT" "$B"
mountpoint -q /sys/fs/bpf || mount -t bpf bpf /sys/fs/bpf
mountpoint -q /sys/kernel/tracing || mount -t tracefs tracefs /sys/kernel/tracing 2>/dev/null
mountpoint -q /sys/kernel/security || mount -t securityfs securityfs /sys/kernel/security 2>/dev/null
LSM=$(cat /sys/kernel/security/lsm 2>/dev/null || echo unknown)
echo ",$LSM," | grep -q ",bpf," || { echo "skipped: bpf is not in the LSM list"; exit 0; }
python3 -c 'import cryptography' 2>/dev/null || { echo "needs python3-cryptography to sign 30,000 documents quickly"; exit 1; }

cat > "$B/bench.c" <<'C'
#define _GNU_SOURCE
#include <arpa/inet.h>
#include <fcntl.h>
#include <netinet/in.h>
#include <pthread.h>
#include <spawn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/wait.h>
#include <time.h>
#include <unistd.h>
extern char **environ;
static double now(void) { struct timespec t; clock_gettime(CLOCK_MONOTONIC, &t); return t.tv_sec * 1e9 + t.tv_nsec; }
static int lfd;
static void *acceptor(void *x) { (void)x; for (;;) { int c = accept(lfd, NULL, NULL); if (c >= 0) close(c); } return NULL; }
static int cmp(const void *a, const void *b) { double x = *(double *)a, y = *(double *)b; return (x > y) - (x < y); }
int main(int argc, char **argv) {
    const char *what = argv[1]; long n = atol(argv[2]); int rounds = atoi(argv[3]);
    double per[64];
    struct sockaddr_in udp = { .sin_family = AF_INET, .sin_port = htons(9) };
    inet_pton(AF_INET, "203.0.113.10", &udp.sin_addr);
    struct sockaddr_in lo = { .sin_family = AF_INET };
    inet_pton(AF_INET, "127.0.0.1", &lo.sin_addr);
    if (!strcmp(what, "tcp")) {
        lfd = socket(AF_INET, SOCK_STREAM, 0); int one = 1;
        setsockopt(lfd, SOL_SOCKET, SO_REUSEADDR, &one, sizeof one);
        bind(lfd, (void *)&lo, sizeof lo); listen(lfd, 4096);
        socklen_t l = sizeof lo; getsockname(lfd, (void *)&lo, &l);
        pthread_t t; pthread_create(&t, NULL, acceptor, NULL);
    }
    int us = socket(AF_INET, SOCK_DGRAM, 0);
    for (int r = 0; r < rounds; r++) {
        double t0 = now();
        for (long i = 0; i < n; i++) {
            if (!strcmp(what, "exec")) {
                pid_t p; char *av[] = { argv[4], NULL };
                if (posix_spawn(&p, argv[4], NULL, NULL, av, environ) == 0) waitpid(p, NULL, 0);
            } else if (!strcmp(what, "open")) {
                int fd = open(argv[4], O_RDONLY); if (fd >= 0) close(fd);
            } else if (!strcmp(what, "udp")) {
                connect(us, (void *)&udp, sizeof udp);
            } else {
                int s = socket(AF_INET, SOCK_STREAM, 0);
                struct linger lg = { 1, 0 }; setsockopt(s, SOL_SOCKET, SO_LINGER, &lg, sizeof lg);
                connect(s, (void *)&lo, sizeof lo); close(s);
            }
        }
        per[r] = (now() - t0) / n;
    }
    qsort(per, rounds, sizeof per[0], cmp);
    printf("%.0f\n", per[rounds / 2]);
    return 0;
}
C
gcc -O2 -pthread -o "$B/bench" "$B/bench.c" || exit 1
cp /bin/true "$B/true"; echo data > "$B/file"
ip link add edrb0 type dummy 2>/dev/null; ip addr add 203.0.113.10/32 dev edrb0 2>/dev/null; ip link set edrb0 up

printf 'setup\texec_ns\topen_ns\tudp_connect_ns\ttcp_connect_ns\tagent_rss_kb\n' > "$OUT/bench.tsv"
AGENT=0
measure() { # measure <label>
  local e o u t r
  e=$("$B/bench" exec 1000 "$ROUNDS" "$B/true")
  o=$("$B/bench" open 200000 "$ROUNDS" "$B/file")
  u=$("$B/bench" udp 200000 "$ROUNDS")
  t=$("$B/bench" tcp 20000 "$ROUNDS")
  r=$( [ "$AGENT" -gt 1 ] && awk '/VmRSS/ {print $2}' /proc/$AGENT/status || echo -)
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$1" "$e" "$o" "$u" "$t" "$r" | tee -a "$OUT/bench.tsv"
}

echo "== no agent (no LSM programs from this project)"
measure "no agent"

echo "== start the agent"
RUN=$(mktemp -d /tmp/edr_bench_run.XXXXXX)
mkdir -p "$RUN/src" "$RUN/config"
cp -a "$REPO/edr-agent/json_files" "$RUN/json_files"; cp -a "$REPO/edr-agent/ui" "$RUN/ui"
ln -s "$REPO/edr-agent/src/rules.json" "$RUN/src/rules.json"
( umask 077; openssl genpkey -algorithm ed25519 -out "$RUN/key.pem" 2>/dev/null
  python3 -c 'import secrets; print(secrets.token_urlsafe(32))' > "$RUN/agent_token" )
openssl pkey -in "$RUN/key.pem" -pubout -out "$RUN/config/policy_pub.pem"
printf '{"policy":{"serial":1,"guard_mode":"monitor","redact_command_lines":true,"response_actions":["kill_proc"],"management_ports":[22],"logging":{"level":"warn"},"shields":{"max_lifetime_sec":7200,"max_active":32767}}}\n' > "$RUN/policy.json"
python3 "$REPO/scripts/policy_signer.py" sign "$RUN/policy.json" --key "$RUN/key.pem" --pub "$RUN/config/policy_pub.pem" > /dev/null
rm -rf "$PIN" "$SPIN"
for OBJ in raw_syscalls_catcher proc_lifecycle net_flow wx_exec tcp_state file_access_monitor; do
  "$OBJ_DIR/edr_attach_any" -p "$PIN" "$OBJ_DIR/$OBJ.bpf.o" > /dev/null 2>&1
done
cd "$RUN"
start_agent() { # start_agent <log name> <shield object path>
  env HOSTNAME=bench EDR_AGENT_TOKEN_FILE="$RUN/agent_token" PB_DIR="$REPO/edr-agent/playbooks" \
      EDR_POLICY_PUBKEY="$RUN/config/policy_pub.pem" EDR_GUARD_MODE=enforce ${SCOPE:+EDR_GUARD_SCOPE_CGROUP=$SCOPE} \
      EDR_BPF_GUARD_OBJ="$OBJ_DIR/edr_guard.bpf.o" EDR_BPF_SHIELD_OBJ="$2" \
      RUST_LOG=warn "$AGENT_BIN" > "$OUT/agent_$1.log" 2>&1 &
  AGENT=$!
  for _ in $(seq 1 150); do curl -s -o /dev/null "$API/healthz" && break; sleep 0.2; done
  sleep 3
  grep -aE "guard: prevention|^shields:" "$OUT/agent_$1.log" | cut -c1-200
}
stop_agent() {
  mkdir -p /run/edr-guard && touch /run/edr-guard/off; sleep 1
  kill -INT $AGENT 2>/dev/null; for _ in $(seq 1 50); do kill -0 $AGENT 2>/dev/null || break; sleep 0.2; done; kill -9 $AGENT 2>/dev/null
  rm -f /run/edr-guard/off; AGENT=0; sleep 1
}

echo "== the agent with its sensors and the guard, no shield programs"
start_agent noshield /nonexistent/edr_shield.bpf.o
measure "agent, no shield programs"
stop_agent
rm -rf "$RUN/state" "$RUN/logs"

echo "== the agent with the shield programs"
start_agent shields "$OBJ_DIR/edr_shield.bpf.o"

# fill <rows per kind>: signs and posts shields until each kind has that many rows.
fill() {
  python3 - "$1" "$RUN" "$B" <<'PY'
import http.client, json, os, random, sys, time
from cryptography.hazmat.primitives.serialization import load_pem_private_key
want, run, b = int(sys.argv[1]), sys.argv[2], sys.argv[3]
key = load_pem_private_key(open(f"{run}/key.pem", "rb").read(), None)
token = open(f"{run}/agent_token").read().strip()
conn = http.client.HTTPConnection("127.0.0.1", 8080, timeout=30)
def get():
    conn.request("GET", "/shields", headers={"Authorization": "Bearer " + token})
    return json.loads(conn.getresponse().read())
have = get()["rows"]
dev = os.stat(b + "/file").st_dev
now = int(time.time())
sent = bad = 0
for kind in ("exec_deny", "file_deny", "connect_deny"):
    for i in range(have[kind], want):
        if kind == "connect_deny":
            match = {"cidr": f"198.{18 + (i >> 16)}.{(i >> 8) & 255}.{i & 255}", "port": 9999}
        else:
            # Inode numbers no file has, on the benchmark file's device.
            match = {"dev": dev, "ino": 4_000_000_000 + i + (0 if kind == "exec_deny" else 50_000)}
        doc = {"shield": {"id": f"bench.{kind}.{i}", "serial": 1, "kind": kind, "match": match, "scope": {},
                          "mode": "enforce", "not_before": now, "expires_at": now + 7000, "reason": "overhead benchmark"}}
        text = json.dumps(doc)
        body = json.dumps({"document": text, "signature": key.sign(text.encode()).hex()})
        conn.request("POST", "/shields", body=body, headers={"Authorization": "Bearer " + token, "Content-Type": "application/json"})
        r = conn.getresponse(); r.read()
        sent += 1; bad += r.status != 201
print(f"posted {sent}, refused {bad}, rows now {get()['rows']}")
PY
}

echo "== agent running, 0 shield rows"
measure "shields, 0 rows"
echo "== 100 rows of each kind"
fill 100
measure "shields, 100 rows of each kind"
echo "== 10,000 rows of each kind"
# The agent's memory right before and after the fill, with no benchmark load in
# between: what 29,700 more shields cost the agent. The other RSS figures also hold
# the events the benchmark itself makes (every open is a sensor event).
sleep 5; R0=$(awk '/VmRSS/ {print $2}' /proc/$AGENT/status)
T0=$(date +%s); fill 10000; echo "filled in $(( $(date +%s) - T0 )) s"
sleep 5; R1=$(awk '/VmRSS/ {print $2}' /proc/$AGENT/status)
echo "agent RSS before the fill ${R0} kB, after ${R1} kB" | tee -a "$OUT/agent_memory.txt"
measure "shields, 10,000 rows of each kind"
grep -E "VmHWM|VmRSS" /proc/$AGENT/status | tr -s ' ' | tee -a "$OUT/agent_memory.txt"

echo "== stop"
stop_agent
rm -rf "$PIN" "$SPIN"
ip link del edrb0 2>/dev/null
chmod -R a+rX "$OUT"
column -t -s "$(printf '\t')" "$OUT/bench.tsv"
