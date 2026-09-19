"""A user's *exact* configuration.

The player workflow starts here. What matters is not "which mods do you have"
but precisely which artifacts, at which versions, with which content hashes,
in which order, and -- critically -- **how much of that we actually know**.

``files_known_complete`` is the field that keeps the analysis honest: if we only
have a partial view of an installation, the absence of a file proves nothing,
and every check that depends on absence must return unknown rather than false.
"""
from __future__ import annotations

import dataclasses
import json
import re
import zlib
from pathlib import Path
from typing import Any, Iterable

from ..inspect import inspect_path
from ..inspect.base import Inspection

_LEADING_RELATIVE = re.compile(r"^(?:\.{1,2}/)+")


def normalize_path(path: str) -> str:
    """One canonical form for every path comparison.

    Lowercased, backslashes turned into forward slashes, and leading ``./`` or
    ``../`` segments removed -- upstream metadata writes game-root paths as
    ``..\\loader.exe`` and installations list them without the prefix. Both
    sides of every comparison go through this function, so a rule can never be
    missed because of separator or case differences.
    """
    norm = path.strip().replace("\\", "/").lower()
    return _LEADING_RELATIVE.sub("", norm)


@dataclasses.dataclass
class InstalledArtifact:
    """One artifact in a configuration, identified by content where possible."""

    name: str
    path: str | None = None
    sha256: str | None = None
    crc32: int | None = None
    version: str | None = None
    mod_id: str | None = None
    active: bool = True
    load_index: int | None = None
    inspection: Inspection | None = None
    declared_dependencies: list[dict] = dataclasses.field(default_factory=list)

    @property
    def key(self) -> str:
        return (self.mod_id or self.name).lower()

    def as_dict(self) -> dict[str, Any]:
        out = {k: v for k, v in dataclasses.asdict(self).items() if k != "inspection"}
        if self.crc32 is not None:
            out["crc32_hex"] = f"{self.crc32:08X}"
        return out


@dataclasses.dataclass
class Installation:
    """Everything we know about one configuration, and the limits of that knowledge."""

    game: str
    game_version: str | None = None
    loader_versions: dict[str, str] = dataclasses.field(default_factory=dict)
    artifacts: list[InstalledArtifact] = dataclasses.field(default_factory=list)
    files: set[str] = dataclasses.field(default_factory=set)
    files_known_complete: bool = False
    platform: str | None = None
    notes: list[str] = dataclasses.field(default_factory=list)

    # -- lookups ---------------------------------------------------------
    def artifact_names(self) -> set[str]:
        return {normalize_path(a.name) for a in self.artifacts}

    def active_names(self) -> set[str]:
        return {normalize_path(a.name) for a in self.artifacts if a.active}

    def by_name(self, name: str) -> InstalledArtifact | None:
        target = normalize_path(name)
        for a in self.artifacts:
            if normalize_path(a.name) == target:
                return a
        return None

    def has_file(self, path: str) -> bool | None:
        """Three-valued: True, False, or None when our file view is incomplete."""
        norm = normalize_path(path)
        if norm in self.files:
            return True
        # A plugin we know about is a file we know about.
        if norm in self.artifact_names():
            return True
        if self.files_known_complete:
            return False
        # We know the plugin list completely even when we do not know every file.
        if norm.endswith((".esp", ".esm", ".esl")) and self.artifacts:
            return False
        return None

    def known_paths(self) -> set[str]:
        return self.files | self.artifact_names()

    def as_dict(self) -> dict[str, Any]:
        return {
            "game": self.game,
            "game_version": self.game_version,
            "loader_versions": self.loader_versions,
            "platform": self.platform,
            "artifacts": [a.as_dict() for a in self.artifacts],
            "file_count": len(self.files),
            "files_known_complete": self.files_known_complete,
            "notes": self.notes,
        }

    # -- construction ----------------------------------------------------
    @classmethod
    def from_paths(cls, game: str, paths: Iterable[str | Path], *,
                   game_version: str | None = None,
                   files_known_complete: bool = False) -> "Installation":
        """Build a configuration by inspecting real files on disk."""
        inst = cls(game=game, game_version=game_version,
                   files_known_complete=files_known_complete)
        for index, path in enumerate(paths):
            p = Path(path)
            ins = inspect_path(p, game=game)
            crc = None
            try:
                crc = zlib.crc32(p.read_bytes()) & 0xFFFFFFFF
            except (OSError, MemoryError):
                pass
            inst.artifacts.append(InstalledArtifact(
                name=p.name,
                path=str(p),
                sha256=ins.sha256,
                crc32=crc,
                version=ins.fact("version"),
                mod_id=ins.fact("mod_id"),
                load_index=index,
                inspection=ins,
                declared_dependencies=ins.fact("dependencies") or [],
            ))
            inst.files.add(normalize_path(p.name))
            for entry in ins.entries:
                inst.files.add(normalize_path(entry))
        return inst

    @classmethod
    def from_json(cls, path: str | Path) -> "Installation":
        """Load a configuration description (e.g. one a player exported)."""
        data = json.loads(Path(path).read_text(encoding="utf-8"))
        inst = cls(
            game=data["game"],
            game_version=data.get("game_version"),
            loader_versions=data.get("loader_versions", {}),
            files=set(normalize_path(f) for f in data.get("files", [])),
            files_known_complete=data.get("files_known_complete", False),
            platform=data.get("platform"),
            notes=data.get("notes", []),
        )
        for index, raw in enumerate(data.get("artifacts", [])):
            crc = raw.get("crc32")
            if isinstance(crc, str):
                crc = int(crc, 16)
            inst.artifacts.append(InstalledArtifact(
                name=raw["name"],
                path=raw.get("path"),
                sha256=raw.get("sha256"),
                crc32=crc,
                version=raw.get("version"),
                mod_id=raw.get("mod_id"),
                active=raw.get("active", True),
                load_index=raw.get("load_index", index),
                declared_dependencies=raw.get("dependencies", []),
            ))
        return inst
