"""Cross-domain view: task-intent counts per domain over the kept queries, and cross-domain intent families
(the same task intent recurring in many domains, e.g. "does X work with Y"), which lexical clustering splits
by domain. Output: data/exports/intent_matrix.md. Counts are query counts in this store, not search volumes.
"""
from __future__ import annotations

import collections
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from pipeline import store  # noqa: E402
from pipeline.cluster import INTENT, intent_of  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
INTENTS = [name for name, _ in INTENT] + ["informational"]


def run(con, tag: str = "pass2") -> Path:
    rows = con.execute("SELECT query, domain FROM queries WHERE status='kept' OR status IS NULL").fetchall()
    matrix: dict[str, collections.Counter] = collections.defaultdict(collections.Counter)
    for q, dom in rows:
        matrix[dom or "?"][intent_of(q)] += 1
    task = [i for i in INTENTS if i not in ("informational", "commercial_nav")]
    md = [f"# Intent by domain ({tag}; {len(rows)} kept queries in this store; counts are stored queries, not search volume)\n",
          "| domain | kept | task share | " + " | ".join(task) + " |", "|---|---|---|" + "---|" * len(task)]
    for dom, c in sorted(matrix.items(), key=lambda kv: -sum(kv[1].values())):
        n = sum(c.values()); t = sum(c[i] for i in task)
        md.append(f"| {dom} | {n} | {t / n:.2f} | " + " | ".join(str(c[i]) for i in task) + " |")
    md.append("\n## Cross-domain families\n")
    md.append("Task intents that recur in many domains. `domains` is the number of domains with at least five such queries; "
              "these are candidates for one product spanning domains, which per-domain clustering cannot surface.\n")
    md.append("| intent | queries | domains (>=5) | top domains |\n|---|---|---|---|")
    for i in task:
        per = {d: c[i] for d, c in matrix.items() if c[i] >= 5}
        top = ", ".join(f"{d} {n}" for d, n in sorted(per.items(), key=lambda kv: -kv[1])[:8])
        md.append(f"| {i} | {sum(c[i] for c in matrix.values())} | {len(per)} | {top} |")
    p = ROOT / "data" / "exports" / f"intent_matrix_{tag}.md"
    p.write_text("\n".join(md) + "\n")
    return p


if __name__ == "__main__":
    print(run(store.connect(), sys.argv[1] if len(sys.argv) > 1 else "pass2"))
