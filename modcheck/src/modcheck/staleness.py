"""Propagating upstream change into the knowledge base.

Detecting that a source changed is only half of keeping knowledge current. The
other half is finding everything that was written on the basis of that source
and marking it as needing revalidation, rather than silently keeping a
conclusion that happened to be favourable.

Invalidation here is deliberately conservative: any record that cites a changed
source, in evidence or in `provenance.verified_against`, is marked stale. That
over-marks -- a change to one paragraph of a page invalidates records that
depended on a different paragraph -- and that is the right trade while we
cannot tell which part of a source a record relied on. Narrowing it later must
not cost correctness.

Marking is explicit and reversible: `stale: true` plus a `stale_reason` naming
the source and what changed. Clearing it is a human act of revalidation, done
with `modcheck sources revalidate`.
"""
from __future__ import annotations

import dataclasses
import datetime as dt
from pathlib import Path
from typing import Any, Iterable

from . import yamlio
from .store import Pack, Record, Store
from .validate import _evidence_refs


@dataclasses.dataclass
class StaleMark:
    record_kind: str
    record_id: str
    path: Path
    sources: list[str]
    reason: str


def dependents(pack: Pack, source_ids: Iterable[str]) -> list[tuple[Record, list[str]]]:
    """Records that cite any of these sources, with which ones they cite."""
    wanted = {s for s in source_ids}
    out: list[tuple[Record, list[str]]] = []
    for record in pack.all_records():
        cited = {ref["source"] for ref in _evidence_refs(record.data)}
        cited |= set((record.get("provenance") or {}).get("verified_against") or [])
        hit = sorted(cited & wanted)
        if hit:
            out.append((record, hit))
    return out


def _rewrite(path: Path, record_id: str, mutate) -> bool:
    """Rewrite one record in a YAML file, preserving the rest of the file."""
    data = yamlio.load_path(path)
    entries = data if isinstance(data, list) else [data]
    changed = False
    for entry in entries:
        if isinstance(entry, dict) and entry.get("id") == record_id:
            if mutate(entry):
                changed = True
    if changed:
        payload = entries if isinstance(data, list) else entries[0]
        path.write_text(yamlio.dump(payload, width=100), encoding="utf-8")
    return changed


def mark_stale(pack: Pack, source_ids: Iterable[str], *, reasons: dict[str, str] | None = None,
               write: bool = True) -> list[StaleMark]:
    """Mark every record depending on a changed source as needing revalidation."""
    reasons = reasons or {}
    marks: list[StaleMark] = []
    for record, cited in dependents(pack, source_ids):
        detail = "; ".join(f"{sid}: {reasons.get(sid, 'changed upstream')}" for sid in cited)
        reason = (f"needs revalidation: {detail} "
                  f"(marked {dt.date.today().isoformat()})")

        def mutate(entry: dict[str, Any], reason=reason) -> bool:
            provenance = entry.setdefault("provenance", {})
            if provenance.get("stale") is True and provenance.get("stale_reason") == reason:
                return False
            provenance["stale"] = True
            provenance["stale_reason"] = reason
            return True

        if write:
            if not _rewrite(record.path, record.id, mutate):
                continue
        marks.append(StaleMark(record_kind=record.kind, record_id=record.id,
                               path=record.path, sources=cited, reason=reason))
    return marks


def clear_stale(pack: Pack, record_ids: Iterable[str], *, verified_against: list[str] | None = None) -> list[str]:
    """Revalidation: clear the stale flag and record the date it was rechecked."""
    wanted = set(record_ids)
    cleared: list[str] = []
    for record in pack.all_records():
        if record.id not in wanted:
            continue

        def mutate(entry: dict[str, Any]) -> bool:
            provenance = entry.setdefault("provenance", {})
            if not provenance.get("stale"):
                return False
            provenance.pop("stale", None)
            provenance.pop("stale_reason", None)
            provenance["last_verified"] = dt.date.today().isoformat()
            if verified_against:
                provenance["verified_against"] = sorted(
                    set(provenance.get("verified_against") or []) | set(verified_against))
            return True

        if _rewrite(record.path, record.id, mutate):
            cleared.append(record.id)
    return cleared


def stale_records(store: Store, game: str | None = None) -> list[Record]:
    packs = [store.pack(game)] if game else store.packs()
    out = []
    for pack in packs:
        for record in pack.all_records():
            if (record.get("provenance") or {}).get("stale"):
                out.append(record)
    return out
