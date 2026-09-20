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

CATEGORIES = [
    ("reddit", r"reddit\.com"), ("stackexchange", r"stackoverflow\.com|stackexchange\.com|superuser\.com|serverfault\.com|askubuntu\.com"),
    ("forum", r"forum|discussions\.|community\.|/t/|boards\.|\.proboards|discourse|answers\.|/threads/|quora\.com|/topic/"),
    ("youtube", r"youtube\.com|youtu\.be"), ("github", r"github\.com|gitlab\.com|bitbucket\.org"),
    ("docs", r"docs\.|documentation|/docs/|/manual/|readthedocs|developer\.|learn\.microsoft|developer\.mozilla|/wiki/|wiki\."),
    ("wikipedia", r"wikipedia\.org"), ("video_other", r"vimeo\.com|tiktok\.com"),
    ("marketplace", r"amazon\.|ebay\.|etsy\.|aliexpress|walmart\.|homedepot\.|lowes\.|assetstore|marketplace|gumroad|itch\.io"),
    ("qa_platform", r"answers\.|quora\.com"), ("government", r"\.gov|\.edu|nih\.gov|europa\.eu"),
    ("large_platform", r"microsoft\.com|apple\.com|google\.com|adobe\.com|autodesk\.com|unity\.com|epicgames\.com|ifixit\.com|wikihow\.com"),
    ("blog_editorial", r"medium\.com|substack|blog|/articles?/|magazine|news|\.io/[a-z-]+$|all3dp\.com|tomshardware\.com|howtogeek|lifehacker|thespruce|bobvila|familyhandyman|wikihow"),
    ("recipe_site", r"allrecipes|kingarthurbaking|seriouseats|foodnetwork|bbcgoodfood|recipe"),
    ("vendor_content", r"autozone\.com|kbb\.com|repairpal|carparts|advanceautoparts|creality|sovol3d|prusa3d|bambulab|makerbot|simplify3d|overture3d|homedepot|lowes"),
]
# an unmatched domain is an independent site (a blog, a shop, a company page), not "unknown"
FALLBACK = "independent_site"


def classify(url: str) -> str:
    u = url.lower()
    for name, pat in CATEGORIES:
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
    """Fragmentation view of one SERP: category shares, dominant site share, fragmented share."""
    if not results:
        return {}
    cats = {}
    sites = {}
    for r in results:
        cats[r["category"]] = cats.get(r["category"], 0) + 1
        sites[r["site"]] = sites.get(r["site"], 0) + 1
    n = len(results)
    fragmented = sum(v for k, v in cats.items() if k in ("reddit", "stackexchange", "forum", "youtube", "github", "blog_editorial", "qa_platform", "video_other", "independent_site", "recipe_site", "vendor_content"))
    return {"n": n, "categories": cats, "dominant_site": max(sites, key=sites.get), "dominant_share": max(sites.values()) / n,
            "fragmented_share": fragmented / n}
