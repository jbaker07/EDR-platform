"""Result pages: Bing HTML (parseable, proxy for Google), YouTube results (video count), and a classifier.

Google's own result page is not retrievable from this environment (it serves a script challenge);
real Google SERPs need a SERP API or the WebSearch tool driven by the analyst. Every stored SERP row
names its engine so a proxy is never mistaken for Google.
"""
from __future__ import annotations

import base64
import html
import re
import urllib.parse

from pipeline import http

# Classes used for fragmentation analysis. Curated sites first (taxonomy/site_classes.yaml), then URL
# patterns, then "unclassified" -- which is reported separately and never counted as fragmented.
import yaml
from pathlib import Path

_SITE_CLASSES: dict[str, str] = {}
for _cls, _sites in yaml.safe_load((Path(__file__).resolve().parents[2] / "taxonomy" / "site_classes.yaml").read_text()).items():
    for _s in _sites:
        _SITE_CLASSES[_s] = _cls

PATTERNS = [
    ("community", r"reddit\.com|stackoverflow\.com|stackexchange\.com|superuser\.com|serverfault\.com|askubuntu\.com|quora\.com|"
                  r"youtube\.com|youtu\.be|github\.com|gitlab\.com|forum|discussions\.|community\.|/threads/|discourse|boards\.|"
                  r"answers\.|/topic/|substack\.com|medium\.com|tiktok\.com|vimeo\.com|facebook\.com"),
    ("reference", r"wikipedia\.org|fandom\.com|wiki\."),
    ("official", r"\.gov(\.|/|$)|\.edu(\.|/|$)|\.nhs\.uk|europa\.eu|\.org\.uk"),
    ("marketplace", r"amazon\.|ebay\.|etsy\.|aliexpress|walmart\.|homedepot\.|lowes\.|assetstore|gumroad|itch\.io|store\."),
    ("vendor", r"docs\.|/docs/|/manual/|readthedocs|developer\.|support\.|help\.|helpx\.|learn\.|apps\.apple|play\.google|"
               r"microsoft\.com|apple\.com|google\.com|adobe\.com|autodesk\.com|unity\.com|epicgames\.com"),
    ("editorial", r"blog|/articles?/|magazine|news|/guide/|/guides/|lifehacker|thespruce|bobvila|familyhandyman|wikihow|ifixit|allrecipes|seriouseats"),
]
FALLBACK = "unclassified"
# title words that name a tool page; "maker's" (a brand) and "database" (a product category) are excluded as false positives
TOOL_TITLE = re.compile(r"\b(calculator|generator|template|templates|maker(?!'s)|converter|checker|finder|planner|tracker|simulator|"
                        r"lookup|estimator|builder|free tool|online tool)\b", re.I)


def classify(url: str) -> str:
    s = site(url)
    if s in _SITE_CLASSES:
        return _SITE_CLASSES[s]
    u = url.lower()
    for name, pat in PATTERNS:
        if re.search(pat, u):
            return name
    return FALLBACK


def site(url: str) -> str:
    return urllib.parse.urlparse(url).netloc.lower().removeprefix("www.")


def bing(q: str, n: int = 10) -> list[dict]:
    url = f"https://www.bing.com/search?q={urllib.parse.quote(q)}&setlang=en&cc=US&count={n}"
    code, body = http.get(url, ua=http.UA_RESEARCH, min_interval=2.0)
    if code != 200:
        return []
    text = body.decode("utf-8", errors="replace")
    out = []
    for i, block in enumerate(re.findall(r'<li class="b_algo".*?</li>', text, re.S)):
        m = re.search(r'<h2[^>]*>\s*<a[^>]*?href="([^"]+)"[^>]*>(.*?)</a>', block, re.S)
        if not m:
            continue
        href = html.unescape(m.group(1))
        real = href
        um = re.search(r"[?&]u=a1([A-Za-z0-9_\-=]+)", href)
        if um:
            b = um.group(1)
            b += "=" * (-len(b) % 4)
            try:
                real = base64.urlsafe_b64decode(b).decode("utf-8", errors="replace")
            except Exception:
                real = href
        title = re.sub(r"<[^>]+>", "", m.group(2))
        out.append({"rank": len(out) + 1, "url": real, "title": html.unescape(title), "site": site(real), "category": classify(real)})
    return out


def youtube_results(q: str) -> dict:
    url = f"https://www.youtube.com/results?search_query={urllib.parse.quote(q)}"
    code, body = http.get(url, ua=http.UA_BROWSER, min_interval=2.0)
    if code != 200:
        return {}
    text = body.decode("utf-8", errors="replace")
    est = re.search(r'"estimatedResults":"(\d+)"', text)
    titles = re.findall(r'"videoRenderer":\{"videoId":"([^"]+)".*?"title":\{"runs":\[\{"text":"(.*?)"\}\]', text)[:10]
    views = re.findall(r'"viewCountText":\{"simpleText":"([^"]+)"\}', text)[:10]
    return {"estimated_results": int(est.group(1)) if est else None,
            "top": [{"id": v, "title": t, "views": views[i] if i < len(views) else None} for i, (v, t) in enumerate(titles)]}


def mix(results: list[dict]) -> dict:
    """Fragmentation view of one or more SERPs: class shares, dominant site, and the counts the ranking shows.

    fragmented_share = community + editorial + aggregator results (answers that must be pieced together).
    tool_hits = results from a curated tool site or whose title names a tool (calculator, generator, ...).
    vendor_share / official_share = the answer is owned by the product vendor or an official source.
    """
    if not results:
        return {}
    cats: dict[str, int] = {}
    sites: dict[str, int] = {}
    tool_hits = 0
    for r in results:
        cats[r["category"]] = cats.get(r["category"], 0) + 1
        sites[r["site"]] = sites.get(r["site"], 0) + 1
        if r["category"] == "tool" or (r.get("title") and TOOL_TITLE.search(r["title"])):
            tool_hits += 1
    n = len(results)
    frag = sum(v for k, v in cats.items() if k in ("community", "editorial", "aggregator"))
    return {"n": n, "categories": cats, "dominant_site": max(sites, key=sites.get), "dominant_share": round(max(sites.values()) / n, 2),
            "fragmented_share": round(frag / n, 2), "tool_hits": tool_hits, "vendor_share": round(cats.get("vendor", 0) / n, 2),
            "official_share": round(cats.get("official", 0) / n, 2), "unclassified_share": round(cats.get(FALLBACK, 0) / n, 2)}
