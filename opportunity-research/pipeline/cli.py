"""Command line: expand | clean | cluster | enrich | serp | score | import-kp | stats."""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import yaml  # noqa: E402

from pipeline import clean, cluster, expand, score, store  # noqa: E402
from pipeline.sources import kp_import, serp, stackexchange, trends  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]


def cmd_expand(a):
    tax = yaml.safe_load((ROOT / "taxonomy" / "domains.yaml").read_text())
    con = store.connect()
    for d in tax["domains"]:
        if a.domain and d["id"] != a.domain:
            continue
        for sd in d["subdomains"]:
            if a.subdomain and sd["id"] != a.subdomain:
                continue
            for seed in sd["seeds"][: a.seeds_per_subdomain]:
                r = expand.expand_seed(con, seed, domain=d["id"], subdomain=sd["id"], soup=not a.no_soup,
                                       depth2_cap=a.depth2, sources=tuple(a.sources.split(",")))
                print(json.dumps(r), flush=True)


def cmd_clean(a):
    print(json.dumps(clean.run(store.connect())))


def cmd_cluster(a):
    print(json.dumps(cluster.run(store.connect(), threshold=a.threshold, min_size=a.min_size, domain=a.domain)))


def cmd_enrich(a):
    """Proxies for a sample of kept queries: Stack Exchange totals, YouTube estimated results."""
    con = store.connect()
    rows = con.execute("SELECT query FROM queries WHERE status='kept' ORDER BY RANDOM() LIMIT ?", (a.limit,)).fetchall()
    for (q,) in rows:
        if "se" in a.kinds:
            t = stackexchange.total(q, site=a.se_site)
            if t is not None:
                store.add_metric(con, q, "stackexchange", "se_question_total", t, extra={"site": a.se_site})
        if "yt" in a.kinds:
            y = serp.youtube_results(q)
            if y.get("estimated_results") is not None:
                store.add_metric(con, q, "youtube", "estimated_results", y["estimated_results"], extra={"top": y["top"][:3]})
        con.commit()
        print(q, flush=True)


def cmd_serp(a):
    con = store.connect()
    rows = con.execute("SELECT query FROM queries WHERE status='kept' ORDER BY RANDOM() LIMIT ?", (a.limit,)).fetchall()
    for (q,) in rows:
        results = serp.bing(q)
        if results:
            store.add_serp(con, q, "bing_html", results)
            con.commit()
            print(q, json.dumps(serp.mix(results)), flush=True)


def cmd_score(a):
    reports = score.all_clusters(store.connect())
    out = ROOT / "data" / "exports" / "clusters.json"
    out.write_text(json.dumps(reports, indent=1))
    for r in reports[: a.top]:
        print(f"{r['cluster_id']}  {r['members']:>4} q  {r['distinct_canonical_keys']:>4} keys  "
              f"vol={r['demand']['sum_avg_monthly_searches']:.0f} ({r['demand']['members_with_volume']} w/ vol)  "
              f"{r['intent']:<13} {r['domain']:<18} {r['label']}")
    print(f"wrote {out}")


def cmd_import_kp(a):
    n = kp_import.import_file(store.connect(), Path(a.file), a.geo, a.lang, a.note)
    print(f"imported {n} keywords from {a.file}")


def cmd_stats(a):
    con = store.connect()
    for row in con.execute("SELECT status, COUNT(*) FROM queries GROUP BY status"):
        print("queries", row)
    print("observations", con.execute("SELECT COUNT(*) FROM observations").fetchone()[0])
    for row in con.execute("SELECT provider, kind, COUNT(*) FROM metrics GROUP BY provider, kind"):
        print("metrics", row)
    for row in con.execute("SELECT engine, COUNT(DISTINCT query) FROM serp GROUP BY engine"):
        print("serp", row)
    print("clusters", con.execute("SELECT COUNT(*) FROM clusters").fetchone()[0])


def main():
    p = argparse.ArgumentParser()
    sub = p.add_subparsers(dest="cmd", required=True)
    e = sub.add_parser("expand"); e.add_argument("--domain"); e.add_argument("--subdomain"); e.add_argument("--seeds-per-subdomain", type=int, default=99)
    e.add_argument("--no-soup", action="store_true"); e.add_argument("--depth2", type=int, default=40); e.add_argument("--sources", default="google,youtube,bing"); e.set_defaults(fn=cmd_expand)
    sub.add_parser("clean").set_defaults(fn=cmd_clean)
    c = sub.add_parser("cluster"); c.add_argument("--threshold", type=float, default=0.5); c.add_argument("--min-size", type=int, default=3); c.add_argument("--domain"); c.set_defaults(fn=cmd_cluster)
    en = sub.add_parser("enrich"); en.add_argument("--limit", type=int, default=20); en.add_argument("--kinds", default="se,yt"); en.add_argument("--se-site", default="stackoverflow"); en.set_defaults(fn=cmd_enrich)
    s = sub.add_parser("serp"); s.add_argument("--limit", type=int, default=20); s.set_defaults(fn=cmd_serp)
    sc = sub.add_parser("score"); sc.add_argument("--top", type=int, default=30); sc.set_defaults(fn=cmd_score)
    k = sub.add_parser("import-kp"); k.add_argument("file"); k.add_argument("--geo", default="US"); k.add_argument("--lang", default="en"); k.add_argument("--note", default=""); k.set_defaults(fn=cmd_import_kp)
    sub.add_parser("stats").set_defaults(fn=cmd_stats)
    a = p.parse_args()
    a.fn(a)


if __name__ == "__main__":
    main()
