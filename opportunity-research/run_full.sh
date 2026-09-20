#!/bin/sh
# Full Phase-2 expansion, one worker per domain, resumable: a seed already present in queries.seed is skipped.
# Usage: ./run_full.sh [workers] [extra cli args]      e.g. ./run_full.sh 4 --no-soup --depth2 30
# Each worker paces itself per host (pipeline/http.py), so N workers ≈ N× throughput until an engine throttles;
# the measured pilot rate was ~2 minutes per seed with --no-soup and depth2 15 across three engines.
set -eu
WORKERS="${1:-3}"; shift || true
mkdir -p data/logs
.venv/bin/python - <<'PY' > data/logs/domains.txt
import yaml; t = yaml.safe_load(open("taxonomy/domains.yaml"))
print("\n".join(d["id"] for d in t["domains"]))
PY
xargs -P "$WORKERS" -I{} sh -c '.venv/bin/python -m pipeline.cli expand --domain {} '"$*"' > data/logs/expand_{}.log 2>&1 && echo "done {}" || echo "FAILED {}"' < data/logs/domains.txt
