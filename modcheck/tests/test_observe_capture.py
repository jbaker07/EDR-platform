"""An observation is bound to one real run, or it is not issued.

`runtime verify` re-hashes files in our own cache. That establishes the
prediction still describes the artifacts it was written for -- and nothing
about which files the game loaded, what versions ran, or whether the transcript
in hand came from that run. These tests pin the binding that closes the gap,
and the refusal that follows when it does not hold.

A refused binding is deliberately a third outcome. It is not a contradiction
(the prediction may well be right) and not a pass. It means we cannot say what
this transcript is evidence of.
"""
from __future__ import annotations

import hashlib
import json

import pytest

from modcheck.cli import build_parser
from modcheck.observe import load_predictions
from modcheck.observe.capture import (Capture, CapturedFile, load_capture,
                                      validate)
from modcheck.paths import project_root

PREDICTIONS = project_root() / "evaluation" / "runtime" / "predictions"

SUMMARY_TEXT = """=====================
== Content patches ==
=====================
The following patches were loaded. For each patch:

(Filtered to asset name: Animals/horse.)

Bear Mounts:
------------

   Patches:
      loaded  | conditions | applied | priority  | name + details
      ------- | ---------- | ------- | --------- | --------------
      [X]     | [X]        | [ ]     | Exclusive | entry #1 > The Bear (Load Animals/horse)

   No current changes.
"""

DUMP_TEXT = """Here are the active patches grouped by their current target value.

Animals/horse
-------------
   [ ] Load Bear Mounts > entry #1 > The Bear
   [ ] Load ModCheck Conflict Probe > entry #1 > Probe Horse
"""


def _prediction(case_id: str):
    return next(p for p in load_predictions(PREDICTIONS) if p.id == case_id)


def _write(path, text: str) -> str:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text)
    return hashlib.sha256(path.read_bytes()).hexdigest()


@pytest.fixture
def staged(tmp_path):
    """A capture that binds correctly, for the tests to then break one way."""
    prediction = _prediction("sv_deliberate_conflict")
    summary_hash = _write(tmp_path / "t" / "summary.txt", SUMMARY_TEXT)
    dump_hash = _write(tmp_path / "t" / "dump.txt", DUMP_TEXT)
    capture = Capture(
        prediction=prediction.id, run_id="run-1", captured_at="2026-09-21T14:03:00Z",
        versions={"stardew_valley": "1.6.15", "smapi": "4.1.10",
                  "content_patcher": "2.8.0"},
        # Installed paths are the game's, not ours. The binding matches on the
        # digest for exactly that reason.
        installed=[CapturedFile(path=f"Mods/[CP] Probe/{item.path.rsplit('/', 1)[-1]}",
                                sha256=item.sha256)
                   for item in prediction.inputs],
        transcripts=[
            CapturedFile(path="console", sha256=summary_hash, role="transcript",
                         command='patch summary asset "Animals/horse" full',
                         local_path="t/summary.txt"),
            CapturedFile(path="console", sha256=dump_hash, role="transcript",
                         command="patch dump applied", local_path="t/dump.txt"),
        ])
    return prediction, capture, tmp_path


def test_a_correct_binding_is_accepted(staged):
    prediction, capture, root = staged
    binding = validate(capture, prediction, root)
    assert binding.ok, binding.problems
    assert binding.missing_commands == []


def test_a_capture_for_another_prediction_is_refused(staged):
    prediction, capture, root = staged
    capture.prediction = "sv_baseline_bear_mounts"
    binding = validate(capture, prediction, root)
    assert not binding.ok
    assert any("names prediction" in p for p in binding.problems)


def test_a_wrong_artifact_hash_is_refused(staged):
    """The probe's Exclusive and High variants produce opposite outcomes."""
    prediction, capture, root = staged
    capture.installed[0].sha256 = "0" * 64
    binding = validate(capture, prediction, root)
    assert not binding.ok
    assert any("loaded different bytes" in p for p in binding.problems)


def test_missing_versions_are_refused(staged):
    prediction, capture, root = staged
    capture.versions.pop("content_patcher")
    binding = validate(capture, prediction, root)
    assert not binding.ok
    assert any("content_patcher version" in p for p in binding.problems)


def test_a_transcript_edited_after_capture_is_refused(staged):
    prediction, capture, root = staged
    (root / "t" / "summary.txt").write_text(SUMMARY_TEXT + "\n# tidied up\n")
    binding = validate(capture, prediction, root)
    assert not binding.ok
    assert any("changed after capture" in p for p in binding.problems)


def test_a_missing_transcript_file_is_refused(staged):
    prediction, capture, root = staged
    (root / "t" / "dump.txt").unlink()
    binding = validate(capture, prediction, root)
    assert not binding.ok
    assert any("is missing" in p for p in binding.problems)


def test_commands_the_capture_lacks_are_reported_not_ignored(staged):
    prediction, capture, root = staged
    capture.transcripts = [capture.transcripts[0]]      # drop `patch dump applied`
    binding = validate(capture, prediction, root)
    assert binding.ok, binding.problems
    assert "patch dump applied" in binding.missing_commands


# -- the CLI must not print claim verdicts on a refused binding -------------

def _capture_file(root, capture) -> str:
    path = root / "capture.yaml"
    payload = {
        "prediction": capture.prediction, "run_id": capture.run_id,
        "captured_at": capture.captured_at, "versions": capture.versions,
        "installed": [{"path": f.path, "sha256": f.sha256} for f in capture.installed],
        "transcripts": [{"path": f.path, "sha256": f.sha256, "command": f.command,
                         "local_path": f.local_path} for f in capture.transcripts],
    }
    from modcheck import yamlio
    path.write_text(yamlio.dump(payload))
    return str(path)


def test_cli_observe_refuses_and_prints_no_verdict(staged, capsys, monkeypatch):
    prediction, capture, root = staged
    capture.installed[0].sha256 = "0" * 64
    path = _capture_file(root, capture)
    monkeypatch.setattr("modcheck.cli.project_root", lambda: root)
    args = build_parser().parse_args(
        ["runtime", "observe", "--id", prediction.id, "--capture", path,
         "--predictions", str(PREDICTIONS)])
    assert args.func(args) == 1
    out = capsys.readouterr().out
    assert "binding: REFUSED" in out
    assert "No result is issued" in out
    for word in ("MATCH", "WRONG"):
        assert word not in out, "a refused binding must not print claim verdicts"


def test_cli_observe_runs_on_a_good_binding(staged, capsys, monkeypatch):
    prediction, capture, root = staged
    path = _capture_file(root, capture)
    monkeypatch.setattr("modcheck.cli.project_root", lambda: root)
    args = build_parser().parse_args(
        ["runtime", "observe", "--id", prediction.id, "--capture", path,
         "--predictions", str(PREDICTIONS), "--json"])
    code = args.func(args)
    payload = json.loads(capsys.readouterr().out)
    assert payload["run_id"] == "run-1"
    assert payload["versions"]["content_patcher"] == "2.8.0"
    assert payload["verdict"] in ("matched", "incomplete", "contradicted")
    assert code in (0, 1)


def test_observe_requires_a_capture():
    with pytest.raises(SystemExit):
        build_parser().parse_args(["runtime", "observe", "--id", "x"])


def test_load_capture_round_trips(staged):
    prediction, capture, root = staged
    path = _capture_file(root, capture)
    loaded = load_capture(path)
    assert loaded.prediction == capture.prediction
    assert loaded.versions == capture.versions
    assert validate(loaded, prediction, root).ok
