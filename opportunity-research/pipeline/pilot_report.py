"""Write data/pilot_report.md from the store: calibration facts for the full run, no opportunity claims."""
from __future__ import annotations

import json
import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from pipeline import store  # noqa: E402
from pipeline.clean import canonical_key  # noqa: E402
from pipeline.sources import serp  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]


def main() -> None:
    con = store.connect()
    out = ["# Pilot report (calibration only)\n",
           "Measured on the pilot expansion; every number below is read from `data/research.db`. "
           "Nothing here is a demand estimate: no Keyword Planner data has been imported yet.\n"]
    per_seed = con.execute("SELECT seed, domain, subdomain, COUNT(*), SUM(status='kept'), SUM(status='variant'), SUM(status NOT IN ('kept','variant','raw')) FROM queries GROUP BY seed ORDER BY 4 DESC").fetchall()
    out.append("## Queries per seed\n\n| seed | domain / subdomain | raw | kept | variants | filtered |\n|---|---|---|---|---|---|")
    for seed, d, sd, n, k, v, f in per_seed:
        out.append(f"| {seed} | {d} / {sd} | {n} | {k or 0} | {v or 0} | {f or 0} |")
    status = dict(con.execute("SELECT status, COUNT(*) FROM queries GROUP BY status").fetchall())
    out.append(f"\nStatus totals: {json.dumps(status)}\n")
    src = Counter(r[0] for r in con.execute("SELECT source FROM observations"))
    corroboration = Counter(r[0] for r in con.execute("SELECT COUNT(DISTINCT source) FROM observations GROUP BY query"))
    out.append(f"Observations by source: {dict(src)}\n\nQueries by number of distinct sources that suggested them: {dict(sorted(corroboration.items()))}\n")
    intents = Counter(r[0] for r in con.execute("SELECT json_extract(meta,'$.intent_probe') FROM observations WHERE meta IS NOT NULL"))
    out.append(f"Suggestions by intent probe family: {dict(intents.most_common())}\n")
    # drift: kept queries whose canonical tokens share nothing with the seed's tokens
    drift = []
    for seed, q in con.execute("SELECT seed, query FROM queries WHERE status='kept' AND depth=1"):
        st = set(canonical_key(seed).split()); qt = set(canonical_key(q).split())
        if st and not (st & qt):
            drift.append((seed, q))
    out.append(f"## Seed drift\n\n{len(drift)} kept depth-1 queries share no canonical token with their seed (a sign the seed was ambiguous or the probe pulled a neighbouring topic). Examples:\n")
    for seed, q in drift[:15]:
        out.append(f"- `{seed}` -> `{q}`")
    clusters = con.execute("SELECT c.cluster_id, c.label, c.intent, c.domain, COUNT(m.query) FROM clusters c JOIN cluster_members m ON m.cluster_id=c.cluster_id WHERE c.cluster_id!='_variants' GROUP BY c.cluster_id ORDER BY 5 DESC").fetchall()
    sizes = Counter(n for *_, n in clusters)
    out.append(f"\n## Lexical pre-clustering\n\n{len(clusters)} clusters of size >= 3; size distribution (size: count): {dict(sorted(sizes.items()))}\n")
    out.append("Largest pre-clusters (unreviewed; a lexical component, not yet a problem):\n\n| cluster | label | intent | domain | queries | sample |\n|---|---|---|---|---|---|")
    for cid, label, intent, dom, n in clusters[:20]:
        sample = [r[0] for r in con.execute("SELECT query FROM cluster_members WHERE cluster_id=? LIMIT 3", (cid,))]
        out.append(f"| {cid} | {label} | {intent} | {dom} | {n} | {'; '.join(sample)} |")
    met = con.execute("SELECT provider, kind, COUNT(*), AVG(value) FROM metrics GROUP BY provider, kind").fetchall()
    out.append("\n## Proxies collected on a sample (never summed with search volume)\n\n| provider | kind | queries | mean value |\n|---|---|---|---|")
    for p, k, n, avg in met:
        out.append(f"| {p} | {k} | {n} | {avg:.1f} |" if avg is not None else f"| {p} | {k} | {n} | - |")
    out.append("\n## SERP mixes on a sample\n\n| engine | query | fragmented share | dominant site (share) | categories |\n|---|---|---|---|---|")
    for engine, q in con.execute("SELECT DISTINCT engine, query FROM serp ORDER BY engine, query"):
        rows = [{"category": r[0], "site": r[1]} for r in con.execute("SELECT category, site FROM serp WHERE query=? AND engine=?", (q, engine))]
        m = serp.mix(rows)
        out.append(f"| {engine} | {q} | {m['fragmented_share']:.2f} | {m['dominant_site']} ({m['dominant_share']:.2f}) | {m['categories']} |")
    notes = ROOT / "data" / "pilot_notes.md"
    out.append("\n" + (notes.read_text() if notes.exists() else "## What the pilot changes in the full-run plan\n\n(no analyst notes yet)\n"))
    (ROOT / "data" / "pilot_report.md").write_text("\n".join(out) + "\n")
    print("wrote data/pilot_report.md")


if __name__ == "__main__":
    main()
