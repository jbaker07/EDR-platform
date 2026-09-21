#!/bin/sh
# Export the raw tables (compressed) and commit them, so a lost container loses at most one interval.
set -eu
cd "$(dirname "$0")"
.venv/bin/python - <<'PY'
import csv, gzip, sqlite3
con = sqlite3.connect("data/research.db", timeout=120)
for table in ("queries", "observations", "metrics", "serp", "clusters", "cluster_members"):
    cur = con.execute(f"SELECT * FROM {table}")
    with gzip.open(f"data/exports/raw_{table}.csv.gz", "wt", newline="") as fh:
        w = csv.writer(fh); w.writerow([d[0] for d in cur.description]); w.writerows(cur)
n = con.execute("SELECT COUNT(*) FROM queries").fetchone()[0]
print(f"checkpoint: {n} queries")
PY
cd .. && git add opportunity-research/data/exports opportunity-research/data/logs opportunity-research/pipeline opportunity-research/*.sh opportunity-research/README.md opportunity-research/taxonomy opportunity-research/data/pilot_notes.md opportunity-research/data/trends_chain_state.json 2>/dev/null || true
git -c user.name="Jermaine Baker" -c user.email="jermainebaker1512@gmail.com" commit -q -m "opportunity-research: expansion checkpoint $(date -u +%Y-%m-%dT%H:%MZ)

Co-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_01HUfEp6TTNp3nTvWF78qAmv" 2>/dev/null && git push -q origin claude/modcheck-platform-foundation-mcrrrw 2>&1 | tail -1 || echo "nothing to commit"
