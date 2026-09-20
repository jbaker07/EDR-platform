#!/usr/bin/env python3
"""Build the ModCheck probe content pack.

This pack exists to be the *second* pack in a deliberate collision. It is
entirely our own content -- a solid-colour sprite sheet generated here, not an
asset taken from anyone's mod -- so nothing is redistributed and the bytes are
ours to publish.

It is generated rather than committed as opaque binary so that anyone can
re-derive the exact bytes the predictions are pinned to. Determinism is the
whole point: the PNG is written with fixed filter bytes and a fixed zlib level,
so the sha256 recorded in a prediction stays reproducible.

Usage:
    python evaluation/runtime/build_probe_pack.py [--priority Exclusive|High|...]

`--priority` writes the Load patch's Priority field, which is the *controlled
change* the runtime loop makes between cases. Omitting it writes no Priority,
which Content Patcher reads as Exclusive.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import struct
import zlib
from pathlib import Path

HERE = Path(__file__).resolve().parent
PACK = HERE / "packs" / "modcheck-conflict-probe"

# Stardew's Animals/horse sprite sheet. The probe matches the vanilla
# dimensions so a Load of it is a plausible replacement rather than something
# Content Patcher or the game would reject for the wrong reason.
WIDTH, HEIGHT = 128, 32
# Opaque magenta: unmistakable on screen, so a human looking at the game can
# tell which pack won without reading a log.
PIXEL = bytes((255, 0, 255, 255))
# The overlay marker is deliberately small, so that in overlay mode the horse
# underneath stays visible. A full-size opaque overlay would look exactly like a
# replacement on screen, which is the thing this case exists to distinguish.
MARK_WIDTH, MARK_HEIGHT = 8, 8


def _chunk(kind: bytes, payload: bytes) -> bytes:
    return (struct.pack(">I", len(payload)) + kind + payload
            + struct.pack(">I", zlib.crc32(kind + payload) & 0xFFFFFFFF))


def png_bytes(width: int = WIDTH, height: int = HEIGHT, pixel: bytes = PIXEL) -> bytes:
    """A minimal 8-bit RGBA PNG, written deterministically."""
    raw = b"".join(b"\x00" + pixel * width for _ in range(height))  # filter type 0 per row
    return (b"\x89PNG\r\n\x1a\n"
            + _chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 6, 0, 0, 0))
            + _chunk(b"IDAT", zlib.compress(raw, 9))
            + _chunk(b"IEND", b""))


MANIFEST = {
    "Name": "ModCheck Conflict Probe",
    "Author": "ModCheck",
    "Version": "1.0.0",
    "Description": (
        "A deliberate collision probe. Loads Animals/horse so a second pack "
        "targeting the same asset can be observed resolving against it."),
    "UniqueID": "ModCheck.ConflictProbe",
    # Priority needs Content Patcher 2.0; see FORMAT below.
    "ContentPackFor": {"UniqueID": "Pathoschild.ContentPatcher", "MinimumVersion": "2.0.0"},
}

# Content Patcher's Migration_2_0 REJECTS a pack that uses `Priority` while
# declaring an older Format -- the whole pack fails to load with
# "using Priority ... isn't supported before Content Patcher 2.0", rather than
# the field being ignored. Declaring 1.3 here (as the probe first did) would
# have meant the selected-replacement case tested a rejected pack while the
# prediction claimed the patch was loaded and merely superseded: the same
# "did not apply" outcome for an entirely different reason.
FORMAT = "2.0"


def content(priority: str | None, mode: str = "load") -> dict:
    """The probe's one patch.

    `mode` is what makes the positive demonstration possible:

    * ``load`` competes for the asset -- two Loads on one asset is the conflict.
    * ``overlay`` composes with it. An EditImage does not contest the Load, so
      both packs apply: Bear Mounts supplies the horse and the probe draws over
      part of it. `Priority: Late` follows Content Patcher's own advice for
      "a cosmetic overlay meant to be applied over base edits from all mods".
    """
    if mode == "overlay":
        patch: dict[str, object] = {
            "LogName": "Probe Saddle Mark",
            "Action": "EditImage",
            "Target": "Animals/horse",
            "FromFile": "assets/probe_mark.png",
            "PatchMode": "Overlay",
            "ToArea": {"X": 0, "Y": 0, "Width": MARK_WIDTH, "Height": MARK_HEIGHT},
        }
        if priority is not None:
            patch["Priority"] = priority
        return {"Format": FORMAT, "Changes": [patch]}

    patch = {
        "LogName": "Probe Horse",
        "Action": "Load",
        "Target": "Animals/horse",
        "FromFile": "assets/probe_horse.png",
    }
    if priority is not None:
        patch["Priority"] = priority
    return {"Format": FORMAT, "Changes": [patch]}


def build(priority: str | None = None, root: Path = PACK,
          mode: str = "load") -> dict[str, str]:
    """Write the pack and return each file's sha256, for pinning in a prediction."""
    root.mkdir(parents=True, exist_ok=True)
    (root / "assets").mkdir(exist_ok=True)

    files = {
        "manifest.json": json.dumps(MANIFEST, indent=2).encode() + b"\n",
        "content.json": json.dumps(content(priority, mode), indent=2).encode() + b"\n",
    }
    if mode == "overlay":
        files["assets/probe_mark.png"] = png_bytes(MARK_WIDTH, MARK_HEIGHT)
    else:
        files["assets/probe_horse.png"] = png_bytes()
    digests = {}
    for name, data in files.items():
        (root / name).write_bytes(data)
        digests[name] = hashlib.sha256(data).hexdigest()
    return digests


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--priority", default=None,
                        help="Priority for the patch; omit for Content Patcher's "
                             "default (Exclusive for a Load, Default for an edit)")
    parser.add_argument("--mode", default="load", choices=("load", "overlay"),
                        help="load competes for the asset; overlay composes with it")
    args = parser.parse_args()
    digests = build(args.priority, mode=args.mode)
    label = args.priority or ("Exclusive (by omission)" if args.mode == "load"
                              else "Default (by omission)")
    print(f"built {PACK} in {args.mode} mode with priority {label}")
    for name, digest in digests.items():
        print(f"  {digest}  {name}  ({(PACK / name).stat().st_size} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
