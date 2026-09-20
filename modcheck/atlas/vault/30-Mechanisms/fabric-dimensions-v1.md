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
- mixin classes: 7 found by annotation, 7 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.dimension.v1.DimensionEvents.MODIFY_ATTRIBUTES|DimensionEvents.MODIFY_ATTRIBUTES]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/com.mojang.datafixers.types.templates.TaggedChoice|TaggedChoice]].`lambda$apply$0` | `(Lcom/mojang/datafixers/util/Pair;)Lcom/mojang/datafixers/types/Type;` | name_only | @Inject | RETURN | both | 1000 (default) | `TaggedChoiceMixin.onApply` |
| [[40-Interfaces/com.mojang.datafixers.types.templates.TaggedChoice_TaggedChoiceType|TaggedChoice$TaggedChoiceType]].`getMapCodec` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | name_only | @Inject | HEAD | both | 1000 (default) | `TaggedChoiceTaggedChoiceTypeMixin.onGetCodec` |
| [[40-Interfaces/net.minecraft.util.datafix.schemas.V2832|V2832]].`lambda$registerTypes$2` | `(Lcom/mojang/datafixers/schemas/Schema;)Lcom/mojang/datafixers/types/templates/TypeTemplate;` | name_only | @Redirect | INVOKE `Lcom/mojang/datafixers/DSL;taggedChoiceLazy(Ljava/lang/String;Lcom/mojang/datafixers/types/Type;Ljava/util/Map;)Lcom/mojang/datafixers/types/templates/TaggedChoice;` (exact) | both | 1000 (default) | `V2832Mixin.redirectTaggedChoiceLazy` |
| [[40-Interfaces/net.minecraft.util.datafix.schemas.V2832|V2832]].`lambda$registerTypes$4` | `(Lcom/mojang/datafixers/schemas/Schema;)Lcom/mojang/datafixers/types/templates/TypeTemplate;` | name_only | @Redirect | INVOKE `Lcom/mojang/datafixers/DSL;taggedChoiceLazy(Ljava/lang/String;Lcom/mojang/datafixers/types/Type;Ljava/util/Map;)Lcom/mojang/datafixers/types/templates/TaggedChoice;` (exact) | both | 1000 (default) | `V2832Mixin.redirectTaggedChoiceLazy` |
| [[40-Interfaces/net.minecraft.world.level.levelgen.WorldDimensions|WorldDimensions]].`bake` | `(Lnet/minecraft/core/Registry;)Lnet/minecraft/world/level/levelgen/WorldDimensions$Complete;` | name_only | @WrapMethod | - | both | 1000 (default) | `WorldDimensionsMixin.wrapBakeToProvideContext` |
| [[40-Interfaces/net.minecraft.world.level.levelgen.WorldDimensions|WorldDimensions]].`checkStability` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/dimension/LevelStem;)Lcom/mojang/serialization/Lifecycle;` | name_only | @Inject | HEAD | both | 1000 (default) | `WorldDimensionsMixin.betterModdedStabilityCheck` |
| [[40-Interfaces/net.minecraft.world.level.levelgen.WorldDimensions|WorldDimensions]].`lambda$static$0` | `(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;` | name_only | @Redirect | INVOKE `Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;group(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` (inherited_exact) | both | 1000 (default) | `WorldDimensionsMixin.useFailSoftMap` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.dimension.v1.DimensionEvents|DimensionEvents]] (class, 2 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
