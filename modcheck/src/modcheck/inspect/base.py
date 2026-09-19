"""Shared inspection types."""
from __future__ import annotations

import dataclasses
import hashlib
import zipfile
from pathlib import Path
from typing import Any


class InspectionError(Exception):
    """The artifact could not be read as the kind we thought it was."""


@dataclasses.dataclass(frozen=True)
class Fact:
    """One thing we know about an artifact, and how we know it."""

    key: str
    value: Any
    evidence_class: str  # "declared" | "extracted"
    locator: str = ""  # file/offset inside the artifact

    def as_dict(self) -> dict[str, Any]:
        d = {"key": self.key, "value": self.value, "evidence_class": self.evidence_class}
        if self.locator:
            d["locator"] = self.locator
        return d


@dataclasses.dataclass
class Inspection:
    """The result of inspecting one artifact."""

    path: str
    sha256: str
    bytes: int
    kind: str
    game: str | None = None
    loader: str | None = None
    facts: list[Fact] = dataclasses.field(default_factory=list)
    entries: list[str] = dataclasses.field(default_factory=list)
    checked: list[str] = dataclasses.field(default_factory=list)
    not_checked: list[str] = dataclasses.field(default_factory=list)
    warnings: list[str] = dataclasses.field(default_factory=list)

    def add(self, key: str, value: Any, evidence_class: str, locator: str = "") -> None:
        self.facts.append(Fact(key, value, evidence_class, locator))

    def fact(self, key: str) -> Any:
        for f in self.facts:
            if f.key == key:
                return f.value
        return None

    def facts_dict(self) -> dict[str, Any]:
        return {f.key: f.value for f in self.facts}

    def as_dict(self) -> dict[str, Any]:
        return {
            "path": self.path,
            "sha256": self.sha256,
            "bytes": self.bytes,
            "kind": self.kind,
            "game": self.game,
            "loader": self.loader,
            "facts": [f.as_dict() for f in self.facts],
            "entries": self.entries,
            "coverage": {"checked": self.checked, "not_checked": self.not_checked},
            "warnings": self.warnings,
        }


@dataclasses.dataclass(frozen=True)
class Artifact:
    """A file on disk, identified by content."""

    path: Path
    sha256: str
    size: int

    @classmethod
    def of(cls, path: str | Path) -> "Artifact":
        p = Path(path)
        if not p.exists():
            raise InspectionError(f"no such artifact: {p}")
        h = hashlib.sha256()
        size = 0
        with p.open("rb") as fh:
            for chunk in iter(lambda: fh.read(1 << 20), b""):
                h.update(chunk)
                size += len(chunk)
        return cls(path=p, sha256=h.hexdigest(), size=size)

    def base_inspection(self, kind: str) -> Inspection:
        return Inspection(path=str(self.path), sha256=self.sha256, bytes=self.size, kind=kind)


def zip_entries(path: Path, limit: int = 5000) -> list[str]:
    try:
        with zipfile.ZipFile(path) as zf:
            return zf.namelist()[:limit]
    except (zipfile.BadZipFile, OSError) as exc:
        raise InspectionError(f"not a readable zip: {exc}") from exc


def read_zip_member(path: Path, member: str, max_bytes: int = 4 << 20) -> bytes:
    """Read one member. Refuses oversized members rather than inflating a zip bomb."""
    with zipfile.ZipFile(path) as zf:
        info = zf.getinfo(member)
        if info.file_size > max_bytes:
            raise InspectionError(
                f"{member} is {info.file_size} bytes, over the {max_bytes} byte inspection limit")
        with zf.open(info) as fh:
            return fh.read(max_bytes + 1)[:max_bytes]


def find_member(entries: list[str], *names: str) -> str | None:
    """Find a manifest by exact name at any depth, preferring the shallowest."""
    wanted = {n.lower() for n in names}
    hits = [e for e in entries if e.split("/")[-1].lower() in wanted]
    if not hits:
        return None
    return min(hits, key=lambda e: (e.count("/"), len(e)))
