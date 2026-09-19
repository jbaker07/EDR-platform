"""Live version resolution for creator project setup.

Toolchain versions are fetched from upstream at scaffold time, never recalled.
Minecraft moved from ``1.21.x`` to a year-based scheme (``26.1`` onwards) during
2026; a generator with versions baked in would have produced a project that
cannot resolve its dependencies.
"""
from __future__ import annotations

import re
from dataclasses import dataclass, field
from typing import Any

from ..acquire import FetchError, fetch

FABRIC_TEMPLATE = ("https://raw.githubusercontent.com/FabricMC/fabric-example-mod/"
                   "{branch}/gradle.properties")
FABRIC_LOADER_META = "https://meta.fabricmc.net/v2/versions/loader"
MOJANG_MANIFEST = "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json"


@dataclass
class ResolvedVersions:
    values: dict[str, str] = field(default_factory=dict)
    sources: dict[str, str] = field(default_factory=dict)

    def __getitem__(self, key: str) -> str:
        return self.values[key]

    def get(self, key: str, default: Any = None) -> Any:
        return self.values.get(key, default)


def _parse_properties(raw: bytes) -> dict[str, str]:
    out: dict[str, str] = {}
    for line in raw.decode("utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, _, value = line.partition("=")
        out[key.strip()] = value.strip()
    return out


def latest_minecraft_release() -> tuple[str, str]:
    """Return (version, source_url) for the current Minecraft release."""
    result = fetch(MOJANG_MANIFEST, suffix=".json")
    if result.status != 200:
        raise FetchError(f"Mojang version manifest: HTTP {result.status}")
    import json
    data = json.loads(result.path.read_bytes())
    return data["latest"]["release"], MOJANG_MANIFEST


def fabric_versions(minecraft_version: str | None = None) -> ResolvedVersions:
    """Resolve the toolchain versions for a Fabric project.

    The Fabric example mod publishes a branch per Minecraft version whose
    gradle.properties is the authoritative set of matching versions. We read
    that rather than guessing compatible combinations.
    """
    if minecraft_version is None:
        minecraft_version, _ = latest_minecraft_release()
    url = FABRIC_TEMPLATE.format(branch=minecraft_version)
    result = fetch(url, suffix=".properties")
    if result.status != 200:
        raise FetchError(
            f"no Fabric example-mod branch for Minecraft {minecraft_version} "
            f"(HTTP {result.status} for {url}). Pass a Minecraft version that has one.")
    props = _parse_properties(result.path.read_bytes())

    required = ("minecraft_version", "loader_version", "loom_version")
    missing = [k for k in required if k not in props]
    if missing:
        raise FetchError(f"upstream template is missing {missing}")

    values = {
        "minecraft_version": props["minecraft_version"],
        "loader_version": props["loader_version"],
        "loom_version": props["loom_version"],
        "fabric_api_version": props.get("fabric_api_version", ""),
    }
    return ResolvedVersions(values=values, sources={k: url for k in values})


JAVA_FOR_MIXIN = re.compile(r"JAVA_(\d+)")


def java_release_for(minecraft_version: str) -> int:
    """Java release the Minecraft version's mod toolchain targets.

    Derived from the upstream example mod's own mixin compatibility level, so
    it tracks upstream rather than a table we would have to maintain.
    """
    url = (f"https://raw.githubusercontent.com/FabricMC/fabric-example-mod/"
           f"{minecraft_version}/src/main/resources/modid.mixins.json")
    result = fetch(url, suffix=".json")
    if result.status != 200:
        raise FetchError(f"cannot determine Java release for {minecraft_version}: "
                         f"HTTP {result.status}")
    import json
    data = json.loads(result.path.read_bytes())
    match = JAVA_FOR_MIXIN.search(str(data.get("compatibilityLevel", "")))
    if not match:
        raise FetchError(f"upstream mixin config has no compatibilityLevel for "
                         f"{minecraft_version}")
    return int(match.group(1))


FABRIC_TEMPLATE_BUILD = ("https://raw.githubusercontent.com/FabricMC/fabric-example-mod/"
                         "{branch}/build.gradle")


@dataclass
class BuildStyle:
    """How a Fabric build script must be written for one Minecraft version.

    Minecraft stopped shipping obfuscated during the 26.x line. That removed the
    remapping step, so ``mappings loom.officialMojangMappings()`` became invalid
    and ``modImplementation`` became plain ``implementation``. Rather than
    encode a cutoff version we would have to maintain, this is read from the
    upstream example mod's own build script for the target version.
    """

    needs_mappings: bool
    mod_dependency_configuration: str
    source_url: str


def fabric_build_style(minecraft_version: str) -> BuildStyle:
    url = FABRIC_TEMPLATE_BUILD.format(branch=minecraft_version)
    result = fetch(url, suffix=".gradle")
    if result.status != 200:
        raise FetchError(f"cannot read upstream build script for Minecraft "
                         f"{minecraft_version}: HTTP {result.status}")
    text = result.path.read_text(encoding="utf-8", errors="replace")
    needs_mappings = "officialMojangMappings()" in text
    configuration = "modImplementation" if "modImplementation" in text else "implementation"
    return BuildStyle(needs_mappings=needs_mappings,
                      mod_dependency_configuration=configuration,
                      source_url=url)
