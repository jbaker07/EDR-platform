---
type: "mechanism"
module: "fabric-serialization-api-v1"
version: "2.0.7+74ed1ea55d"
sha256: "eb0799d38e825cb6e82fe40ce1dd80da2ed9f142d60952e04609ccbbba153691"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-serialization-api-v1

**Version** `2.0.7+74ed1ea55d` -- **artifact sha256** `eb0799d38e825cb6e82fe40ce1dd80da2ed9f142d60952e04609ccbbba153691`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `null`
- mixin configs: `["fabric-serialization-api-v1.mixins.json"]`
- access widener: `fabric-serialization-api-v1.classtweaker`
- mixin classes: 4 found by annotation, 4 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.serialization.v1.MoreCodecs|MoreCodecs]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.serialization.v1.value.FabricValueInput|FabricValueInput]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.serialization.v1.value.FabricValueOutput|FabricValueOutput]] (interface, 2 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
