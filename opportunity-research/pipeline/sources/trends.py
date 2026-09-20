"""Google Trends through a cookie session (unofficial endpoints; rate-limited; pace to a few requests a minute).

Trends gives RELATIVE interest (0-100 against the other terms in the same request) and related/rising
queries. It is a discovery and comparison source, not a volume source. Stored as provider 'google_trends'.
"""
from __future__ import annotations

import json
import time
import urllib.parse

from pipeline import http

_cookie: str | None = None


def session() -> str | None:
    global _cookie
    if _cookie:
        return _cookie
    import urllib.request
    req = urllib.request.Request("https://trends.google.com/trends/?geo=US", headers={"User-Agent": http.UA_BROWSER})
    try:
        with urllib.request.urlopen(req, timeout=20) as resp:
            cookies = resp.headers.get_all("Set-Cookie") or []
    except Exception:
        return None
    _cookie = "; ".join(c.split(";")[0] for c in cookies)
    return _cookie


def explore(keywords: list[str], geo: str = "", timeframe: str = "today 12-m") -> dict | None:
    ck = session()
    if not ck:
        return None
    req = {"comparisonItem": [{"keyword": k, "geo": geo, "time": timeframe} for k in keywords], "category": 0, "property": ""}
    url = f"https://trends.google.com/trends/api/explore?hl=en-US&tz=0&req={urllib.parse.quote(json.dumps(req))}"
    code, data = http.get_json(url, ua=http.UA_BROWSER, cookies=ck, min_interval=6.0)
    return data if code == 200 else None


def _widget(explored: dict, wid: str) -> dict | None:
    return next((w for w in explored.get("widgets", []) if w.get("id") == wid), None)


def related_queries(keyword: str, geo: str = "") -> dict | None:
    ex = explore([keyword], geo)
    w = _widget(ex, "RELATED_QUERIES") if ex else None
    if not w:
        return None
    url = (f"https://trends.google.com/trends/api/widgetdata/relatedsearches?hl=en-US&tz=0"
           f"&req={urllib.parse.quote(json.dumps(w['request']))}&token={w['token']}")
    code, data = http.get_json(url, ua=http.UA_BROWSER, cookies=session(), min_interval=6.0)
    if code != 200 or not data:
        return None
    ranked = data["default"]["rankedList"]
    return {"top": [(x["query"], x["value"]) for x in ranked[0].get("rankedKeyword", [])],
            "rising": [(x["query"], x.get("formattedValue")) for x in ranked[1].get("rankedKeyword", [])] if len(ranked) > 1 else []}


def interest_over_time(keywords: list[str], geo: str = "") -> list[dict] | None:
    """Up to five keywords compared on one 0-100 scale; include an anchor term to compare across batches."""
    ex = explore(keywords, geo)
    w = _widget(ex, "TIMESERIES") if ex else None
    if not w:
        return None
    url = (f"https://trends.google.com/trends/api/widgetdata/multiline?hl=en-US&tz=0"
           f"&req={urllib.parse.quote(json.dumps(w['request']))}&token={w['token']}")
    code, data = http.get_json(url, ua=http.UA_BROWSER, cookies=session(), min_interval=6.0)
    if code != 200 or not data:
        return None
    return [{"time": p["formattedTime"], "values": p["value"]} for p in data["default"]["timelineData"]]
