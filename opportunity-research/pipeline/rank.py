"""Problem-oriented candidate ranking with visible metrics and no single score.

Every cluster gets the same set of measured columns; the ordering rule is stated in the output header.
Nothing here is a search volume. `problem_members` counts members whose intent regex (cluster.INTENT)
is a task intent (fix, how-to, compare, decide, calculate, ...) rather than plain informational; it is
the free proxy for "people are trying to get something done", the brief's core filter.
"""
from __future__ import annotations

import collections
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from pipeline import demand_free  # noqa: E402
from pipeline.cluster import intent_of  # noqa: E402
from pipeline.sources import serp  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
ORDER = "organic_problem_members desc, corroborated_2plus desc, members desc (Trends and fragmentation shown, not used for order until placed/measured for all rows)"


def metrics(con, min_members: int = 15) -> list[dict]:
    corr = dict(con.execute("SELECT query, COUNT(DISTINCT source) FROM observations GROUP BY query").fetchall())
    rows = con.execute("""SELECT m.cluster_id, m.query, c.label, c.domain, c.confidence, q.subdomain
                          FROM cluster_members m JOIN clusters c ON c.cluster_id=m.cluster_id
                          LEFT JOIN queries q ON q.query=m.query WHERE m.cluster_id!='_variants'""").fetchall()
    # organic problem intent: the engine volunteered the task words on a bare probe (the seed itself, or a
    # depth-2 re-probe of a query that is itself plain informational). Echoes of my own intent probes
    # ("fix X" -> "fix X on windows") do not count, since every seed gets the same 50 probes.
    members = {r[1] for r in rows}
    organic: set[str] = set()
    for q, probe, meta in con.execute("SELECT query, probe, meta FROM observations"):
        if q not in members or q in organic:
            continue
        m = json.loads(meta) if meta else {}
        ip = m.get("intent_probe")
        if ip == "bare" or (ip == "depth2" and intent_of(probe) in ("informational", "commercial_nav")):
            organic.add(q)
    agg: dict[str, dict] = {}
    for cid, q, label, dom, conf, sub in rows:
        a = agg.setdefault(cid, {"cluster_id": cid, "label": label, "domain": dom, "confidence": conf, "members": 0,
                                 "problem_members": 0, "organic_problem_members": 0, "corroborated_2plus": 0,
                                 "intents": collections.Counter(), "subdomains": set(), "problem_examples": []})
        a["members"] += 1
        it = intent_of(q); a["intents"][it] += 1
        c = corr.get(q, 0)
        if c >= 2:
            a["corroborated_2plus"] += 1
        if it not in ("informational", "commercial_nav"):
            a["problem_members"] += 1
            org = q in organic
            a["organic_problem_members"] += org
            a["problem_examples"].append((-org, -(c), len(q), q, it))
        if sub:
            a["subdomains"].add(sub)
    out = []
    for a in agg.values():
        if a["members"] < min_members:
            continue
        a["problem_share"] = round(a["problem_members"] / a["members"], 2)
        a["intents"] = dict(a["intents"].most_common(4))
        a["subdomains"] = sorted(a["subdomains"])
        a["problem_examples"] = [(q, it) for _, _, _, q, it in sorted(a["problem_examples"])[:6]]
        sig = demand_free.signals(con, a["cluster_id"])
        a["head_term"] = sig.get("head_term")
        a["trends_chain_value"] = sig.get("trends_chain_value")
        a["trends_error_pct"] = (sig.get("trends_chain_meta") or {}).get("error_pct")
        serp_rows = con.execute("""SELECT s.engine, s.category, s.site FROM serp s JOIN cluster_members m ON m.query=s.query
                                   WHERE m.cluster_id=?""", (a["cluster_id"],)).fetchall()
        a["fragmentation"] = {e: serp.mix([{"category": r[1], "site": r[2]} for r in serp_rows if r[0] == e])
                              for e in {r[0] for r in serp_rows}}
        out.append(a)
    return out


def write(con, top: int = 150, min_members: int = 20, min_problem_share: float = 0.45, tag: str = "pass1") -> Path:
    rows = [r for r in metrics(con, min_members) if r["problem_share"] >= min_problem_share]
    rows.sort(key=lambda r: (-r["organic_problem_members"], -r["corroborated_2plus"], -r["members"]))
    rows = rows[:top]
    (ROOT / "data" / "exports" / f"candidates_{tag}.json").write_text(json.dumps(rows, indent=1, default=list))
    md = [f"# Candidate problem clusters ({tag}, free route)\n",
          f"Filter: members >= {min_members} and problem-intent share >= {min_problem_share}. Order: {ORDER}. "
          "No monthly search volume exists in this dataset; `trends` is the head term's relative interest on the chained "
          "Google Trends scale (root anchor = 100) with its rounding error, blank when not yet placed. `frag` is the "
          "fragmented share of the web-search results checked so far (blank = not checked). Cluster labels are lexical.\n",
          "| # | head term | domain | members | problem (share) | organic | corr | intents | trends (±%) | frag | problem-intent examples (organic first) |",
          "|---|---|---|---|---|---|---|---|---|---|---|"]
    for i, r in enumerate(rows, 1):
        tv = ""
        if r["trends_chain_value"] is not None:
            err = r["trends_error_pct"] or 0
            tv = f"{r['trends_chain_value']:.1f} (±{err})" if err < 50 else f"~{r['trends_chain_value']:.0f} (unreliable, ±{err})"
        fr = "; ".join(f"{e}:{m.get('fragmented_share', 0):.2f}/{m.get('results', 0)}" for e, m in r["fragmentation"].items())
        ints = ", ".join(f"{k}:{v}" for k, v in r["intents"].items())
        ex = "; ".join(q for q, _ in r["problem_examples"][:3])
        md.append(f"| {i} | {r['head_term'] or r['label']} | {r['domain']} | {r['members']} | {r['problem_members']} ({r['problem_share']}) | "
                  f"{r['organic_problem_members']} | {r['corroborated_2plus']} | {ints} | {tv} | {fr} | {ex} |")
    p = ROOT / "data" / "exports" / f"candidates_{tag}.md"
    p.write_text("\n".join(md) + "\n")
    return p


if __name__ == "__main__":
    from pipeline import store
    con = store.connect()
    tag = sys.argv[1] if len(sys.argv) > 1 else "pass1"
    print(write(con, tag=tag))
