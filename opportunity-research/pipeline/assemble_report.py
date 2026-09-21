"""Assemble one self-contained report from the committed deliverables, for reading outside this repository.

Order: executive summary (authored, data/exports/executive_summary.md if present), method and access facts (README),
what was produced, relative demand, the PCPartPicker conclusion, Top 5 and #1, Top 20 (all twenty fields), Top 100,
intent matrix, competitor syntheses, and an evidence appendix listing every checked query with its result URLs.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[1]
EXP = ROOT / "data" / "exports"


def section(title: str, path: Path, level: int = 1) -> str:
    if not path.exists():
        return f"\n{'#' * level} {title}\n\n(not available)\n"
    body = path.read_text()
    # demote the file's own headings under this section
    body = "\n".join(("#" + line if line.startswith("#") else line) for line in body.splitlines())
    return f"\n{'#' * level} {title}\n\n{body}\n"


def competitor_appendix() -> str:
    out = ["\n# Appendix A. Competitor syntheses (24 clusters, fetched pages, quotes in the YAML records)\n"]
    for path in sorted((ROOT / "data" / "competitors").glob("c_*.yaml")):
        d = yaml.safe_load(path.read_text())
        s = d.get("synthesis", {})
        out.append(f"\n## {d['head_term']} (`{d['cluster_id']}`; {d.get('pages_fetched')} pages fetched, {d.get('pages_failed')} failed)\n")
        out.append("| site | kind | fetch | business model | currency | install | account |\n|---|---|---|---|---|---|---|")
        for c in d.get("competitors", []):
            out.append(f"| {c.get('site')} | {c.get('kind')} | {str(c.get('fetch_status'))[:14]} | {str(c.get('business_model'))[:90]} | "
                       f"{str(c.get('currency'))[:40]} | {c.get('needs_install')} | {c.get('needs_account')} |")
        for k in ("pieced_together_answer", "single_destination_would_need", "existing_tools_that_do_this",
                  "data_source_and_reproducibility", "ai_resilience"):
            out.append(f"\n**{k.replace('_', ' ')}.** {s.get(k, '')}")
        oq = s.get("open_questions", [])
        if oq:
            out.append("\n**open questions.** " + " | ".join(str(q) for q in oq))
    return "\n".join(out) + "\n"


def serp_appendix() -> str:
    out = ["\n# Appendix B. Web-search evidence for the analysed clusters (query, rank, site class, URL)\n",
           "Results come from the web-search tool available in the research environment (a US-only proxy, not Google). "
           "Classes are from taxonomy/site_classes.yaml, then URL patterns; `unclassified` is never counted as fragmented.\n"]
    for path in sorted((ROOT / "data" / "exports" / "phase7_inputs").glob("c_*.json")):
        d = json.loads(path.read_text())
        out.append(f"\n## {d['head_term']} (`{d['cluster_id']}`)\n")
        for r in d["serp"]:
            out.append(f"- `{r['query']}` #{r['rank']} [{r['category']}] {r['url']}")
    return "\n".join(out) + "\n"


def build() -> Path:
    parts = ["# Search-opportunity research: complete report (free route, 2026-09-21)\n",
             "Assembled from the committed deliverables in `opportunity-research/`. Measured facts, inference and hypothesis are "
             "labelled throughout; no monthly search volume, traffic, revenue or user count is stated anywhere because none was measured.\n"]
    parts.append(section("1. Executive summary", EXP / "executive_summary.md"))
    parts.append(section("2. Method, access facts and pipeline (README)", ROOT / "README.md"))
    parts.append(section("3. Relative demand", EXP / "demand_relative.md"))
    parts.append(section("4. Incumbent check: PCPartPicker", EXP / "pcpartpicker_conclusion.md"))
    parts.append(section("5. Top 5 finalists and #1", EXP / "top5_and_no1.md"))
    parts.append(section("6. Top 20 with all twenty fields", EXP / "top20.md"))
    parts.append(section("7. Final Top 100", EXP / "top100_final_free_route.md"))
    parts.append(section("8. Intent by domain and cross-domain families (pass 3)", EXP / "intent_matrix_pass3.md"))
    parts.append(competitor_appendix())
    parts.append(serp_appendix())
    p = EXP / "final_report_full.md"
    p.write_text("\n".join(parts))
    return p


if __name__ == "__main__":
    p = build()
    print(p, p.stat().st_size)
