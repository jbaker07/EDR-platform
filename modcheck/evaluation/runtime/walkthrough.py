#!/usr/bin/env python3
"""The creator loop, run end to end on real artifacts.

A creator arrives with an outcome they want, not a finding they want cleared:

    "I want my horse replacement to be the one players see, and I do not want
     to break Bear Mounts for anyone who has both installed."

This script runs the loop that request implies, and prints what a reviewer
would see at each step:

    1. requested outcome
    2. static analysis of the current state          -- what is wrong now
    3. proposed change, as a reviewable diff         -- one field, shown in full
    4. static re-check after the change              -- did the outcome change
    5. the prediction that re-check becomes          -- pinned, pre-registered
    6. native observation                            -- BLOCKED here, and why
    7. the creator report and the player report      -- same evidence, two readers

Step 6 is the honest one. This container has no game, so the loop stops there
and says so rather than printing a conclusion it did not earn. Steps 1-5 and 7
run on the pinned published pack and produce real output.

Run:  python evaluation/runtime/walkthrough.py
"""
from __future__ import annotations

import difflib
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import build_probe_pack  # noqa: E402

from modcheck.analyze.config import Installation, InstalledArtifact  # noqa: E402
from modcheck.observe import load_predictions  # noqa: E402
from modcheck.observe.render import render_prediction  # noqa: E402
from modcheck.paths import project_root  # noqa: E402
from modcheck.report import analyze_installation, render  # noqa: E402
from modcheck.store import Store  # noqa: E402

BEAR_PACK_ENV = "MODCHECK_BEAR_MOUNTS_PACK"
REQUESTED_OUTCOME = (
    "I want my horse replacement to be the one players see, and I do not want to "
    "break Bear Mounts for anyone who has both installed.")


def _rule(title: str) -> None:
    print()
    print(f"== {title} " + "=" * max(0, 74 - len(title)))
    print()


def _bear_pack() -> Path | None:
    """The published pack, reassembled from the evidence cache.

    It is not committed: it is someone else's release, fetched read-only into a
    gitignored cache. Where the cache is absent, the static half of this
    walkthrough is blocked too, and says so.
    """
    import os
    override = os.environ.get(BEAR_PACK_ENV)
    if override:
        return Path(override)

    cache = project_root() / "evidence_cache"
    files = {
        "content.json": "17/1700837712028ded37fd7b7d4c238a1338fcbd8ddfa8eb9c430acc9fb336bc0e.json",
        "manifest.json": "0e/0ebb1577ab1eadb414f98ed19b4a7a5946e49831f934804f551d09b970df5049.json",
        "LICENSE": "f8/f8b2baeeab8ddb1a7810ff51ea21f08d089525967d7f90b2a6a2fab7ac12df2a.txt",
    }
    if not all((cache / rel).exists() for rel in files.values()):
        return None
    staged = Path(tempfile.mkdtemp()) / "bearmounts"
    staged.mkdir(parents=True)
    for name, rel in files.items():
        (staged / name).write_bytes((cache / rel).read_bytes())
    return staged


def _analyse(bear: Path, probe: Path) -> object:
    installation = Installation.from_paths(
        "stardewvalley", [str(bear), str(probe)], files_known_complete=True)
    # Content Patcher itself is declared present rather than inspected. Both
    # packs require it, and a player who has either one has it; leaving it out
    # would make every run report a missing dependency that is not the subject
    # of this walkthrough. It is recorded as a declared installed mod, not a
    # file we parsed, because we did not parse one.
    installation.artifacts.append(InstalledArtifact(
        name="ContentPatcher (declared present, not inspected)",
        mod_id="Pathoschild.ContentPatcher", version="2.8.0",
        load_index=-1))
    return analyze_installation(installation, Store())


def _findings(report) -> str:
    lines = []
    for finding in report.findings:
        if not finding.code.startswith("contentpatcher."):
            continue
        lines.append(f"  [{finding.severity}] {finding.code}")
        lines.append(f"    {finding.summary}")
        for resolution in finding.resolutions:
            lines.append(f"    -> {resolution['step']}")
        if finding.not_established:
            lines.append(f"    not established: {finding.not_established}")
    return "\n".join(lines) or "  (no Content Patcher finding)"


def main() -> int:
    bear = _bear_pack()
    if bear is None:
        print("BLOCKED: the published Bear Mounts pack is not in the evidence cache.")
        print("The cache is gitignored on purpose -- it holds someone else's release,")
        print("fetched read-only and never redistributed. Restore it with")
        print("`modcheck sources fetch --game stardewvalley`, or point")
        print(f"{BEAR_PACK_ENV} at a local copy of the pinned pack.")
        return 2

    work = Path(tempfile.mkdtemp())
    # Same directory name on both sides: the artifact's name is what the
    # findings cite, and it must read as one pack before and after the change,
    # not as two different mods.
    before_dir = work / "before" / "modcheck-conflict-probe"
    after_dir = work / "after" / "modcheck-conflict-probe"
    build_probe_pack.build(None, before_dir)          # Exclusive, by omission
    build_probe_pack.build("High", after_dir)         # the proposed change

    _rule("1. requested outcome")
    print(f"  {REQUESTED_OUTCOME}")

    _rule("2. static analysis of the current state")
    before = _analyse(bear, before_dir)
    print(_findings(before))

    _rule("3. proposed change, as a reviewable diff")
    diff = difflib.unified_diff(
        (before_dir / "content.json").read_text().splitlines(keepends=True),
        (after_dir / "content.json").read_text().splitlines(keepends=True),
        fromfile="modcheck-conflict-probe/content.json",
        tofile="modcheck-conflict-probe/content.json", n=3)
    print("".join(f"  {line}" for line in diff).rstrip())
    print()
    print("  One field, in the creator's own pack. Nothing in Bear Mounts is")
    print("  touched: it stays the published release throughout.")

    _rule("4. static re-check after the change")
    after = _analyse(bear, after_dir)
    print(_findings(after))

    _rule("5. what the re-check does NOT settle")
    print("  The error clears, and that is not the same as the requested outcome.")
    print("  The creator asked for THEIR replacement to be the one players see.")
    print("  Exclusive is int.MaxValue in SMAPI, so High does not outrank it: the")
    print("  change makes their pack lose quietly instead of making both fail")
    print("  loudly. Reporting 'resolved' here would answer a question the")
    print("  creator did not ask. The honest answer is that the outcome they")
    print("  asked for is not reachable while Bear Mounts holds an Exclusive")
    print("  load on the same asset, and that the reachable outcomes are:")
    print("    - their pack loses, Bear Mounts applies (this change), or")
    print("    - they target a different asset, or use an Edit that composes,")
    print("      which changes what their mod does, or")
    print("    - the two authors agree which one declares a lower priority.")

    _rule("6. native observation")
    predictions = {p.id: p for p in load_predictions(
        project_root() / "evaluation" / "runtime" / "predictions")}
    for case in ("sv_deliberate_conflict", "sv_selected_replacement"):
        prediction = predictions[case]
        print(f"  {prediction.id}: {prediction.observation_status}")
        for command in prediction.commands:
            print(f"    $ {command}")
    print()
    print("  BLOCKED in this environment: no Stardew Valley installation, no")
    print("  SMAPI, no licence. The predictions above are committed and closed.")
    print("  Until a transcript is filed against them, steps 2 and 4 remain")
    print("  static analysis of a documented rule -- not a verified prediction")
    print("  about a running game.")

    _rule("7. the two reports, from the same evidence")
    print("-- player --")
    print(render(before, "player"))
    print()
    print("-- creator --")
    print(render(before, "creator"))

    _rule("appendix: the prediction record for the change in step 3")
    print(render_prediction(predictions["sv_selected_replacement"]))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
