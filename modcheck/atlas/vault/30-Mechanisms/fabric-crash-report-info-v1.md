---
type: "mechanism"
module: "fabric-crash-report-info-v1"
version: "1.0.7+fcdff87f5d"
sha256: "8c89611954926210a9d7c388a2566a61ffbaec4a9d87d64a3f7eb86c8aff9a5a"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-crash-report-info-v1

**Version** `1.0.7+fcdff87f5d` -- **artifact sha256** `8c89611954926210a9d7c388a2566a61ffbaec4a9d87d64a3f7eb86c8aff9a5a`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `null`
- mixin configs: `["fabric-crash-report-info-v1.mixins.json"]`
- mixin classes: 2 found by annotation, 2 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.SystemReport|SystemReport]].`<init>` | `()V` | name_only | @Inject | RETURN | both | 1000 (default) | `SystemReportMixin.fillSystemDetails` |
| [[40-Interfaces/net.minecraft.server.dedicated.ServerWatchdog|ServerWatchdog]].`createWatchdogCrashReport` | `(Ljava/lang/String;J)Lnet/minecraft/CrashReport;` | exact | @ModifyArg | INVOKE `Ljava/lang/StringBuilder;append(Ljava/lang/Object;)Ljava/lang/StringBuilder;` (exact) | both | 1000 (default) | `ServerWatchdogMixin.printEntireThreadDump` |

## API surface


## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
