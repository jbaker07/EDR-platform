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


QUESTION_WORDS = ("how ", "why ", "what ", "when ", "where ", "which ", "who ", "can ", "does ", "do ", "is ", "are ", "should ")


def head_term(con, cluster_id: str) -> str | None:
    """A Trends-suitable head: 1-3 tokens, not a question, containing the label's top token,
    preferring members suggested by several engines. Falls back to the label's top two tokens."""
    label = con.execute("SELECT label FROM clusters WHERE cluster_id=?", (cluster_id,)).fetchone()
    if not label:
        return None
    top = label[0].split()[:2]
    members = [r[0] for r in con.execute("SELECT query FROM cluster_members WHERE cluster_id=?", (cluster_id,))]
    if not members:
        return None
    q = ",".join("?" * len(members))
    corr = {row[0]: row[1] for row in con.execute(f"SELECT query, COUNT(DISTINCT source) FROM observations WHERE query IN ({q}) GROUP BY query", members)}
    from pipeline.clean import canonical_key, topic_key
    def ok(m):
        # short, not a question, carries the label's top token, and contains no generic/intent word
        return (1 <= len(m.split()) <= 3 and not m.startswith(QUESTION_WORDS) and top[0][:4] in m
                and canonical_key(m) == topic_key(m))
    cands = [m for m in members if ok(m)]
    if cands:
        return max(cands, key=lambda m: (corr.get(m, 0), -len(m)))
    # no short non-question member: take the most corroborated member of at most 5 tokens, else nothing
    short = [m for m in members if len(m.split()) <= 5]
    return max(short, key=lambda m: (corr.get(m, 0), -len(m))) if short else None


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
