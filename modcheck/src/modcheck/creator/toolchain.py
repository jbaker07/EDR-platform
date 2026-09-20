"""Resolving the exact JDK and Gradle a build needs.

A Fabric project for Minecraft 26.3 needs Java 25 and Gradle 9.5.1. A container
with Java 21 and Gradle 8.14.3 on PATH does not build it -- and the way it fails
is instructive rather than obvious: Gradle 8 reports a plugin variant mismatch
against Loom, and Gradle 9 with Java 21 reports "release version 25 not
supported". Neither message says "install a newer toolchain", and neither is a
defect in the project.

So the toolchain is resolved rather than assumed, and what was resolved is
recorded in the build attestation. Three sources, in order:

1. ``MODCHECK_JAVA_HOME`` / ``MODCHECK_GRADLE_HOME`` -- an operator's explicit choice.
2. ``modcheck/toolchains/`` -- distributions provisioned by
   ``modcheck toolchain install``, matched by the version a project declares.
3. whatever is on PATH -- which may well be too old, and is reported as such
   rather than silently used.

Nothing here downloads anything by itself. Provisioning is a separate, explicit
command, because fetching and executing a third-party JDK is exactly the kind of
step that should not happen as a side effect of asking a question.
"""
from __future__ import annotations

import dataclasses
import os
import re
import shutil
import subprocess
from pathlib import Path

from ..paths import project_root

_JAVA_VERSION = re.compile(r'version "(?P<version>[0-9][0-9._+]*)"')
_GRADLE_VERSION = re.compile(r"^Gradle (?P<version>[0-9][0-9.]*)", re.M)


@dataclasses.dataclass
class Tool:
    name: str
    path: Path | None
    version: str | None
    source: str          # "env" | "provisioned" | "path" | "missing"

    @property
    def found(self) -> bool:
        return self.path is not None

    def major(self) -> int | None:
        if not self.version:
            return None
        head = self.version.split(".")[0]
        return int(head) if head.isdigit() else None


def toolchains_dir() -> Path:
    return project_root() / "toolchains"


def _java_version(java_home: Path) -> str | None:
    binary = java_home / "bin" / "java"
    if not binary.exists():
        return None
    try:
        proc = subprocess.run([str(binary), "-version"], capture_output=True,
                              text=True, timeout=60)
    except (OSError, subprocess.SubprocessError):
        return None
    match = _JAVA_VERSION.search((proc.stderr or "") + (proc.stdout or ""))
    return match.group("version") if match else None


def _gradle_version(home: Path) -> str | None:
    binary = home / "bin" / "gradle"
    if not binary.exists():
        return None
    try:
        proc = subprocess.run([str(binary), "--version"], capture_output=True,
                              text=True, timeout=120)
    except (OSError, subprocess.SubprocessError):
        return None
    match = _GRADLE_VERSION.search((proc.stdout or "") + (proc.stderr or ""))
    return match.group("version") if match else None


def find_java(required_major: int | None = None) -> Tool:
    candidates: list[tuple[Path, str]] = []
    env_home = os.environ.get("MODCHECK_JAVA_HOME")
    if env_home:
        candidates.append((Path(env_home), "env"))
    root = toolchains_dir()
    if root.is_dir():
        candidates += [(p, "provisioned") for p in sorted(root.glob("jdk-*"))
                       if (p / "bin" / "java").exists()]
    on_path = shutil.which("java")
    if on_path:
        candidates.append((Path(on_path).resolve().parent.parent, "path"))

    best: Tool | None = None
    for home, source in candidates:
        version = _java_version(home)
        tool = Tool("java", home, version, source)
        if required_major is not None and tool.major() != required_major:
            best = best or Tool("java", home, version, source)
            continue
        return tool
    return best or Tool("java", None, None, "missing")


def find_gradle(required_version: str | None = None) -> Tool:
    candidates: list[tuple[Path, str]] = []
    env_home = os.environ.get("MODCHECK_GRADLE_HOME")
    if env_home:
        candidates.append((Path(env_home), "env"))
    root = toolchains_dir()
    if root.is_dir():
        candidates += [(p, "provisioned") for p in sorted(root.glob("gradle-*"))
                       if (p / "bin" / "gradle").exists()]
    on_path = shutil.which("gradle")
    if on_path:
        candidates.append((Path(on_path).resolve().parent.parent, "path"))

    best: Tool | None = None
    for home, source in candidates:
        version = _gradle_version(home)
        tool = Tool("gradle", home, version, source)
        if required_version and version != required_version:
            best = best or tool
            continue
        return tool
    return best or Tool("gradle", None, None, "missing")


@dataclasses.dataclass
class Resolution:
    java: Tool
    gradle: Tool
    required_java: int | None = None
    required_gradle: str | None = None

    @property
    def satisfied(self) -> bool:
        if not (self.java.found and self.gradle.found):
            return False
        if self.required_java is not None and self.java.major() != self.required_java:
            return False
        if self.required_gradle and self.gradle.version != self.required_gradle:
            return False
        return True

    def problems(self) -> list[str]:
        out = []
        if not self.java.found:
            out.append("no JDK found")
        elif self.required_java is not None and self.java.major() != self.required_java:
            out.append(
                f"the project targets Java {self.required_java} but the JDK found is "
                f"{self.java.version} (from {self.java.source}). Gradle reports this as "
                f"'release version {self.required_java} not supported', which reads like "
                "a project defect and is not one")
        if not self.gradle.found:
            out.append("no Gradle found")
        elif self.required_gradle and self.gradle.version != self.required_gradle:
            out.append(
                f"the project's wrapper declares Gradle {self.required_gradle} but "
                f"{self.gradle.version} was found (from {self.gradle.source}). An older "
                "Gradle reports a plugin variant mismatch against Loom rather than a "
                "version error")
        return out

    def env(self) -> dict[str, str]:
        """Environment additions that point a build at the resolved toolchain."""
        out: dict[str, str] = {}
        if self.java.path:
            out["JAVA_HOME"] = str(self.java.path)
            out["PATH"] = (f"{self.java.path / 'bin'}:{self.gradle.path / 'bin'}"
                           f":{os.environ.get('PATH', '/usr/bin:/bin')}"
                           if self.gradle.path else
                           f"{self.java.path / 'bin'}:{os.environ.get('PATH', '/usr/bin:/bin')}")
        return out

    def as_dict(self) -> dict[str, object]:
        return {
            "java": {"version": self.java.version, "source": self.java.source,
                     "path": str(self.java.path) if self.java.path else None},
            "gradle": {"version": self.gradle.version, "source": self.gradle.source,
                       "path": str(self.gradle.path) if self.gradle.path else None},
            "required_java": self.required_java,
            "required_gradle": self.required_gradle,
            "satisfied": self.satisfied,
            "problems": self.problems(),
        }


def required_for(project: Path) -> tuple[int | None, str | None]:
    """What a Gradle project declares it needs, read from its own files."""
    project = Path(project)
    java_major = None
    build_file = project / "build.gradle"
    if build_file.exists():
        text = build_file.read_text(encoding="utf-8", errors="replace")
        match = re.search(r"options\.release\s*=\s*(\d+)", text)
        if not match:
            match = re.search(r"VERSION_(\d+)", text)
        if match:
            java_major = int(match.group(1))

    gradle_version = None
    wrapper = project / "gradle" / "wrapper" / "gradle-wrapper.properties"
    if wrapper.exists():
        match = re.search(r"gradle-(?P<v>[0-9][0-9.]*)-(?:bin|all)\.zip",
                          wrapper.read_text(encoding="utf-8", errors="replace"))
        if match:
            gradle_version = match.group("v")
    return java_major, gradle_version


def resolve(project: Path) -> Resolution:
    """Resolve the toolchain a specific project needs."""
    java_major, gradle_version = required_for(project)
    return Resolution(java=find_java(java_major), gradle=find_gradle(gradle_version),
                      required_java=java_major, required_gradle=gradle_version)
