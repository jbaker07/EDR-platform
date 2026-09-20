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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.SystemReport|SystemReport]] | `<init>` | injects_into `@Inject at RETURN` | both | `SystemReportMixin.fillSystemDetails` |
| [[40-Interfaces/net.minecraft.server.dedicated.ServerWatchdog|ServerWatchdog]] | `createWatchdogCrashReport(Ljava/lang/String;J)Lnet/minecraft/CrashReport;` | injects_into `@ModifyArg at INVOKE Ljava/lang/StringBuilder;append(Ljava/lang/Object;)Ljava/lang/StringBuilder;` | both | `ServerWatchdogMixin.printEntireThreadDump` |

## API surface


## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
