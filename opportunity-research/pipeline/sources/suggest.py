"""Autocomplete suggestions: Google (two clients), YouTube, Bing, DuckDuckGo.

What a suggestion establishes: that the engine has seen enough of this query to
propose it. It does NOT establish a volume. Google's chrome client also returns
a relevance score per suggestion (google:suggestrelevance), kept as metadata.
"""
from __future__ import annotations

import json
import re
import urllib.parse

from pipeline import http


def google(q: str, hl: str = "en", gl: str = "us") -> list[dict]:
    url = f"https://www.google.com/complete/search?client=chrome&hl={hl}&gl={gl}&q={urllib.parse.quote(q)}"
    code, data = http.get_json(url, ua=http.UA_BROWSER, min_interval=0.7)
    if code != 200 or not data:
        return []
    out = []
    rel = (data[4] or {}).get("google:suggestrelevance", []) if len(data) > 4 and isinstance(data[4], dict) else []
    types = (data[4] or {}).get("google:suggesttype", []) if len(data) > 4 and isinstance(data[4], dict) else []
    for i, s in enumerate(data[1]):
        out.append({"query": s, "rank": i + 1, "relevance": rel[i] if i < len(rel) else None,
                    "type": types[i] if i < len(types) else None})
    return out


def google_firefox(q: str, hl: str = "en") -> list[dict]:
    url = f"https://suggestqueries.google.com/complete/search?client=firefox&hl={hl}&q={urllib.parse.quote(q)}"
    code, data = http.get_json(url, ua=http.UA_BROWSER, min_interval=0.7)
    return [{"query": s, "rank": i + 1} for i, s in enumerate(data[1])] if code == 200 and data else []


def youtube(q: str, hl: str = "en") -> list[dict]:
    url = f"https://suggestqueries.google.com/complete/search?client=youtube&ds=yt&hl={hl}&q={urllib.parse.quote(q)}"
    code, body = http.get(url, ua=http.UA_BROWSER, min_interval=0.7)
    if code != 200:
        return []
    text = body.decode("utf-8", errors="replace")
    m = re.search(r"\((\[.*\])\)\s*$", text, re.S)
    if not m:
        return []
    try:
        data = json.loads(m.group(1))
    except json.JSONDecodeError:
        return []
    return [{"query": s[0] if isinstance(s, list) else s, "rank": i + 1} for i, s in enumerate(data[1])]


def bing(q: str) -> list[dict]:
    code, data = http.get_json(f"https://api.bing.com/osjson.aspx?query={urllib.parse.quote(q)}", min_interval=0.5)
    return [{"query": s, "rank": i + 1} for i, s in enumerate(data[1])] if code == 200 and data else []


def duckduckgo(q: str) -> list[dict]:
    code, data = http.get_json(f"https://duckduckgo.com/ac/?q={urllib.parse.quote(q)}&type=list", min_interval=0.5)
    return [{"query": s, "rank": i + 1} for i, s in enumerate(data[1])] if code == 200 and data else []


SOURCES = {"google": google, "google_firefox": google_firefox, "youtube": youtube, "bing": bing, "duckduckgo": duckduckgo}
