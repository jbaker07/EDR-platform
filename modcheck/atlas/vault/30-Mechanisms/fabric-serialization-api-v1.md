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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.serialization.v1.MoreCodecs|MoreCodecs]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.serialization.v1.value.FabricValueInput|FabricValueInput]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.serialization.v1.value.FabricValueOutput|FabricValueOutput]] (interface, 2 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
