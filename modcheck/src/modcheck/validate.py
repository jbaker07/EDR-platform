"""Validation of the knowledge base.

Two layers:

1. **Schema** — every record must satisfy its JSON Schema in ``schemas/``.
2. **Integrity** — rules that keep claims tied to evidence. These are the ones
   that stop the knowledge base drifting into confident-sounding fiction:

   * every evidence reference must resolve to a real source record;
   * a source we could not actually read (blocked/deleted/discord-only) is a
     *gap*, and may only be cited as ``unresolved`` evidence;
   * a capability may only be called ``supported`` if it names a test;
   * a recipe may only claim ``build_tested``/``game_tested`` with a recorded
     attestation;
   * cross-record ids (resolutions, addresses) must resolve.
"""
from __future__ import annotations

import dataclasses
import json
import re
from functools import lru_cache
from pathlib import Path
from typing import Any, Iterable

from jsonschema import Draft202012Validator
from referencing import Registry, Resource

from .paths import CAPABILITIES, schemas_dir
from .store import RECORD_DIRS, Pack, Record, Store

SEVERITIES = ("error", "warning")

# Sources we could not actually read. Citing one as anything but `unresolved`
# would be claiming knowledge we do not have.
UNREADABLE_ACCESS = {"blocked", "deleted", "discord_only", "paywalled"}


@dataclasses.dataclass(frozen=True)
class Issue:
    severity: str
    code: str
    where: str
    message: str

    def format(self) -> str:
        return f"[{self.severity}] {self.code} {self.where}: {self.message}"


@lru_cache(maxsize=1)
def _registry() -> Registry:
    """Schema registry so `$ref: common.schema.json#/...` resolves locally."""
    resources = []
    for path in sorted(schemas_dir().glob("*.schema.json")):
        schema = json.loads(path.read_text(encoding="utf-8"))
        resource = Resource.from_contents(schema)
        resources.append((path.name, resource))
        if "$id" in schema:
            resources.append((schema["$id"], resource))
    return Registry().with_resources(resources)


@lru_cache(maxsize=32)
def validator_for(name: str) -> Draft202012Validator:
    schema_path = schemas_dir() / f"{name}.schema.json"
    schema = json.loads(schema_path.read_text(encoding="utf-8"))
    return Draft202012Validator(schema, registry=_registry())


def _schema_issues(name: str, data: dict[str, Any], where: str) -> list[Issue]:
    issues = []
    for err in sorted(validator_for(name).iter_errors(data), key=lambda e: list(e.path)):
        loc = "/".join(str(p) for p in err.path) or "<root>"
        issues.append(Issue("error", "schema", f"{where}#{loc}", err.message))
    return issues


def _evidence_refs(obj: Any) -> Iterable[dict[str, Any]]:
    """Walk a record and yield every evidence-reference-shaped dict."""
    if isinstance(obj, dict):
        if "source" in obj and "evidence_class" in obj and isinstance(obj.get("source"), str):
            yield obj
        for value in obj.values():
            yield from _evidence_refs(value)
    elif isinstance(obj, list):
        for item in obj:
            yield from _evidence_refs(item)


def validate_pack(pack: Pack, store: Store | None = None) -> list[Issue]:
    issues: list[Issue] = []
    rel = lambda p: str(p).split("/packs/")[-1]  # noqa: E731

    # ---- manifest ------------------------------------------------------
    if not pack.manifest_path.exists():
        return [Issue("error", "missing_pack", pack.game, "pack.yaml not found")]
    manifest = pack.manifest
    issues += _schema_issues("gamepack", manifest, rel(pack.manifest_path))
    if manifest.get("game") != pack.game:
        issues.append(Issue("error", "game_mismatch", rel(pack.manifest_path),
                            f"manifest game {manifest.get('game')!r} != directory {pack.game!r}"))

    caps = manifest.get("capabilities") or {}
    for cap in CAPABILITIES:
        entry = caps.get(cap)
        if not isinstance(entry, dict):
            continue
        status = entry.get("status")
        if status == "supported" and not entry.get("tests"):
            issues.append(Issue("error", "unbacked_capability", f"{pack.game}/{cap}",
                                "status 'supported' requires at least one test id"))
        if status == "partial" and not entry.get("implemented_by"):
            issues.append(Issue("warning", "unbacked_capability", f"{pack.game}/{cap}",
                                "status 'partial' should name what implements it"))
        if status == "blocked" and not entry.get("blocker"):
            issues.append(Issue("error", "unstated_blocker", f"{pack.game}/{cap}",
                                "status 'blocked' requires a blocker"))

    # ---- sources -------------------------------------------------------
    for sid, source in sorted(pack.sources.items()):
        where = f"{rel(pack.sources_path)}#{sid}"
        issues += _schema_issues("source", source, where)
        access = source.get("access")
        if access in UNREADABLE_ACCESS and not source.get("gap"):
            issues.append(Issue("error", "gap_not_stated", where,
                                f"access={access} requires 'gap' saying what we therefore do not know"))
        if access == "public" and source.get("http_status") == 200 and not source.get("sha256"):
            issues.append(Issue("warning", "no_content_hash", where,
                                "retrieved public source has no sha256; evidence is not re-verifiable"))

    # ---- records -------------------------------------------------------
    known_ids: dict[str, set[str]] = {kind: {r.id for r in pack.records(kind)} for kind in RECORD_DIRS}
    seen: dict[tuple[str, str], Path] = {}

    for kind in RECORD_DIRS:
        for rec in pack.records(kind):
            where = f"{rel(rec.path)}#{rec.id}"
            issues += _schema_issues(kind, rec.data, where)

            key = (kind, rec.id)
            if key in seen:
                issues.append(Issue("error", "duplicate_id", where,
                                    f"id already defined in {rel(seen[key])}"))
            seen[key] = rec.path

            if rec.get("game") != pack.game:
                issues.append(Issue("error", "game_mismatch", where,
                                    f"record game {rec.get('game')!r} != pack {pack.game!r}"))

            # evidence must resolve, and unreadable sources may only be cited as unresolved
            for ref in _evidence_refs(rec.data):
                src_id = ref["source"]
                source = pack.sources.get(src_id)
                if source is None:
                    issues.append(Issue("error", "dangling_evidence", where,
                                        f"evidence cites unknown source {src_id!r}"))
                    continue
                if source.get("access") in UNREADABLE_ACCESS and ref["evidence_class"] != "unresolved":
                    issues.append(Issue(
                        "error", "unreadable_source_cited", where,
                        f"source {src_id!r} is {source.get('access')}; it may only back "
                        f"'unresolved' evidence, not {ref['evidence_class']!r}"))

            # cross-record ids
            for field, target_kind in (("resolutions", "resolution"), ("addresses", None)):
                for ref_id in rec.get(field, []) or []:
                    if target_kind:
                        pool = known_ids[target_kind]
                    else:
                        pool = known_ids["failure"] | known_ids["interaction"]
                    if ref_id not in pool:
                        issues.append(Issue("error", "dangling_reference", where,
                                            f"{field} -> {ref_id!r} does not resolve"))

            issues += _record_integrity(pack, rec, where)

    return issues


def _record_integrity(pack: Pack, rec: Record, where: str) -> list[Issue]:
    issues: list[Issue] = []

    if rec.kind == "recipe":
        state = rec.get("verification_state")
        if state in ("build_tested", "game_tested"):
            att = pack.root / "attestations" / f"{rec.id}.json"
            if not att.exists():
                issues.append(Issue(
                    "error", "unattested_verification", where,
                    f"verification_state={state!r} requires attestations/{rec.id}.json "
                    "produced by an actual run"))
        build = rec.get("build") or {}
        for check in rec.get("validation", []) or []:
            if "command" not in check and "modcheck_check" not in check:
                issues.append(Issue("warning", "unexecutable_validation", where,
                                    f"validation step {check.get('method')!r} names no command or check id"))
        if build.get("commands") and state == "documented":
            issues.append(Issue("warning", "unrun_build", where,
                                "recipe declares build commands but has never been build-tested here"))

    if rec.kind == "interaction":
        for part in rec.get("participants", []) or []:
            art = part.get("artifact", {})
            if art.get("any_version") and len(rec.get("evidence", [])) < 2:
                issues.append(Issue(
                    "warning", "overbroad_version_claim", where,
                    f"artifact {art.get('id')!r} claims any_version on a single piece of evidence"))
        outcome = rec.get("outcome", {})
        if outcome.get("kind") in ("conflict", "incompatible") and not rec.get("conditions"):
            if outcome.get("evidence_class") == "derived":
                issues.append(Issue(
                    "warning", "unconditioned_conflict", where,
                    "a derived conflict with no stated conditions is usually an overlap, not a conflict"))

    if rec.kind == "failure":
        det = rec.get("detector", {})
        if det.get("detectable") in ("yes", "partial") and not det.get("check_id"):
            issues.append(Issue("warning", "undetected_failure", where,
                                "marked detectable but names no implemented check"))
        if det.get("detectable") == "no" and not det.get("why_not"):
            issues.append(Issue("warning", "unexplained_undetectable", where,
                                "detectable=no should say why"))

    return issues


def validate_store(store: Store, games: Iterable[str] | None = None) -> list[Issue]:
    issues: list[Issue] = []
    for game in (games or store.games):
        issues += validate_pack(store.pack(game), store)
    return issues


def summarize(issues: list[Issue]) -> dict[str, int]:
    return {sev: sum(1 for i in issues if i.severity == sev) for sev in SEVERITIES}
