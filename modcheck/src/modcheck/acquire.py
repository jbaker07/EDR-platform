"""Knowledge acquisition: fetch upstream material and record its provenance.

One pipeline for all ten games. Every fetch produces a *source record*:
url, retrieval time, HTTP status, content hash, size, and whatever revision
identity the server gave us (ETag / Last-Modified / commit sha). Nothing enters
the knowledge base without one.

Retrieved bytes go to ``evidence_cache/`` which is gitignored: the cache is
re-fetchable and its contents are frequently not ours to redistribute. The
provenance -- which is ours -- is committed.

Content is *untrusted input*. It is stored and hashed, never executed, and
never treated as an instruction.
"""
from __future__ import annotations

import datetime as dt
import gzip
import hashlib
import json
import re
import urllib.error
import urllib.request
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from .paths import evidence_cache

USER_AGENT = "ModCheck/0.1 (mod compatibility research; contact: founder)"
DEFAULT_TIMEOUT = 30
MAX_BYTES = 64 << 20


class FetchError(Exception):
    pass


@dataclass
class Fetched:
    url: str
    status: int
    sha256: str
    bytes: int
    content_type: str
    retrieved_at: str
    path: Path | None
    content_sha256: str = ""
    path_bytes: bytes = b""
    upstream_revision: str | None = None
    headers: dict[str, str] = field(default_factory=dict)

    def source_record(self, source_id: str, title: str, kind: str, **extra: Any) -> dict:
        """Build a schema-valid source record from this fetch."""
        rec: dict[str, Any] = {
            "id": source_id,
            "title": title,
            "url": self.url,
            "kind": kind,
            "access": "public" if self.status == 200 else "blocked",
            "retrieved_at": self.retrieved_at,
            "http_status": self.status,
            "reuse": {"redistribute": "unknown", "commercial": "unknown", "derive": "unknown"},
        }
        if self.sha256:
            rec["sha256"] = self.sha256
            rec["bytes"] = self.bytes
        if self.content_sha256:
            rec["content_sha256"] = self.content_sha256
        if self.content_type:
            rec["content_type"] = self.content_type
        if self.upstream_revision:
            rec["upstream_revision"] = self.upstream_revision
        if self.path:
            rec["evidence_path"] = str(self.path.relative_to(evidence_cache()))
        rec.update(extra)
        return rec


# Many documentation sites inject a per-request token: a bot-management nonce,
# an analytics script id, a signed asset URL, a MediaWiki parser timing comment.
# Those change on every fetch while the documentation does not. Hashing raw
# bytes therefore reports permanent drift for such pages, which would mark every
# record depending on them stale forever and destroy the signal we need.
#
# So every source carries two hashes: the raw bytes, and a *content* hash over
# the meaningful text with volatile markup removed. Drift that moves only the
# raw hash is reported as volatile, not as an upstream change.
_SCRIPT_OR_STYLE = re.compile(rb"<(script|style)\b[^>]*>.*?</\1>",
                              re.IGNORECASE | re.DOTALL)
_HTML_COMMENT = re.compile(rb"<!--.*?-->", re.DOTALL)
_TAG = re.compile(rb"<[^>]+>")
_WHITESPACE = re.compile(rb"\s+")

TEXTUAL_TYPES = ("text/", "application/json", "application/xml", "application/x-yaml",
                 "application/javascript", "+json", "+xml")


def is_textual(content_type: str, raw: bytes = b"") -> bool:
    ct = (content_type or "").lower()
    if any(marker in ct for marker in TEXTUAL_TYPES):
        return True
    if ct:
        return False
    # No content type: treat as text only if it decodes cleanly.
    try:
        raw[:8192].decode("utf-8")
        return True
    except UnicodeDecodeError:
        return False


def content_digest(raw: bytes, content_type: str = "") -> str:
    """Hash of the meaningful content, ignoring per-request volatile markup.

    For HTML this strips script and style blocks, comments and tags, then
    collapses whitespace. For other text it collapses whitespace only.

    Returns "" for binary content. A release artifact has no injected nonce, so
    if its bytes change it has genuinely changed, and softening that comparison
    would hide exactly the drift we most need to catch.
    """
    if not is_textual(content_type, raw):
        return ""
    text = raw
    if b"<html" in raw[:4096].lower() or "html" in (content_type or "").lower():
        text = _SCRIPT_OR_STYLE.sub(b" ", text)
        text = _HTML_COMMENT.sub(b" ", text)
        text = _TAG.sub(b" ", text)
    text = _WHITESPACE.sub(b" ", text).strip()
    return hashlib.sha256(text).hexdigest()


def cache_path_for(sha256: str, suffix: str = "") -> Path:
    root = evidence_cache()
    return root / sha256[:2] / f"{sha256}{suffix}"


def fetch(url: str, *, timeout: int = DEFAULT_TIMEOUT, store: bool = True,
          suffix: str = "", headers: dict[str, str] | None = None) -> Fetched:
    """Fetch a URL. Never raises for HTTP status; a 403 is a recorded gap, not a crash."""
    now = dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT,
                                               "Accept-Encoding": "gzip",
                                               **(headers or {})})
    try:
        with urllib.request.urlopen(req, timeout=timeout) as resp:
            raw = resp.read(MAX_BYTES + 1)
            status = resp.status
            resp_headers = {k.lower(): v for k, v in resp.headers.items()}
    except urllib.error.HTTPError as exc:
        return Fetched(url=url, status=exc.code, sha256="", bytes=0, content_sha256="",
                       content_type=exc.headers.get("Content-Type", "") if exc.headers else "",
                       retrieved_at=now, path=None,
                       headers={k.lower(): v for k, v in (exc.headers or {}).items()})
    except (urllib.error.URLError, TimeoutError, OSError) as exc:
        raise FetchError(f"{url}: {exc}") from exc

    if len(raw) > MAX_BYTES:
        raise FetchError(f"{url}: response exceeds {MAX_BYTES} byte limit")
    if resp_headers.get("content-encoding") == "gzip":
        try:
            raw = gzip.decompress(raw)
        except OSError:
            pass

    digest = hashlib.sha256(raw).hexdigest()
    path = None
    if store:
        path = cache_path_for(digest, suffix)
        path.parent.mkdir(parents=True, exist_ok=True)
        if not path.exists():
            path.write_bytes(raw)

    revision = (resp_headers.get("etag") or resp_headers.get("last-modified")
                or resp_headers.get("x-github-request-id"))
    if revision:
        revision = revision.strip('"')

    content_type = resp_headers.get("content-type", "").split(";")[0]
    return Fetched(url=url, status=status, sha256=digest, bytes=len(raw),
                   content_type=content_type, path_bytes=raw,
                   content_sha256=content_digest(raw, content_type),
                   retrieved_at=now, path=path, upstream_revision=revision,
                   headers=resp_headers)


def verify(source: dict) -> tuple[bool, str]:
    """Re-fetch a source and report whether its *content* still matches.

    This is the invalidation primitive: when upstream really changes, everything
    that cited it must be revalidated. A page whose raw bytes move on every
    fetch because of an injected nonce has not changed, and saying it has would
    make the signal worthless.
    """
    try:
        result = fetch(source["url"], store=False)
    except FetchError as exc:
        return False, f"unreachable: {exc}"
    if result.status != 200:
        return False, f"http {result.status}"

    recorded = source.get("sha256")
    if not recorded:
        return False, "no recorded hash to compare against"
    if result.sha256 == recorded:
        return True, "unchanged"

    # Raw bytes moved. Compare the meaningful content before calling it a change.
    recorded_content = source.get("content_sha256")
    if not recorded_content:
        try:
            recorded_content = content_digest(read_cached(source),
                                              source.get("content_type", ""))
        except FetchError:
            recorded_content = None
    if recorded_content and result.content_sha256 == recorded_content:
        return True, ("volatile: raw bytes differ on every fetch (injected token or "
                      "timestamp) but the content is unchanged")

    detail = f"changed: {recorded[:12]} -> {result.sha256[:12]}"
    # Say WHAT changed where we can, so whoever revalidates knows where to look.
    # This never suppresses the change -- invalidation stays conservative,
    # because "only the numbers moved" also describes "Java 21 -> Java 25".
    hint = _numeric_only_hint(source, result)
    return False, f"{detail}{hint}"


_DIGITS = re.compile(rb"[0-9][0-9,.]*")


def _numeric_only_hint(source: dict, result: "Fetched") -> str:
    """If the only differences are numeric, say so. Still a change."""
    try:
        old = read_cached(source)
    except FetchError:
        return ""
    try:
        fresh = fetch(source["url"], store=False)
        if fresh.status != 200 or not fresh.path_bytes:
            return ""
        new = fresh.path_bytes
    except (FetchError, AttributeError):
        return ""
    content_type = source.get("content_type", "")

    def masked(raw: bytes) -> str:
        if not is_textual(content_type, raw):
            return ""
        stripped = raw
        if b"<html" in raw[:4096].lower() or "html" in content_type.lower():
            stripped = _SCRIPT_OR_STYLE.sub(b" ", stripped)
            stripped = _HTML_COMMENT.sub(b" ", stripped)
            stripped = _TAG.sub(b" ", stripped)
        stripped = _DIGITS.sub(b"#", stripped)
        return hashlib.sha256(_WHITESPACE.sub(b" ", stripped).strip()).hexdigest()

    old_masked, new_masked = masked(old), masked(new)
    if old_masked and old_masked == new_masked:
        return (" -- the only differences are numeric (often a view or download "
                "counter); still treated as changed, revalidate to confirm")
    return ""


def read_cached(source: dict) -> bytes:
    """Read a source's cached bytes, checking the hash before returning them."""
    rel = source.get("evidence_path")
    if not rel:
        raise FetchError(f"source {source['id']} has no evidence_path")
    path = evidence_cache() / rel
    if not path.exists():
        raise FetchError(f"evidence not cached: {path} (re-run `modcheck acquire fetch`)")
    raw = path.read_bytes()
    digest = hashlib.sha256(raw).hexdigest()
    if source.get("sha256") and digest != source["sha256"]:
        raise FetchError(f"cached evidence for {source['id']} does not match its recorded hash")
    return raw


def fetch_json(url: str, **kwargs: Any) -> Any:
    result = fetch(url, **kwargs)
    if result.status != 200:
        raise FetchError(f"{url}: http {result.status}")
    return json.loads(read_cached({"id": url, "evidence_path": str(
        result.path.relative_to(evidence_cache())), "sha256": result.sha256}))
