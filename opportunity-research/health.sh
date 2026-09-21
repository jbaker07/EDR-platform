#!/bin/sh
# Progress and engine health. Timestamps in the store are ISO-8601 with 'T'; compare with the same format.
cd "$(dirname "$0")"
.venv/bin/python - <<'PY'
import sqlite3
con = sqlite3.connect("data/research.db", timeout=120)
n = con.execute("SELECT COUNT(*) FROM queries").fetchone()[0]
print("queries", n, "kept-or-raw", con.execute("SELECT COUNT(*) FROM queries WHERE status IN ('raw','kept')").fetchone()[0])
for src, c in con.execute("SELECT source, COUNT(*) FROM observations WHERE observed_at > strftime('%Y-%m-%dT%H:%M:%S','now','-10 minutes') GROUP BY source"):
    print("last 10 min", src, c)
print("seeds expanded", con.execute("SELECT COUNT(DISTINCT seed) FROM queries WHERE depth=2").fetchone()[0],
      "subdomains touched", con.execute("SELECT COUNT(DISTINCT domain||'/'||subdomain) FROM queries").fetchone()[0])
PY
echo "workers alive: $(pgrep -f 'pipeline.cli expand' | wc -l)"
