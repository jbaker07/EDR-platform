#!/bin/sh
# Delta-export the raw tables and commit them, so a lost container loses at most one interval.
# Pass --full at pass boundaries to also snapshot clusters and cluster_members.
set -eu
cd "$(dirname "$0")"
.venv/bin/python -m pipeline.checkpoint "$@"
cd .. && git add opportunity-research/.gitignore opportunity-research/data/exports opportunity-research/data/logs opportunity-research/pipeline opportunity-research/*.sh opportunity-research/README.md opportunity-research/taxonomy opportunity-research/data/pilot_notes.md opportunity-research/data/pilot_report.md opportunity-research/data/trends_chain_state.json opportunity-research/data/competitors opportunity-research/data/top20 opportunity-research/data/tasks opportunity-research/data/lineage opportunity-research/audit opportunity-research/requirements.txt opportunity-research/ENVIRONMENT.md 2>/dev/null || true
git -c user.name="Jermaine Baker" -c user.email="jermainebaker1512@gmail.com" commit -q -m "opportunity-research: expansion checkpoint $(date -u +%Y-%m-%dT%H:%MZ)

Co-Authored-By: Claude Opus 5 <noreply@anthropic.com>
Claude-Session: https://claude.ai/code/session_01HUfEp6TTNp3nTvWF78qAmv" 2>/dev/null && git push -q origin claude/modcheck-platform-foundation-mcrrrw 2>&1 | tail -1 || echo "nothing to commit"
