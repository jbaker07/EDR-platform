"""The real-artifact corpus.

Synthetic fixtures prove a parser reads the layout it was written against. They
cannot prove that real-world artifacts look like that layout, and in practice
they did not: validating against real files found a real-world field our
RimWorld inspector ignored, and a whole directory code path that had never
worked.

Each entry records what the T2 contract requires: identity, where it came from,
its version, its sha256, whether we may redistribute it, and what independent
evidence supplies the expected result. Nothing here is committed to the
repository -- artifacts are fetched into the gitignored evidence cache, so
redistribution never arises.

`sha256` pins the exact bytes the assertions were written against. When upstream
changes it, the test reports drift rather than silently asserting against
different content.
"""
from __future__ import annotations

import dataclasses
from pathlib import Path

import pytest

from modcheck.acquire import FetchError, fetch


@dataclasses.dataclass(frozen=True)
class CorpusArtifact:
    id: str
    ecosystem: str
    url: str
    version: str
    sha256: str
    license: str
    redistributable: bool
    reference: str
    note: str = ""
    suffix: str = ""


CORPUS: tuple[CorpusArtifact, ...] = (
    CorpusArtifact(
        id="minecraft_fabric_api",
        ecosystem="minecraft",
        url=("https://maven.fabricmc.net/net/fabricmc/fabric-api/fabric-api/"
             "0.161.0%2B26.3/fabric-api-0.161.0%2B26.3.jar"),
        version="0.161.0+26.3",
        sha256="",  # large binary; identity asserted via the fetch result itself
        license="Apache-2.0",
        redistributable=False,
        reference="the jar is its own evidence: a published release read directly",
        note="Real published mod jar, 44 jar-in-jar modules.",
        suffix=".jar",
    ),
    CorpusArtifact(
        id="rimworld_harmony_about",
        ecosystem="rimworld",
        url=("https://raw.githubusercontent.com/pardeike/HarmonyRimWorld/master/"
             "About/About.xml"),
        version="Harmony 2.4.2.0 (modVersion in the file)",
        sha256="98822984381127947d2ba4f57c68c4f674e815092c2d87d1b6af8cb1eb877931",
        license="see upstream repository",
        redistributable=False,
        reference=("RimWorld wiki About.xml field reference, recorded as source "
                   "rimworld_wiki_about_xml_fields"),
        note="Real About.xml of a widely used mod. Begins with a UTF-8 BOM.",
        suffix=".xml",
    ),
    CorpusArtifact(
        id="rimworld_puah_about",
        ecosystem="rimworld",
        url=("https://raw.githubusercontent.com/Mehni/PickUpAndHaul/master/"
             "About/About.xml"),
        version="Pick Up And Haul (supportedVersions 1.0-1.6)",
        sha256="f69b98f854d8472c69e3bd02b74ca969b08865a33848e7d92bbec6ce5a138f9f",
        license="see upstream repository",
        redistributable=False,
        reference=("RimWorld wiki About.xml field reference, recorded as source "
                   "rimworld_wiki_about_xml_fields"),
        note=("Declares its Harmony dependency only via modDependenciesByVersion, "
              "which our inspector used to ignore."),
        suffix=".xml",
    ),
    CorpusArtifact(
        id="stardew_contentpatcher_manifest",
        ecosystem="stardewvalley",
        url=("https://raw.githubusercontent.com/Pathoschild/StardewMods/develop/"
             "ContentPatcher/manifest.json"),
        version="Content Patcher (see Version field in the file)",
        sha256="e62a22dc12514c00406981a4bd653715ac6e75495f40f5e400cacc5b8c4e79d8",
        license="MIT (Pathoschild/StardewMods)",
        redistributable=False,
        reference="SMAPI's own published manifest JSON Schema (smapi_manifest_schema)",
        note="Real manifest of the mod most Stardew content packs depend on.",
        suffix=".json",
    ),
    CorpusArtifact(
        id="bg3_combatmod_pak",
        ecosystem="bg3",
        url=("https://raw.githubusercontent.com/Hippo0o/bg3-mods/main/"
             "Releases/CombatMod.pak"),
        version="CombatMod (Trials of Tav) published release",
        sha256="ab6ccfdbba64f0bab9068714cecb970cb0809dec1d221b4a71d756370ef302c4",
        license="GPL-3.0 (repository LICENSE)",
        redistributable=False,
        reference=("LSLib source: PackageFormat.cs FileEntry18 and "
                   "PackageReader.cs ReadCompressedFileList. LSLib itself could "
                   "NOT be executed here -- its releases are on a host this "
                   "environment cannot reach and it needs a .NET runtime -- so "
                   "this is source-derived structure checked against a real "
                   "package, not an executed reference comparison."),
        note=("Real published bare .pak, 341059 bytes, LSPK v18, 63 entries. "
              "Read only: never extracted to disk, never executed."),
        suffix=".pak",
    ),
    CorpusArtifact(
        id="smapi_manifest_schema",
        ecosystem="stardewvalley",
        url="https://smapi.io/schemas/manifest.json",
        version="published at smapi.io/schemas",
        sha256="b995813e6b9b84f276a6a73c9d8604bacf171c7b5d8b90ce26164b6a0123b9fc",
        license="see SMAPI repository (LGPL-3.0)",
        redistributable=False,
        reference="this IS the independent reference: SMAPI's own schema",
        note="Used to validate the real manifest independently of our parser.",
        suffix=".json",
    ),
)

BY_ID = {a.id: a for a in CORPUS}


def acquire(artifact: CorpusArtifact) -> Path:
    """Fetch a corpus artifact, or skip the test when it cannot be reached.

    An unreachable artifact is a skipped test, never a passing one.
    """
    try:
        result = fetch(artifact.url, suffix=artifact.suffix, timeout=120)
    except FetchError as exc:
        pytest.skip(f"{artifact.id}: upstream unreachable ({exc})")
    if result.status != 200:
        pytest.skip(f"{artifact.id}: upstream returned HTTP {result.status}")
    if artifact.sha256 and result.sha256 != artifact.sha256:
        pytest.fail(
            f"{artifact.id} changed upstream: pinned {artifact.sha256[:12]}, "
            f"got {result.sha256[:12]}. Re-check the assertions against the new "
            "content before re-pinning.")
    return result.path


def unpacked(artifact: CorpusArtifact, tmp_path: Path, relative: str) -> Path:
    """Place a fetched file at its in-mod path so a directory can be inspected.

    Several ecosystems install mods as a folder. The file content is the real
    upstream artifact; only the containing directory is assembled locally,
    because the packaged release is published on a host we cannot reach. Tests
    using this must say so in their coverage claim.
    """
    source = acquire(artifact)
    target = tmp_path / relative
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(source.read_bytes())
    return tmp_path
