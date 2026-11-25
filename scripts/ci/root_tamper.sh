#!/bin/bash
# What a root attacker can still do to the agent on plain Linux. Run as root on a
# throwaway machine whose kernel has bpf in its LSM list (or a privileged container on
# such a kernel):
#
#   sudo bash scripts/ci/root_tamper.sh <dir with the .bpf.o files and edr_attach_any> <agent binary>
#
# It starts the agent with the guard enforcing, an enforcing connect_deny shield and a
# test signing key, then tries, as root:
#   SIGKILL and SIGTERM of the agent, systemctl stop (when systemd is pid 1), cgroup.kill
#   and cgroup.freeze, ptrace attach, /proc/<pid>/mem and process_vm_readv, detaching
#   the agent's BPF links and opening its programs with bpftool, deleting and editing
#   shield map rows with bpftool (by id, by pin, and through a bind mount), editing the
#   agent's key and anti-rollback state, the off switch with and without its token,
#   restarting with weaker settings, and rolling back to an older signed policy, live
#   and across a restart.
# Every attempt writes its exact commands and output to $OUT/NN_name.log, and one row
# to $OUT/results.tsv: id, attempt, outcome for the attacker (blocked, succeeded, not
# run), evidence. The table is the "before" column for a later comparison, so nothing
# here is tuned to look good: an attempt that works is written down as working.
set -u
OBJ_DIR=$(cd "$1" && pwd)
AGENT_BIN=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
REPO=$(cd "$(dirname "$0")/../.." && pwd)
OUT=${OUT:-$PWD/tamper-out}
API=http://127.0.0.1:8080
PIN=/sys/fs/bpf/edr
SPIN=/sys/fs/bpf/edr_shield
CG=/sys/fs/cgroup
ACG=$CG/edr_tamper_agent
T=/tmp/tampertest
mkdir -p "$OUT" "$T"
printf 'id\tattempt\toutcome\tevidence\n' > "$OUT/results.tsv"
N=0
row() { # row <name> <blocked|succeeded|not run> <evidence>
  printf '%02d\t%s\t%s\t%s\n' "$N" "$1" "$2" "$3" >> "$OUT/results.tsv"
  printf '%02d %-46s %-10s %s\n' "$N" "$1" "$2" "$3"
}
attempt() { N=$((N + 1)); L=$(printf '%s/%02d_%s.log' "$OUT" "$N" "$1"); : > "$L"; }
sh_log() { echo "\$ $*" >> "$L"; "$@" >> "$L" 2>&1; local rc=$?; echo "rc=$rc" >> "$L"; return $rc; }
# Signals go to the agent's pid only. An empty or 0 pid would make kill signal the
# whole process group, this script included.
kill_agent() { if [ "${AGENT:-0}" -gt 1 ]; then sh_log kill "$1" "$AGENT"; else echo "no agent pid ($AGENT), $1 not sent" >> "$L"; return 1; fi; }

mountpoint -q /sys/fs/bpf || mount -t bpf bpf /sys/fs/bpf
mountpoint -q /sys/kernel/tracing || mount -t tracefs tracefs /sys/kernel/tracing 2>/dev/null
mountpoint -q /sys/kernel/debug || mount -t debugfs debugfs /sys/kernel/debug 2>/dev/null
mountpoint -q /sys/kernel/security || mount -t securityfs securityfs /sys/kernel/security 2>/dev/null
LSM=$(cat /sys/kernel/security/lsm 2>/dev/null || echo unknown)
echo "== kernel $(uname -r) $(uname -m), LSMs: $LSM, pid 1 is $(cat /proc/1/comm)"
if ! echo ",$LSM," | grep -q ",bpf,"; then
  echo "skipped: bpf is not in the LSM list, the guard cannot run here"; exit 0
fi

# ---- A run directory with a test key, two signed policies and an agent cgroup ----
RUN=$(mktemp -d /tmp/edr_tamper_run.XXXXXX)
mkdir -p "$RUN/src" "$RUN/config" "$RUN/logs" "$RUN/state"
cp -a "$REPO/edr-agent/json_files" "$RUN/json_files"; cp -a "$REPO/edr-agent/ui" "$RUN/ui"
ln -s "$REPO/edr-agent/src/rules.json" "$RUN/src/rules.json"
( umask 077
  openssl genpkey -algorithm ed25519 -out "$T/signing_key.pem" 2>/dev/null
  openssl genpkey -algorithm ed25519 -out "$T/attacker_key.pem" 2>/dev/null
  python3 -c 'import secrets; print(secrets.token_urlsafe(32))' > "$RUN/agent_token"
  python3 -c 'import secrets; print(secrets.token_urlsafe(32))' > "$T/off_token" )
openssl pkey -in "$T/signing_key.pem" -pubout -out "$RUN/config/policy_pub.pem"
openssl pkey -in "$T/attacker_key.pem" -pubout -out "$T/attacker_pub.pem"
OFF_HASH=$(python3 -c "import hashlib; print(hashlib.sha256(open('$T/off_token').read().strip().encode()).hexdigest())")
AUTH="Authorization: Bearer $(cat "$RUN/agent_token")"
export EDR_AGENT_TOKEN_FILE="$RUN/agent_token"
policy() { # policy <serial> <guard_mode>
  printf '{"policy":{"serial":%s,"guard_mode":"%s","redact_command_lines":true,"response_actions":["kill_proc","block_egress","isolate_host"],"management_ports":[22],"logging":{"level":"info"},"shields":{"max_lifetime_sec":3600,"max_active":16}}}\n' "$1" "$2"
}
signwith() { python3 "$REPO/scripts/policy_signer.py" sign "$1" --key "$2" --pub "$3" > /dev/null; }
policy 1 monitor > "$T/policy_v1.json"; signwith "$T/policy_v1.json" "$T/signing_key.pem" "$RUN/config/policy_pub.pem"
policy 2 enforce > "$T/policy_v2.json"; signwith "$T/policy_v2.json" "$T/signing_key.pem" "$RUN/config/policy_pub.pem"
cp "$T/policy_v2.json" "$RUN/policy.json"; cp "$T/policy_v2.json.sig" "$RUN/policy.json.sig"
mkdir -p "$ACG"
# The agent runs from a copy, so the attempt to replace its binary meets the guard and
# not a read only mount.
cp "$AGENT_BIN" "$T/forensic_hooks"; AGENT_BIN="$T/forensic_hooks"
ip link add edrt0 type dummy 2>/dev/null; ip addr add 203.0.113.10/32 dev edrt0 2>/dev/null; ip link set edrt0 up
python3 -c '
import socket, threading
s = socket.socket(); s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1); s.bind(("203.0.113.10", 8000)); s.listen(64)
while True:
    c, _ = s.accept(); c.close()' > /dev/null 2>&1 &
LISTEN=$!
conn() { python3 -c '
import errno, socket
try:
    socket.create_connection(("203.0.113.10", 8000), timeout=2).close(); print("ok")
except OSError as e:
    print(errno.errorcode.get(e.errno, str(e)))'; }

echo "== load the sensors"
rm -rf "$PIN" "$SPIN"
for OBJ in raw_syscalls_catcher proc_lifecycle net_flow wx_exec tcp_state file_access_monitor; do
  "$OBJ_DIR/edr_attach_any" -p "$PIN" "$OBJ_DIR/$OBJ.bpf.o" > "$OUT/load_$OBJ.log" 2>&1 || echo "load $OBJ failed"
done
SYSTEMD=0; [ "$(cat /proc/1/comm)" = systemd ] && command -v systemd-run > /dev/null && SYSTEMD=1
AGENT=0
UNIT=edr-tamper-agent-none
# start_agent <log name> [VAR=value ...]: the agent in its own cgroup, so cgroup.kill
# and cgroup.freeze reach only the agent. Under systemd it runs as a transient unit.
start_agent() {
  local name=$1; shift
  rm -f /run/edr-guard/off
  # A killed agent can take a moment to exit and release its lock file.
  for _ in $(seq 1 75); do flock -n /var/run/forensic_hooks.lock true 2>/dev/null && break; sleep 0.2; done
  cd "$RUN"
  local envs=(HOSTNAME=tamper EDR_AGENT_TOKEN_FILE="$RUN/agent_token" PB_DIR="$REPO/edr-agent/playbooks"
    EDR_POLICY_PUBKEY="$RUN/config/policy_pub.pem" EDR_GUARD_MODE=enforce EDR_GUARD_OFF_SHA256="$OFF_HASH"
    EDR_BPF_GUARD_OBJ="$OBJ_DIR/edr_guard.bpf.o" EDR_BPF_SHIELD_OBJ="$OBJ_DIR/edr_shield.bpf.o" RUST_LOG=info)
  envs+=("$@")
  if [ $SYSTEMD = 1 ]; then
    # A unit an earlier attempt left behind can keep its name taken for a while, so
    # each start gets a unit of its own.
    UNIT="edr-tamper-agent-$name"
    systemd-run --unit="$UNIT" --working-directory="$RUN" -p TimeoutStopSec=10 -p StandardOutput=file:"$OUT/agent_$name.log" \
      -p StandardError=file:"$OUT/agent_$name.log" $(printf -- '--setenv=%s ' "${envs[@]}") "$AGENT_BIN" > /dev/null
    sleep 1; AGENT=$(systemctl show -p MainPID --value "$UNIT")
  else
    bash -c 'echo $$ > "$0/cgroup.procs" && exec env "$@"' "$ACG" "${envs[@]}" "$AGENT_BIN" > "$OUT/agent_$name.log" 2>&1 &
    AGENT=$!
  fi
  for _ in $(seq 1 150); do grep -q "perf subscriber active" "$OUT/agent_$name.log" 2>/dev/null && break; sleep 0.2; done
  for _ in $(seq 1 100); do curl -s -o /dev/null "$API/healthz" && break; sleep 0.2; done
  sleep 1
}
# Only a real pid: kill -0 0 would test the whole process group.
alive() { [ "${AGENT:-0}" -gt 1 ] && kill -0 "$AGENT" 2>/dev/null && [ "$(awk '{print $3}' /proc/$AGENT/stat 2>/dev/null)" != Z ]; }
# The agent's own cgroup v2 directory: $ACG here, the unit's cgroup under systemd.
# Never the root cgroup: its cgroup.kill would reach every process.
agent_cg() {
  local rel
  rel=$(awk -F: '$1 == "0" {print $3}' /proc/$AGENT/cgroup 2>/dev/null)
  [ -n "$rel" ] && [ "$rel" != "/" ] && echo "$CG$rel"
}
stop_agent() {
  echo "$(cat "$T/off_token")" > /run/edr-guard/off 2>/dev/null || { mkdir -p /run/edr-guard; cat "$T/off_token" > /run/edr-guard/off; }
  sleep 1
  if [ $SYSTEMD = 1 ]; then systemctl stop "$UNIT" 2>/dev/null; systemctl reset-failed "$UNIT" 2>/dev/null; fi
  [ "${AGENT:-0}" -gt 1 ] && kill -INT "$AGENT" 2>/dev/null; for _ in $(seq 1 50); do alive || break; sleep 0.2; done
  if alive && [ -n "$(agent_cg)" ]; then echo 1 > "$(agent_cg)/cgroup.kill" 2>/dev/null; fi
  sleep 0.5
  rm -f /run/edr-guard/off
}
shield_doc() { # shield_doc <id> -> signs and posts a connect_deny for 203.0.113.10:8000, then promotes it
  python3 "$REPO/scripts/shieldctl.py" new connect_deny --cidr 203.0.113.10 --port 8000 --ttl 1800 --id "$1" --reason "tamper target" > "$T/$1.json"
  signwith "$T/$1.json" "$T/signing_key.pem" "$RUN/config/policy_pub.pem"
  python3 "$REPO/scripts/shieldctl.py" apply "$T/$1.json" > /dev/null && python3 "$REPO/scripts/shieldctl.py" promote "$1" > /dev/null
}
guard_line() { grep -aE "guard: prevention|prevention mode is off" "$OUT/agent_$1.log" | tail -1 | cut -c1-160; }

echo "== start the agent"
start_agent first
alive || { echo "the agent did not start"; tail -30 "$OUT/agent_first.log"; exit 1; }
shield_doc tamper.shield
echo "agent pid $AGENT, $(guard_line first), shield: connect 203.0.113.10:8000 -> $(conn)"
echo
printf '%s\n' "id attempt                                        outcome    evidence"

# ---- Signals ----
attempt sigkill
kill_agent -9; sleep 0.5
alive && row "SIGKILL the agent (kill -9)" blocked "$(tail -2 "$L" | head -1 | cut -c1-90)" || row "SIGKILL the agent (kill -9)" succeeded "the agent is gone"
attempt sigterm
kill_agent -TERM; sleep 0.5
alive && row "SIGTERM the agent" blocked "$(tail -2 "$L" | head -1 | cut -c1-90)" || row "SIGTERM the agent" succeeded "the agent is gone"
attempt sigstop
kill_agent -STOP; ST=$(awk '{print $3}' /proc/$AGENT/stat)
echo "state $ST" >> "$L"
[ "$ST" = T ] && { row "SIGSTOP the agent" succeeded "state T"; [ "${AGENT:-0}" -gt 1 ] && kill -CONT "$AGENT"; } || row "SIGSTOP the agent" blocked "state $ST, $(grep -m1 -o 'Operation not permitted' "$L")"
attempt systemctl_stop
if [ $SYSTEMD = 1 ]; then
  sh_log systemctl stop "$UNIT"; sleep 1
  ST=$(systemctl is-active "$UNIT" 2>&1); echo "unit: $ST, agent pid $AGENT alive: $(alive && echo yes || echo no)" >> "$L"
  if alive; then
    row "systemctl stop" blocked "systemctl returned after its stop timeout and calls the unit $ST, but SIGTERM and SIGKILL were refused and pid $AGENT still runs"
  else
    row "systemctl stop" succeeded "unit $ST, the agent is gone"; start_agent after_systemctl
  fi
else
  echo "pid 1 is $(cat /proc/1/comm), not systemd" >> "$L"
  row "systemctl stop" "not run" "pid 1 is $(cat /proc/1/comm), no systemd here. systemd ends with SIGKILL, see the cgroup.kill row"
fi

# ---- ptrace and memory ----
attempt ptrace_attach
python3 - "$AGENT" >> "$L" 2>&1 <<'PY'
import ctypes, os, sys
libc = ctypes.CDLL(None, use_errno=True)
r = libc.ptrace(16, int(sys.argv[1]), 0, 0)  # PTRACE_ATTACH
print("ptrace(PTRACE_ATTACH) =", r, os.strerror(ctypes.get_errno()) if r else "attached")
if r == 0:
    libc.ptrace(17, int(sys.argv[1]), 0, 0)  # PTRACE_DETACH
PY
grep -q "attached" "$L" && row "ptrace attach (PTRACE_ATTACH)" succeeded "$(tail -1 "$L")" || row "ptrace attach (PTRACE_ATTACH)" blocked "$(tail -1 "$L")"
attempt gdb_strace
command -v strace > /dev/null && sh_log timeout 3 strace -p "$AGENT" -e trace=none
command -v gdb > /dev/null && sh_log timeout 10 gdb -p "$AGENT" -batch -ex 'info threads'
grep -qiE "Operation not permitted|ptrace.*denied|Could not attach" "$L" && row "strace -p and gdb -p" blocked "$(grep -m1 -iE 'not permitted|denied' "$L" | cut -c1-90)" || row "strace -p and gdb -p" succeeded "see the log"
attempt proc_mem
python3 - "$AGENT" >> "$L" 2>&1 <<'PY'
import ctypes, os, sys
pid = int(sys.argv[1])
try:
    open(f"/proc/{pid}/mem", "r+b"); print("/proc/pid/mem opened for writing")
except OSError as e:
    print("/proc/pid/mem:", e)
class iov(ctypes.Structure):
    _fields_ = [("base", ctypes.c_void_p), ("len", ctypes.c_size_t)]
buf = ctypes.create_string_buffer(8)
local = iov(ctypes.cast(buf, ctypes.c_void_p), 8); remote = iov(0x400000, 8)
libc = ctypes.CDLL(None, use_errno=True)
n = libc.process_vm_readv(pid, ctypes.byref(local), 1, ctypes.byref(remote), 1, 0)
print("process_vm_readv =", n, os.strerror(ctypes.get_errno()))
PY
grep -q "opened for writing" "$L" && row "write /proc/<pid>/mem, process_vm_readv" succeeded "$(head -2 "$L" | tr '\n' ' ')" || row "write /proc/<pid>/mem, process_vm_readv" blocked "$(tr '\n' ' ' < "$L" | cut -c1-110)"

# ---- BPF objects ----
ids() { # ids <link_id|prog_id|map_id>: the BPF objects the agent holds, from its fdinfo
  grep -h "^$1:" /proc/$AGENT/fdinfo/* 2>/dev/null | awk '{print $2}' | sort -un | tr '\n' ' '
}
attempt bpftool_list
echo "agent link ids: $(ids link_id), prog ids: $(ids prog_id), map ids: $(ids map_id)" >> "$L"
sh_log bpftool prog show
sh_log bpftool link show
grep -q "Operation not permitted" "$L" && row "bpftool prog show, link show" blocked "listing stops at the agent's ids: $(grep -m1 'not permitted' "$L" | cut -c1-70)" || row "bpftool prog show, link show" succeeded "listed"
attempt link_detach_by_id
for id in $(ids link_id); do sh_log bpftool link detach id "$id"; done
alive && [ "$(conn)" = EPERM ] && row "bpftool link detach id <each agent link>" blocked "$(grep -c 'not permitted' "$L") of $(ids link_id | wc -w) refused, shield still EPERM" || row "bpftool link detach id <each agent link>" succeeded "see the log"
attempt link_detach_pinned
for l in "$SPIN"/links/*; do sh_log bpftool link detach pinned "$l"; done
[ "$(conn)" = EPERM ] && row "bpftool link detach pinned <shield links>" blocked "$(grep -m1 -oE 'Operation not permitted|not supported' "$L"), shield still EPERM" || row "bpftool link detach pinned <shield links>" succeeded "connect now $(conn)"
attempt link_detach_bind_mount
mkdir -p "$T/bindpin"; sh_log mount --bind "$SPIN" "$T/bindpin"
for l in "$T"/bindpin/links/*; do sh_log bpftool link detach pinned "$l"; done
sh_log bpftool prog show pinned "$T/bindpin/links/shield_connect"
umount "$T/bindpin" 2>/dev/null
R=$(conn)
if [ "$R" = EPERM ]; then
  row "the same through a bind mount of the pin dir" blocked "the name check misses, then: $(grep -m1 -oE 'not supported|Operation not permitted|Invalid argument' "$L"), shield still EPERM"
else
  row "the same through a bind mount of the pin dir" succeeded "connect now $R"
fi
attempt map_edit_by_id
KEY="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
for id in $(ids map_id); do sh_log bpftool map delete id "$id" key $KEY; done
[ "$(conn)" = EPERM ] && row "bpftool map delete id <each agent map>" blocked "$(grep -c 'not permitted' "$L") of $(ids map_id | wc -w) refused" || row "bpftool map delete id <each agent map>" succeeded "connect now $(conn)"
attempt map_edit_pinned
sh_log bpftool map dump pinned "$SPIN/maps/shield_conn"
sh_log bpftool map update pinned "$SPIN/maps/shield_cfg" key 3 0 0 0 value 0 0 0 0 0 0 0 0
mkdir -p "$T/bindmap"; mount --bind "$SPIN" "$T/bindmap" 2>/dev/null
sh_log bpftool map update pinned "$T/bindmap/maps/shield_cfg" key 3 0 0 0 value 0 0 0 0 0 0 0 0
umount "$T/bindmap" 2>/dev/null
[ "$(conn)" = EPERM ] && row "bpftool map dump/update pinned, also via bind mount" blocked "$(grep -c 'not permitted' "$L") of 3 refused (lsm/bpf_map judges by map id)" || row "bpftool map dump/update pinned, also via bind mount" succeeded "connect now $(conn)"
attempt pin_unlink_rename
sh_log rm -f "$SPIN/links/shield_connect"
sh_log mv "$SPIN" /sys/fs/bpf/renamed_shield
sh_log ln "$SPIN/links/shield_connect" /sys/fs/bpf/shield_alias
[ -e "$SPIN/links/shield_connect" ] && [ "$(conn)" = EPERM ] && row "rm, mv and ln of the shield pins" blocked "$(grep -c '^rc=[1-9]' "$L") of 3 refused, shield still EPERM" || row "rm, mv and ln of the shield pins" succeeded "see the log"
attempt sensor_blind
# The sensors are pinned by their loader. Removing a link pin detaches that sensor
# when nothing else holds the link, and the self filter map decides which pid the
# file sensor skips.
SL=$(ls "$PIN" | grep -m1 '_link$'); echo "sensor link pin: $PIN/$SL" >> "$L"
sh_log rm -f "$PIN/$SL"
sh_log bpftool map update pinned "$PIN/edr_self_tgid" key 0 0 0 0 value 1
for id in $(bpftool link show 2>/dev/null | awk -F: '/^[0-9]+:/ {print $1}' | head -3); do sh_log bpftool link detach id "$id"; done
[ -e "$PIN/$SL" ] && row "blind the sensors: rm a sensor link pin, edit a sensor map" blocked "$(grep -c '^rc=[1-9]' "$L") refused, the pin is still there" || row "blind the sensors: rm a sensor link pin, edit a sensor map" succeeded "the pin is gone"

# ---- Files ----
attempt edit_key_and_state
sh_log sh -c "cat $T/attacker_pub.pem > $RUN/config/policy_pub.pem"
sh_log sh -c "echo 0 > $RUN/state/policy_serial"
sh_log truncate -s 0 "$RUN/state/shield_serials.json"
sh_log rm -f "$RUN/state/policy_last.json"
sh_log mv "$RUN/state/shields.json" "$RUN/state/x.json"
# Replacing the binary the way a package upgrade does: a new file renamed over it.
# (cp onto a running binary fails with ETXTBSY before any LSM is asked.)
cp /bin/true "$T/evil_agent"; sh_log mv "$T/evil_agent" "$AGENT_BIN"
B=$(grep -c "^rc=[1-9]" "$L")
[ "$B" = 6 ] && row "overwrite the key, edit, truncate, delete, rename the state, replace the binary" blocked "6 of 6 refused: $(grep -oE 'Operation not permitted|Permission denied' "$L" | sort | uniq -c | tr -s ' ' | tr '\n' ' ')" || row "overwrite the key, edit, truncate, delete, rename the state, replace the binary" succeeded "$B of 6 refused, see the log"
attempt edit_policy_live
sh_log sh -c "sed 's/\"enforce\"/\"off\"/' $T/policy_v2.json > $RUN/policy.json"
sleep 3
S=$(curl -s -H "$AUTH" "$API/policy/status")
echo "$S" >> "$L"
echo "$S" | grep -q '"serial":2' && row "edit policy.json while it runs (no new signature)" blocked "file writable by design, the reload refuses it: $(echo "$S" | python3 -c 'import json,sys; print(json.load(sys.stdin)["last"].get("why","")[:70])')" || row "edit policy.json while it runs (no new signature)" succeeded "$S"
attempt rollback_live
sh_log cp "$T/policy_v1.json" "$RUN/policy.json"; sh_log cp "$T/policy_v1.json.sig" "$RUN/policy.json.sig"
sleep 3
S=$(curl -s -H "$AUTH" "$API/policy/status"); echo "$S" >> "$L"
echo "$S" | grep -q '"serial":2' && row "roll back to the older signed policy, live" blocked "still serial 2: $(echo "$S" | python3 -c 'import json,sys; print(json.load(sys.stdin)["last"].get("why","")[:60])')" || row "roll back to the older signed policy, live" succeeded "$S"

# ---- The off switch ----
attempt off_switch_wrong_token
sh_log sh -c "echo guess > /run/edr-guard/off || (mkdir -p /run/edr-guard && echo guess > /run/edr-guard/off)"
sleep 1; kill_agent -9; sleep 0.5
rm -f /run/edr-guard/off
alive && row "off switch without the token (EDR_GUARD_OFF_SHA256 set)" blocked "kill -9 still refused" || row "off switch without the token (EDR_GUARD_OFF_SHA256 set)" succeeded "the agent died"
attempt environ_read
sh_log sh -c "tr '\\0' '\\n' < /proc/$AGENT/environ | grep -E 'OFF_SHA256|TOKEN'"
grep -q "EDR_GUARD_OFF_SHA256" "$L" && row "read the agent's environment" succeeded "root sees the off token's hash, not the token, and the API token file's path" || row "read the agent's environment" blocked "-"
attempt api_with_root_read_token
TOK=$(cat "$(tr '\0' '\n' < /proc/$AGENT/environ | sed -n 's/^EDR_AGENT_TOKEN_FILE=//p')" 2>>"$L")
C=$(curl -s -o "$L.body" -w '%{http_code}' -X DELETE -H "Authorization: Bearer $TOK" -H 'Content-Type: application/json' -d '{"reason":"root"}' "$API/shields/tamper.shield")
cat "$L.body" >> "$L"; rm -f "$L.body"; R=$(conn); echo "DELETE /shields/tamper.shield: $C, connect now $R" >> "$L"
[ "$C" = 200 ] && row "read the API token file, then DELETE /shields over the API" succeeded "HTTP $C, the shield is lifted (connect $R), audit logged" || row "read the API token file, then DELETE /shields over the API" blocked "HTTP $C"
shield_doc tamper.shield2

# ---- Kernel-raised signals ----
attempt cgroup_freeze
ACGR=$(agent_cg); echo "agent cgroup $ACGR" >> "$L"
sh_log sh -c "echo 1 > $ACGR/cgroup.freeze"; sleep 1
FZ=$(grep frozen "$ACGR/cgroup.events" 2>/dev/null); echo "$FZ" >> "$L"
CODE=$(curl -s -m 2 -o /dev/null -w '%{http_code}' "$API/healthz"); echo "healthz while frozen: $CODE" >> "$L"
R=$(conn); echo "shield while frozen: $R" >> "$L"
sh_log sh -c "echo 0 > $ACGR/cgroup.freeze"
echo "$FZ" | grep -q "frozen 1" && row "freeze the agent (cgroup.freeze)" succeeded "agent frozen, API $CODE, kernel shield still $R" || row "freeze the agent (cgroup.freeze)" blocked "$FZ"
attempt cgroup_kill
sh_log sh -c "echo 1 > $(agent_cg)/cgroup.kill"; sleep 1
if alive; then
  row "SIGKILL from the kernel (cgroup.kill)" blocked "the agent lives"
else
  R=$(conn); echo "after the agent died, the pinned shield: $R" >> "$L"
  row "SIGKILL from the kernel (cgroup.kill)" succeeded "the agent is dead, the guard detached, the pinned shield still answers $R"
  attempt lift_shield_after_death
  sh_log bpftool map dump pinned "$SPIN/maps/shield_conn"
  sh_log rm -rf "$SPIN"
  R=$(conn); echo "connect: $R" >> "$L"
  [ "$R" = ok ] && row "lift the shields once the agent is dead (rm the pins)" succeeded "pins removed, connect $R" || row "lift the shields once the agent is dead (rm the pins)" blocked "connect $R"
fi

# ---- Restart with weaker settings ----
attempt restart_guard_off
start_agent guard_off EDR_GUARD_MODE=off
echo "$(guard_line guard_off)" >> "$L"; grep -a "signed policy requires" "$OUT/agent_guard_off.log" >> "$L"
guard_line guard_off | grep -q "self_protect=enforce" && row "restart with EDR_GUARD_MODE=off" blocked "the signed floor (enforce) wins: $(guard_line guard_off | cut -c40-130)" || row "restart with EDR_GUARD_MODE=off" succeeded "$(guard_line guard_off)"
stop_agent
attempt restart_rollback_policy
cp "$T/policy_v1.json" "$RUN/policy.json"; cp "$T/policy_v1.json.sig" "$RUN/policy.json.sig"
start_agent rollback_restart
grep -aE "rollback refused|kept copy|^policy: serial" "$OUT/agent_rollback_restart.log" >> "$L"
grep -aq "^policy: serial 2" "$OUT/agent_rollback_restart.log" && row "restart on the older signed policy" blocked "the kept copy (serial 2) is used: $(grep -am1 'kept copy' "$OUT/agent_rollback_restart.log" | cut -c1-80)" || row "restart on the older signed policy" succeeded "$(grep -am1 '^policy: serial' "$OUT/agent_rollback_restart.log")"
stop_agent
attempt restart_rollback_state_wiped
sh_log rm -rf "$RUN/state"
start_agent rollback_wiped
grep -aE "^policy: serial|guard: prevention" "$OUT/agent_rollback_wiped.log" >> "$L"
grep -aq "^policy: serial 1" "$OUT/agent_rollback_wiped.log" && row "stop it, delete the state dir, restart on the older policy" succeeded "$(grep -am1 '^policy: serial' "$OUT/agent_rollback_wiped.log" | cut -c1-60)" || row "stop it, delete the state dir, restart on the older policy" blocked "$(grep -am1 '^policy: serial' "$OUT/agent_rollback_wiped.log")"
stop_agent
attempt restart_attacker_key
policy 999 off > "$T/evil.json"; signwith "$T/evil.json" "$T/attacker_key.pem" "$T/attacker_pub.pem"
cp "$T/evil.json" "$RUN/policy.json"; cp "$T/evil.json.sig" "$RUN/policy.json.sig"
start_agent attacker_key EDR_POLICY_PUBKEY="$T/attacker_pub.pem" EDR_GUARD_MODE=off
grep -aE "^policy: serial|prevention mode" "$OUT/agent_attacker_key.log" >> "$L"
grep -aq "prevention mode is off" "$OUT/agent_attacker_key.log" && row "restart with root's own key and policy (EDR_POLICY_PUBKEY)" succeeded "$(grep -am1 'prevention mode is off' "$OUT/agent_attacker_key.log" | cut -c1-90)" || row "restart with root's own key and policy (EDR_POLICY_PUBKEY)" blocked "$(guard_line attacker_key)"
stop_agent
attempt restart_dev_no_auth
policy 1000 off > "$RUN/policy.json"; echo 00 > "$RUN/policy.json.sig"
start_agent dev_no_auth EDR_DEV_NO_AUTH=1 EDR_GUARD_MODE=off
grep -aE "UNVERIFIED|prevention mode|NO authentication" "$OUT/agent_dev_no_auth.log" >> "$L"
grep -aq "prevention mode is off" "$OUT/agent_dev_no_auth.log" && row "restart with EDR_DEV_NO_AUTH=1 and an unsigned policy" succeeded "unverified policy loaded, guard off, API without a token" || row "restart with EDR_DEV_NO_AUTH=1 and an unsigned policy" blocked "$(guard_line dev_no_auth)"
stop_agent
attempt off_switch_default
cp "$T/policy_v2.json" "$RUN/policy.json"; cp "$T/policy_v2.json.sig" "$RUN/policy.json.sig"
start_agent off_default EDR_GUARD_OFF_SHA256=
sh_log sh -c "mkdir -p /run/edr-guard && touch /run/edr-guard/off"; sleep 1
kill_agent -9; sleep 0.5
alive && row "off switch with no token configured (the default)" blocked "kill still refused" || row "off switch with no token configured (the default)" succeeded "self protection off within 200 ms, then kill -9 worked"
rm -f /run/edr-guard/off

N=$((N + 1)); row "load a kernel module, write /dev/mem, kexec, boot without bpf in lsm=" "not run" "each replaces or bypasses the kernel the LSM programs run in. Root can, and no LSM program survives them"

# ---- Clean up ----
stop_agent 2>/dev/null
kill $LISTEN 2>/dev/null
rm -rf "$PIN" "$SPIN" /sys/fs/bpf/renamed_shield /sys/fs/bpf/shield_alias
rmdir "$ACG" 2>/dev/null
ip link del edrt0 2>/dev/null
nft delete table inet edr_agent 2>/dev/null
chmod -R a+rX "$OUT"
echo
echo "results: $OUT/results.tsv ($(grep -c blocked "$OUT/results.tsv") blocked, $(grep -c succeeded "$OUT/results.tsv") succeeded, $(grep -c 'not run' "$OUT/results.tsv") not run)"
