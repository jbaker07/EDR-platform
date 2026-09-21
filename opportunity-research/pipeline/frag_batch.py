"""Phase 6 batch builder: pick representative problem-intent queries per candidate cluster for SERP checks.

Candidate set = union of the top clusters by organic problem members, by problem members and by
corroboration (members >= 20, problem share >= 0.45). Two queries per cluster: organic problem-intent
first, most corroborated, different intents where possible. Queries that already have rows from the
given engine are skipped, so re-running after re-clustering only adds what is new.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from pipeline import rank, store  # noqa: E402
from pipeline.cluster import intent_of  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]


def build(con, engine="websearch_tool", per_cluster=2, top_each=(100, 100, 60), min_members=20, min_share=0.45):
    m = [r for r in rank.metrics(con, min_members) if r["problem_share"] >= min_share]
    chosen = {}
    for key, n in zip(("organic_problem_members", "problem_members", "corroborated_2plus"), top_each):
        for r in sorted(m, key=lambda r: -r[key])[:n]:
            chosen[r["cluster_id"]] = r
    done = {r[0] for r in con.execute("SELECT DISTINCT query FROM serp WHERE engine=?", (engine,))}
    corr = dict(con.execute("SELECT query, COUNT(DISTINCT source) FROM observations GROUP BY query").fetchall())
    items = []
    for cid, r in chosen.items():
        # problem_examples are already ordered organic-first, then by corroboration
        picks, intents = [], set()
        pool = [(q, it) for q, it in r["problem_examples"]]
        pool += [(q, intent_of(q)) for (q,) in con.execute("SELECT query FROM cluster_members WHERE cluster_id=?", (cid,))
                 if intent_of(q) not in ("informational", "commercial_nav")]
        pool = sorted(dict(pool).items(), key=lambda kv: (kv[0] not in dict(r["problem_examples"]), -corr.get(kv[0], 0)))
        for q, it in pool:
            if it in intents and len(pool) > per_cluster:
                continue
            picks.append((q, it)); intents.add(it)
            if len(picks) == per_cluster:
                break
        for q, it in picks:
            if q not in done:
                items.append({"cluster_id": cid, "head_term": r["head_term"], "query": q, "intent": it})
    return items


if __name__ == "__main__":
    con = store.connect()
    tag = sys.argv[1] if len(sys.argv) > 1 else "pass1"
    parts = int(sys.argv[2]) if len(sys.argv) > 2 else 6
    items = build(con)
    out = ROOT / "data" / "exports" / f"frag_batch_{tag}"
    out.mkdir(parents=True, exist_ok=True)
    for old in out.glob("batch_*.json"):
        old.unlink()
    for k in range(parts):
        (out / f"batch_{k + 1}.json").write_text(json.dumps(items[k::parts], indent=1))
    print(json.dumps({"clusters": len({i['cluster_id'] for i in items}), "queries": len(items), "parts": parts, "dir": str(out)}))
