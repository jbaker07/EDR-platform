"""Assemble the Top-20 report: measured fields from the store, evidence from the competitor records,
and the analyst's authored fields from data/top20/<cluster_id>.yaml. Every one of the brief's twenty
fields is present for every candidate; fields that cannot be measured on the free route say so.

Authored file schema (data/top20/<cluster_id>.yaml):
  cluster_id, head_term, rank, problem, proposed_free_resource, why_users_choose_it, indexable_pages,
  repeat_use, advertising_suitability, other_monetization, maintenance, ai_threat, direct_answer_threat,
  primary_risk, evidence_confidence, verdict
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from pipeline import rank, store  # noqa: E402
from pipeline.cluster import intent_of  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
AUTHORED = ("problem", "proposed_free_resource", "why_users_choose_it", "indexable_pages", "repeat_use",
            "advertising_suitability", "other_monetization", "maintenance", "ai_threat", "direct_answer_threat",
            "primary_risk", "evidence_confidence", "verdict")


def _measured(con, cid: str) -> dict:
    m = {r["cluster_id"]: r for r in rank.metrics(con, min_members=1)}
    return m[cid]


def build(con) -> list[dict]:
    # metrics come from the pass-1 candidate snapshot the analysis was done on; cluster ids change when the
    # store is re-clustered, so the pass-1 -> pass-2 mapping (majority vote of members) is attached separately
    metrics = {r["cluster_id"]: r for r in json.loads((ROOT / "data" / "exports" / "candidates_pass1.json").read_text())}
    mapping_path = next((ROOT / "data" / "exports" / f"cluster_map_pass1_to_pass{n}.json" for n in (3, 2)
                         if (ROOT / "data" / "exports" / f"cluster_map_pass1_to_pass{n}.json").exists()), ROOT / "missing")
    mapping = json.loads(mapping_path.read_text()) if mapping_path.exists() else {}
    out = []
    for path in sorted((ROOT / "data" / "top20").glob("c_*.yaml")):
        a = yaml.safe_load(path.read_text())
        cid = a["cluster_id"]
        m = metrics[cid]
        comp = yaml.safe_load((ROOT / "data" / "competitors" / f"{cid}.yaml").read_text())
        rep = [q for q, _ in m.get("problem_examples", [])][:8]
        f = m["fragmentation"]
        out.append({"authored": a, "metrics": m, "competitors": comp, "representative": rep,
                    "fragmentation": f, "mapping": mapping.get(cid, {})})
    out.sort(key=lambda r: r["authored"].get("rank", 999))
    return out


def write(con) -> Path:
    rows = build(con)
    md = ["# Top 20 opportunities (free route; analysed on pass-1 clusters, mapped to the final pass)\n",
          "Every candidate carries the brief's twenty fields. Measured facts come from the research store (autocomplete "
          "corroboration, Google Trends chain, web-search proxy results, fetched competitor pages); inference and hypothesis "
          "are labelled. No monthly search volume, traffic, revenue or user count is stated anywhere, because none was measured.\n"]
    for r in rows:
        a, m, c, f = r["authored"], r["metrics"], r["competitors"], r["fragmentation"]
        s = c.get("synthesis", {})
        tv = ""
        if m.get("trends_chain_value") is not None:
            err = m.get("trends_error_pct") or 0
            tv = (f"head term '{m['head_term']}' at {m['trends_chain_value']:.1f} on the chained Trends scale (root = 100, rounding error ±{err}%)"
                  if err < 50 else f"head term placed at ~{m['trends_chain_value']:.0f} but with ±{err}% error, so unreliable")
        else:
            tv = "head term not yet placed on the Trends chain"
        md.append(f"\n## {a.get('rank', '?')}. {a.get('head_term', m['head_term'])}  (`{a['cluster_id']}`, domain {m['domain']})\n")
        md.append(f"**1. Problem.** {a['problem']}\n")
        md.append("**2. Representative searches (measured; most-corroborated task-intent members first).** " + "; ".join(f"`{q}`" for q in r["representative"]) + "\n")
        md.append(f"**3. Search-volume evidence.** None measured on the free route. Relative signals only: {tv}; "
                  f"{m['corroborated_2plus']} of {m['members']} members were suggested by two or more engines.\n")
        md.append(f"**4. Aggregate cluster demand (measured, pass 1).** {m['members']} member queries, {m['problem_members']} with task intent "
                  f"({m['problem_share']} share), {m['organic_problem_members']} of them volunteered by the engines on bare probes; intent mix "
                  + ", ".join(f"{k} {v}" for k, v in m["intents"].items()) + f"; subdomains touched: {', '.join(m['subdomains'])}. "
                  "Member counts reflect one seed per subdomain. "
                  + (f"After the final pass (three seeds per subdomain, re-clustered) the same problem maps to cluster `{r['mapping'].get('new_cluster_id')}` "
                     f"with {r['mapping'].get('new_size')} members ({r['mapping'].get('votes')} of the analysed members vote for it; a low vote "
                     f"means the recursive split ladder broke the pass-1 cluster apart, not that demand fell)." if r["mapping"] else "") + "\n")
        md.append(f"**5. Existing search workflow (from fetched pages).** {s.get('pieced_together_answer', 'n/a')}\n")
        comps = c.get("competitors", [])
        md.append(f"**6. Major competitors ({len(comps)} reviewed, {c.get('pages_fetched')} pages fetched, {c.get('pages_failed')} failed).**\n")
        md.append("| site | kind | fetch | business model | currency | install | account |\n|---|---|---|---|---|---|---|")
        for x in comps:
            md.append(f"| {x.get('site')} | {x.get('kind')} | {str(x.get('fetch_status'))[:12]} | {str(x.get('business_model'))[:80]} | {str(x.get('currency'))[:40]} | {x.get('needs_install')} | {x.get('needs_account')} |")
        md.append("")
        md.append(f"**7. Why competitors do not completely solve it.** {s.get('single_destination_would_need', 'n/a')} Existing tools: {s.get('existing_tools_that_do_this', 'n/a')}\n")
        cats = ", ".join(f"{k} {v}" for k, v in sorted(f.get("categories", {}).items(), key=lambda kv: -kv[1]))
        md.append(f"**8. Fragmentation evidence (measured on {f.get('n', 0)} web-search proxy results for {len(m['queries_checked'])} queries).** "
                  f"Fragmented share {f.get('fragmented_share')}, tool hits {f.get('tool_hits')}, vendor share {f.get('vendor_share')}, "
                  f"unclassified {f.get('unclassified_share')}, dominant site {f.get('dominant_site')} ({f.get('dominant_share')}); classes: {cats}. "
                  f"Queries checked: {'; '.join(m['queries_checked'])}.\n")
        md.append(f"**9. Proposed free resource.** {a['proposed_free_resource']}\n")
        md.append(f"**10. Why users would choose it.** {a['why_users_choose_it']}\n")
        md.append(f"**11. Indexable useful pages / tools.** {a['indexable_pages']}\n")
        md.append(f"**12. Repeat-use mechanism.** {a['repeat_use']}\n")
        md.append(f"**13. Advertising suitability.** {a['advertising_suitability']}\n")
        md.append(f"**14. Other monetization.** {a['other_monetization']}\n")
        md.append(f"**15. Maintenance requirements.** {a['maintenance']} Data source and reproducibility (from the competitor record): {s.get('data_source_and_reproducibility', 'n/a')}\n")
        md.append(f"**16. AI threat.** {a['ai_threat']} Competitor-record view: {s.get('ai_resilience', 'n/a')}\n")
        md.append(f"**17. Search-engine direct-answer threat.** {a['direct_answer_threat']}\n")
        md.append(f"**18. Primary risk.** {a['primary_risk']}\n")
        md.append(f"**19. Evidence quality / confidence.** {a['evidence_confidence']} Open questions from the competitor record: " + " | ".join(str(q) for q in s.get("open_questions", [])[:4]) + "\n")
        md.append(f"**20. Verdict.** {a['verdict']}\n")
    p = ROOT / "data" / "exports" / "top20.md"
    p.write_text("\n".join(md) + "\n")
    return p


if __name__ == "__main__":
    print(write(store.connect()))
