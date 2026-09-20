---
type: "mechanism"
module: "fabric-dimensions-v1"
version: "5.1.19+47f74c985d"
sha256: "1b2d1d92f0e32719b571a51de70acc363b388f888bd8abf0882580a9a0a4f4d5"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-dimensions-v1

**Version** `5.1.19+47f74c985d` -- **artifact sha256** `1b2d1d92f0e32719b571a51de70acc363b388f888bd8abf0882580a9a0a4f4d5`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "minecraft": ">=1.16-rc.3", "fabric-api-base": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.dimension.DimensionModificationImpl"]}`
- mixin configs: `["fabric-dimensions-v1.mixins.json"]`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.dimension.v1.DimensionEvents.MODIFY_ATTRIBUTES|DimensionEvents.MODIFY_ATTRIBUTES]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/com.mojang.datafixers.types.templates.TaggedChoice|TaggedChoice]] | `lambda$apply$0` | injects_into `@Inject at RETURN` | both | `TaggedChoiceMixin.onApply` |
| [[40-Interfaces/com.mojang.datafixers.types.templates.TaggedChoice_TaggedChoiceType|TaggedChoice$TaggedChoiceType]] | `getMapCodec` | injects_into `@Inject at HEAD` | both | `TaggedChoiceTaggedChoiceTypeMixin.onGetCodec` |
| [[40-Interfaces/net.minecraft.util.datafix.schemas.V2832|V2832]] | `lambda$registerTypes$2` | wraps `@Redirect at INVOKE Lcom/mojang/datafixers/DSL;taggedChoiceLazy(Ljava/lang/String;Lcom/mojang/datafixers/types/Type;Ljava/util/Map;)Lcom/mojang/datafixers/types/templates/TaggedChoice;` | both | `V2832Mixin.redirectTaggedChoiceLazy` |
| [[40-Interfaces/net.minecraft.util.datafix.schemas.V2832|V2832]] | `lambda$registerTypes$4` | wraps `@Redirect at INVOKE Lcom/mojang/datafixers/DSL;taggedChoiceLazy(Ljava/lang/String;Lcom/mojang/datafixers/types/Type;Ljava/util/Map;)Lcom/mojang/datafixers/types/templates/TaggedChoice;` | both | `V2832Mixin.redirectTaggedChoiceLazy` |
| [[40-Interfaces/net.minecraft.world.level.levelgen.WorldDimensions|WorldDimensions]] | `checkStability` | injects_into `@Inject at HEAD` | both | `WorldDimensionsMixin.betterModdedStabilityCheck` |
| [[40-Interfaces/net.minecraft.world.level.levelgen.WorldDimensions|WorldDimensions]] | `lambda$static$0` | wraps `@Redirect at INVOKE Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;group(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | both | `WorldDimensionsMixin.useFailSoftMap` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.dimension.v1.DimensionEvents|DimensionEvents]] (class, 3 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
