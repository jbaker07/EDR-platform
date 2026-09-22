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
ORDER = ("clusters with >= 10 checked results first, by editorial-plus-community share desc, then tool hits asc, then organic problem members desc; "
         "unchecked clusters after, by organic problem members desc, corroboration desc")


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
        if ip == "bare" or (ip == "depth2" and intent_of(probe) in ("informational", "site_navigation")):
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
        if it not in ("informational", "site_navigation"):
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
        serp_rows = con.execute("""SELECT s.engine, s.category, s.site, s.title, s.query FROM serp s JOIN cluster_members m ON m.query=s.query
                                   WHERE m.cluster_id=? AND s.engine='websearch_tool'""", (a["cluster_id"],)).fetchall()
        a["fragmentation"] = serp.mix([{"category": r[1], "site": r[2], "title": r[3]} for r in serp_rows]) if serp_rows else {}
        a["queries_checked"] = sorted({r[4] for r in serp_rows})
        out.append(a)
    return out


def write(con, top: int = 100, min_members: int = 20, min_problem_share: float = 0.45, tag: str = "pass1") -> Path:
    rows = [r for r in metrics(con, min_members) if r["problem_share"] >= min_problem_share]
    def key(r):
        f = r["fragmentation"]
        checked = f.get("n", 0) >= 10
        return (0 if checked else 1, -(f.get("fragmented_share", 0) if checked else 0), f.get("tool_hits", 0) if checked else 0,
                -r["organic_problem_members"], -r["corroborated_2plus"], -r["members"])
    rows.sort(key=key)
    rows = rows[:top]
    (ROOT / "data" / "exports" / f"candidates_{tag}.json").write_text(json.dumps(rows, indent=1, default=list))
    md = [f"# Candidate problem clusters ({tag}, free route)\n",
          f"Filter: members >= {min_members} and problem-intent share >= {min_problem_share}. Order: {ORDER}. "
          "No monthly search volume exists in this dataset; `trends` is the head term's relative interest on the chained "
          "Google Trends scale (root anchor = 100) with its rounding error, blank when not yet placed. `frag` is the "
          "`ed+comm` is the editorial-plus-community share of the web-search results for the cluster's checked queries (result-source "
          "diversity, measurement A: a description of the result set, not a failure rate), `domains` the distinct domains among them, `tool` the "
          "number of those results from a tool site or with a tool-naming title, `vendor` the product-vendor share, `uncl` the unclassified "
          "share, `n` the results checked (blank = not checked; the web-search tool is a US-only proxy, not Google). Cluster labels are lexical.\n",
          "| # | head term | domain | members | problem (share) | organic | corr | intents | trends (±%) | ed+comm | domains | tool | vendor | uncl | n | problem-intent examples (organic first) |",
          "|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|"]
    for i, r in enumerate(rows, 1):
        tv = ""
        if r["trends_chain_value"] is not None:
            err = r["trends_error_pct"] or 0
            tv = f"{r['trends_chain_value']:.1f} (±{err})" if err < 50 else f"~{r['trends_chain_value']:.0f} (unreliable, ±{err})"
        f = r["fragmentation"]
        fr = f"{f['editorial_community_share']:.2f}" if f else ""
        dd = str(f.get("distinct_domains", "")) if f else ""
        tool = str(f.get("tool_hits", "")) if f else ""
        vend = f"{f['vendor_share']:.2f}" if f else ""
        uncl = f"{f['unclassified_share']:.2f}" if f else ""
        nres = str(f.get("n", "")) if f else ""
        ints = ", ".join(f"{k}:{v}" for k, v in r["intents"].items())
        ex = "; ".join(q for q, _ in r["problem_examples"][:3])
        md.append(f"| {i} | {r['head_term'] or r['label']} | {r['domain']} | {r['members']} | {r['problem_members']} ({r['problem_share']}) | "
                  f"{r['organic_problem_members']} | {r['corroborated_2plus']} | {ints} | {tv} | {fr} | {dd} | {tool} | {vend} | {uncl} | {nres} | {ex} |")
    p = ROOT / "data" / "exports" / f"candidates_{tag}.md"
    p.write_text("\n".join(md) + "\n")
    return p


if __name__ == "__main__":
    from pipeline import store
    con = store.connect()
    tag = sys.argv[1] if len(sys.argv) > 1 else "pass1"
    print(write(con, tag=tag))
