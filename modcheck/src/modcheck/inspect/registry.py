"""Dispatch an artifact to the inspector that recognises it."""
from __future__ import annotations

from pathlib import Path

from . import bethesda, bg3, cyberpunk, generic, jvm, projectzomboid, rimworld, sims4, smapi
from .base import Artifact, Inspection, InspectionError

# Order matters: the most specific detector wins. `generic` always matches and
# must stay last.
INSPECTORS = (
    ("minecraft_jar", jvm),
    ("bethesda_plugin", bethesda),
    ("sims4", sims4),
    ("bg3_mod", bg3),
    ("smapi_mod", smapi),
    ("rimworld_mod", rimworld),
    ("projectzomboid_mod", projectzomboid),
    ("cyberpunk_mod", cyberpunk),
    ("unknown", generic),
)

# A Bethesda plugin's format is shared across the three Bethesda games, so the
# file alone cannot say which game it is. The master it declares can.
BETHESDA_MASTERS = {
    "skyrim.esm": "skyrimse",
    "update.esm": "skyrimse",
    "dawnguard.esm": "skyrimse",
    "hearthfires.esm": "skyrimse",
    "dragonborn.esm": "skyrimse",
    "fallout4.esm": "fallout4",
    "dlcrobot.esm": "fallout4",
    "dlccoast.esm": "fallout4",
    "dlcnukaworld.esm": "fallout4",
    "falloutnv.esm": "falloutnv",
    "deadmoney.esm": "falloutnv",
    "honesthearts.esm": "falloutnv",
    "oldworldblues.esm": "falloutnv",
    "lonesomeroad.esm": "falloutnv",
}


def detect_kind(path: str | Path) -> str:
    p = Path(path)
    for name, module in INSPECTORS:
        try:
            if module.detect(p):
                return name
        except Exception:  # a broken detector must not mask the others
            continue
    return "unknown"


def _infer_bethesda_game(ins: Inspection) -> None:
    masters = [str(m).lower() for m in (ins.fact("masters") or [])]
    for master in masters:
        game = BETHESDA_MASTERS.get(master)
        if game:
            ins.game = game
            ins.add("game_inferred_from", master, "derived", "master file name")
            return
    ins.warnings.append(
        "plugin declares no recognised game master, so the target game is undetermined; "
        "pass --game to state it")


def inspect_path(path: str | Path, game: str | None = None) -> Inspection:
    """Inspect one artifact, falling back to identity-only inspection."""
    artifact = Artifact.of(path)
    kind = detect_kind(artifact.path)
    module = dict(INSPECTORS)[kind]
    try:
        if module is bethesda:
            ins = bethesda.inspect(artifact, game=game)
            if game is None:
                _infer_bethesda_game(ins)
            return ins
        ins = module.inspect(artifact)
    except InspectionError as exc:
        ins = generic.inspect(artifact)
        ins.warnings.append(f"{kind} inspection failed: {exc}")
        return ins
    if game and ins.game and ins.game != game:
        ins.warnings.append(
            f"artifact inspected as {ins.game!r} but caller said {game!r}")
    return ins
