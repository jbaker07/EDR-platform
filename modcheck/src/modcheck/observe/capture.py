"""The binding between a prediction and one real game run.

`modcheck runtime verify` re-hashes the files in *our* cache. That establishes
the prediction still describes the artifacts it was written for. It says nothing
about which files the game loaded, which versions ran, or whether the transcript
in front of us came from that run at all -- and those are exactly the things a
confirmed result depends on.

So an observation is bound to a capture manifest: one small file, written at
capture time, naming the run. `validate` refuses the pairing unless every part
of the binding holds:

* the manifest names the prediction being judged;
* every artifact the prediction pinned is present in the game's mod directory
  under the same sha256 (matched by digest, not by path -- the installed path is
  not our cache path);
* the game, SMAPI and Content Patcher versions are recorded, because a
  prediction confirmed on one Content Patcher version is not confirmed on all;
* every transcript file still hashes to what the manifest recorded, so a file
  edited after capture cannot be judged as if it were the capture.

A failed binding is not a contradiction and not a pass. It is `unbound`: we
cannot say what this transcript is evidence of.
"""
from __future__ import annotations

import dataclasses
import hashlib
from pathlib import Path
from typing import Any

from .. import yamlio
from .prediction import Prediction

REQUIRED_VERSIONS = ("stardew_valley", "smapi", "content_patcher")


@dataclasses.dataclass
class CapturedFile:
    """A file as it existed in the game's own directories at capture time."""
    path: str          # path inside the game installation, for a human to find again
    sha256: str
    role: str = "mod"  # "mod" | "config" | "transcript"
    command: str = ""  # for transcripts: the command whose output this is
    local_path: str = ""  # where the captured copy lives in this repository


@dataclasses.dataclass
class Capture:
    prediction: str
    run_id: str
    captured_at: str
    versions: dict[str, str] = dataclasses.field(default_factory=dict)
    installed: list[CapturedFile] = dataclasses.field(default_factory=list)
    transcripts: list[CapturedFile] = dataclasses.field(default_factory=list)
    notes: str = ""
    path: Path | None = None

    @classmethod
    def from_dict(cls, data: dict[str, Any], path: Path | None = None) -> "Capture":
        def files(key: str, role: str) -> list[CapturedFile]:
            return [CapturedFile(role=role, **entry) for entry in (data.get(key) or [])]

        return cls(
            prediction=data["prediction"], run_id=str(data["run_id"]),
            captured_at=str(data.get("captured_at", "")),
            versions={k: str(v) for k, v in (data.get("versions") or {}).items()},
            installed=files("installed", "mod"),
            transcripts=files("transcripts", "transcript"),
            notes=data.get("notes", ""), path=path)

    def transcript_for(self, command: str) -> CapturedFile | None:
        for entry in self.transcripts:
            if entry.command.strip() == command.strip():
                return entry
        return None


def load_capture(path: Path) -> Capture:
    return Capture.from_dict(yamlio.load_path(Path(path)), Path(path))


@dataclasses.dataclass
class Binding:
    """Whether this capture may be used to judge this prediction."""
    ok: bool
    problems: list[str] = dataclasses.field(default_factory=list)
    confirmed: list[str] = dataclasses.field(default_factory=list)
    missing_commands: list[str] = dataclasses.field(default_factory=list)


def validate(capture: Capture, prediction: Prediction, root: Path) -> Binding:
    """Check every part of the binding. Nothing here is advisory."""
    problems: list[str] = []
    confirmed: list[str] = []

    if capture.prediction != prediction.id:
        problems.append(
            f"the capture names prediction {capture.prediction!r} but is being used to "
            f"judge {prediction.id!r}")
    if not capture.run_id:
        problems.append("the capture has no run_id, so two runs cannot be told apart")

    for key in REQUIRED_VERSIONS:
        value = capture.versions.get(key)
        if not value:
            problems.append(
                f"no {key} version recorded. A prediction confirmed on one version is "
                "not confirmed on all, so an unversioned capture cannot confirm one")
        else:
            confirmed.append(f"{key} {value}")

    # The artifacts the prediction is about must be the artifacts the game
    # loaded. Matched by digest: the installed path is not our cache path, and
    # requiring the paths to agree would fail for the right reason in the wrong
    # place.
    installed = {entry.sha256.lower(): entry for entry in capture.installed}
    for item in prediction.inputs:
        entry = installed.get(item.sha256.lower())
        if entry is None:
            problems.append(
                f"the prediction is pinned to {item.path} (sha256 {item.sha256[:16]}...) "
                f"but the capture lists no installed file with that hash. Either the "
                f"game loaded different bytes than the prediction is about, or the "
                f"capture did not record them")
        else:
            confirmed.append(f"{entry.path} == {item.path} (sha256 {item.sha256[:16]}...)")

    # A transcript edited after capture is not the capture.
    for entry in capture.transcripts:
        if not entry.local_path:
            problems.append(f"transcript for {entry.command!r} has no local_path")
            continue
        actual = root / entry.local_path
        if not actual.exists():
            problems.append(f"transcript {entry.local_path} is missing")
            continue
        digest = hashlib.sha256(actual.read_bytes()).hexdigest()
        if digest != entry.sha256:
            problems.append(
                f"transcript {entry.local_path} hashes to {digest[:16]}..., but the "
                f"capture recorded {entry.sha256[:16]}...: the file changed after "
                "capture, so it is no longer the captured output")
        else:
            confirmed.append(f"transcript {entry.local_path} ({entry.command})")

    # Commands the prediction names but the capture does not carry are not an
    # error -- the claims that need them simply stay unobserved. Reporting them
    # keeps that visible instead of silent.
    missing = [command for command in prediction.commands
               if capture.transcript_for(command) is None]

    return Binding(ok=not problems, problems=problems, confirmed=confirmed,
                   missing_commands=missing)


def render_binding(binding: Binding) -> str:
    lines = ["binding: " + ("OK" if binding.ok else "REFUSED"), ""]
    for item in binding.confirmed:
        lines.append(f"  confirmed  {item}")
    for problem in binding.problems:
        lines.append(f"  PROBLEM    {problem}")
    if binding.missing_commands:
        lines.append("")
        lines.append("  the prediction names commands this capture does not carry;")
        lines.append("  claims needing them stay unobserved:")
        for command in binding.missing_commands:
            lines.append(f"    $ {command}")
    if not binding.ok:
        lines.append("")
        lines.append("  No result is issued. A refused binding is not a contradiction")
        lines.append("  and not a pass: we cannot say what this transcript is evidence")
        lines.append("  of.")
    return "\n".join(lines)
