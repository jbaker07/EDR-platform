"""Retrieval over what we have already fetched.

258 sources are registered and 243 are in the evidence cache, but until now the
creator path could not read any of them: `guide` ranked recipes and nothing
searched the material behind those recipes. This closes that gap with the two
cheapest useful retrievals, and deliberately stops there.

**Exact symbol lookup** over installed toolchain definitions -- the jars Loom
actually resolved for a build. This is the strongest evidence available short
of running the game, because it is the artifact the creator's code will compile
against, not a description of it. Minecraft 26.3 ships non-obfuscated, so the
signatures come out readable. Documentation would have told us `SavedData` has
`load`/`save(CompoundTag)` methods; the jar says the 26.3 API is
`SavedDataType(Identifier, Supplier<T>, Codec<T>, DataFixTypes)` and that the
accessor is `ServerLevel.getDataStorage()` returning `SavedDataStorage`, not
`DimensionDataStorage`. Only one of those is checkable.

**Exact text search** over cached documents, returning the source id, the line,
and the hash of the bytes searched -- so a citation can be re-checked rather
than taken on trust.

What this is not: an embedding index, a knowledge graph, or a ranking model.
Nothing here scores relevance. A hit is an exact match in a named file at a
named offset, or it is not a hit. More elaborate retrieval waits for evidence
that this is insufficient.

Byte discipline: code, schemas and artifacts are matched and hashed as raw
bytes. The documentation-text normalisation used by the drift detector exists
to tolerate injected nonces in rendered HTML, and it must never be used to call
two pieces of code equivalent.
"""
from __future__ import annotations

import dataclasses
import hashlib
import re
import subprocess
import zipfile
from pathlib import Path
from typing import Iterable, Iterator

from .paths import evidence_cache
from .store import Store


@dataclasses.dataclass
class SymbolHit:
    """One exact symbol, as the compiled artifact declares it."""
    symbol: str
    kind: str                 # "class" | "method" | "field"
    signature: str
    artifact: str             # path to the jar
    artifact_sha256: str
    entry: str                # the class file inside the jar
    entry_sha256: str
    source_id: str | None = None

    def citation(self) -> str:
        return (f"{self.symbol} :: {self.signature} "
                f"[{Path(self.artifact).name}@{self.artifact_sha256[:12]} "
                f"{self.entry}@{self.entry_sha256[:12]}]")


@dataclasses.dataclass
class TextHit:
    source_id: str
    path: str
    line: int
    text: str
    sha256: str

    def citation(self) -> str:
        return f"{self.source_id}:{self.line} [{self.sha256[:12]}]"


def _sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


class JarIndex:
    """Exact-symbol lookup in a compiled artifact.

    Class names are read from the archive's own entry list, so the index is
    what the jar contains rather than what anything says it contains.
    """

    def __init__(self, jar: Path, javap: Path | None = None) -> None:
        self.jar = Path(jar)
        if not self.jar.is_file():
            raise FileNotFoundError(f"no such artifact: {self.jar}")
        self.javap = Path(javap) if javap else None
        self._sha: str | None = None
        self._classes: dict[str, str] | None = None   # fqcn -> zip entry name

    @property
    def sha256(self) -> str:
        if self._sha is None:
            self._sha = _sha256(self.jar.read_bytes())
        return self._sha

    @property
    def classes(self) -> dict[str, str]:
        if self._classes is None:
            out: dict[str, str] = {}
            with zipfile.ZipFile(self.jar) as zf:
                for name in zf.namelist():
                    if not name.endswith(".class") or "$" in name:
                        continue
                    out[name[: -len(".class")].replace("/", ".")] = name
            self._classes = out
        return self._classes

    def find_classes(self, pattern: str, limit: int = 40) -> list[str]:
        """Class names matching a regex. Exact text, never fuzzy."""
        rx = re.compile(pattern, re.I)
        return sorted(c for c in self.classes if rx.search(c))[:limit]

    def _entry_sha(self, fqcn: str) -> str:
        entry = self.classes[fqcn]
        with zipfile.ZipFile(self.jar) as zf:
            return _sha256(zf.read(entry))

    def symbol(self, fqcn: str) -> list[SymbolHit]:
        """Declared members of a class, as `javap` reports them.

        `javap` is the JDK's own class reader. Reusing it rather than parsing
        class files ourselves is the whole point: a disassembler we wrote could
        disagree with the compiler, and this is evidence a creator's code will
        be written against.
        """
        if fqcn not in self.classes:
            return []
        javap = str(self.javap) if self.javap else "javap"
        try:
            proc = subprocess.run(
                [javap, "-cp", str(self.jar), fqcn],
                capture_output=True, text=True, timeout=120,
                # JAVA_TOOL_OPTIONS prints a banner onto stderr that is not
                # output; the env is trimmed so the parse stays clean.
                env={"PATH": "/usr/bin:/bin", "JAVA_TOOL_OPTIONS": ""})
        except (OSError, subprocess.SubprocessError) as exc:
            raise RuntimeError(f"javap failed for {fqcn}: {exc}") from exc
        if proc.returncode != 0:
            return []

        entry = self.classes[fqcn]
        entry_sha = self._entry_sha(fqcn)
        hits: list[SymbolHit] = []
        for raw in proc.stdout.splitlines():
            line = raw.strip()
            if not line or line.startswith("Compiled from") or line in ("}", "{"):
                continue
            if line.endswith("{"):
                hits.append(SymbolHit(fqcn, "class", line.rstrip(" {"), str(self.jar),
                                      self.sha256, entry, entry_sha))
                continue
            if not line.endswith(";"):
                continue
            kind = "method" if "(" in line else "field"
            hits.append(SymbolHit(fqcn, kind, line.rstrip(";"), str(self.jar),
                                  self.sha256, entry, entry_sha))
        return hits

    def member(self, fqcn: str, name: str) -> list[SymbolHit]:
        """Members of a class whose signature mentions `name`, exactly."""
        needle = name.lower()
        return [h for h in self.symbol(fqcn)
                if h.kind != "class" and needle in h.signature.lower()]


class EvidenceIndex:
    """Exact text search over the fetched evidence cache.

    Every hit carries the hash of the file it came from, so a citation names
    bytes rather than a title. Binary sources are skipped rather than
    force-decoded: a match inside a mangled decode is not a match.
    """

    def __init__(self, store: Store | None = None, cache: Path | None = None) -> None:
        self.store = store or Store()
        self.cache = Path(cache) if cache else evidence_cache()

    def _entries(self, game: str | None) -> Iterator[tuple[str, dict]]:
        for pack in self.store.packs():
            if game and pack.game != game:
                continue
            for source_id, record in pack.sources.items():
                yield source_id, record

    def search(self, pattern: str, *, game: str | None = None,
               source_ids: Iterable[str] | None = None,
               limit: int = 30, regex: bool = False) -> list[TextHit]:
        rx = re.compile(pattern if regex else re.escape(pattern), re.I)
        wanted = set(source_ids) if source_ids else None
        hits: list[TextHit] = []
        for source_id, record in self._entries(game):
            if wanted is not None and source_id not in wanted:
                continue
            rel = record.get("evidence_path")
            if not rel:
                continue
            path = self.cache / rel
            if not path.is_file():
                continue
            raw = path.read_bytes()
            try:
                text = raw.decode("utf-8")
            except UnicodeDecodeError:
                continue
            digest = _sha256(raw)
            for number, line in enumerate(text.splitlines(), start=1):
                if rx.search(line):
                    hits.append(TextHit(source_id=source_id, path=str(path),
                                        line=number, text=line.strip()[:300],
                                        sha256=digest))
                    if len(hits) >= limit:
                        return hits
        return hits
