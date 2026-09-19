"""Reading and writing packs/<game>/sources.yaml deterministically."""
from __future__ import annotations

from pathlib import Path
from typing import Any

import yaml

from . import yamlio

HEADER = """\
# Source records for this game pack.
#
# Every source is something we actually retrieved. `sha256` is the hash of the
# bytes we read, so any claim citing this source can be re-verified with
# `modcheck sources verify`. Sources whose access is blocked/deleted/discord_only
# are GAPS: they record what we could NOT read, and may only back `unresolved`
# evidence.
#
# Written by `modcheck sources add`; edit by hand only to add licensing, reuse
# terms and notes.
"""


def load(path: Path) -> list[dict[str, Any]]:
    if not path.exists():
        return []
    raw = yamlio.load_path(path)
    if not raw:
        return []
    return raw.get("sources", []) if isinstance(raw, dict) else list(raw)


def save(path: Path, sources: list[dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    ordered = sorted(sources, key=lambda s: s.get("id", ""))
    body = yaml.safe_dump({"sources": ordered}, sort_keys=False, allow_unicode=True,
                          default_flow_style=False, width=100)
    path.write_text(HEADER + body, encoding="utf-8")


def upsert(path: Path, record: dict[str, Any]) -> str:
    """Insert or replace a source by id. Returns 'added' or 'updated'."""
    sources = load(path)
    existing = {s["id"]: i for i, s in enumerate(sources) if "id" in s}
    if record["id"] in existing:
        # Keep hand-written fields that a re-fetch cannot know.
        prior = sources[existing[record["id"]]]
        for key in ("license", "reuse", "notes", "games", "gap", "publisher"):
            if key in prior and key not in record:
                record[key] = prior[key]
            elif key == "reuse" and prior.get("reuse", {}).get("redistribute") != "unknown":
                record["reuse"] = prior["reuse"]
        sources[existing[record["id"]]] = record
        action = "updated"
    else:
        sources.append(record)
        action = "added"
    save(path, sources)
    return action
