"""Dependency and version analysis over a real configuration.

Works from what the artifacts actually declare plus what was extracted from
them, and resolves against what is actually installed -- including modules
shipped inside other mods (jar-in-jar), which naive resolution reports as
missing.

Where a version or a constraint cannot be decided, the result is `unresolved`.
Reporting a dependency as satisfied because we could not prove otherwise, or as
missing because we could not see it, are both failures.
"""
from __future__ import annotations

from typing import Any

from .config import Installation, InstalledArtifact
from .findings import Finding
from .versions import satisfies

# Dependency ids that refer to the platform rather than to another mod.
PLATFORM_IDS = {
    "minecraft": "game_version",
    "java": "java_version",
    "fabricloader": "loader:fabric",
    "fabric-loader": "loader:fabric",
    "quilt_loader": "loader:quilt",
    "forge": "loader:forge",
    "neoforge": "loader:neoforge",
}


class Provision:
    """What an installation provides, and by what."""

    def __init__(self, installation: Installation) -> None:
        self.by_id: dict[str, list[tuple[InstalledArtifact, str | None, str]]] = {}
        for artifact in installation.artifacts:
            ins = artifact.inspection
            mod_id = (artifact.mod_id or artifact.name).lower()
            self._add(mod_id, artifact, artifact.version, "itself")
            if ins is None:
                continue
            for provided in ins.fact("provides") or []:
                self._add(str(provided).lower(), artifact, artifact.version, "provides")
            for nested in ins.fact("nested_mods") or []:
                if nested.get("mod_id"):
                    self._add(nested["mod_id"].lower(), artifact, nested.get("version"),
                              f"nested in {artifact.name}")
                for provided in nested.get("provides") or []:
                    self._add(str(provided).lower(), artifact, nested.get("version"),
                              f"nested in {artifact.name}")

    def _add(self, key: str, artifact: InstalledArtifact, version: str | None,
             how: str) -> None:
        self.by_id.setdefault(key, []).append((artifact, version, how))

    def get(self, dep_id: str):
        return self.by_id.get(dep_id.lower(), [])


def _platform_version(installation: Installation, dep_id: str) -> str | None:
    slot = PLATFORM_IDS.get(dep_id.lower())
    if slot is None:
        return None
    if slot == "game_version":
        return installation.game_version
    if slot == "java_version":
        return installation.loader_versions.get("java")
    if slot.startswith("loader:"):
        return installation.loader_versions.get(slot.split(":", 1)[1])
    return None


def analyze(installation: Installation, *, loader: str | None = None) -> list[Finding]:
    findings: list[Finding] = []
    provision = Provision(installation)

    # Two artifacts claiming the same mod id is a genuine collision, not an overlap.
    for mod_id, entries in provision.by_id.items():
        owners = {a.name for a, _, how in entries if how == "itself"}
        if len(owners) > 1:
            findings.append(Finding(
                code="dependency.duplicate_mod_id",
                severity="error",
                subject=mod_id,
                summary=f"{len(owners)} installed artifacts declare the mod id {mod_id!r}",
                detail="Loaders identify mods by id; two artifacts with the same id cannot "
                       f"both load. Artifacts: {', '.join(sorted(owners))}.",
                evidence_class="extracted",
                targets=[{"kind": "registry_key", "id": mod_id}],
                not_established="which of them the loader would pick",
            ))

    fallback_loader = loader or installation.infer_loader()
    for artifact in installation.artifacts:
        art_loader = loader
        if art_loader is None and artifact.inspection is not None:
            art_loader = artifact.inspection.loader
        if art_loader is None:
            art_loader = fallback_loader
        for dep in artifact.declared_dependencies or []:
            findings += _resolve_one(artifact, dep, installation, provision, art_loader)
        findings += _breaks(artifact, installation, provision)
    return findings


def _resolve_one(artifact: InstalledArtifact, dep: dict[str, Any],
                 installation: Installation, provision: Provision,
                 loader: str | None) -> list[Finding]:
    dep_id = str(dep.get("id") or "").strip()
    if not dep_id:
        return []
    constraint = dep.get("versions") or "*"
    required = dep.get("required", True)
    subject = artifact.name

    platform_version = _platform_version(installation, dep_id)
    if dep_id.lower() in PLATFORM_IDS:
        if platform_version is None:
            return [Finding(
                code="dependency.platform_unknown",
                severity="unresolved",
                subject=subject,
                summary=f"{subject} requires {dep_id} {constraint}; the installed "
                        f"{dep_id} version was not supplied",
                evidence_class="unresolved",
                targets=[{"kind": "registry_key", "id": dep_id}],
                not_established=f"the installed {dep_id} version",
            )]
        return _version_finding(subject, dep_id, platform_version, constraint, loader,
                                required, how="the platform")

    candidates = provision.get(dep_id)
    if not candidates:
        if not required:
            return []
        return [Finding(
            code="dependency.missing",
            severity="error",
            subject=subject,
            summary=f"{subject} requires {dep_id} {constraint}, which is not installed",
            evidence_class="derived",
            targets=[{"kind": "registry_key", "id": dep_id}],
            not_established="whether the dependency is provided under a different id",
        )]

    findings: list[Finding] = []
    for provider, version, how in candidates:
        if version is None:
            findings.append(Finding(
                code="dependency.version_unknown",
                severity="unresolved",
                subject=subject,
                summary=f"{subject} requires {dep_id} {constraint}; {dep_id} is installed "
                        "but its version could not be read",
                evidence_class="unresolved",
                targets=[{"kind": "registry_key", "id": dep_id}],
                not_established="the installed version, so the constraint is undecided",
            ))
            continue
        result = _version_finding(subject, dep_id, version, constraint, loader, required,
                                  how=how if how != "itself" else provider.name)
        findings += result
    return findings


def _version_finding(subject: str, dep_id: str, version: str, constraint,
                     loader: str | None, required: bool, how: str) -> list[Finding]:
    verdict = satisfies(loader or "", version, constraint)
    if verdict is True:
        if how.startswith("nested in"):
            return [Finding(
                code="dependency.satisfied_by_nested_module",
                severity="note",
                subject=subject,
                summary=f"{dep_id} {version} satisfies {constraint} and is provided {how}",
                detail="Resolution that ignores jar-in-jar modules would report this as "
                       "missing.",
                evidence_class="extracted",
                not_established="whether the nested module is actually loaded at runtime",
            )]
        return []
    if verdict is False:
        return [Finding(
            code="dependency.version_mismatch",
            severity="error" if required else "warning",
            subject=subject,
            summary=f"{subject} requires {dep_id} {constraint} but {version} is installed "
                    f"({how})",
            evidence_class="derived",
            targets=[{"kind": "registry_key", "id": dep_id}],
            not_established="whether the mod actually fails with this version, or only "
                            "declares a narrower range than it needs",
        )]
    return [Finding(
        code="dependency.constraint_undecided",
        severity="unresolved",
        subject=subject,
        summary=f"cannot decide whether {dep_id} {version} satisfies {constraint}",
        detail=f"Loader {loader or 'unknown'}: the version or the constraint is not in a "
               "form this resolver can compare.",
        evidence_class="unresolved",
        targets=[{"kind": "registry_key", "id": dep_id}],
        not_established="whether the constraint is satisfied",
    )]


def _breaks(artifact: InstalledArtifact, installation: Installation,
            provision: Provision) -> list[Finding]:
    """Fabric `breaks` / `conflicts` declarations, resolved against what is present."""
    ins = artifact.inspection
    if ins is None:
        return []
    out: list[Finding] = []
    for key, severity in (("breaks", "error"), ("conflicts", "warning")):
        for entry in ins.fact(key) or []:
            dep_id = str(entry.get("id") or "")
            candidates = provision.get(dep_id)
            if not candidates:
                continue
            constraint = entry.get("versions") or "*"
            for provider, version, how in candidates:
                verdict = satisfies(ins.loader or "", version or "", constraint) \
                    if version else None
                if verdict is False:
                    continue
                out.append(Finding(
                    code=f"dependency.{key}",
                    severity=severity if verdict is True else "unresolved",
                    subject=artifact.name,
                    summary=(f"{artifact.name} declares it {key} {dep_id} {constraint}; "
                             f"{dep_id} {version or 'of unknown version'} is installed"),
                    evidence_class="declared" if verdict is True else "unresolved",
                    targets=[{"kind": "registry_key", "id": dep_id}],
                    not_established="what actually goes wrong when both are loaded",
                ))
    return out


def coverage() -> tuple[list[str], list[str]]:
    return (
        ["declared dependencies of every inspected artifact, including jar-in-jar modules",
         "version constraints, using each loader's own constraint syntax",
         "duplicate mod ids across artifacts",
         "declared breaks/conflicts against what is installed"],
        ["whether a mod's code actually needs what its manifest declares",
         "dependencies that are not declared at all",
         "load order and initialisation order",
         "anything about behaviour when both mods are loaded"],
    )
