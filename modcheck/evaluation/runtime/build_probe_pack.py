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
    "ContentPackFor": {"UniqueID": "Pathoschild.ContentPatcher", "MinimumVersion": "1.3.0"},
}


def content(priority: str | None) -> dict:
    patch: dict[str, object] = {
        "LogName": "Probe Horse",
        "Action": "Load",
        "Target": "Animals/horse",
        "FromFile": "assets/probe_horse.png",
    }
    if priority is not None:
        patch["Priority"] = priority
    return {"Format": "1.3", "Changes": [patch]}


def build(priority: str | None = None, root: Path = PACK) -> dict[str, str]:
    """Write the pack and return each file's sha256, for pinning in a prediction."""
    root.mkdir(parents=True, exist_ok=True)
    (root / "assets").mkdir(exist_ok=True)

    files = {
        "manifest.json": json.dumps(MANIFEST, indent=2).encode() + b"\n",
        "content.json": json.dumps(content(priority), indent=2).encode() + b"\n",
        "assets/probe_horse.png": png_bytes(),
    }
    digests = {}
    for name, data in files.items():
        (root / name).write_bytes(data)
        digests[name] = hashlib.sha256(data).hexdigest()
    return digests


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--priority", default=None,
                        help="Priority for the Load patch; omit for Content "
                             "Patcher's default, which is Exclusive")
    args = parser.parse_args()
    digests = build(args.priority)
    label = args.priority or "Exclusive (by omission)"
    print(f"built {PACK} with Load priority {label}")
    for name, digest in digests.items():
        print(f"  {digest}  {name}  ({(PACK / name).stat().st_size} bytes)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
