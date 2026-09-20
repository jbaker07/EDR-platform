"""The atlas extractors, on independently compiled inputs.

The fixtures under tests/fixtures/mixin_inputs were compiled by
tests/fixtures/mixin_inputs/compile.sh with the pinned JDK against the real
sponge-mixin, MixinExtras, fabric-loader and fabric-api-base jars. The compiled
classes are committed, so these tests need neither the JDK nor the Gradle cache.
"""
from __future__ import annotations

import importlib.util
import json
import sys
from pathlib import Path

import pytest

ROOT = Path(__file__).resolve().parents[1]
EXTRACT = ROOT / "atlas" / "extract"
FIX = ROOT / "tests" / "fixtures" / "mixin_inputs" / "classes"
sys.path.insert(0, str(EXTRACT))

import classfile as cf  # noqa: E402
import jvm  # noqa: E402
import resolve  # noqa: E402


def load_mixin(name: str) -> dict:
    return jvm.mixin_facts(cf.read_class((FIX / "fixtures" / "mixins" / f"{name}.class").read_bytes()))


def injections(name: str) -> dict[str, dict]:
    return {i["handler"]["name"]: i for i in load_mixin(name)["injections"]}


def test_marker_overwrite_without_arguments_is_found_with_exact_descriptor():
    f = load_mixin("MarkerOverwriteMixin")
    ov = [i for i in f["injections"] if i["injector"] == "Overwrite"]
    assert len(ov) == 1
    assert ov[0]["effect"] == "replace_method_body"
    assert ov[0]["selectors"] == [{"kind": "handler", "owner": None, "name": "compute", "desc": "(I)I", "exact_resolvable": True}]
    assert {(s["name"], s["static"]) for s in f["shadows"]} == {("counter", True), ("value", False), ("helper", False)}
    assert [u["name"] for u in f["uniques"]] == ["extra"]
    assert f["extraction_failures"] == []


def test_multiple_injection_points_selectors_slices_and_desc_targets():
    inj = injections("MultiAtMixin")
    two = inj["twoPointsTwoMethods"]
    assert [s["name"] for s in two["selectors"]] == ["run", "init"]
    assert [a["value"] for a in two["at"]] == ["HEAD", "INVOKE"]
    assert two["at"][1]["target"] == "Lfixtures/target/Target;helper()V" and two["at"][1]["ordinal"] == 0
    assert two["cancellable"] is True and two["require"] == 1
    sliced = inj["withDescriptorAndSlice"]
    assert sliced["selectors"][0]["desc"] == "(Ljava/lang/String;)Ljava/lang/String;"
    assert sliced["slice"][0]["to"]["value"] == "INVOKE"
    desc = inj["descTarget"]["selectors"][0]
    assert desc["kind"] == "desc" and desc["name"] == "compute" and desc["args"] == ["I"] and desc["ret"] == "I"
    assert inj["ctor"]["selectors"][0]["name"] == "<init>" and inj["clinit"]["selectors"][0]["name"] == "<clinit>"
    owner_q = inj["ownerQualified"]
    assert owner_q["selectors"][0]["owner"] == "fixtures/target/Target" and owner_q["selectors"][0]["desc"] == "()V"
    assert owner_q["at"][0]["shift"] == "AFTER" and owner_q["at"][0]["opcode"] == 179 and owner_q["at"][0]["by"] == 1
    assert load_mixin("MultiAtMixin")["priority"] == 900 and load_mixin("MultiAtMixin")["remap"] is False


def test_quantified_selector_is_recorded_as_unsupported_not_dropped():
    wild = injections("MultiAtMixin")["wildcard"]
    assert wild["selectors"][0]["exact_resolvable"] is False
    assert "quantified" in wild["selectors"][0]["unsupported"]
    assert wild["unsupported"]


def test_mixinextras_namespace_injectors_are_all_recognised():
    inj = injections("ExtrasMixin")
    assert {i["injector"] for i in inj.values()} == {"WrapOperation", "ModifyReturnValue", "ModifyExpressionValue",
                                                     "WrapWithCondition", "WrapMethod", "ModifyReceiver"}
    assert inj["wrapHelper"]["effect"] == "wrap_call_site" and inj["wrapGreet"]["effect"] == "wrap_method"
    assert load_mixin("ExtrasMixin")["extraction_failures"] == []


def test_redirect_and_modify_family():
    inj = injections("RedirectAndModifyMixin")
    assert inj["redirectHelper"]["effect"] == "replace_call_site"
    assert inj["modifyArg"]["index"] == 0 and inj["modifyVar"]["ordinal"] == 0
    assert inj["modifyConst"]["constant"] == [{"intValue": 2}]


def test_accessor_invoker_string_target_environment_and_pseudo():
    acc = load_mixin("AccessorMixin")
    assert acc["kind"] == "interface"
    assert {(a["kind"], a["target"]) for a in acc["accessors"]} == {("Accessor", "counter"), ("Invoker", "helper")}
    st = load_mixin("StringTargetClientMixin")
    assert st["targets"] == [{"kind": "string", "name": "fixtures/target/Target$Inner"}]
    assert st["priority"] == 1100 and st["environment"] == "client" and st["pseudo"] is True


def test_unknown_annotation_in_mixin_namespace_is_an_explicit_failure():
    f = load_mixin("UnsupportedMixin")
    assert f["injections"] == []
    assert len(f["extraction_failures"]) == 1
    assert f["extraction_failures"][0]["annotation"] == "Lorg/spongepowered/asm/mixin/injection/FakeInjector;"


def test_reads_writes_and_event_fires_are_per_method_with_opcodes():
    ev = cf.read_class((FIX / "fixtures" / "target" / "EventsFixture.class").read_bytes())
    r = {m["name"]: m for m in jvm.class_refs(ev)["methods"]}
    assert [(x["opcode"], x["access"]) for x in r["writeOnly"]["refs"] if x["kind"] == "field"] == [("putfield", "write")]
    rw = [(x["opcode"], x["access"], x["name"]) for x in r["readAndWrite"]["refs"] if x["kind"] == "field"]
    assert rw == [("getfield", "read", "writes"), ("getstatic", "read", "reads"), ("putfield", "write", "writes")]
    assert r["readOnly"]["event_fires"] == []
    fire = r["fire"]["event_fires"][0]
    assert fire["event_field"] == "TICK" and fire["callback_name"] == "onTick"
    assert fire["callback_desc"] == "(Lfixtures/target/EventsFixture;)V" and fire["offset"] > fire["getstatic_offset"]


def test_surface_keeps_descriptors_and_static_initialiser():
    t = cf.read_class((FIX / "fixtures" / "target" / "Target.class").read_bytes())
    s = jvm.surface(t)
    names = {(m["name"], m["desc"]) for m in s["methods"]}
    assert ("<clinit>", "()V") in names and ("<init>", "()V") in names and ("compute", "(I)I") in names
    assert any(f["name"] == "counter" and f["desc"] == "I" and f["access"] & 0x0008 for f in s["fields"])


def test_resolver_states_on_fixture_classes():
    class _Dir:
        pass
    r = resolve.Resolver([], FIX)  # a directory of classes
    assert r.resolve("fixtures/target/Target", "compute", "(I)I")["state"] == "exact"
    assert r.resolve("fixtures/target/Target", "compute", None)["state"] == "name_only"
    assert r.resolve("fixtures/target/Target", "<clinit>", None)["state"] == "exact"
    assert r.resolve("fixtures/target/Target", "<init>", "()V")["state"] == "exact"
    assert r.resolve("fixtures/target/Target", "nothing", None)["state"] == "unresolved"
    assert r.resolve("fixtures/target/Missing", "x", None)["state"] == "owner_missing"
    sel = jvm.parse_selector("lambda$*")
    assert r.resolve_selector(sel, "fixtures/target/Target")["state"] == "selector_unsupported"
    assert resolve.parse_target("Lfixtures/target/Target;counter:I")["kind"] == "field"


def test_hierarchy_walk_is_cycle_safe(tmp_path):
    # a class whose interface list points back at itself must not loop
    r = resolve.Resolver([], FIX)
    walk = r.hierarchy("fixtures/target/Target")
    assert walk[0] == "fixtures/target/Target" and len(walk) == len(set(walk))


@pytest.mark.skipif(not (ROOT / "atlas" / "extracted" / "edges.json").exists(), reason="atlas not extracted")
def test_edges_carry_resolution_and_opcode_split():
    e = json.loads((ROOT / "atlas" / "extracted" / "edges.json").read_text())
    assert e["counts"]["writes"] > 0 and e["counts"]["reads"] > 0
    assert set(e["resolution_by_relation"]["reads"]) <= {"exact", "inherited_exact"}
    inj = [x for x in e["edges"] if x["relation"] == "injects_into"]
    assert all("resolution" in x["to"] and "points" in x for x in inj)
    assert e["counts"]["replaces"] == 1, "Fabric API 0.161.0+26.3 carries exactly one @Overwrite"
    assert sum(1 for x in e["edges"] if x.get("injector") == "WrapOperation") > 100


@pytest.mark.skipif(not (ROOT / "atlas" / "extracted" / "mixin_transformation_tests.json").exists(), reason="no harness output")
def test_transformation_evidence_is_labelled_and_matches_the_static_priority_rule():
    t = json.loads((ROOT / "atlas" / "extracted" / "mixin_transformation_tests.json").read_text())
    assert t["evidence_class"] == "executed_transformation"
    by = {s["id"]: s for s in t["scenarios"]}
    assert by["A"]["trace"] == ["A2.head", "A1.overwrite", "A2.tail"]
    assert not by["B"]["transformed"] and "cannot inject into" in by["B"]["transform_error"]
    assert by["K"]["trace"] == ["K1.overwrite.calls_step", "K2.at_step.p1100", "K.step"]
    assert not by["M"]["transformed"] and "(0/1) succeeded" in by["M"]["transform_error"]
    assert t["static_checks"]["checkPriority_overriders"]["MethodHead"] == ["iconst_1", "ireturn"]
    assert t["static_checks"]["checkPriority_overriders"]["BeforeInvoke"] is None
