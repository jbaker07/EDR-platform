"""Loading and querying the knowledge base.

The knowledge base is plain YAML on disk under ``packs/<game>/``:

    pack.yaml            game pack manifest (capabilities, ecosystem, toolchain, gaps)
    sources.yaml         every upstream artifact we retrieved, with provenance
    recipes/*.yaml       development recipes
    failures/*.yaml      documented failure cases
    interactions/*.yaml  known interaction rules
    resolutions/*.yaml   resolutions with their verification scope
    examples/*.yaml      reusable templates, libraries, analyzers

Files are the source of truth; this module only indexes them.
"""
from __future__ import annotations

import dataclasses
from pathlib import Path
from typing import Any, Iterator

import yaml

from .paths import GAMES, packs_dir

RECORD_DIRS = {
    "recipe": "recipes",
    "failure": "failures",
    "interaction": "interactions",
    "resolution": "resolutions",
    "example": "examples",
}


@dataclasses.dataclass(frozen=True)
class Record:
    """One knowledge record plus where it came from."""

    kind: str
    game: str
    id: str
    data: dict[str, Any]
    path: Path

    def __getitem__(self, key: str) -> Any:
        return self.data[key]

    def get(self, key: str, default: Any = None) -> Any:
        return self.data.get(key, default)


def _load_yaml(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as fh:
        return yaml.safe_load(fh)


class Pack:
    """One game's pack."""

    def __init__(self, game: str, root: Path) -> None:
        self.game = game
        self.root = root
        self._manifest: dict[str, Any] | None = None
        self._sources: dict[str, dict[str, Any]] | None = None
        self._records: dict[str, list[Record]] | None = None

    # -- manifest ---------------------------------------------------------
    @property
    def manifest(self) -> dict[str, Any]:
        if self._manifest is None:
            path = self.root / "pack.yaml"
            self._manifest = _load_yaml(path) if path.exists() else {}
        return self._manifest

    @property
    def manifest_path(self) -> Path:
        return self.root / "pack.yaml"

    def capability(self, name: str) -> dict[str, Any]:
        return (self.manifest.get("capabilities") or {}).get(name, {})

    def capability_status(self, name: str) -> str:
        return self.capability(name).get("status", "unsupported")

    # -- sources ----------------------------------------------------------
    @property
    def sources(self) -> dict[str, dict[str, Any]]:
        if self._sources is None:
            path = self.root / "sources.yaml"
            raw = _load_yaml(path) if path.exists() else None
            items = (raw or {}).get("sources", []) if isinstance(raw, dict) else (raw or [])
            self._sources = {s["id"]: s for s in items if isinstance(s, dict) and "id" in s}
        return self._sources

    @property
    def sources_path(self) -> Path:
        return self.root / "sources.yaml"

    # -- records ----------------------------------------------------------
    def _load_records(self) -> dict[str, list[Record]]:
        if self._records is None:
            out: dict[str, list[Record]] = {k: [] for k in RECORD_DIRS}
            for kind, subdir in RECORD_DIRS.items():
                d = self.root / subdir
                if not d.is_dir():
                    continue
                for path in sorted(d.glob("*.yaml")):
                    data = _load_yaml(path)
                    if data is None:
                        continue
                    entries = data if isinstance(data, list) else [data]
                    for entry in entries:
                        if not isinstance(entry, dict) or "id" not in entry:
                            continue
                        out[kind].append(
                            Record(kind=kind, game=self.game, id=entry["id"], data=entry, path=path)
                        )
            self._records = out
        return self._records

    def records(self, kind: str) -> list[Record]:
        return list(self._load_records().get(kind, []))

    def all_records(self) -> Iterator[Record]:
        for kind in RECORD_DIRS:
            yield from self.records(kind)

    def record(self, kind: str, record_id: str) -> Record | None:
        for rec in self.records(kind):
            if rec.id == record_id:
                return rec
        return None

    def counts(self) -> dict[str, int]:
        return {kind: len(self.records(kind)) for kind in RECORD_DIRS} | {
            "source": len(self.sources)
        }


class Store:
    """All game packs."""

    def __init__(self, root: Path | None = None) -> None:
        self.root = Path(root) if root else packs_dir()
        self._packs: dict[str, Pack] = {}

    def pack(self, game: str) -> Pack:
        if game not in self._packs:
            self._packs[game] = Pack(game, self.root / game)
        return self._packs[game]

    @property
    def games(self) -> list[str]:
        """Games that have a pack directory on disk, in the canonical order."""
        present = {p.name for p in self.root.iterdir() if p.is_dir()} if self.root.is_dir() else set()
        ordered = [g for g in GAMES if g in present]
        ordered += sorted(present - set(GAMES))
        return ordered

    def packs(self) -> list[Pack]:
        return [self.pack(g) for g in self.games]

    def all_records(self, kind: str | None = None) -> Iterator[Record]:
        for pack in self.packs():
            if kind:
                yield from pack.records(kind)
            else:
                yield from pack.all_records()

    def find(self, kind: str, record_id: str, game: str | None = None) -> Record | None:
        packs = [self.pack(game)] if game else self.packs()
        for pack in packs:
            rec = pack.record(kind, record_id)
            if rec is not None:
                return rec
        return None

    def search(self, query: str, kinds: tuple[str, ...] | None = None,
               game: str | None = None) -> list[Record]:
        """Substring search over record id, title and intent/summary text."""
        q = query.lower().strip()
        hits: list[Record] = []
        packs = [self.pack(game)] if game else self.packs()
        for pack in packs:
            for rec in pack.all_records():
                if kinds and rec.kind not in kinds:
                    continue
                haystack = " ".join(
                    str(rec.get(field, ""))
                    for field in ("id", "title", "intent", "symptom", "mechanism", "category")
                ).lower()
                if q in haystack:
                    hits.append(rec)
        return hits
