"""Google Ads API: Keyword Planner historical metrics (needs credentials; not runnable here yet).

Environment: GOOGLE_ADS_DEVELOPER_TOKEN, GOOGLE_ADS_CLIENT_ID, GOOGLE_ADS_CLIENT_SECRET,
GOOGLE_ADS_REFRESH_TOKEN, GOOGLE_ADS_CUSTOMER_ID (10 digits, no dashes), optional GOOGLE_ADS_LOGIN_CUSTOMER_ID.
Endpoint: customers/{cid}:generateKeywordHistoricalMetrics (KeywordPlanIdeaService) via REST.
Stores provider 'google_keyword_planner' with the same kinds as the CSV importer, so reports do not care
which path the numbers came from. Volumes are Google's rounded estimates and are labelled approximate.
"""
from __future__ import annotations

import json
import os
import urllib.parse
import urllib.request

from pipeline import store

API_VERSION = os.environ.get("GOOGLE_ADS_API_VERSION", "v21")
GEO = {"US": "2840", "GB": "2826", "CA": "2124", "AU": "2036", "DE": "2276", "FR": "2250", "IN": "2356"}
LANG = {"en": "1000", "de": "1001", "fr": "1002", "es": "1003"}


def _access_token() -> str:
    data = urllib.parse.urlencode({
        "client_id": os.environ["GOOGLE_ADS_CLIENT_ID"], "client_secret": os.environ["GOOGLE_ADS_CLIENT_SECRET"],
        "refresh_token": os.environ["GOOGLE_ADS_REFRESH_TOKEN"], "grant_type": "refresh_token"}).encode()
    with urllib.request.urlopen(urllib.request.Request("https://oauth2.googleapis.com/token", data=data), timeout=30) as r:
        return json.load(r)["access_token"]


def historical_metrics(keywords: list[str], geo: str = "US", lang: str = "en") -> list[dict]:
    """Up to the API's per-request cap of keywords (documented as 10,000)."""
    cid = os.environ["GOOGLE_ADS_CUSTOMER_ID"]
    body = {"keywords": keywords, "geoTargetConstants": [f"geoTargetConstants/{GEO[geo]}"],
            "language": f"languageConstants/{LANG[lang]}", "keywordPlanNetwork": "GOOGLE_SEARCH", "includeAdultKeywords": False}
    req = urllib.request.Request(
        f"https://googleads.googleapis.com/{API_VERSION}/customers/{cid}:generateKeywordHistoricalMetrics",
        data=json.dumps(body).encode(), method="POST",
        headers={"Authorization": f"Bearer {_access_token()}", "developer-token": os.environ["GOOGLE_ADS_DEVELOPER_TOKEN"],
                 "Content-Type": "application/json", **({"login-customer-id": os.environ["GOOGLE_ADS_LOGIN_CUSTOMER_ID"]}
                                                        if os.environ.get("GOOGLE_ADS_LOGIN_CUSTOMER_ID") else {})})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.load(r).get("results", [])


def ingest(con, results: list[dict], geo: str, lang: str) -> int:
    n = 0
    for r in results:
        kw = r.get("text")
        m = r.get("keywordMetrics") or {}
        if not kw:
            continue
        store.add_query(con, kw)
        store.add_metric(con, kw, "google_keyword_planner", "avg_monthly_searches", float(m.get("avgMonthlySearches", 0) or 0), geo=geo, lang=lang,
                         extra={"competition": m.get("competition"), "source": "google_ads_api", "close_variants": r.get("closeVariants")})
        if m.get("competitionIndex") is not None:
            store.add_metric(con, kw, "google_keyword_planner", "competition_index", float(m["competitionIndex"]), geo=geo, lang=lang)
        for key, kind in (("lowTopOfPageBidMicros", "top_of_page_bid_low"), ("highTopOfPageBidMicros", "top_of_page_bid_high")):
            if m.get(key) is not None:
                store.add_metric(con, kw, "google_keyword_planner", kind, float(m[key]) / 1e6, geo=geo, lang=lang)
        for mv in m.get("monthlySearchVolumes", []):
            month = {"JANUARY": 1, "FEBRUARY": 2, "MARCH": 3, "APRIL": 4, "MAY": 5, "JUNE": 6, "JULY": 7, "AUGUST": 8,
                     "SEPTEMBER": 9, "OCTOBER": 10, "NOVEMBER": 11, "DECEMBER": 12}.get(mv.get("month"), 0)
            store.add_metric(con, kw, "google_keyword_planner", "monthly", float(mv.get("monthlySearches", 0) or 0), geo=geo, lang=lang,
                             period=f"{mv.get('year')}-{month:02d}")
        n += 1
    con.commit()
    return n
