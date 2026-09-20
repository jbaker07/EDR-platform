"""Run the controlled Mixin transformation scenarios and record the evidence.

Evidence class ``executed_transformation``: the pinned sponge-mixin transformer
(and MixinExtras) was executed by this JVM over independently compiled classes.
It establishes how the transformer composes mixins. It is NOT Minecraft runtime
evidence: no game class, no loader, no Knot classloader was involved.

Output: atlas/extracted/mixin_transformation_tests.json
"""
from __future__ import annotations

import hashlib
import json
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
HARNESS = ROOT / "harness" / "mixin_transform"
EXTRACTED = ROOT / "extracted"
sys.path.insert(0, str(ROOT / "extract"))
from classfile import read_class_from_jar  # noqa: E402

# what each scenario is set up to test; the result is read from the harness
SCENARIOS = {
    "A": {"question": "Does an @Overwrite from one mod remove a HEAD/TAIL @Inject on the same method from another mod (equal priority 1000)?",
          "setup": "moda A_Overwrite: @Overwrite run(); modb A_Inject: @Inject HEAD and TAIL into run()"},
    "B": {"question": "Does an INVOKE-point @Inject (require=1) into a method overwritten by an equal-priority mod apply?",
          "setup": "moda B_Overwrite (body no longer calls step); modb B_InjectInvoke at INVOKE step(), require=1"},
    "B2": {"question": "Same as B with require=0, expect=0: is the failure a target-count failure or a validation refusal?",
           "setup": "moda B2_Overwrite; modb B2_InjectInvokeOptional at INVOKE step(), require=0, expect=0"},
    "C": {"question": "Do two @Redirects of the same call site from two mods both apply?",
          "setup": "moda C_Redirect and modb C_Redirect2 both redirect INVOKE step() in run()"},
    "D": {"question": "Does a MixinExtras @WrapOperation compose with another mod's @Redirect of the same call?",
          "setup": "moda D_Redirect; modb D_Wrap (@WrapOperation) on INVOKE step()"},
    "E": {"question": "In what order do equal-priority HEAD and TAIL injections from two mods run?",
          "setup": "moda E_Inject and modb E_Inject2, both HEAD+TAIL, priority 1000, configs added moda then modb"},
    "F": {"question": "What happens when two mods @Overwrite the same method at equal priority?",
          "setup": "moda F_Overwrite and modb F_Overwrite2"},
    "G": {"question": "Does mixin priority order HEAD injections from two mods?",
          "setup": "moda G_Inject1100 (priority 1100) and modb G_Inject900 (priority 900), both HEAD"},
    "H": {"question": "Does a cancelling HEAD injection from one mod suppress another mod's TAIL and RETURN injections?",
          "setup": "moda H_Cancel: HEAD cancellable, ci.cancel(); modb H_TailReturn: TAIL and RETURN"},
    "J": {"question": "Does a @Redirect into a method overwritten by an equal-priority mod apply when the overwritten body still contains the call?",
          "setup": "moda J_OverwriteKeepsStep: @Overwrite run() calling step(); modb J_Redirect on INVOKE step()"},
    "K": {"question": "Does an INVOKE-point @Inject with HIGHER priority than the overwriting mixin apply to the overwritten body?",
          "setup": "moda K_OverwriteKeepsStep (1000); modb K_InjectInvoke1100 (1100) at INVOKE step()"},
    "L": {"question": "Do HEAD/TAIL injections with LOWER priority than the overwriting mixin apply?",
          "setup": "moda L_Overwrite (1000); modb L_InjectHead900 (900) HEAD and TAIL"},
    "M": {"question": "With higher priority, what happens when the overwritten body no longer contains the INVOKE target?",
          "setup": "moda M_OverwriteNoStep (1000); modb M_InjectInvoke1100 (1100) at INVOKE step(), require=1"},
    "N": {"question": "Does a HEAD injection with higher priority than the overwrite apply?",
          "setup": "moda N_Overwrite (1000); modb N_InjectHead1100 (1100) HEAD"},
}


def sha256(p: Path) -> str:
    return hashlib.sha256(p.read_bytes()).hexdigest()


def static_checks(mixin_jar: Path) -> dict:
    """Read, from the pinned Mixin jar, the code the scenarios exercise."""
    ip = read_class_from_jar(mixin_jar, "org/spongepowered/asm/mixin/injection/InjectionPoint")
    default = next(m for m in ip.methods if m.name == "checkPriority")
    overriders = {}
    for cls in ("MethodHead", "BeforeReturn", "BeforeFinalReturn", "BeforeInvoke", "AfterInvoke",
                "BeforeFieldAccess", "BeforeNew", "BeforeConstant", "JumpInsnPoint", "BeforeLoadLocal", "AfterStoreLocal"):
        try:
            c = read_class_from_jar(mixin_jar, f"org/spongepowered/asm/mixin/injection/points/{cls}")
        except KeyError:
            continue
        m = next((x for x in c.methods if x.name == "checkPriority"), None)
        overriders[cls] = [i.mnemonic for i in m.code] if m and m.code else None
    passes = read_class_from_jar(mixin_jar, "org/spongepowered/asm/mixin/transformer/MixinApplicatorStandard$ApplicatorPass")
    pass_order = [f.name for f in passes.fields if f.access & 0x4000]  # enum constants, declaration order
    mixin_ann = read_class_from_jar(mixin_jar, "org/spongepowered/asm/mixin/Mixin")
    defaults = {m.name: m.annotation_default for m in mixin_ann.methods if m.annotation_default is not None}
    return {
        "InjectionPoint.checkPriority.default_bytecode": [i.mnemonic for i in default.code],
        "InjectionPoint.checkPriority.reading": "returns true iff targetPriority < mixinPriority (iload_1, iload_2, if_icmpge)",
        "checkPriority_overriders": overriders,
        "ApplicatorPass.order": pass_order,
        "Mixin.annotation_defaults": defaults,
    }


def main() -> int:
    result_path = HARNESS / "build" / "result.json"
    subprocess.run([str(HARNESS / "run.sh"), str(result_path)], check=True, capture_output=True, text=True)
    result = json.loads(result_path.read_text())
    cache = Path("/root/.gradle/caches/modules-2/files-2.1")
    mixin_jar = next(cache.glob("net.fabricmc/sponge-mixin/0.17.4+mixin.0.8.7/*/sponge-mixin-0.17.4+mixin.0.8.7.jar"))
    extras_jar = next(cache.glob("io.github.llamalad7/mixinextras-fabric/0.5.5/*/mixinextras-fabric-0.5.5.jar"))
    scenarios = []
    for s in result["scenarios"]:
        key = s["target"].split(".")[-1]
        scenarios.append({"id": key, **SCENARIOS.get(key, {}), "target": s["target"], "transformed": s["transformed"],
                          "bytes_changed": s["bytes_changed"], "transform_error": s["transform_error"],
                          "run_error": s["run_error"], "applied": s["applied"], "trace": s["trace"]})
    out = {
        "generated_at": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "evidence_class": "executed_transformation",
        "what_this_is": "The pinned Mixin transformer run in a plain JVM over independently compiled target and mixin "
                        "classes (two mixin configs standing in for two mods). Not a Minecraft run: no game class, "
                        "no fabric-loader, no transforming classloader.",
        "runtime": {"mixin_version": result["mixin_version"], "java": result["java"],
                    "sponge_mixin": {"path": str(mixin_jar), "sha256": sha256(mixin_jar)},
                    "mixinextras": {"path": str(extras_jar), "sha256": sha256(extras_jar)},
                    "harness": str(HARNESS.relative_to(ROOT.parents[0]))},
        "sources": {"fixtures": sorted(str(p.relative_to(ROOT.parents[0])) for p in (HARNESS / "src" / "fixtures").rglob("*.java")),
                    "configs": ["moda.mixins.json", "modb.mixins.json"], "config_default_require": 1},
        "static_checks": static_checks(mixin_jar),
        "scenarios": scenarios,
        "audit": result["audit"],
    }
    (EXTRACTED / "mixin_transformation_tests.json").write_text(json.dumps(out, indent=1))
    ok = sum(1 for s in scenarios if s["transformed"])
    print(f"wrote mixin_transformation_tests.json: {len(scenarios)} scenarios, {ok} transformed, "
          f"{len(scenarios) - ok} refused at transformation")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
