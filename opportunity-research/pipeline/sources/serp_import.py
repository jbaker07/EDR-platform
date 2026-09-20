"""Ingest SERP rows collected outside the scripts (the WebSearch tool, or a SERP API export).

File format: JSON list of {"query": ..., "engine": "websearch_tool" | "google_serpapi" | "google_dataforseo",
"results": [{"rank": 1, "url": ..., "title": ...}, ...]}. Category and site are assigned here so the
classification rules stay in one place (pipeline/sources/serp.py).
"""
from __future__ import annotations

import json
from pathlib import Path

from pipeline import store
from pipeline.sources import serp


def import_file(con, path: Path) -> int:
    rows = json.loads(path.read_text())
    n = 0
    for r in rows:
        results = [{"rank": x.get("rank", i + 1), "url": x["url"], "title": x.get("title"), "site": serp.site(x["url"]),
                    "category": serp.classify(x["url"])} for i, x in enumerate(r["results"])]
        store.add_serp(con, r["query"], r["engine"], results)
        n += 1
    con.commit()
    return n
