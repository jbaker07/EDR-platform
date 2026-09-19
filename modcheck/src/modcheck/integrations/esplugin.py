"""Record-level Bethesda plugin analysis via esplugin.

ModCheck's own Bethesda inspector reads the TES4 header only. Record-level
questions -- how many records a plugin overrides, and whether two plugins touch
any of the same records -- need a real parser, and one already exists:
`esplugin`, the Rust library LOOT itself uses. We call it rather than
reimplement it.

The helper lives in `helpers/esplugin-cli` and is built on demand:

    cd modcheck/helpers/esplugin-cli && cargo build --release

It is optional. When it is not built, record-level analysis is reported as
unavailable rather than silently skipped, and the header-level facts our own
inspector produces are unaffected.

**Licensing.** esplugin is GPL-3.0, so the helper is GPL-3.0-or-later. It is
kept as a separate optional binary that ModCheck invokes as a subprocess, and
is not bundled or distributed with ModCheck. Running a GPL program as a
separate process is not the same question as distributing one: if ModCheck
ever ships this binary, that distribution must comply with GPL-3.0. That is
recorded here rather than assumed away.

**Overlap is not conflict.** Two plugins touching the same record means the
later one wins for that record, which is how Bethesda modding works and is
frequently intentional. A reported overlap is a place to look, and the finding
says so.
"""
from __future__ import annotations

import json
import shutil
import subprocess
from pathlib import Path
from typing import Any

from ..analyze.config import Installation
from ..analyze.findings import Finding
from ..paths import project_root

GAMES = {"skyrimse", "fallout4", "falloutnv"}
HELPER_RELATIVE = Path("helpers/esplugin-cli/target/release/modcheck-esplugin")


class HelperUnavailable(RuntimeError):
    pass


def helper_path() -> Path | None:
    candidate = project_root() / HELPER_RELATIVE
    if candidate.is_file():
        return candidate
    found = shutil.which("modcheck-esplugin")
    return Path(found) if found else None


def available() -> bool:
    return helper_path() is not None


def run(game: str, paths: list[str | Path], *, full: bool = False,
        timeout: int = 300) -> dict[str, Any]:
    """Invoke the helper and return its JSON. Reads files; writes nothing."""
    helper = helper_path()
    if helper is None:
        raise HelperUnavailable(
            "the esplugin helper is not built. Build it with: "
            "cd modcheck/helpers/esplugin-cli && cargo build --release")
    argv = [str(helper), game]
    if full:
        argv.append("--full")
    argv += [str(p) for p in paths]
    proc = subprocess.run(argv, capture_output=True, text=True, timeout=timeout)
    if not proc.stdout.strip():
        raise HelperUnavailable(f"esplugin helper produced no output: {proc.stderr[:300]}")
    return json.loads(proc.stdout)


def analyze(installation: Installation, *, full: bool = False) -> list[Finding]:
    """Record counts and, with full parsing, record-level overlap."""
    if installation.game not in GAMES:
        return []
    paths = [a.path for a in installation.artifacts
             if a.path and Path(a.path).suffix.lower() in (".esp", ".esm", ".esl")]
    if not paths:
        return []

    data = run(installation.game, paths, full=full)
    findings: list[Finding] = []

    for entry in data.get("plugins", []):
        if "error" in entry:
            findings.append(Finding(
                code="esplugin.unreadable",
                severity="warning",
                subject=entry.get("path", "?"),
                summary=f"esplugin could not parse this plugin: {entry['error']}",
                evidence_class="extracted",
                not_established="anything about this plugin's contents",
            ))
            continue
        name = entry.get("filename") or entry.get("path")
        if entry.get("is_light") and entry.get("is_valid_as_light") is False:
            findings.append(Finding(
                code="esplugin.invalid_light_plugin",
                severity="error",
                subject=name,
                summary=f"{name} is flagged light but contains FormIDs outside the range "
                        "a light plugin may use",
                detail="esplugin checks the new records' FormIDs against the light "
                       "plugin range. The game will not load this correctly.",
                evidence_class="extracted",
                not_established="which specific records are out of range",
            ))
        if full and entry.get("override_record_count") is not None:
            findings.append(Finding(
                code="esplugin.override_records",
                severity="note",
                subject=name,
                summary=f"{name} overrides {entry['override_record_count']} records from "
                        "its masters",
                evidence_class="extracted",
                not_established="which records, and whether any other plugin overrides "
                                "the same ones",
            ))

    for overlap in data.get("overlaps", []):
        findings.append(Finding(
            code="esplugin.record_overlap",
            severity="note",
            subject=f"{overlap['a']}, {overlap['b']}",
            summary=f"{overlap['a']} and {overlap['b']} both touch at least one of the "
                    "same records",
            detail="In Bethesda games the plugin loaded later wins for a shared record. "
                   "This is normal and often deliberate -- it is a place to look, not a "
                   "conflict on its own.",
            evidence_class="extracted",
            targets=[{"kind": "record", "id": f"{overlap['a']}~{overlap['b']}",
                      "label": "shared records"}],
            not_established="which records overlap, whether the later plugin's values "
                            "are the intended ones, and whether a patch already exists",
        ))
    return findings


def coverage(game: str, ran: bool, full: bool) -> tuple[list[str], list[str]]:
    if game not in GAMES:
        return [], []
    if not ran:
        return [], ["record-level plugin analysis: the esplugin helper is not built "
                    "(cd modcheck/helpers/esplugin-cli && cargo build --release)"]
    checked = ["plugin masters, flags and light-plugin validity, via esplugin"]
    not_checked = ["field-level differences between overlapping records",
                   "whether an overlap is intentional or a problem"]
    if full:
        checked.append("record-level overlap between every pair of plugins, via esplugin")
        checked.append("override record counts, via esplugin")
    else:
        not_checked.append("record-level overlap: full parsing was not requested")
    return checked, not_checked
