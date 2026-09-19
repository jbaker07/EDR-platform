"""Project setup: generate a real, buildable project for a creator.

The generator writes the project itself rather than copying an upstream
template wholesale, but every *version* in it is resolved live from upstream so
the result matches the current toolchain. What it produces is intended to be
read and reviewed: ordinary files in a normal layout, not a black box.
"""
from __future__ import annotations

import dataclasses
import re
from pathlib import Path
from typing import Any

from .versions import (BuildStyle, ResolvedVersions, fabric_build_style,
                       fabric_versions, java_release_for)

MOD_ID_RE = re.compile(r"^[a-z][a-z0-9_-]{1,63}$")
PACKAGE_RE = re.compile(r"^[a-z][a-z0-9_]*(\.[a-z][a-z0-9_]*)*$")


class ScaffoldError(ValueError):
    pass


@dataclasses.dataclass
class ScaffoldResult:
    root: Path
    game: str
    loader: str
    mod_id: str
    files: list[str]
    versions: dict[str, str]
    version_sources: dict[str, str]
    build_commands: list[str]
    notes: list[str] = dataclasses.field(default_factory=list)

    def as_dict(self) -> dict[str, Any]:
        return {
            "root": str(self.root), "game": self.game, "loader": self.loader,
            "mod_id": self.mod_id, "files": self.files, "versions": self.versions,
            "version_sources": self.version_sources,
            "build_commands": self.build_commands, "notes": self.notes,
        }


def _write(root: Path, rel: str, content: str, files: list[str]) -> None:
    path = root / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    files.append(rel)


def _class_name(mod_id: str) -> str:
    return "".join(part.capitalize() for part in re.split(r"[_-]", mod_id)) or "Mod"


def scaffold_fabric(root: Path, mod_id: str, *, package: str | None = None,
                    minecraft_version: str | None = None,
                    mod_name: str | None = None,
                    versions: ResolvedVersions | None = None) -> ScaffoldResult:
    if not MOD_ID_RE.match(mod_id):
        raise ScaffoldError(
            f"mod id {mod_id!r} is invalid: Fabric requires lowercase letters, digits, "
            "hyphen or underscore, 2-64 characters, starting with a letter")
    package = package or f"com.example.{mod_id.replace('-', '')}"
    if not PACKAGE_RE.match(package):
        raise ScaffoldError(f"package {package!r} is not a valid Java package name")

    versions = versions or fabric_versions(minecraft_version)
    mc = versions["minecraft_version"]
    java_release = java_release_for(mc)
    style = fabric_build_style(mc)
    cls = _class_name(mod_id)
    mod_name = mod_name or cls
    pkg_path = package.replace(".", "/")
    files: list[str] = []

    _write(root, "gradle.properties", f"""\
org.gradle.jvmargs=-Xmx2G
org.gradle.parallel=true
# IntelliJ IDEA is not yet fully compatible with the configuration cache.
org.gradle.configuration-cache=false

# Toolchain versions, resolved from the upstream Fabric example mod for
# Minecraft {mc}. Check https://fabricmc.net/develop before changing them.
minecraft_version={mc}
loader_version={versions['loader_version']}
loom_version={versions['loom_version']}

# Mod properties
version=1.0.0
group={package}

# Dependencies
fabric_api_version={versions.get('fabric_api_version', '')}
""", files)

    _write(root, "settings.gradle", f"""\
pluginManagement {{
    repositories {{
        maven {{
            name = 'Fabric'
            url = 'https://maven.fabricmc.net/'
        }}
        mavenCentral()
        gradlePluginPortal()
    }}
}}

rootProject.name = '{mod_id}'
""", files)

    config = style.mod_dependency_configuration
    mappings_line = ("    mappings loom.officialMojangMappings()\n"
                     if style.needs_mappings else
                     "    // Minecraft " + mc + " is not obfuscated, so no mappings are needed.\n")
    api_dependency = (
        f'    {config} "net.fabricmc.fabric-api:fabric-api:${{project.fabric_api_version}}"'
        if versions.get("fabric_api_version") else
        "    // No Fabric API version was published for this Minecraft version.")

    _write(root, "build.gradle", f"""\
plugins {{
    id 'net.fabricmc.fabric-loom' version "${{loom_version}}"
    id 'maven-publish'
}}

version = project.version
group = project.group

base {{
    archivesName = '{mod_id}'
}}

repositories {{
    // Loom adds the Minecraft and Fabric repositories itself.
}}

dependencies {{
    minecraft "com.mojang:minecraft:${{project.minecraft_version}}"
{mappings_line}    {config} "net.fabricmc:fabric-loader:${{project.loader_version}}"
{api_dependency}
}}

processResources {{
    inputs.property "version", project.version
    filesMatching("fabric.mod.json") {{
        expand "version": project.version
    }}
}}

tasks.withType(JavaCompile).configureEach {{
    it.options.release = {java_release}
}}

java {{
    withSourcesJar()
    sourceCompatibility = JavaVersion.VERSION_{java_release}
    targetCompatibility = JavaVersion.VERSION_{java_release}
}}
""", files)

    _write(root, "src/main/resources/fabric.mod.json", f"""\
{{
    "schemaVersion": 1,
    "id": "{mod_id}",
    "version": "${{version}}",
    "name": "{mod_name}",
    "description": "Created with ModCheck.",
    "authors": [],
    "license": "MIT",
    "environment": "*",
    "entrypoints": {{
        "main": [
            "{package}.{cls}"
        ]
    }},
    "mixins": [
        "{mod_id}.mixins.json"
    ],
    "depends": {{
        "fabricloader": ">={versions['loader_version']}",
        "minecraft": "~{mc}",
        "java": ">={java_release}"
    }}
}}
""", files)

    _write(root, f"src/main/resources/{mod_id}.mixins.json", f"""\
{{
    "required": true,
    "package": "{package}.mixin",
    "compatibilityLevel": "JAVA_{java_release}",
    "mixins": [],
    "injectors": {{
        "defaultRequire": 1
    }}
}}
""", files)

    _write(root, f"src/main/java/{pkg_path}/{cls}.java", f"""\
package {package};

import net.fabricmc.api.ModInitializer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class {cls} implements ModInitializer {{
    public static final String MOD_ID = "{mod_id}";
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);

    @Override
    public void onInitialize() {{
        LOGGER.info("{mod_name} initialised");
    }}
}}
""", files)

    _write(root, "gradle/wrapper/gradle-wrapper.properties", """\
distributionBase=GRADLE_USER_HOME
distributionPath=wrapper/dists
distributionUrl=https\\://services.gradle.org/distributions/gradle-9.5.1-bin.zip
networkTimeout=10000
validateDistributionUrl=true
zipStoreBase=GRADLE_USER_HOME
zipStorePath=wrapper/dists
""", files)

    _write(root, ".gitignore", """\
.gradle/
build/
run/
*.class
""", files)

    return ScaffoldResult(
        root=root, game="minecraft", loader="fabric", mod_id=mod_id, files=sorted(files),
        versions=dict(versions.values), version_sources=dict(versions.sources),
        build_commands=["gradle build"],
        notes=[
            f"Targets Minecraft {mc} and requires a JDK {java_release} toolchain.",
            "Uses a single main source set rather than the upstream template's split "
            "client/main sets; both are supported Fabric layouts.",
            ("Declares Mojang mappings, as upstream does for this version."
             if style.needs_mappings else
             f"Minecraft {mc} ships non-obfuscated, so the build declares no mappings and "
             f"uses '{style.mod_dependency_configuration}' for mod dependencies. A build "
             "script carried over from the 1.21.x era fails here with 'Cannot use Mojang "
             "mappings in a non-obfuscated environment'."),
        ])


GENERATORS = {("minecraft", "fabric"): scaffold_fabric}


def scaffold(game: str, loader: str, root: Path, mod_id: str, **kwargs: Any) -> ScaffoldResult:
    key = (game, loader)
    if key not in GENERATORS:
        supported = ", ".join(f"{g}/{l}" for g, l in sorted(GENERATORS))
        raise ScaffoldError(
            f"no project generator for {game}/{loader}. Implemented: {supported}")
    root.mkdir(parents=True, exist_ok=True)
    if any(root.iterdir()):
        raise ScaffoldError(f"{root} is not empty; refusing to overwrite an existing project")
    return GENERATORS[key](root, mod_id, **kwargs)
