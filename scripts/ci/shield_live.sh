#!/bin/bash
# Live test of the runtime shields, signed shield documents, live policy reload and
# expiring firewall rules. Run as root on a throwaway Linux machine whose kernel has
# bpf in its LSM list (a privileged container on such a kernel works too).
#
#   sudo bash scripts/ci/shield_live.sh <dir with the .bpf.o files and edr_attach_any> <agent binary>
#
# For each shield kind it shows: the attack is refused while the agent runs, normal
# work is untouched, monitor mode reports would_block and lets the action through,
# and the block ends on time while the agent is stopped with SIGSTOP, because the
# kernel checks each row's deadline. It also shows that tampered, replayed, over
# lifetime, never-block and over the limit shields are refused, that a newer signed
# policy applies live (a guard allowlist changes in the kernel) while an older one is
# refused, and that playbook hits propose shields.
#
# Everything uses a throwaway Ed25519 key made here, never a real signing key. Test
# traffic goes to documentation addresses (RFC 5737) on a dummy interface. Results go
# to $OUT (default ./shield-out), with one line per check in checks.txt.
# SCOPE=<cgroup v2 dir> limits the guard and every shield to that subtree (the live
# container run uses /sys/fs/cgroup, the container's own cgroup).
set -u
OBJ_DIR=$(cd "$1" && pwd)
AGENT_BIN=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
REPO=$(cd "$(dirname "$0")/../.." && pwd)
OUT=${OUT:-$PWD/shield-out}
API=http://127.0.0.1:8080
PIN=/sys/fs/bpf/edr
SPIN=/sys/fs/bpf/edr_shield
T=/tmp/shieldtest
CG=/sys/fs/cgroup
mkdir -p "$OUT"
: > "$OUT/checks.txt"
FAIL=0
check() { # check <name> <ok:0|1> <detail>
  if [ "$2" = 0 ]; then echo "PASS $1: $3" | tee -a "$OUT/checks.txt"; else echo "FAIL $1: $3" | tee -a "$OUT/checks.txt"; FAIL=1; fi
}
step() { echo; echo "== $*"; }

mountpoint -q /sys/fs/bpf || mount -t bpf bpf /sys/fs/bpf
mountpoint -q /sys/kernel/tracing || mount -t tracefs tracefs /sys/kernel/tracing 2>/dev/null
mountpoint -q /sys/kernel/debug || mount -t debugfs debugfs /sys/kernel/debug 2>/dev/null
mountpoint -q /sys/kernel/security || mount -t securityfs securityfs /sys/kernel/security 2>/dev/null
LSM=$(cat /sys/kernel/security/lsm 2>/dev/null || echo unknown)
echo "== kernel $(uname -r) $(uname -m), LSMs: $LSM"
if ! echo ",$LSM," | grep -q ",bpf,"; then
  echo "shield checks skipped: bpf is not in the LSM list, so shields cannot enforce here"
  exit 0
fi

# ---- Test key, policies, files, network, cgroups ----
RUN=$(mktemp -d /tmp/edr_shield_run.XXXXXX)
mkdir -p "$RUN/src" "$RUN/config" "$RUN/state" "$RUN/logs" "$T"
cp -a "$REPO/edr-agent/json_files" "$RUN/json_files"
cp -a "$REPO/edr-agent/ui" "$RUN/ui"
ln -s "$REPO/edr-agent/src/rules.json" "$RUN/src/rules.json"
( umask 077
  openssl genpkey -algorithm ed25519 -out "$RUN/test_signing_key.pem" 2>/dev/null
  python3 -c 'import secrets; print(secrets.token_urlsafe(32))' > "$RUN/agent_token" )
openssl pkey -in "$RUN/test_signing_key.pem" -pubout -out "$RUN/config/policy_pub.pem"
export EDR_AGENT_TOKEN_FILE="$RUN/agent_token"
AUTH="Authorization: Bearer $(cat "$RUN/agent_token")"
sign() { python3 "$REPO/scripts/policy_signer.py" sign "$1" --key "$RUN/test_signing_key.pem" --pub "$RUN/config/policy_pub.pem" > /dev/null; }
api() { curl -s -H "$AUTH" -H 'Content-Type: application/json' "$@"; }
code() { curl -s -o "$OUT/last_body.json" -w '%{http_code}' -H "$AUTH" -H 'Content-Type: application/json' "$@"; }

policy() { # policy <serial> <max_active> <memfd_allow json list>
  cat <<P
{
  "policy": {
    "serial": $1,
    "guard_mode": "monitor",
    "guard_allow": { "memfd_allow": $3 },
    "redact_command_lines": true,
    "response_actions": ["kill_proc", "block_egress", "isolate_host"],
    "management_ports": [22],
    "logging": { "level": "info" },
    "shields": {
      "max_lifetime_sec": 3600,
      "max_active": $2,
      "never_block": { "paths": ["$T/nb_tool"], "cidrs": ["203.0.113.99/32"] },
      "auto_promote": { "kinds": ["file_deny"], "trial_sec": 6 },
      "auto_apply": { "kinds": ["connect_deny"], "max_lifetime_sec": 300 }
    }
  }
}
P
}
policy 1 5 '[]' > "$RUN/policy_v1.json"; sign "$RUN/policy_v1.json"
policy 2 10 "[\"$T/memfd_ok\"]" > "$RUN/policy_v2.json"; sign "$RUN/policy_v2.json"
cp "$RUN/policy_v1.json" "$RUN/policy.json"; cp "$RUN/policy_v1.json.sig" "$RUN/policy.json.sig"

cp /bin/true "$T/payload"; cp /bin/true "$T/benign"; cp /bin/true "$T/nb_tool"; cp /bin/true "$T/short_exec"
echo "secret" > "$T/secret.txt"; echo "other" > "$T/other.txt"; echo "short" > "$T/short.txt"
PY=$(readlink -f "$(command -v python3)")
cp "$PY" "$T/memfd_ok"
cat > "$T/memfd_exec.py" <<'PY'
import os, sys
fd = os.memfd_create("shieldtest", 0)
os.write(fd, open("/bin/true", "rb").read())
pid = os.fork()
if pid == 0:
    try:
        os.execv(f"/proc/self/fd/{fd}", ["shieldtest"])
    except PermissionError:
        os._exit(77)
_, st = os.waitpid(pid, 0)
print("refused" if os.WEXITSTATUS(st) == 77 else "ran")
PY
cat > "$T/conn.py" <<'PY'
# conn.py <tcp|udp|v6mapped> <ip> <port>: prints ok, or the errno name.
import errno, socket, sys
kind, ip, port = sys.argv[1], sys.argv[2], int(sys.argv[3])
try:
    if kind == "tcp":
        socket.create_connection((ip, port), timeout=2).close()
    elif kind == "udp":
        s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        s.sendto(b"x", (ip, port))
    else:
        s = socket.socket(socket.AF_INET6, socket.SOCK_STREAM)
        s.settimeout(2)
        s.connect(("::ffff:" + ip, port))
    print("ok")
except OSError as e:
    print(errno.errorcode.get(e.errno, str(e)))
PY
cat > "$T/listen.py" <<'PY'
import socket, threading, sys
def serve(ip, port):
    s = socket.socket(); s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    s.bind((ip, port)); s.listen(64)
    while True:
        c, _ = s.accept(); c.close()
for spec in sys.argv[1:]:
    ip, port = spec.rsplit(":", 1)
    threading.Thread(target=serve, args=(ip, int(port)), daemon=True).start()
threading.Event().wait()
PY
ip link add edrs0 type dummy 2>/dev/null
for a in 203.0.113.10 203.0.113.11 203.0.113.12 203.0.113.13 203.0.113.14; do ip addr add $a/32 dev edrs0 2>/dev/null; done
ip link set edrs0 up
python3 "$T/listen.py" 203.0.113.10:8000 203.0.113.10:4444 203.0.113.11:22 203.0.113.12:443 203.0.113.13:7777 203.0.113.14:7777 > /dev/null 2>&1 &
LISTEN=$!
sleep 0.5
SCOPE=${SCOPE:-}
mkdir -p "$CG/shield_in" "$CG/shield_out" 2>/dev/null
in_cg() { local g=$1; shift; bash -c 'echo $$ > "$0/cgroup.procs" && exec "$@"' "$CG/$g" "$@"; }
in_cg shield_out true 2>/dev/null && CG_OK=1 || CG_OK=0
echo "test cgroups usable: $CG_OK"

# ---- Sensors and the agent ----
step "load the sensors"
rm -rf "$PIN" "$SPIN"
for OBJ in raw_syscalls_catcher proc_lifecycle net_flow wx_exec tcp_state file_access_monitor; do
  "$OBJ_DIR/edr_attach_any" -p "$PIN" "$OBJ_DIR/$OBJ.bpf.o" > "$OUT/load_$OBJ.log" 2>&1 || check load_$OBJ 1 "see $OUT/load_$OBJ.log"
done
# The webshell trigger's web server: a copy of python3 named nginx.
[ -x /usr/local/sbin/nginx ] || { mkdir -p /usr/local/sbin; cp "$PY" /usr/local/sbin/nginx; }
rm -f /run/edr-guard/off
cd "$RUN"
step "start the agent (guard enforce, test key)"
env HOSTNAME=shield-live EDR_AGENT_TOKEN_FILE="$RUN/agent_token" PB_DIR="$REPO/edr-agent/playbooks" \
    EDR_POLICY_PUBKEY="$RUN/config/policy_pub.pem" EDR_GUARD_MODE=enforce ${SCOPE:+EDR_GUARD_SCOPE_CGROUP=$SCOPE} \
    EDR_BPF_GUARD_OBJ="$OBJ_DIR/edr_guard.bpf.o" EDR_BPF_SHIELD_OBJ="$OBJ_DIR/edr_shield.bpf.o" \
    EDR_AUTH_LOG="$RUN/auth.log" RUST_LOG=info "$AGENT_BIN" > "$OUT/agent.log" 2>&1 &
AGENT=$!
for _ in $(seq 1 150); do grep -q "perf subscriber active" "$OUT/agent.log" && break; sleep 0.2; done
for _ in $(seq 1 100); do curl -s -o /dev/null "$API/healthz" && break; sleep 0.2; done
kill -0 $AGENT 2>/dev/null || { check agent_start 1 "the agent exited"; tail -40 "$OUT/agent.log"; exit 1; }
grep -aE "^policy:|guard: prevention|^shields:|^guard:" "$OUT/agent.log" | cut -c1-240
E=$(api "$API/shields" | python3 -c 'import json,sys; print(json.load(sys.stdin)["enforcement"])')
check shields_loaded "$([ "$E" = kernel ] && echo 0 || echo 1)" "enforcement=$E"

newdoc() { python3 "$REPO/scripts/shieldctl.py" new "$@"; }
apply() { # apply <doc> -> http code
  python3 - "$1" > "$OUT/apply_body.json" <<'PY'
import json, sys
print(json.dumps({"document": open(sys.argv[1]).read(), "signature": open(sys.argv[1] + ".sig").read().strip()}))
PY
  code -X POST --data @"$OUT/apply_body.json" "$API/shields"
}
field() { python3 -c "import json,sys; d=json.load(open('$OUT/last_body.json')); print($1)"; }
shield() { api "$API/shields" | python3 -c "import json,sys; d={s['id']:s for s in json.load(sys.stdin)['shields']}; s=d.get('$1'); print(s['$2'] if s else 'absent')"; }
promote() { code -X POST -d '{"reason":"live test"}' "$API/shields/$1/promote"; }
run_rc() { "$@" > /dev/null 2>&1; echo $?; }

# ---- exec_deny ----
step "exec_deny"
newdoc exec_deny --path "$T/payload" --ttl 600 --id live.exec --reason "live test exec_deny" > "$T/exec.json"; sign "$T/exec.json"
C=$(apply "$T/exec.json"); check exec_apply "$([ "$C" = 201 ] && echo 0 || echo 1)" "POST /shields $C, mode $(field 'd["shield"]["mode"]')"
RC=$(run_rc "$T/payload"); H=$(shield live.exec hits)
check exec_monitor_allows "$([ "$RC" = 0 ] && [ "$H" -ge 1 ] && echo 0 || echo 1)" "monitor: exec rc=$RC, kernel hits=$H"
C=$(promote live.exec); RC=$(run_rc "$T/payload")
check exec_enforce_refuses "$([ "$C" = 200 ] && [ "$RC" = 126 ] && echo 0 || echo 1)" "promote $C, exec rc=$RC (126 is EPERM from execve)"
RC=$(run_rc "$T/benign"); check exec_normal_work "$([ "$RC" = 0 ] && echo 0 || echo 1)" "another program rc=$RC, ls rc=$(run_rc ls /)"
cp "$T/payload" "$T/payload_copy"; RC=$(run_rc "$T/payload_copy")
check exec_copy_is_another_inode "$([ "$RC" = 0 ] && echo 0 || echo 1)" "a copy has a new inode and runs (rc=$RC), file_deny below closes that"

# ---- file_deny ----
step "file_deny (auto promoted after the policy's 6 s trial)"
newdoc file_deny --path "$T/secret.txt" --ttl 600 --id live.file --reason "live test file_deny" > "$T/file.json"; sign "$T/file.json"
C=$(apply "$T/file.json"); check file_apply "$([ "$C" = 201 ] && echo 0 || echo 1)" "POST /shields $C, auto_promote_at $(field 'd["shield"]["auto_promote_at"]')"
RC=$(run_rc cat "$T/secret.txt"); check file_monitor_allows "$([ "$RC" = 0 ] && echo 0 || echo 1)" "monitor: cat rc=$RC, hits=$(shield live.file hits)"
sleep 8
M=$(shield live.file mode); RC=$(run_rc cat "$T/secret.txt")
check file_auto_promote_refuses "$([ "$M" = enforce ] && [ "$RC" = 1 ] && echo 0 || echo 1)" "mode $M after the trial, cat rc=$RC ($(cat "$T/secret.txt" 2>&1 | head -1))"
RC=$(run_rc cat "$T/other.txt"); check file_normal_work "$([ "$RC" = 0 ] && echo 0 || echo 1)" "another file rc=$RC"
newdoc file_deny --path "$T/payload" --ttl 600 --id live.file.payload --reason "no copies of the payload" > "$T/file2.json"; sign "$T/file2.json"
C=$(apply "$T/file2.json"); sleep 8; rm -f "$T/payload_copy2"; RC=$(run_rc cp "$T/payload" "$T/payload_copy2")
check file_deny_stops_the_copy "$([ "$C" = 201 ] && [ "$RC" != 0 ] && echo 0 || echo 1)" "file_deny on the payload's inode, cp rc=$RC"

# ---- connect_deny ----
step "connect_deny"
newdoc connect_deny --cidr 203.0.113.10 --port 8000 --ttl 600 --id live.conn --reason "live test connect_deny" > "$T/conn.json"; sign "$T/conn.json"
C=$(apply "$T/conn.json"); R=$(python3 "$T/conn.py" tcp 203.0.113.10 8000)
check conn_monitor_allows "$([ "$C" = 201 ] && [ "$R" = ok ] && echo 0 || echo 1)" "POST $C, monitor: connect $R, hits=$(shield live.conn hits)"
C=$(promote live.conn)
R1=$(python3 "$T/conn.py" tcp 203.0.113.10 8000); R2=$(python3 "$T/conn.py" udp 203.0.113.10 8000); R3=$(python3 "$T/conn.py" v6mapped 203.0.113.10 8000)
check conn_enforce_refuses "$([ "$R1" = EPERM ] && [ "$R2" = EPERM ] && [ "$R3" = EPERM ] && echo 0 || echo 1)" "tcp connect $R1, udp sendto $R2, ::ffff: mapped $R3"
R1=$(python3 "$T/conn.py" tcp 203.0.113.10 4444); R2=$(python3 "$T/conn.py" tcp 203.0.113.12 443)
check conn_normal_work "$([ "$R1" = ok ] && [ "$R2" = ok ] && echo 0 || echo 1)" "same address other port $R1, other address $R2"
if [ "$CG_OK" = 1 ]; then
  newdoc connect_deny --cidr 203.0.113.11 --port 22 --cgroup "$CG/shield_in" --ttl 600 --id live.conn.scoped --reason "scope test" > "$T/conn2.json"; sign "$T/conn2.json"
  C=$(apply "$T/conn2.json"); promote live.conn.scoped > /dev/null
  RI=$(in_cg shield_in python3 "$T/conn.py" tcp 203.0.113.11 22); RO=$(in_cg shield_out python3 "$T/conn.py" tcp 203.0.113.11 22)
  check conn_scope "$([ "$C" = 201 ] && [ "$RI" = EPERM ] && [ "$RO" = ok ] && echo 0 || echo 1)" "scoped to $CG/shield_in: inside $RI, outside $RO"
else
  # No usable test cgroups here, so a fifth shield without a scope keeps the count.
  newdoc connect_deny --cidr 192.0.2.70 --ttl 600 --id live.conn.filler --reason "filler" > "$T/conn2.json"; sign "$T/conn2.json"
  apply "$T/conn2.json" > /dev/null
fi

# ---- refusals ----
step "refusals"
newdoc connect_deny --cidr 192.0.2.50 --ttl 60 --id live.tamper --reason "tamper test" > "$T/tamper.json"; sign "$T/tamper.json"
sed -i 's/192.0.2.50/192.0.2.51/' "$T/tamper.json"
C=$(apply "$T/tamper.json"); check refuse_tampered "$([ "$C" = 403 ] && echo 0 || echo 1)" "$C $(field 'd.get("refused")')"
C=$(apply "$T/exec.json"); check refuse_replay "$([ "$C" = 409 ] && echo 0 || echo 1)" "the exec document again: $C $(field 'd.get("refused")')"
newdoc connect_deny --cidr 192.0.2.52 --ttl 7200 --id live.long --reason "too long" > "$T/long.json"; sign "$T/long.json"
C=$(apply "$T/long.json"); check refuse_over_lifetime "$([ "$C" = 403 ] && echo 0 || echo 1)" "$C $(field 'd.get("error")')"
NB=""
for spec in "exec_deny --path /proc/1/exe" "exec_deny --path $AGENT_BIN" "exec_deny --path $T/nb_tool" "connect_deny --cidr 127.0.0.1" "connect_deny --cidr 203.0.113.99"; do
  # shellcheck disable=SC2086
  newdoc $spec --ttl 60 --id "live.nb.$RANDOM" --reason "never-block test" > "$T/nb.json"; sign "$T/nb.json"
  C=$(apply "$T/nb.json"); NB="$NB $C"
  echo "  $spec: $C $(field 'd.get("error")')"
done
check refuse_never_block "$([ "$NB" = " 403 403 403 403 403" ] && echo 0 || echo 1)" "pid 1, the agent, a policy path, loopback, a policy network:$NB"
N=$(api "$API/shields" | python3 -c 'import json,sys; print(json.load(sys.stdin)["active"])')
newdoc connect_deny --cidr 192.0.2.60 --ttl 60 --id live.extra --reason "over the limit" > "$T/extra.json"; sign "$T/extra.json"
C=$(apply "$T/extra.json"); check refuse_over_max_active "$([ "$C" = 429 ] && echo 0 || echo 1)" "$N active, policy max 5: $C"

# ---- live policy reload ----
step "live policy reload"
R0=$("$T/memfd_ok" "$T/memfd_exec.py")
cp "$RUN/policy_v2.json" "$RUN/policy.json"; cp "$RUN/policy_v2.json.sig" "$RUN/policy.json.sig"
for _ in $(seq 1 30); do [ "$(api "$API/policy/status" | python3 -c 'import json,sys; print(json.load(sys.stdin)["serial"])')" = 2 ] && break; sleep 0.5; done
S=$(api "$API/policy/status" | python3 -c 'import json,sys; print(json.load(sys.stdin)["serial"])')
R1=$("$T/memfd_ok" "$T/memfd_exec.py"); R2=$(python3 "$T/memfd_exec.py")
check policy_reload_applies "$([ "$S" = 2 ] && [ "$R0" = refused ] && [ "$R1" = ran ] && [ "$R2" = refused ] && echo 0 || echo 1)" "serial $S, memfd exec by memfd_ok before $R0 after $R1, by python3 $R2 (guard allowlist changed in the kernel, one bank switch)"
cp "$RUN/policy_v1.json" "$RUN/policy.json"; cp "$RUN/policy_v1.json.sig" "$RUN/policy.json.sig"
sleep 3
S=$(api "$API/policy/status" | python3 -c 'import json,sys; d=json.load(sys.stdin); print(d["serial"], d["last"]["action"])')
INC=$(api "$API/incidents" | grep -c "Policy refused")
RB=$(grep -c "rollback refused" "$RUN/logs/policy_audit.jsonl")
check policy_rollback_refused "$([ "$S" = "2 refuse" ] && [ "$INC" -ge 1 ] && [ "$RB" -ge 1 ] && echo 0 || echo 1)" "an older signed policy (serial 1): running serial and last action '$S', rollback refusals in the audit log $RB, incident raised: $INC"
policy 3 10 '[]' > "$RUN/policy.json"; sign "$RUN/policy.json"; sed -i 's/"max_active": 10/"max_active": 999/' "$RUN/policy.json"
sleep 3
S=$(api "$API/policy/status" | python3 -c 'import json,sys; d=json.load(sys.stdin); print(d["serial"], d["last"]["action"], "signature" in d["last"].get("why",""))')
check policy_tampered_refused "$([ "$S" = "2 refuse True" ] && echo 0 || echo 1)" "serial 3 edited after signing: '$S'"
cp "$RUN/policy_v2.json" "$RUN/policy.json"; cp "$RUN/policy_v2.json.sig" "$RUN/policy.json.sig"

# ---- playbook proposals ----
step "playbook hits propose shields"
/usr/local/sbin/nginx -c 'import subprocess; subprocess.run(["bash", "-c", "exec 3<>/dev/tcp/203.0.113.10/4444; echo hi >&3; sleep 20"])' > /dev/null 2>&1 &
WEB=$!
cp "$(command -v curl)" "$T/dropper"
python3 - "$T/dropper" <<'PY'
import mmap, subprocess, sys, time
m = mmap.mmap(-1, 4096, prot=mmap.PROT_READ | mmap.PROT_WRITE | mmap.PROT_EXEC)
m.write(b"\xc3" * 16); m.close()
time.sleep(1.5)
subprocess.run([sys.argv[1], "-s", "-m", "2", "-o", "/dev/null", "http://203.0.113.12:443/"])
PY
# Wait until both proposals are on incidents. A plain grep for exec_deny would also
# match the shield incidents of the exec_deny step above.
proposal_kinds() { api "$API/incidents" | python3 -c 'import json,sys; print(" ".join(sorted({p["kind"] for i in json.load(sys.stdin) for p in i.get("shield_proposals") or []})))'; }
for _ in $(seq 1 45); do [ "$(proposal_kinds)" = "connect_deny exec_deny" ] && break; sleep 2; done
api "$API/incidents" > "$OUT/incidents_proposals.json"
python3 - "$OUT/incidents_proposals.json" > "$OUT/proposals.txt" <<'PY'
import json, sys
for i in json.load(open(sys.argv[1])):
    for p in i.get("shield_proposals") or []:
        a = p.get("auto_apply", {})
        print(i["id"], p["kind"], p["id"], a.get("status"), p["summary"])
PY
cat "$OUT/proposals.txt"
AUTO=$(api "$API/shields" | python3 -c 'import json,sys; print(sum(1 for s in json.load(sys.stdin)["shields"] if s["source"].startswith("auto:pb_webshell")))')
check proposal_reverse_shell_auto_applied "$(grep -q "connect_deny .* 201 connect_deny 203.0.113.10 port 4444" "$OUT/proposals.txt" && [ "$AUTO" = 1 ] && echo 0 || echo 1)" "webshell hit: connect_deny 203.0.113.10:4444 proposed and auto applied in monitor ($AUTO)"
EXEC_INC=$(awk '$2 == "exec_deny" {print $1; exit}' "$OUT/proposals.txt")
if [ -n "$EXEC_INC" ]; then
  IDX=$(python3 -c "import json; i=[x for x in json.load(open('$OUT/incidents_proposals.json')) if x['id']=='$EXEC_INC'][0]; print([p['kind'] for p in i['shield_proposals']].index('exec_deny'))")
  python3 "$REPO/scripts/shieldctl.py" propose "$EXEC_INC" "$IDX" > "$T/proposal.json"; sign "$T/proposal.json"
  PID=$(python3 -c "import json; print(json.load(open('$T/proposal.json'))['shield']['id'])")
  C=$(apply "$T/proposal.json"); P=$(promote "$PID"); RC=$(run_rc "$T/dropper" -s -m 1 http://203.0.113.12:443/)
  check proposal_dropped_payload_signed "$([ "$C" = 201 ] && [ "$P" = 200 ] && [ "$RC" = 126 ] && echo 0 || echo 1)" "exec_deny proposal signed and applied $C, promoted $P, the dropper now gets rc=$RC"
else
  check proposal_dropped_payload_signed 1 "no exec_deny proposal on any incident"
fi
kill $WEB 2>/dev/null

# ---- expiring firewall rules ----
step "isolate_host with a 6 s expiry (nftables element timeout)"
if command -v nft > /dev/null; then
  C=$(code -X POST -d '{"reason":"isolation expiry test","expires_in_sec":6}' "$API/actions/isolate_host")
  R1=$(python3 "$T/conn.py" tcp 203.0.113.13 7777); sleep 8; R2=$(python3 "$T/conn.py" tcp 203.0.113.13 7777)
  check isolate_expires "$([ "$C" = 200 ] && [ "$R1" != ok ] && [ "$R2" = ok ] && echo 0 || echo 1)" "isolate $C, during $R1, after 8 s $R2"
fi

# ---- expiry with the agent stopped ----
step "expiry while the agent is stopped with SIGSTOP"
NOW=$(date +%s)
newdoc exec_deny --path "$T/short_exec" --ttl 25 --id live.short.exec --reason "expiry test" > "$T/s1.json"; sign "$T/s1.json"
newdoc file_deny --path "$T/short.txt" --ttl 25 --id live.short.file --reason "expiry test" > "$T/s2.json"; sign "$T/s2.json"
newdoc connect_deny --cidr 203.0.113.13 --port 7777 --ttl 25 --id live.short.conn --reason "expiry test" > "$T/s3.json"; sign "$T/s3.json"
for f in s1 s2 s3; do apply "$T/$f.json" > /dev/null; done
for id in live.short.exec live.short.file live.short.conn; do promote $id > /dev/null; done
command -v nft > /dev/null && code -X POST -d '{"dst":"203.0.113.14","port":7777,"reason":"egress expiry test","expires_in_sec":25}' "$API/actions/block_egress" > /dev/null
B1="$(run_rc "$T/short_exec") $(run_rc cat "$T/short.txt") $(python3 "$T/conn.py" tcp 203.0.113.13 7777) $(python3 "$T/conn.py" tcp 203.0.113.14 7777)"
mkdir -p /run/edr-guard && touch /run/edr-guard/off
sleep 1
kill -STOP $AGENT; ST=$(awk '{print $3}' /proc/$AGENT/stat)
echo "agent state after SIGSTOP: $ST (T is stopped). Deadline $((NOW + 25)), waiting."
while [ "$(date +%s)" -le $((NOW + 27)) ]; do sleep 1; done
B2="$(run_rc "$T/short_exec") $(run_rc cat "$T/short.txt") $(python3 "$T/conn.py" tcp 203.0.113.13 7777) $(python3 "$T/conn.py" tcp 203.0.113.14 7777)"
ST2=$(awk '{print $3}' /proc/$AGENT/stat)
kill -CONT $AGENT
OKB1=1; case "$B1" in "126 1 EPERM ok") ;; "126 1 EPERM "*) OKB1=0 ;; esac
check expiry_with_agent_stopped "$([ "$ST" = T ] && [ "$ST2" = T ] && [ $OKB1 = 0 ] && [ "$B2" = "0 0 ok ok" ] && echo 0 || echo 1)" "before the deadline (exec rc, open rc, connect, egress): $B1, after it with the agent still in state $ST2: $B2"
sleep 3
GONE=$(api "$API/shields" | python3 -c 'import json,sys; print(sum(1 for s in json.load(sys.stdin)["shields"] if s["id"].startswith("live.short")))')
EXP=$(grep -c '"action":"expire"' "$RUN/logs/shields_audit.jsonl")
check sweep_after_resume "$([ "$GONE" = 0 ] && [ "$EXP" -ge 3 ] && echo 0 || echo 1)" "after SIGCONT the sweep removed them: $GONE left, $EXP expire audit lines"

# ---- incidents, audit ----
step "incidents and audit logs"
api "$API/incidents" > "$OUT/incidents.json"
SI=$(python3 -c "import json; print(sum(1 for i in json.load(open('$OUT/incidents.json')) if 'shield' in (i.get('sources') or [])))")
check shield_incidents "$([ "$SI" -ge 1 ] && echo 0 || echo 1)" "incidents with source shield: $SI"
cp "$RUN"/logs/*.jsonl "$OUT/" 2>/dev/null
api "$API/shields" > "$OUT/shields_final.json"
api "$API/policy/status" > "$OUT/policy_status.json"
ls -l "$RUN/logs/shields_audit.jsonl" "$RUN/logs/policy_audit.jsonl" | tee -a "$OUT/checks.txt"

# ---- stop and clean up ----
step "stop"
kill -INT $AGENT 2>/dev/null
for _ in $(seq 1 50); do kill -0 $AGENT 2>/dev/null || break; sleep 0.2; done
kill -9 $AGENT 2>/dev/null
kill $LISTEN 2>/dev/null
rm -rf "$PIN" "$SPIN"
rmdir "$CG/shield_in" "$CG/shield_out" 2>/dev/null
ip link del edrs0 2>/dev/null
nft delete table inet edr_agent 2>/dev/null
rm -f /run/edr-guard/off
chmod -R a+rX "$OUT"
grep -c ^PASS "$OUT/checks.txt" | xargs echo "passed:"
[ $FAIL = 0 ] && echo "shield live: PASS" || echo "shield live: FAIL"
exit $FAIL
