"""DataForSEO keyword metrics adapter (untested: no credentials in this environment).

Reads DATAFORSEO_LOGIN / DATAFORSEO_PASSWORD from the environment. Stores each keyword's monthly volume, CPC,
competition and the provider's monthly time series in `metrics` with provider='dataforseo', kind='search_volume',
geo and lang as requested, period=retrieval month, extra={location_code, language_code, network, monthly_searches,
cpc, competition, retrieved_at}. US and global are separate calls (location_code 2840 for US; omit for worldwide).
Never merges variants: the provider's close-variant grouping is not available on this endpoint, so overlap stays unresolved.
"""
from __future__ import annotations

import base64
import json
import os
import urllib.request
from datetime import datetime, timezone

from pipeline import store

ENDPOINT = "https://api.dataforseo.com/v3/keywords_data/google_ads/search_volume/live"


def fetch(keywords: list[str], location_code: int | None = 2840, language_code: str = "en") -> list[dict]:
    login, pw = os.environ.get("DATAFORSEO_LOGIN"), os.environ.get("DATAFORSEO_PASSWORD")
    if not login or not pw:
        raise SystemExit("set DATAFORSEO_LOGIN and DATAFORSEO_PASSWORD in the environment (never in chat or the repository)")
    task = {"keywords": keywords[:1000], "language_code": language_code, "search_partners": False}
    if location_code:
        task["location_code"] = location_code
    req = urllib.request.Request(ENDPOINT, data=json.dumps([task]).encode(), method="POST",
                                 headers={"Authorization": "Basic " + base64.b64encode(f"{login}:{pw}".encode()).decode(),
                                          "Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=120) as resp:
        body = json.loads(resp.read())
    return body["tasks"][0].get("result") or []


def ingest(con, rows: list[dict], geo: str, lang: str = "en") -> int:
    now = datetime.now(timezone.utc).isoformat(timespec="seconds")
    n = 0
    for r in rows:
        extra = {"location_code": r.get("location_code"), "language_code": r.get("language_code"), "network": "google_search",
                 "monthly_searches": r.get("monthly_searches"), "cpc": r.get("cpc"), "competition": r.get("competition"),
                 "competition_index": r.get("competition_index"), "retrieved_at": now}
        store.add_metric(con, r["keyword"], "dataforseo", "search_volume", geo, lang, now[:7], r.get("search_volume"), extra)
        n += 1
    con.commit()
    return n
