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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.KeyMapping_Category|KeyMapping$Category]] | `register(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/KeyMapping$Category;` | injects_into `@Inject at RETURN` | client | `KeyMappingCategoryMixin.onReturnRegister` |
| [[40-Interfaces/net.minecraft.client.Options|Options]] | `load()V` | injects_into `@Inject at HEAD` | client | `OptionsMixin.loadHook` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.keymapping.v1.KeyMappingHelper|KeyMappingHelper]] (class, 2 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
