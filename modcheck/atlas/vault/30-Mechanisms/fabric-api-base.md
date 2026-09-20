---
type: "mechanism"
module: "fabric-api-base"
version: "2.0.6+fcdff87f5d"
sha256: "88485b1edbcb642fa28b8f53e173835b19f6b6e49aa5b088e3fe16653ef67a13"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-api-base

**Version** `2.0.6+fcdff87f5d` -- **artifact sha256** `88485b1edbcb642fa28b8f53e173835b19f6b6e49aa5b088e3fe16653ef67a13`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `null`
- mixin configs: `null`
- mixin classes: 0 found by annotation, 0 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.event.AutoInvokingEvent|AutoInvokingEvent]] (annotation, 0 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.Event|Event]] (abstract_class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.EventFactory|EventFactory]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.util.BooleanFunction|BooleanFunction]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.util.EventResult|EventResult]] (enum, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.util.NbtType|NbtType]] (class, 13 members)
- [[40-Interfaces/net.fabricmc.fabric.api.util.TriState|TriState]] (enum, 16 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
