"""Stack Exchange API v2.3 (keyless: 300 requests/day per IP; with a registered key: 10,000).

A question count is a community-activity metric. It is stored as kind
'se_question_total' and is never treated as search volume.
"""
from __future__ import annotations

import os
import urllib.parse

from pipeline import http

KEY = os.environ.get("STACKEXCHANGE_KEY")


def _url(path: str, params: dict) -> str:
    if KEY:
        params["key"] = KEY
    return f"https://api.stackexchange.com/2.3/{path}?" + urllib.parse.urlencode(params)


def total(q: str, site: str = "stackoverflow") -> int | None:
    code, data = http.get_json(_url("search/advanced", {"q": q, "site": site, "filter": "total", "pagesize": 1}), min_interval=0.4)
    return data.get("total") if code == 200 and data else None


def top(q: str, site: str = "stackoverflow", n: int = 5) -> list[dict]:
    code, data = http.get_json(_url("search/advanced", {"q": q, "site": site, "order": "desc", "sort": "relevance", "pagesize": n}), min_interval=0.4)
    if code != 200 or not data:
        return []
    return [{"title": i.get("title"), "score": i.get("score"), "answers": i.get("answer_count"),
             "accepted": i.get("is_answered"), "views": i.get("view_count"), "created": i.get("creation_date"),
             "link": i.get("link"), "tags": i.get("tags")} for i in data.get("items", [])]
