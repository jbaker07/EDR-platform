---
type: "mechanism"
module: "fabric-advancement-api-v1"
version: "1.0.0+38c7a2d55d"
sha256: "89e2094ca63a5e3e4687ebd5530eb56a3f051fc568b7ab60243b69a6c8c3e74e"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-advancement-api-v1

**Version** `1.0.0+38c7a2d55d` -- **artifact sha256** `89e2094ca63a5e3e4687ebd5530eb56a3f051fc568b7ab60243b69a6c8c3e74e`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-resource-loader-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-advancement-api-v1.mixins.json"]`
- access widener: `fabric-advancement-api-v1.classtweaker`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.advancement.v1.AdvancementEvents.ALL_LOADED|AdvancementEvents.ALL_LOADED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.advancement.v1.AdvancementEvents.MODIFY|AdvancementEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.advancement.v1.AdvancementEvents.REPLACE|AdvancementEvents.REPLACE]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.advancements.Advancement_Builder|Advancement$Builder]] | `addCriterion(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/advancements/Advancement$Builder;` | injects_into `@Inject at HEAD` | both | `AdvancementBuilderMixin.addModifiedCriterion` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.advancement.v1.AdvancementEvents|AdvancementEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.advancement.v1.AdvancementSource|AdvancementSource]] (class, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.advancement.v1.FabricAdvancementBuilder|FabricAdvancementBuilder]] (interface, 11 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
