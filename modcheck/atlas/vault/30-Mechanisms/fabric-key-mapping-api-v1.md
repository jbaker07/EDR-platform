---
type: "mechanism"
module: "fabric-key-mapping-api-v1"
version: "2.0.8+3434d6d95d"
sha256: "6ec1d5560d333f443626837379e495aef614b30cee6ccb4b1eb3856830b49e16"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-key-mapping-api-v1

**Version** `2.0.8+3434d6d95d` -- **artifact sha256** `6ec1d5560d333f443626837379e495aef614b30cee6ccb4b1eb3856830b49e16`

## Declared (fabric.mod.json)

- environment: `client`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `null`
- mixin configs: `["fabric-key-mapping-api-v1.mixins.json"]`
- mixin classes: 3 found by annotation, 3 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.KeyMapping_Category|KeyMapping$Category]].`register` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/KeyMapping$Category;` | exact | @Inject | RETURN | client | 1000 (default) | `KeyMappingCategoryMixin.onReturnRegister` |
| [[40-Interfaces/net.minecraft.client.Options|Options]].`load` | `()V` | exact | @Inject | HEAD | client | 1000 (default) | `OptionsMixin.loadHook` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.keymapping.v1.KeyMappingHelper|KeyMappingHelper]] (class, 2 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
