"""Reviewable project changes.

The creator path has to produce changes, not advice. Each change generator
reads the real project, produces the new content for the files it touches, and
returns a unified diff. Nothing is written unless the caller asks for it, so the
diff is reviewed first.

Generators here are deliberately the version-stable kinds of change -- wiring,
manifests, build files, mixin registration. Content changes that depend on a
specific game version's registries belong in recipes with their own evidence,
not in a generator that would silently rot.
"""
from __future__ import annotations

import dataclasses
import difflib
import json
import re
from pathlib import Path
from typing import Any, Callable

from .scaffold import MOD_ID_RE


class ApplyError(ValueError):
    pass


@dataclasses.dataclass
class FileChange:
    path: str          # project-relative
    before: str | None  # None when the file is created
    after: str

    @property
    def created(self) -> bool:
        return self.before is None

    def diff(self) -> str:
        before = (self.before or "").splitlines(keepends=True)
        after = self.after.splitlines(keepends=True)
        return "".join(difflib.unified_diff(
            before, after,
            fromfile=f"a/{self.path}" if self.before is not None else "/dev/null",
            tofile=f"b/{self.path}"))


@dataclasses.dataclass
class ChangeSet:
    generator: str
    description: str
    changes: list[FileChange]
    notes: list[str] = dataclasses.field(default_factory=list)
    follow_up: list[str] = dataclasses.field(default_factory=list)

    def diff(self) -> str:
        return "\n".join(c.diff() for c in self.changes)

    def write(self, root: Path) -> list[str]:
        written = []
        for change in self.changes:
            path = root / change.path
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(change.after, encoding="utf-8")
            written.append(change.path)
        return written

    def as_dict(self) -> dict[str, Any]:
        return {
            "generator": self.generator,
            "description": self.description,
            "files": [{"path": c.path, "created": c.created} for c in self.changes],
            "notes": self.notes,
            "follow_up": self.follow_up,
            "diff": self.diff(),
        }


# --- project reading -----------------------------------------------------
def _read(root: Path, rel: str) -> str:
    path = root / rel
    if not path.exists():
        raise ApplyError(f"{rel} not found in {root}; is this a Fabric project?")
    return path.read_text(encoding="utf-8")


def _fabric_manifest_path(root: Path) -> str:
    candidates = list(root.glob("src/*/resources/fabric.mod.json"))
    if not candidates:
        raise ApplyError("no src/*/resources/fabric.mod.json; not a Fabric project")
    return str(candidates[0].relative_to(root))


def _load_manifest(root: Path) -> tuple[str, dict]:
    rel = _fabric_manifest_path(root)
    text = _read(root, rel)
    # `version` is templated by processResources; keep the raw text for writing back.
    try:
        data = json.loads(text.replace("${version}", "0.0.0-template"))
    except json.JSONDecodeError as exc:
        raise ApplyError(f"{rel} is not valid JSON: {exc}") from exc
    return rel, data


def _dump_manifest(original: str, data: dict) -> str:
    text = json.dumps(data, indent=4)
    return text.replace('"0.0.0-template"', '"${version}"') + "\n"


# --- generators ----------------------------------------------------------
def add_dependency(root: Path, *, mod_id: str, versions: str = "*",
                   required: bool = True) -> ChangeSet:
    """Declare a dependency in fabric.mod.json."""
    rel, data = _load_manifest(root)
    before = _read(root, rel)
    key = "depends" if required else "recommends"
    section = dict(data.get(key) or {})
    if section.get(mod_id) == versions:
        raise ApplyError(f"{mod_id} is already declared in {key} with {versions!r}")
    section[mod_id] = versions
    data[key] = section
    return ChangeSet(
        generator="fabric.add_dependency",
        description=f"declare {'required' if required else 'optional'} dependency "
                    f"{mod_id} {versions}",
        changes=[FileChange(rel, before, _dump_manifest(before, data))],
        notes=["Declaring a dependency does not add it to the build classpath.",
               "Fabric enforces `depends` at load time; `recommends` is advisory only."],
        follow_up=[f"add the matching build dependency for {mod_id} in build.gradle if "
                   "you compile against its API",
                   "run `modcheck build <project> --allow-execute` to confirm it still builds"])


def add_mixin(root: Path, *, target_class: str, mixin_name: str | None = None,
              side: str = "main") -> ChangeSet:
    """Add a mixin class and register it in the mixin config.

    Mixins are where Minecraft mods actually collide, so a mod that adds one is
    adding a precise, declarable interaction surface: the registered class list
    is what ModCheck later extracts from the built jar.
    """
    if side not in ("main", "client", "server"):
        raise ApplyError(f"side must be main, client or server, not {side!r}")
    rel_manifest, manifest = _load_manifest(root)
    mod_id = manifest.get("id")
    if not mod_id:
        raise ApplyError("fabric.mod.json declares no mod id")

    configs = manifest.get("mixins") or []
    config_names = [c["config"] if isinstance(c, dict) else c for c in configs]
    if not config_names:
        raise ApplyError("this project declares no mixin config to register into")
    config_rel = next((str(p.relative_to(root)) for name in config_names
                       for p in root.glob(f"src/*/resources/{name}")), None)
    if config_rel is None:
        raise ApplyError(f"mixin config {config_names[0]!r} declared but not found on disk")

    config_before = _read(root, config_rel)
    try:
        config = json.loads(config_before)
    except json.JSONDecodeError as exc:
        raise ApplyError(f"{config_rel} is not valid JSON: {exc}") from exc

    simple = target_class.rsplit(".", 1)[-1]
    mixin_name = mixin_name or f"{simple}Mixin"
    if not re.match(r"^[A-Z][A-Za-z0-9_]*$", mixin_name):
        raise ApplyError(f"mixin class name {mixin_name!r} is not a valid Java class name")

    key = {"main": "mixins", "client": "client", "server": "server"}[side]
    existing = list(config.get(key) or [])
    if mixin_name in existing:
        raise ApplyError(f"{mixin_name} is already registered in {config_rel}")
    config[key] = existing + [mixin_name]

    package = config.get("package")
    if not package:
        raise ApplyError(f"{config_rel} declares no mixin package")
    source_root = Path(config_rel).parts[1]  # src/<root>/resources/...
    java_rel = f"src/{source_root}/java/{package.replace('.', '/')}/{mixin_name}.java"

    body = f"""\
package {package};

import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;

import {target_class};

@Mixin({simple}.class)
public class {mixin_name} {{
    // Injecting at HEAD without cancelling is additive: it does not stop other
    // mods' injections into the same method from also running.
    @Inject(method = "<REPLACE: method name>", at = @At("HEAD"))
    private void {mod_id}$onEvent(CallbackInfo ci) {{
        // your behaviour here
    }}
}}
"""
    return ChangeSet(
        generator="fabric.add_mixin",
        description=f"add mixin {mixin_name} targeting {target_class} ({side})",
        changes=[
            FileChange(java_rel, None, body),
            FileChange(config_rel, config_before,
                       json.dumps(config, indent=4) + "\n"),
        ],
        notes=[
            "The method name in @Inject is a placeholder: it must name a real method on "
            f"{target_class} for the mixin to apply, and the build will fail until it does.",
            "An @Inject at HEAD is additive. @Overwrite or a cancelling injection replaces "
            "behaviour and is what actually conflicts with other mods on the same method.",
        ],
        follow_up=[
            "replace <REPLACE: method name> with the target method",
            "run `modcheck build <project> --allow-execute` to confirm the mixin applies",
            "the registered class list is what `modcheck inspect` reads back out of the jar",
        ])


def add_entrypoint(root: Path, *, class_name: str, kind: str = "client") -> ChangeSet:
    """Add an entrypoint class and register it in fabric.mod.json."""
    rel, manifest = _load_manifest(root)
    before = _read(root, rel)
    mod_id = manifest.get("id")
    if not mod_id:
        raise ApplyError("fabric.mod.json declares no mod id")
    main_entrypoints = (manifest.get("entrypoints") or {}).get("main") or []
    if not main_entrypoints:
        raise ApplyError("cannot infer the mod's package: no main entrypoint declared")
    package = str(main_entrypoints[0]).rsplit(".", 1)[0]
    if kind != "main":
        package = f"{package}.{kind}"
    fqcn = f"{package}.{class_name}"

    entrypoints = {k: list(v) for k, v in (manifest.get("entrypoints") or {}).items()}
    if fqcn in entrypoints.get(kind, []):
        raise ApplyError(f"{fqcn} is already registered as a {kind} entrypoint")
    entrypoints.setdefault(kind, []).append(fqcn)
    manifest["entrypoints"] = entrypoints

    interface = {"main": "ModInitializer", "client": "ClientModInitializer",
                 "server": "DedicatedServerModInitializer"}.get(kind)
    if interface is None:
        raise ApplyError(f"unsupported entrypoint kind {kind!r}")
    method = {"main": "onInitialize", "client": "onInitializeClient",
              "server": "onInitializeServer"}[kind]
    import_pkg = "net.fabricmc.api"

    java_rel = f"src/main/java/{package.replace('.', '/')}/{class_name}.java"
    body = f"""\
package {package};

import {import_pkg}.{interface};

public class {class_name} implements {interface} {{
    @Override
    public void {method}() {{
        // runs during {kind} initialisation
    }}
}}
"""
    return ChangeSet(
        generator="fabric.add_entrypoint",
        description=f"add {kind} entrypoint {fqcn}",
        changes=[FileChange(java_rel, None, body),
                 FileChange(rel, before, _dump_manifest(before, manifest))],
        notes=[f"A {kind} entrypoint runs only in that environment."],
        follow_up=["run `modcheck build <project> --allow-execute` to confirm it compiles"])


GENERATORS: dict[str, Callable[..., ChangeSet]] = {
    "fabric.add_dependency": add_dependency,
    "fabric.add_mixin": add_mixin,
    "fabric.add_entrypoint": add_entrypoint,
}


def apply(generator: str, root: Path, **kwargs: Any) -> ChangeSet:
    if generator not in GENERATORS:
        raise ApplyError(f"unknown generator {generator!r}. Available: "
                         + ", ".join(sorted(GENERATORS)))
    root = Path(root)
    if not root.is_dir():
        raise ApplyError(f"no such project directory: {root}")
    return GENERATORS[generator](root, **kwargs)
