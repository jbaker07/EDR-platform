#!/usr/bin/env bash
set -euo pipefail

# Find the binary if not passed
BIN="${1:-}"
if [[ -z "${BIN}" ]]; then
  if [[ -x "./edr-agent/target/debug/forensic_hooks" ]]; then
    BIN="./edr-agent/target/debug/forensic_hooks"
  elif [[ -x "./target/debug/forensic_hooks" ]]; then
    BIN="./target/debug/forensic_hooks"
  else
    echo "❌ forensic_hooks binary not found. Pass path as arg, e.g.:"
    echo "   ./debug_rules.sh ./edr-agent/target/debug/forensic_hooks"
    exit 1
  fi
fi

echo "== Code search (where it's loaded) =="
rg -n --hidden -S 'rules detector not loaded|add_rules_from|detection_rules\.json|rules\.json|RulesConfig|RulesEngine' -g '!target' . || true

echo
echo "== Runtime file opens (strace) =="
# warm sudo so strace isn't blocked
sudo -E true || true

RUN=(sudo -E prlimit --memlock=unlimited -- "${BIN}")
if command -v timeout >/dev/null 2>&1; then
  RUN=(timeout 7s "${RUN[@]}")
fi

strace -f -e trace=openat,statx -s 200 \
  "${RUN[@]}" 2>&1 \
| grep -E 'detection_rules\.json|rules\.json|policy\.json|state/calibration' \
| sed -n '1,160p'
