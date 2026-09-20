"""Reddit through OAuth (script app; credentials from the environment). Blocked without them (403 measured).

Environment: REDDIT_CLIENT_ID, REDDIT_CLIENT_SECRET, REDDIT_USER_AGENT.
Community evidence per query: thread count in a window, recency, and whether threads carry a solved flair.
Stored as provider 'reddit' with kinds 'thread_count_year' and 'threads_top' (metadata); never search volume.
"""
from __future__ import annotations

import base64
import json
import os
import urllib.parse
import urllib.request

_token: str | None = None


def token() -> str | None:
    global _token
    if _token:
        return _token
    cid, sec = os.environ.get("REDDIT_CLIENT_ID"), os.environ.get("REDDIT_CLIENT_SECRET")
    if not cid or not sec:
        return None
    req = urllib.request.Request("https://www.reddit.com/api/v1/access_token", data=b"grant_type=client_credentials",
                                 headers={"Authorization": "Basic " + base64.b64encode(f"{cid}:{sec}".encode()).decode(),
                                          "User-Agent": os.environ.get("REDDIT_USER_AGENT", "OpportunityResearchPipeline/0.1")})
    with urllib.request.urlopen(req, timeout=30) as r:
        _token = json.load(r)["access_token"]
    return _token


def search(q: str, subreddit: str | None = None, limit: int = 25, time_window: str = "year") -> list[dict]:
    tk = token()
    if not tk:
        return []
    base = f"https://oauth.reddit.com/r/{subreddit}/search" if subreddit else "https://oauth.reddit.com/search"
    url = base + "?" + urllib.parse.urlencode({"q": q, "restrict_sr": 1 if subreddit else 0, "sort": "relevance", "t": time_window, "limit": limit})
    req = urllib.request.Request(url, headers={"Authorization": f"Bearer {tk}", "User-Agent": os.environ.get("REDDIT_USER_AGENT", "OpportunityResearchPipeline/0.1")})
    with urllib.request.urlopen(req, timeout=30) as r:
        data = json.load(r)
    return [{"title": c["data"].get("title"), "subreddit": c["data"].get("subreddit"), "score": c["data"].get("score"),
             "comments": c["data"].get("num_comments"), "created": c["data"].get("created_utc"), "flair": c["data"].get("link_flair_text"),
             "url": "https://www.reddit.com" + c["data"].get("permalink", "")} for c in data.get("data", {}).get("children", [])]
