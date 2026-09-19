"""Creator path: project setup, build execution safety, and release reports."""
from __future__ import annotations

import json
import os
from pathlib import Path

import pytest

import modcheck.creator.scaffold as scaffold_mod
from modcheck.creator.build import (BuildRefused, ENV_ALLOWLIST, run_build, scrubbed_env,
                                    write_attestation)
from modcheck.creator.scaffold import ScaffoldError, scaffold
from modcheck.creator.versions import ResolvedVersions
from modcheck.inspect import inspect_path

FIXED = ResolvedVersions(
    values={"minecraft_version": "26.3", "loader_version": "0.19.5",
            "loom_version": "1.17-SNAPSHOT", "fabric_api_version": "0.161.0+26.3"},
    sources={"minecraft_version": "test://fixed"})


@pytest.fixture
def offline_upstream(monkeypatch):
    """Pin the upstream-derived facts so scaffolding is testable without network."""
    from modcheck.creator import versions as versions_mod
    monkeypatch.setattr(scaffold_mod, "java_release_for", lambda mc: 25)
    monkeypatch.setattr(scaffold_mod, "fabric_build_style",
                        lambda mc: versions_mod.BuildStyle(
                            needs_mappings=False,
                            mod_dependency_configuration="implementation",
                            source_url="test://fixed"))


# --- project setup -------------------------------------------------------
def test_scaffold_produces_a_complete_fabric_project(tmp_path, offline_upstream):
    result = scaffold("minecraft", "fabric", tmp_path / "p", "mymod", versions=FIXED)
    assert "build.gradle" in result.files
    assert "src/main/resources/fabric.mod.json" in result.files
    assert any(f.endswith("Mymod.java") for f in result.files)
    manifest = (result.root / "src/main/resources/fabric.mod.json").read_text()
    assert '"id": "mymod"' in manifest
    assert '"minecraft": "~26.3"' in manifest


def test_scaffold_tracks_upstream_build_style_instead_of_hardcoding_it(tmp_path,
                                                                      offline_upstream):
    """Minecraft 26.x is not obfuscated, so no mappings and plain `implementation`."""
    result = scaffold("minecraft", "fabric", tmp_path / "p", "mymod", versions=FIXED)
    gradle = (result.root / "build.gradle").read_text()
    assert "officialMojangMappings" not in gradle
    assert "modImplementation" not in gradle
    assert "implementation \"net.fabricmc:fabric-loader" in gradle
    assert any("non-obfuscated" in note for note in result.notes)


def test_scaffold_emits_mappings_when_upstream_still_uses_them(tmp_path, monkeypatch):
    from modcheck.creator import versions as versions_mod
    monkeypatch.setattr(scaffold_mod, "java_release_for", lambda mc: 21)
    monkeypatch.setattr(scaffold_mod, "fabric_build_style",
                        lambda mc: versions_mod.BuildStyle(
                            needs_mappings=True,
                            mod_dependency_configuration="modImplementation",
                            source_url="test://old"))
    old = ResolvedVersions(values={"minecraft_version": "1.21", "loader_version": "0.16.0",
                                   "loom_version": "1.7-SNAPSHOT",
                                   "fabric_api_version": "0.100.0+1.21"}, sources={})
    result = scaffold("minecraft", "fabric", tmp_path / "p", "mymod", versions=old)
    gradle = (result.root / "build.gradle").read_text()
    assert "loom.officialMojangMappings()" in gradle
    assert "modImplementation" in gradle


def test_scaffold_rejects_an_invalid_mod_id(tmp_path, offline_upstream):
    with pytest.raises(ScaffoldError, match="invalid"):
        scaffold("minecraft", "fabric", tmp_path / "p", "Not Valid!", versions=FIXED)


def test_scaffold_refuses_to_overwrite_an_existing_project(tmp_path, offline_upstream):
    root = tmp_path / "p"
    root.mkdir()
    (root / "existing.txt").write_text("keep me")
    with pytest.raises(ScaffoldError, match="not empty"):
        scaffold("minecraft", "fabric", root, "mymod", versions=FIXED)
    assert (root / "existing.txt").read_text() == "keep me"


def test_scaffold_refuses_an_unimplemented_combination(tmp_path):
    with pytest.raises(ScaffoldError, match="no project generator"):
        scaffold("rimworld", "harmony", tmp_path / "p", "mymod")


def test_generated_manifest_is_readable_by_our_own_inspector(tmp_path, offline_upstream):
    """The creator and player paths must agree on what the project declares."""
    import json
    import zipfile

    result = scaffold("minecraft", "fabric", tmp_path / "p", "mymod", versions=FIXED)
    manifest = json.loads((result.root / "src/main/resources/fabric.mod.json").read_text()
                          .replace("${version}", "1.0.0"))
    jar = tmp_path / "mymod.jar"
    with zipfile.ZipFile(jar, "w") as zf:
        zf.writestr("fabric.mod.json", json.dumps(manifest))
        zf.writestr("mymod.mixins.json",
                    (result.root / "src/main/resources/mymod.mixins.json").read_text())
    ins = inspect_path(jar)
    assert ins.fact("mod_id") == "mymod"
    assert {d["id"] for d in ins.fact("dependencies")} == {"fabricloader", "minecraft", "java"}


# --- build execution safety ---------------------------------------------
def test_build_refuses_without_explicit_authorisation(tmp_path):
    with pytest.raises(BuildRefused):
        run_build(tmp_path, ["echo hi"])


def test_build_environment_carries_no_credentials(monkeypatch):
    monkeypatch.setenv("GITHUB_TOKEN", "super-secret")
    monkeypatch.setenv("MY_API_KEY", "super-secret")
    monkeypatch.setenv("AWS_SECRET_ACCESS_KEY", "super-secret")
    env = scrubbed_env()
    assert "super-secret" not in "".join(env.values())
    assert set(env) <= set(ENV_ALLOWLIST)


def test_build_refuses_to_be_handed_a_secret_explicitly():
    with pytest.raises(BuildRefused):
        scrubbed_env({"SOME_TOKEN": "x"})


def test_build_runs_and_records_what_it_establishes(tmp_path):
    (tmp_path / "out").mkdir()
    result = run_build(tmp_path, ["echo building"], allow_execute=True, timeout=60,
                       artifact_globs=())
    assert result.ok
    assert result.runs[0].returncode == 0
    assert "compiles with the declared toolchain" in " ".join(result.establishes)
    assert any("does not" in s or "in game" in s for s in result.does_not_establish)


def test_failed_build_stops_and_reports_the_failure(tmp_path):
    result = run_build(tmp_path, ["exit 3", "echo never"], allow_execute=True, timeout=60,
                       artifact_globs=())
    assert not result.ok
    assert len(result.runs) == 1  # stopped at the failure
    assert result.artifacts == []


def test_attestation_records_a_real_run(tmp_path):
    result = run_build(tmp_path, ["echo ok"], allow_execute=True, timeout=60,
                       artifact_globs=())
    path = write_attestation(tmp_path / "att" / "r.json", recipe_id="r", game="minecraft",
                             result=result)
    import json
    data = json.loads(path.read_text())
    assert data["recipe"] == "r"
    assert data["verification_state"] == "build_tested"
    assert data["build"]["toolchain"]


# --- the real thing ------------------------------------------------------
# Opt-in: this scaffolds against live upstream metadata and runs a real Gradle
# build, which needs the network, a matching JDK and several minutes on a cold
# cache. Run with MODCHECK_BUILD_TESTS=1. It is the test that actually backs
# the `build_execution` capability claim for Minecraft/Fabric.
build_tests = pytest.mark.skipif(
    os.environ.get("MODCHECK_BUILD_TESTS") != "1",
    reason="set MODCHECK_BUILD_TESTS=1 to run real builds")


@build_tests
def test_scaffolded_fabric_project_really_builds(tmp_path):
    result = scaffold("minecraft", "fabric", tmp_path / "proj", "buildcheck")
    build = run_build(result.root, ["gradle build --no-daemon --console=plain"],
                      allow_execute=True, timeout=1800)
    assert build.ok, build.runs[-1].stderr_tail or build.runs[-1].stdout_tail
    jars = [a for a in build.artifacts if a.path.endswith(".jar")
            and "sources" not in a.path]
    assert jars, "build produced no mod jar"

    # The jar we built must be readable by our own inspector, with the identity
    # the project declared: this is the creator-to-player join.
    ins = inspect_path(jars[0].path)
    assert ins.fact("mod_id") == "buildcheck"
    assert ins.sha256 == jars[0].sha256

    from modcheck.report import release_report
    report = release_report(artifact_path=jars[0].path, game="minecraft",
                            build=build.as_dict())
    assert report.subject["artifact"]["sha256"] == jars[0].sha256
    assert report.subject["build"]["ok"] is True


# --- reviewable changes --------------------------------------------------
from modcheck.creator.apply import ApplyError, apply  # noqa: E402


@pytest.fixture
def project(tmp_path, offline_upstream):
    return scaffold("minecraft", "fabric", tmp_path / "proj", "demo", versions=FIXED).root


def test_apply_produces_a_diff_and_writes_nothing_by_default(project):
    changeset = apply("fabric.add_dependency", project, mod_id="fabric-api", versions=">=0.100.0")
    assert "+" in changeset.diff() and "fabric-api" in changeset.diff()
    manifest = (project / "src/main/resources/fabric.mod.json").read_text()
    assert "fabric-api" not in manifest, "apply must not write until asked"


def test_apply_writes_only_when_asked(project):
    changeset = apply("fabric.add_dependency", project, mod_id="fabric-api", versions="*")
    written = changeset.write(project)
    assert written == ["src/main/resources/fabric.mod.json"]
    manifest = (project / "src/main/resources/fabric.mod.json").read_text()
    assert '"fabric-api": "*"' in manifest
    assert "${version}" in manifest, "the build's version template must survive the edit"


def test_apply_refuses_a_duplicate_change(project):
    apply("fabric.add_dependency", project, mod_id="fabric-api", versions="*").write(project)
    with pytest.raises(ApplyError, match="already declared"):
        apply("fabric.add_dependency", project, mod_id="fabric-api", versions="*")


def test_add_mixin_registers_the_class_and_creates_the_source(project):
    changeset = apply("fabric.add_mixin", project,
                      target_class="net.minecraft.world.item.ItemStack")
    paths = [c.path for c in changeset.changes]
    assert "src/main/java/com/example/demo/mixin/ItemStackMixin.java" in paths
    assert "src/main/resources/demo.mixins.json" in paths
    changeset.write(project)
    config = json.loads((project / "src/main/resources/demo.mixins.json").read_text())
    assert config["mixins"] == ["ItemStackMixin"]
    # The generated mixin must be honest about what it does and does not do.
    source = (project / "src/main/java/com/example/demo/mixin/ItemStackMixin.java").read_text()
    assert "additive" in source
    assert any("placeholder" in n for n in changeset.notes)


def test_add_mixin_rejects_an_invalid_class_name(project):
    with pytest.raises(ApplyError, match="not a valid Java class name"):
        apply("fabric.add_mixin", project, target_class="a.B", mixin_name="not valid")


def test_add_entrypoint_wires_manifest_and_source(project):
    changeset = apply("fabric.add_entrypoint", project, class_name="DemoClient", kind="client")
    changeset.write(project)
    manifest = json.loads((project / "src/main/resources/fabric.mod.json").read_text()
                          .replace("${version}", "0"))
    assert manifest["entrypoints"]["client"] == ["com.example.demo.client.DemoClient"]
    source = (project / "src/main/java/com/example/demo/client/DemoClient.java").read_text()
    assert "implements ClientModInitializer" in source


def test_apply_rejects_an_unknown_generator(project):
    with pytest.raises(ApplyError, match="unknown generator"):
        apply("fabric.make_it_good", project)


def test_apply_rejects_a_non_project_directory(tmp_path):
    with pytest.raises(ApplyError):
        apply("fabric.add_dependency", tmp_path / "nope", mod_id="x")


@build_tests
def test_applied_change_still_builds(tmp_path):
    """A generated change must leave the project buildable."""
    result = scaffold("minecraft", "fabric", tmp_path / "proj", "applycheck")
    apply("fabric.add_entrypoint", result.root, class_name="ApplycheckClient",
          kind="client").write(result.root)
    build = run_build(result.root, ["gradle build --no-daemon --console=plain"],
                      allow_execute=True, timeout=1800)
    assert build.ok, build.runs[-1].stderr_tail or build.runs[-1].stdout_tail
    jar = next(a for a in build.artifacts
               if a.path.endswith(".jar") and "sources" not in a.path)
    ins = inspect_path(jar.path)
    assert "client" in (ins.fact("entrypoints") or {})
