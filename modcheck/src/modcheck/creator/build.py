"""Running builds, and recording what a build actually established.

Building a mod means executing that project's build script. A build script is
untrusted code, so:

* execution is never implicit -- a caller must opt in explicitly;
* the child process gets a scrubbed environment with no credentials in it;
* the build runs in the project directory and nowhere else;
* the result records the exact toolchain and the artifact hashes, so a later
  claim can be tied to the bytes that were actually produced.

A successful build establishes that the project compiles and packages. It does
not establish that the mod works, is compatible, or is safe to release, and the
attestation says so.
"""
from __future__ import annotations

import dataclasses
import datetime as dt
import hashlib
import json
import os
import shutil
import subprocess
from pathlib import Path
from typing import Any, Iterable

from . import toolchain


# Anything matching these is never passed to a build process.
SECRET_PATTERNS = ("TOKEN", "SECRET", "PASSWORD", "PASSWD", "API_KEY", "APIKEY",
                   "CREDENTIAL", "AUTH", "SESSION", "COOKIE", "PRIVATE_KEY")

# Variables a JVM/Gradle build legitimately needs.
ENV_ALLOWLIST = ("PATH", "HOME", "LANG", "LC_ALL", "TMPDIR", "JAVA_HOME",
                 "GRADLE_USER_HOME", "GRADLE_OPTS", "JAVA_TOOL_OPTIONS",
                 "HTTP_PROXY", "HTTPS_PROXY", "NO_PROXY",
                 "http_proxy", "https_proxy", "no_proxy",
                 "SSL_CERT_FILE", "SSL_CERT_DIR", "NIX_SSL_CERT_FILE")


class BuildRefused(RuntimeError):
    """Execution was requested without explicit authorisation."""


def scrubbed_env(extra: dict[str, str] | None = None) -> dict[str, str]:
    """A minimal environment with no credentials in it."""
    env = {k: v for k, v in os.environ.items()
           if k in ENV_ALLOWLIST and not any(p in k.upper() for p in SECRET_PATTERNS)}
    env.setdefault("PATH", "/usr/local/bin:/usr/bin:/bin")
    if extra:
        for key, value in extra.items():
            if any(p in key.upper() for p in SECRET_PATTERNS):
                raise BuildRefused(f"refusing to pass {key!r} into a build environment")
            env[key] = value
    return env


@dataclasses.dataclass
class CommandRun:
    command: str
    returncode: int
    duration_seconds: float
    stdout_tail: str
    stderr_tail: str

    @property
    def ok(self) -> bool:
        return self.returncode == 0


@dataclasses.dataclass
class BuiltArtifact:
    path: str
    sha256: str
    bytes: int


@dataclasses.dataclass
class BuildResult:
    project: str
    ok: bool
    runs: list[CommandRun]
    artifacts: list[BuiltArtifact]
    toolchain: dict[str, str]
    started_at: str
    establishes: list[str]
    does_not_establish: list[str]
    # What the toolchain resolver decided, and why. A build that never ran
    # because the JDK was too old is a different result from a build that ran
    # and failed, and the attestation has to say which.
    toolchain_resolution: dict[str, Any] = dataclasses.field(default_factory=dict)

    def as_dict(self) -> dict[str, Any]:
        return {
            "project": self.project,
            "ok": self.ok,
            "started_at": self.started_at,
            "toolchain": self.toolchain,
            "toolchain_resolution": self.toolchain_resolution,
            "commands": [dataclasses.asdict(r) for r in self.runs],
            "artifacts": [dataclasses.asdict(a) for a in self.artifacts],
            "establishes": self.establishes,
            "does_not_establish": self.does_not_establish,
        }


def _hash(path: Path) -> BuiltArtifact:
    h = hashlib.sha256()
    size = 0
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1 << 20), b""):
            h.update(chunk)
            size += len(chunk)
    return BuiltArtifact(path=str(path), sha256=h.hexdigest(), bytes=size)


def toolchain_report(env: dict[str, str]) -> dict[str, str]:
    """Record the exact tools used, so a build can be reproduced or explained."""
    out: dict[str, str] = {}
    java_home = env.get("JAVA_HOME")
    # Resolve against the BUILD's PATH, not this process's. They differ whenever
    # the toolchain resolver put a provisioned JDK or Gradle in front, and an
    # attestation naming the version we did not build with is worse than none.
    search = env.get("PATH")
    java = (Path(java_home, "bin", "java") if java_home
            else Path(shutil.which("java", path=search) or ""))
    gradle = shutil.which("gradle", path=search) or "gradle"
    for name, argv in (("java", [str(java), "-version"]),
                       ("gradle", [gradle, "--version"])):
        try:
            proc = subprocess.run(argv, capture_output=True, text=True, timeout=90, env=env)
            text = (proc.stdout or "") + (proc.stderr or "")
            lines = [ln.strip() for ln in text.splitlines()
                     if ln.strip() and "JAVA_TOOL_OPTIONS" not in ln
                     and set(ln.strip()) != {"-"}]
            # Gradle prints a banner before its version line; find the real one.
            preferred = next((ln for ln in lines if ln.lower().startswith(name)), None)
            out[name] = preferred or (lines[0] if lines else "unknown")
        except (OSError, subprocess.SubprocessError):
            out[name] = "unavailable"
    return out


def run_build(project: Path, commands: Iterable[str], *, allow_execute: bool = False,
              timeout: int = 1800, env_extra: dict[str, str] | None = None,
              artifact_globs: Iterable[str] = ("build/libs/*.jar",)) -> BuildResult:
    """Run a project's own build commands. Requires explicit authorisation."""
    if not allow_execute:
        raise BuildRefused(
            "running a build executes that project's build script, which is untrusted "
            "code. Pass allow_execute=True (CLI: --allow-execute) to authorise it.")
    project = Path(project).resolve()
    if not project.is_dir():
        raise FileNotFoundError(f"no such project directory: {project}")

    # Resolve the toolchain the project itself declares, rather than trusting
    # PATH. A JDK or Gradle that is merely present but too old fails in ways
    # that read as project defects -- "release version 25 not supported", or a
    # Loom plugin variant mismatch -- so the mismatch is reported as a
    # toolchain problem before the build runs.
    resolution = toolchain.resolve(project)
    env = scrubbed_env({**resolution.env(), **(env_extra or {})})
    if not resolution.satisfied and resolution.problems():
        started = dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()
        return BuildResult(
            project=str(project), ok=False,
            runs=[CommandRun(command="(toolchain resolution)", returncode=1,
                             duration_seconds=0.0, stdout_tail="",
                             stderr_tail="\n".join(resolution.problems()))],
            artifacts=[], toolchain=toolchain_report(env), started_at=started,
            establishes=["nothing: the build was not attempted"],
            does_not_establish=[
                "anything about the project, which was not compiled",
                "that the project is broken -- the toolchain did not match what it "
                "declares, which is a different failure",
            ],
            toolchain_resolution=resolution.as_dict())
    started = dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat()
    runs: list[CommandRun] = []
    ok = True
    for command in commands:
        begin = dt.datetime.now()
        try:
            proc = subprocess.run(command, shell=True, cwd=project, env=env,
                                  capture_output=True, text=True, timeout=timeout)
            rc, out, err = proc.returncode, proc.stdout, proc.stderr
        except subprocess.TimeoutExpired:
            rc, out, err = 124, "", f"timed out after {timeout}s"
        duration = (dt.datetime.now() - begin).total_seconds()
        runs.append(CommandRun(command=command, returncode=rc, duration_seconds=round(duration, 2),
                               stdout_tail="\n".join((out or "").splitlines()[-40:]),
                               stderr_tail="\n".join((err or "").splitlines()[-40:])))
        if rc != 0:
            ok = False
            break

    artifacts: list[BuiltArtifact] = []
    if ok:
        for pattern in artifact_globs:
            for path in sorted(project.glob(pattern)):
                if path.is_file():
                    artifacts.append(_hash(path))

    return BuildResult(
        project=str(project), ok=ok, runs=runs, artifacts=artifacts,
        toolchain=toolchain_report(env), started_at=started,
        toolchain_resolution=resolution.as_dict(),
        establishes=([
            "the project compiles with the declared toolchain",
            "the build produced the listed artifacts, identified by sha256",
        ] if ok else ["the build failed; see the command output"]),
        does_not_establish=[
            "that the mod loads or behaves correctly in game",
            "that it is compatible with any other mod",
            "that its declared versions match what the code requires",
            "anything about assets or data the build merely copies",
        ])


def write_attestation(path: Path, *, recipe_id: str, game: str, result: BuildResult,
                      extra: dict[str, Any] | None = None) -> Path:
    """Record that a recipe's build really ran here, so the claim is checkable."""
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "recipe": recipe_id,
        "game": game,
        "verification_state": "build_tested" if result.ok else "failed",
        "build": result.as_dict(),
    }
    if extra:
        payload.update(extra)
    path.write_text(json.dumps(payload, indent=2, sort_keys=False), encoding="utf-8")
    return path
