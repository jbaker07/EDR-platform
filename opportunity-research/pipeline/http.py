"""One polite HTTP client: identifies itself, paces requests, backs off on 429, never retries forever."""
from __future__ import annotations

import json
import time
import urllib.error
import urllib.parse
import urllib.request

UA_RESEARCH = "OpportunityResearchPipeline/0.1 (research use; contact: jermainebaker1512@gmail.com)"
UA_BROWSER = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0 Safari/537.36"
_last: dict[str, float] = {}


def get(url: str, *, ua: str = UA_RESEARCH, min_interval: float = 0.6, retries: int = 3, timeout: int = 25,
        headers: dict | None = None, cookies: str | None = None) -> tuple[int, bytes]:
    host = urllib.parse.urlparse(url).netloc
    wait = min_interval - (time.time() - _last.get(host, 0))
    if wait > 0:
        time.sleep(wait)
    req = urllib.request.Request(url, headers={"User-Agent": ua, "Accept-Language": "en-US,en;q=0.8", **(headers or {})})
    if cookies:
        req.add_header("Cookie", cookies)
    for attempt in range(retries):
        _last[host] = time.time()
        try:
            with urllib.request.urlopen(req, timeout=timeout) as resp:
                return resp.status, resp.read()
        except urllib.error.HTTPError as e:
            body = e.read() if e.fp else b""
            if e.code in (429, 503) and attempt < retries - 1:
                time.sleep(2 ** (attempt + 1) * 2)
                continue
            return e.code, body
        except (urllib.error.URLError, TimeoutError) as e:
            if attempt < retries - 1:
                time.sleep(2 ** attempt)
                continue
            return 0, str(e).encode()
    return 0, b""


def get_json(url: str, **kw):
    code, body = get(url, **kw)
    if code != 200:
        return code, None
    text = body.decode("utf-8", errors="replace")
    if text.startswith(")]}'"):
        text = text[text.index("\n") + 1:] if "\n" in text else text[4:]
    try:
        return code, json.loads(text)
    except json.JSONDecodeError:
        return code, None
