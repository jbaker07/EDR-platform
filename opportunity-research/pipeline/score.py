"""Phase 5/6 metrics per cluster, computed only from what is in the store.

Demand: sum of Keyword Planner avg_monthly_searches over members that have a value, with the
number of members lacking a value reported alongside (never imputed). Long-tail breadth: member
count and distinct canonical keys. Proxies (kept separate, never summed with volume):
suggest corroboration, Stack Exchange totals, YouTube estimated results, Trends relative interest.
Fragmentation: from stored SERP rows per member query (engine named), the share of top results
that are community/forum/video/blog and the dominant site's share.
"""
from __future__ import annotations

import json
from collections import Counter, defaultdict


def cluster_report(con, cluster_id: str) -> dict:
    members = [r[0] for r in con.execute("SELECT query FROM cluster_members WHERE cluster_id=?", (cluster_id,))]
    meta = con.execute("SELECT label, intent, domain, method, confidence, description FROM clusters WHERE cluster_id=?", (cluster_id,)).fetchone()
    vol = con.execute("""SELECT query, MAX(value) FROM metrics WHERE provider='google_keyword_planner' AND kind='avg_monthly_searches'
                         AND query IN (%s) GROUP BY query""" % ",".join("?" * len(members)), members).fetchall() if members else []
    volumes = {q: v for q, v in vol if v is not None}
    keys = {r[0] for r in con.execute("SELECT DISTINCT canonical_key FROM cluster_members WHERE cluster_id=?", (cluster_id,))}
    proxies = defaultdict(float)
    for q, provider, kind, value in con.execute("""SELECT query, provider, kind, value FROM metrics WHERE query IN (%s)
                                                   AND provider != 'google_keyword_planner'""" % ",".join("?" * len(members)), members):
        if value is not None:
            proxies[f"{provider}:{kind}"] += value
    obs = Counter(r[0] for r in con.execute("SELECT source FROM observations WHERE query IN (%s)" % ",".join("?" * len(members)), members))
    serp_rows = con.execute("SELECT engine, category, site FROM serp WHERE query IN (%s)" % ",".join("?" * len(members)), members).fetchall()
    frag = {}
    for engine in {r[0] for r in serp_rows}:
        rows = [r for r in serp_rows if r[0] == engine]
        cats = Counter(r[1] for r in rows)
        sites = Counter(r[2] for r in rows)
        fragmented = sum(v for k, v in cats.items() if k in ("reddit", "stackexchange", "forum", "youtube", "github", "blog_editorial", "qa_platform", "video_other", "independent_site", "recipe_site", "vendor_content"))
        frag[engine] = {"results": len(rows), "categories": dict(cats), "fragmented_share": round(fragmented / len(rows), 2),
                        "dominant_site": sites.most_common(1)[0][0], "dominant_site_share": round(sites.most_common(1)[0][1] / len(rows), 2)}
    return {
        "cluster_id": cluster_id, "label": meta[0], "intent": meta[1], "domain": meta[2], "method": meta[3], "confidence": meta[4],
        "members": len(members), "distinct_canonical_keys": len(keys),
        "demand": {"provider": "google_keyword_planner", "sum_avg_monthly_searches": sum(volumes.values()),
                   "members_with_volume": len(volumes), "members_without_volume": len(members) - len(volumes),
                   "top": sorted(volumes.items(), key=lambda kv: -kv[1])[:10]},
        "proxies": dict(proxies), "observation_sources": dict(obs), "fragmentation": frag,
        "qualitative": {k: None for k in ("incumbent_strength", "answer_quality", "productizability", "repeatability",
                                          "maintenance_burden", "ai_resilience", "serp_resilience", "monetization")},
    }


def all_clusters(con) -> list[dict]:
    ids = [r[0] for r in con.execute("SELECT cluster_id FROM clusters WHERE cluster_id != '_variants'")]
    return sorted((cluster_report(con, c) for c in ids), key=lambda r: (-r["demand"]["sum_avg_monthly_searches"], -r["members"]))
