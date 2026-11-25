#!/bin/bash
# Live smoke test: load the sensors, start the agent, run scripts/ci/triggers.py and
# check that the expected playbooks became incidents. Run as root on a throwaway Linux
# machine (the GitHub runner, or a privileged container).
#
#   sudo bash scripts/ci/live_smoke.sh <dir with the .bpf.o files and edr_attach_any> <agent binary>
#
# The guard needs the BPF LSM. When "bpf" is not in /sys/kernel/security/lsm (the default
# on GitHub's Ubuntu runners) the agent runs detect-only, the script checks that it says
# so, and the guard checks are skipped. Results go to $OUT (default ./ci-out).
set -u
OBJ_DIR=$(cd "$1" && pwd)
AGENT_BIN=$(cd "$(dirname "$2")" && pwd)/$(basename "$2")
REPO=$(cd "$(dirname "$0")/../.." && pwd)
OUT=${OUT:-$PWD/ci-out}
PIN=/sys/fs/bpf/edr
API=http://127.0.0.1:8080
mkdir -p "$OUT"
FAIL=0
fail() { echo "FAIL: $*"; FAIL=1; }

echo "== kernel $(uname -r) $(uname -m)"
mountpoint -q /sys/fs/bpf || mount -t bpf bpf /sys/fs/bpf
mountpoint -q /sys/kernel/tracing || mount -t tracefs tracefs /sys/kernel/tracing 2>/dev/null
mountpoint -q /sys/kernel/debug || mount -t debugfs debugfs /sys/kernel/debug 2>/dev/null
mountpoint -q /sys/kernel/security || mount -t securityfs securityfs /sys/kernel/security 2>/dev/null
LSM=$(cat /sys/kernel/security/lsm 2>/dev/null || echo unknown)
echo "== active LSMs: $LSM"
if echo ",$LSM," | grep -q ",bpf,"; then GUARD=enforce; else GUARD=off; fi

echo "== load the sensors into $PIN"
rm -rf "$PIN"
for OBJ in raw_syscalls_catcher proc_lifecycle net_flow wx_exec tcp_state file_access_monitor; do
  if "$OBJ_DIR/edr_attach_any" -p "$PIN" "$OBJ_DIR/$OBJ.bpf.o" > "$OUT/load_$OBJ.log" 2>&1; then
    echo "loaded $OBJ"
  else
    fail "load $OBJ (see $OUT/load_$OBJ.log)"; tail -20 "$OUT/load_$OBJ.log"
  fi
done
ls "$PIN" > "$OUT/pins.txt"

# A run directory laid out like edr-agent/, so the repo checkout stays clean.
RUN=$(mktemp -d /tmp/edr_ci_run.XXXXXX)
mkdir -p "$RUN/src" "$RUN/config"
ln -s "$REPO/edr-agent/src/rules.json" "$RUN/src/rules.json"
ln -s "$REPO/edr-agent/policy.json" "$RUN/policy.json"
ln -s "$REPO/edr-agent/policy.json.sig" "$RUN/policy.json.sig"
ln -s "$REPO/edr-agent/config/policy_pub.pem" "$RUN/config/policy_pub.pem"
cp -a "$REPO/edr-agent/json_files" "$RUN/json_files"
cp -a "$REPO/edr-agent/ui" "$RUN/ui"
( umask 077; python3 -c 'import secrets; print(secrets.token_urlsafe(32))' > "$RUN/agent_token"
  printf 'Authorization: Bearer %s\n' "$(cat "$RUN/agent_token")" > "$RUN/auth_header" )

echo "== start the agent (guard: $GUARD)"
cd "$RUN"
env HOSTNAME=ci-smoke EDR_AGENT_TOKEN_FILE="$RUN/agent_token" PB_DIR="$REPO/edr-agent/playbooks" \
    EDR_GUARD_MODE=$GUARD EDR_BPF_GUARD_OBJ="$OBJ_DIR/edr_guard.bpf.o" EDR_BPF_SHIELD_OBJ="$OBJ_DIR/edr_shield.bpf.o" EDR_AUTH_LOG="$RUN/auth.log" \
    RUST_LOG=info "$AGENT_BIN" > "$OUT/agent.log" 2>&1 &
AGENT=$!
for _ in $(seq 1 150); do grep -q "perf subscriber active" "$OUT/agent.log" && break; sleep 0.2; done
for _ in $(seq 1 100); do curl -s -o /dev/null "$API/healthz" && break; sleep 0.2; done
kill -0 $AGENT 2>/dev/null || { fail "the agent exited at start"; tail -40 "$OUT/agent.log"; exit 1; }
grep -aE "playbooks: loaded|policy:|guard: prevention" "$OUT/agent.log" | cut -c1-200
if [ "$GUARD" = off ]; then
  echo "guard checks skipped: bpf is not in the LSM list ($LSM)"
  grep -aq "prevention mode is off" "$OUT/agent.log" || fail "the agent did not say it runs detect-only"
else
  grep -aq "prevention mode on" "$OUT/agent.log" || fail "the guard did not start with bpf in the LSM list"
fi
sleep 3

echo "== triggers"
python3 "$REPO/scripts/ci/triggers.py" | tee "$OUT/triggers.log"

if [ "$GUARD" = enforce ]; then
  echo "== guard: an exec from a memfd is refused"
  python3 - > "$OUT/guard_memfd.log" 2>&1 <<'PY'
import os
fd = os.memfd_create("ci_smoke")
os.write(fd, open("/bin/true", "rb").read())
try:
    os.execv(f"/proc/self/fd/{fd}", ["ci_smoke"])
except PermissionError as e:
    print("refused:", e)
PY
  cat "$OUT/guard_memfd.log"
  grep -q "refused" "$OUT/guard_memfd.log" || fail "the memfd exec was not refused"
fi

WANT="pb_lotl_cred_linux pb_terminal_exec_core pb_outage_ransomware_linux_backup_wipe_first pb_outage_ransomware_linux_fileburst_crypto"
echo "== waiting for: $WANT"
for _ in $(seq 1 60); do
  MISSING=""
  for P in $WANT; do grep -aq "^playbook fired: $P " "$OUT/agent.log" || MISSING="$MISSING $P"; done
  [ -z "$MISSING" ] && break
  sleep 2
done
sleep 4   # one more incident pass
curl -s -H @"$RUN/auth_header" "$API/incidents" > "$OUT/incidents.json"
curl -s -H @"$RUN/auth_header" "$API/pipeline/status" > "$OUT/pipeline.json"
python3 - "$OUT" "$WANT" <<'PY' || FAIL=1
import json, sys
out, want = sys.argv[1], sys.argv[2].split()
inc = json.load(open(f"{out}/incidents.json"))
pipe = json.load(open(f"{out}/pipeline.json"))
seen = set()
for i in inc:
    d = i.get("details") or {}
    if d.get("playbook_id"):
        seen.add(d["playbook_id"])
    for t in i.get("titles") or []:
        seen.add(t)
fired = [w for w in want if w in seen or any(w in json.dumps(i) for i in inc)]
print(f"incidents: {len(inc)}, expected playbooks with an incident: {len(fired)} of {len(want)}")
for w in want:
    print(f"  {'ok  ' if w in fired else 'MISS'} {w}")
rp = pipe.get("record_pipeline", {})
print(f"facts queue dropped {pipe['adapter_queue']['dropped_total']}, records dropped {pipe['records_dropped_total']}, "
      f"rules evaluated {rp.get('rules_evaluated_total')}, rule hits {rp.get('rule_hits_total')}")
sys.exit(0 if len(fired) == len(want) else 1)
PY
KV="$RUN/telemetry_output/kv_telemetry.jsonl"
if grep -aq '"sni":"ci-smoke.example.test"' "$KV" 2>/dev/null; then
  echo "tls: the ClientHello's server name reached kv_telemetry.jsonl"
else
  fail "no TLS hello with sni ci-smoke.example.test in $KV"
fi

echo "== stop"
if [ "$GUARD" = enforce ]; then
  # Self protection refuses signals to the agent until the off file exists.
  mkdir -p /run/edr-guard && touch /run/edr-guard/off
  for _ in $(seq 1 25); do grep -aq "self protection turned off" "$OUT/agent.log" && break; sleep 0.2; done
fi
kill -INT $AGENT 2>/dev/null
for _ in $(seq 1 50); do kill -0 $AGENT 2>/dev/null || break; sleep 0.2; done
kill -9 $AGENT 2>/dev/null
cp "$RUN"/logs/*.jsonl "$OUT/" 2>/dev/null
# The shields' pins outlive the agent on purpose. This run made none, but the agent
# pinned the empty shield programs, so they go too.
rm -rf "$PIN" /sys/fs/bpf/edr_shield
rm -f /run/edr-guard/off
# The runner uploads $OUT as a normal user, and some logs are written 0600.
chmod -R a+rX "$OUT"
[ $FAIL = 0 ] && echo "live smoke: PASS" || echo "live smoke: FAIL"
exit $FAIL
