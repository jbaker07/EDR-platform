"""Shortlist export: clusters ranked by the free signals, with everything needed to audit each one.

Ranking on the free route (stated on every row): Trends chain value of the head term when
placed, else corroborated-member count, then member count. No volume appears. Writes
data/exports/shortlist.md and data/exports/shortlist.json.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from pipeline import demand_free, store  # noqa: E402
from pipeline.sources import serp  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]


def build(con, top: int = 100, min_members: int = 5) -> list[dict]:
    rows = con.execute("""SELECT c.cluster_id, c.label, c.intent, c.domain, c.confidence, COUNT(m.query)
                          FROM clusters c JOIN cluster_members m ON m.cluster_id=c.cluster_id
                          WHERE c.cluster_id != '_variants' GROUP BY c.cluster_id HAVING COUNT(m.query) >= ?""", (min_members,)).fetchall()
    out = []
    for cid, label, intent, domain, conf, n in rows:
        sig = demand_free.signals(con, cid)
        members = [r[0] for r in con.execute("SELECT query FROM cluster_members WHERE cluster_id=? ORDER BY LENGTH(query) LIMIT 8", (cid,))]
        subdomains = [r[0] for r in con.execute("""SELECT DISTINCT q.subdomain FROM cluster_members m JOIN queries q ON q.query=m.query
                                                    WHERE m.cluster_id=?""", (cid,))]
        intents = dict(con.execute("""SELECT '', COUNT(*) FROM cluster_members WHERE cluster_id=?""", (cid,)).fetchall())
        serp_rows = con.execute("""SELECT s.engine, s.category, s.site FROM serp s JOIN cluster_members m ON m.query=s.query WHERE m.cluster_id=?""", (cid,)).fetchall()
        frag = {}
        for engine in {r[0] for r in serp_rows}:
            frag[engine] = serp.mix([{"category": r[1], "site": r[2]} for r in serp_rows if r[0] == engine])
        out.append({"cluster_id": cid, "label": label, "intent": intent, "domain": domain, "subdomains": subdomains, "confidence": conf,
                    "members": n, "signals": sig, "sample": members, "fragmentation": frag,
                    "demand_basis": "relative: trends chain" if sig.get("trends_chain_value") is not None else "relative: corroboration only",
                    "volume": None})
    out.sort(key=lambda r: (-(r["signals"].get("trends_chain_value") or 0), -r["signals"].get("corroborated_2plus", 0), -r["members"]))
    return out[:top]


def write(con, top: int = 100) -> Path:
    rows = build(con, top)
    (ROOT / "data" / "exports" / "shortlist.json").write_text(json.dumps(rows, indent=1))
    md = ["# Shortlist (free route: relative demand only)\n",
          "No monthly search volume exists in this dataset. `trends` is the head term's relative interest on the chained Google Trends scale "
          "(anchor value 100 = the root anchor), with the rounding error of its placement; `corr` is members suggested by two or more engines. "
          "Clusters are lexical components reviewed as noted in `confidence`.\n",
          "| # | cluster | domain | intent | members | corr | trends (±%) | head term | sample |", "|---|---|---|---|---|---|---|---|---|"]
    for i, r in enumerate(rows, 1):
        s = r["signals"]
        tv = f"{s['trends_chain_value']:.1f} (±{(s.get('trends_chain_meta') or {}).get('error_pct', '?')})" if s.get("trends_chain_value") is not None else "-"
        md.append(f"| {i} | {r['label']} | {r['domain']} | {r['intent']} | {r['members']} | {s.get('corroborated_2plus', 0)} | {tv} | {s.get('head_term') or '-'} | {'; '.join(r['sample'][:3])} |")
    p = ROOT / "data" / "exports" / "shortlist.md"
    p.write_text("\n".join(md) + "\n")
    return p
