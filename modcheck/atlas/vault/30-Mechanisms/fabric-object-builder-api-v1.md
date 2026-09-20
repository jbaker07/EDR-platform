---
type: "mechanism"
module: "fabric-object-builder-api-v1"
version: "24.1.9+3434d6d95d"
sha256: "3a5f0ccef440552828d9469420547dc3cdbe3e206af73fa98d0bfcfb5b75c1ba"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-object-builder-api-v1

**Version** `24.1.9+3434d6d95d` -- **artifact sha256** `3a5f0ccef440552828d9469420547dc3cdbe3e206af73fa98d0bfcfb5b75c1ba`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-registry-sync-v0": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-object-builder-v1.mixins.json", {"config": "fabric-object-builder-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-object-builder-api-v1.classtweaker`
- mixin classes: 14 found by annotation, 14 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry.MODIFY|FabricDefaultAttributeRegistry.MODIFY]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.HangingSignEditScreen|HangingSignEditScreen]].`<init>` | `(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraft/world/level/block/entity/SignTextSlot;Z)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/resources/Identifier;withDefaultNamespace(Ljava/lang/String;)Lnet/minecraft/resources/Identifier;` (exact) | client | 1000 (default) | `HangingSignEditScreenMixin.init` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.SignEditScreen|SignEditScreen]].`<init>` | `(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraft/world/level/block/entity/SignTextSlot;Z)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/resources/Identifier;withDefaultNamespace(Ljava/lang/String;)Lnet/minecraft/resources/Identifier;` (exact) | client | 1000 (default) | `SignEditScreenMixin.init` |
| [[40-Interfaces/net.minecraft.core.registries.BuiltInRegistries|BuiltInRegistries]].`freeze` | `()V` | name_only | @Inject | HEAD | both | 1000 (default) | `BuiltInRegistriesMixin.modifyAttributes` |
| [[40-Interfaces/net.minecraft.network.syncher.EntityDataSerializers|EntityDataSerializers]].`<clinit>` | `()V` | exact | @Inject | TAIL | both | 1000 (default) | `EntityDataSerializersMixin.storeVanillaHandlers` |
| [[40-Interfaces/net.minecraft.network.syncher.EntityDataSerializers|EntityDataSerializers]].`registerSerializer` | `(Lnet/minecraft/network/syncher/EntityDataSerializer;)V` | exact | @Inject | HEAD | both | 1000 (default) | `EntityDataSerializersMixin.onHeadRegister` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType|EntityType]].`onlyOpCanSetNbt` | `()Z` | name_only | @Inject | HEAD | both | 1000 (default) | `EntityTypeMixin.onCanPotentiallyExecuteCommands` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType|EntityType]].`trackDeltas` | `()Z` | name_only | @Inject | HEAD | both | 1000 (default) | `EntityTypeMixin.onAlwaysUpdateVelocity` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType_Builder|EntityType$Builder]].`build` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/entity/EntityType;` | name_only | @Inject | RETURN | both | 1000 (default) | `EntityTypeBuilderMixin.applyChildBuilders` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType_Builder|EntityType$Builder]].`build` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/entity/EntityType;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/util/Util;fetchChoiceType(Lcom/mojang/datafixers/DSL$TypeReference;Ljava/lang/String;)Lcom/mojang/datafixers/types/Type;` (exact) | both | 1000 (default) | `EntityTypeBuilderMixin.allowNoModdedDatafixers` |
| [[40-Interfaces/net.minecraft.world.entity.ai.attributes.DefaultAttributes|DefaultAttributes]].`<clinit>` | `?` | selector_unsupported | @Inject | TAIL | both | 1000 (default) | `DefaultAttributesMixin.injectAttributes` |
| [[40-Interfaces/net.minecraft.world.level.block.DetectorRailBlock|DetectorRailBlock]].`getAnalogOutputSignal` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)I` | name_only | @Inject | HEAD | both | 1000 (default) | `DetectorRailBlockMixin.getCustomComparatorOutput` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntityType|BlockEntityType]].`<init>` | `(Lnet/minecraft/world/level/block/entity/BlockEntityType$BlockEntitySupplier;Ljava/util/Set;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `BlockEntityTypeMixin.mutableBlocks` |
| [[40-Interfaces/net.minecraft.world.level.storage.SavedDataStorage|SavedDataStorage]].`readTagFromDisk` | `(Ljava/nio/file/Path;Lnet/minecraft/util/datafix/DataFixTypes;I)Lnet/minecraft/nbt/CompoundTag;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/util/datafix/DataFixTypes;update(Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/nbt/CompoundTag;II)Lnet/minecraft/nbt/CompoundTag;` (exact) | both | 1000 (default) | `SavedDataStorageMixin.handleNullDataFixType` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityType|FabricBlockEntityType]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder|FabricBlockEntityTypeBuilder]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.block.type.BlockSetTypeBuilder|BlockSetTypeBuilder]] (class, 18 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.block.type.WoodTypeBuilder|WoodTypeBuilder]] (class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry|FabricDefaultAttributeRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityDataRegistry|FabricEntityDataRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityType|FabricEntityType]] (interface, 0 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.entity.MinecartComparatorLogic|MinecartComparatorLogic]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.entity.MinecartComparatorLogicRegistry|MinecartComparatorLogicRegistry]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.world.poi.PoiHelper|PoiHelper]] (class, 2 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
