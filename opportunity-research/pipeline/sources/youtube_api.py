"""YouTube Data API v3 (key from YOUTUBE_API_KEY; 10,000 units/day free; a search costs 100 units).

Replaces the HTML scrape in serp.youtube_results with a stable count and metadata. Video counts are a supply
signal (how much video exists for the query), never a demand number. Stored as provider 'youtube_api'.
"""
from __future__ import annotations

import json
import os
import urllib.parse
import urllib.request


def search(q: str, n: int = 10) -> dict:
    key = os.environ.get("YOUTUBE_API_KEY")
    if not key:
        return {}
    url = "https://www.googleapis.com/youtube/v3/search?" + urllib.parse.urlencode(
        {"part": "snippet", "q": q, "type": "video", "maxResults": n, "key": key, "relevanceLanguage": "en"})
    with urllib.request.urlopen(url, timeout=30) as r:
        data = json.load(r)
    return {"total_results": data.get("pageInfo", {}).get("totalResults"),
            "top": [{"id": i["id"].get("videoId"), "title": i["snippet"].get("title"), "channel": i["snippet"].get("channelTitle"),
                     "published": i["snippet"].get("publishedAt")} for i in data.get("items", [])]}
