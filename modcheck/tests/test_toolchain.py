"""A build must use the toolchain the project declares, or say it could not.

This is a verification concern, not a convenience one. A JDK or Gradle that is
present but too old fails in ways that read as project defects:

* Gradle 8 against Fabric Loom reports a plugin *variant* mismatch, which looks
  like a broken build script.
* Gradle 9 with JDK 21 against a Java 25 project reports "release version 25 not
  supported", which looks like a bad `options.release`.

Neither is a defect in the project, and neither says "install a newer
toolchain". So the resolver reads what the project declares, reports the
mismatch as a toolchain problem, and does not attempt the build.
"""
from __future__ import annotations

from modcheck.creator import toolchain
from modcheck.creator.build import run_build
from modcheck.paths import project_root

DEMO = project_root() / "build_workspaces" / "demo"


def _write_project(root, *, release: int | None, gradle: str | None):
    root.mkdir(parents=True, exist_ok=True)
    if release is not None:
        (root / "build.gradle").write_text(
            "tasks.withType(JavaCompile).configureEach {\n"
            f"    it.options.release = {release}\n}}\n")
    if gradle is not None:
        wrapper = root / "gradle" / "wrapper"
        wrapper.mkdir(parents=True, exist_ok=True)
        (wrapper / "gradle-wrapper.properties").write_text(
            "distributionUrl=https\\://services.gradle.org/distributions/"
            f"gradle-{gradle}-bin.zip\n")
    return root


def test_requirements_are_read_from_the_project_not_guessed(tmp_path):
    root = _write_project(tmp_path / "p", release=25, gradle="9.5.1")
    assert toolchain.required_for(root) == (25, "9.5.1")


def test_a_project_declaring_nothing_requires_nothing(tmp_path):
    root = _write_project(tmp_path / "p", release=None, gradle=None)
    assert toolchain.required_for(root) == (None, None)


def test_the_real_demo_project_declares_java_25_and_gradle_9(tmp_path):
    assert DEMO.is_dir(), "the demo workspace is part of the repository"
    assert toolchain.required_for(DEMO) == (25, "9.5.1")


def test_a_version_mismatch_is_reported_as_a_toolchain_problem():
    resolution = toolchain.Resolution(
        java=toolchain.Tool("java", None, None, "missing"),
        gradle=toolchain.Tool("gradle", None, None, "missing"),
        required_java=25, required_gradle="9.5.1")
    assert not resolution.satisfied
    assert "no JDK found" in resolution.problems()


def test_the_wrong_java_names_the_misleading_message():
    """The message Gradle prints is the thing a creator would misread."""
    resolution = toolchain.Resolution(
        java=toolchain.Tool("java", __import__("pathlib").Path("/x"), "21.0.10", "path"),
        gradle=toolchain.Tool("gradle", __import__("pathlib").Path("/y"), "9.5.1", "path"),
        required_java=25, required_gradle="9.5.1")
    assert not resolution.satisfied
    problem = "; ".join(resolution.problems())
    assert "release version 25 not supported" in problem
    assert "is not one" in problem


def test_the_wrong_gradle_names_its_misleading_message():
    from pathlib import Path
    resolution = toolchain.Resolution(
        java=toolchain.Tool("java", Path("/x"), "25.0.4.1", "provisioned"),
        gradle=toolchain.Tool("gradle", Path("/y"), "8.14.3", "path"),
        required_java=25, required_gradle="9.5.1")
    assert not resolution.satisfied
    assert "plugin variant mismatch" in "; ".join(resolution.problems())


def test_a_satisfied_resolution_puts_the_toolchain_on_the_build_path():
    from pathlib import Path
    resolution = toolchain.Resolution(
        java=toolchain.Tool("java", Path("/jdk"), "25.0.4.1", "provisioned"),
        gradle=toolchain.Tool("gradle", Path("/gr"), "9.5.1", "provisioned"),
        required_java=25, required_gradle="9.5.1")
    assert resolution.satisfied
    env = resolution.env()
    assert env["JAVA_HOME"] == "/jdk"
    assert env["PATH"].startswith("/jdk/bin:/gr/bin:")


def test_a_build_refused_for_toolchain_reasons_establishes_nothing(tmp_path, monkeypatch):
    """A build that never ran must not read as a build that failed."""
    root = _write_project(tmp_path / "p", release=99, gradle="0.1")
    result = run_build(root, ["gradle build"], allow_execute=True, timeout=30)
    assert result.ok is False
    assert result.runs[0].command == "(toolchain resolution)"
    assert "nothing: the build was not attempted" in result.establishes
    assert any("which is a different failure" in s for s in result.does_not_establish)
    assert result.toolchain_resolution["satisfied"] is False
    # And it really did not run: no artifacts, no gradle invocation.
    assert result.artifacts == []


def test_the_attestation_records_which_toolchain_was_resolved():
    from pathlib import Path
    resolution = toolchain.resolve(DEMO)
    payload = resolution.as_dict()
    assert set(payload) >= {"java", "gradle", "required_java", "required_gradle",
                            "satisfied", "problems"}
    assert payload["required_java"] == 25
    if payload["satisfied"]:
        assert payload["java"]["version"].startswith("25.")
        assert payload["gradle"]["version"] == "9.5.1"
        assert Path(payload["java"]["path"]).exists()
