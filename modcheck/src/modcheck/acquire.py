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
        if self.content_type:
            rec["content_type"] = self.content_type
        if self.upstream_revision:
            rec["upstream_revision"] = self.upstream_revision
        if self.path:
            rec["evidence_path"] = str(self.path.relative_to(evidence_cache()))
        rec.update(extra)
        return rec


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
        return Fetched(url=url, status=exc.code, sha256="", bytes=0,
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

    return Fetched(url=url, status=status, sha256=digest, bytes=len(raw),
                   content_type=resp_headers.get("content-type", "").split(";")[0],
                   retrieved_at=now, path=path, upstream_revision=revision,
                   headers=resp_headers)


def verify(source: dict) -> tuple[bool, str]:
    """Re-fetch a source and report whether its content still hashes the same.

    This is the invalidation primitive: when upstream changes, everything that
    cited it must be revalidated rather than silently kept.
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
    return False, f"changed: {recorded[:12]} -> {result.sha256[:12]}"


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
