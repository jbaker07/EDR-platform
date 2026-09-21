"""Free demand signals per cluster, kept apart from (absent) volumes.

* corroboration: members suggested by 2 or 3 engines (existence strength);
* google_relevance: Google's own autocomplete relevance score (chrome client) -- max and mean over
  members that carry one; a within-query ordering signal, not a volume;
* trends_chain: the cluster head term's relative interest on the chained Trends scale, if placed.
The cluster head is the shortest member that contains the two highest-IDF label tokens, which is
what a Trends comparison needs (a term, not a long question).
"""
from __future__ import annotations

import json
from collections import Counter


def head_term(con, cluster_id: str) -> str | None:
    label = con.execute("SELECT label FROM clusters WHERE cluster_id=?", (cluster_id,)).fetchone()
    if not label:
        return None
    toks = label[0].split()[:2]
    members = [r[0] for r in con.execute("SELECT query FROM cluster_members WHERE cluster_id=?", (cluster_id,))]
    cands = [m for m in members if all(t[:4] in m for t in toks)]
    return min(cands or members, key=len) if (cands or members) else None


def signals(con, cluster_id: str) -> dict:
    members = [r[0] for r in con.execute("SELECT query FROM cluster_members WHERE cluster_id=?", (cluster_id,))]
    if not members:
        return {}
    q = ",".join("?" * len(members))
    per_query_sources = Counter()
    rel = []
    for query, source, meta in con.execute(f"SELECT query, source, meta FROM observations WHERE query IN ({q})", members):
        per_query_sources[(query, source)] += 1
        if meta:
            r = json.loads(meta).get("relevance")
            if isinstance(r, (int, float)):
                rel.append(r)
    n_sources = Counter(qs for qs, _ in per_query_sources)  # (query, source) pairs -> per query count
    by_query = Counter(query for (query, _) in per_query_sources)
    head = head_term(con, cluster_id)
    chain = con.execute("SELECT value, extra FROM metrics WHERE query=? AND provider='google_trends' AND kind='relative_interest_chain' ORDER BY id DESC LIMIT 1", (head,)).fetchone() if head else None
    return {
        "members": len(members),
        "corroborated_2plus": sum(1 for v in by_query.values() if v >= 2),
        "corroborated_3": sum(1 for v in by_query.values() if v >= 3),
        "google_relevance_max": max(rel) if rel else None,
        "google_relevance_mean": round(sum(rel) / len(rel), 1) if rel else None,
        "head_term": head,
        "trends_chain_value": chain[0] if chain else None,
        "trends_chain_meta": json.loads(chain[1]) if chain and chain[1] else None,
    }
