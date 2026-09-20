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

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry.MODIFY|FabricDefaultAttributeRegistry.MODIFY]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.core.registries.BuiltInRegistries|BuiltInRegistries]] | `freeze` | injects_into `@Inject at HEAD` | both | `BuiltInRegistriesMixin.modifyAttributes` |
| [[40-Interfaces/net.minecraft.network.syncher.EntityDataSerializers|EntityDataSerializers]] | `<clinit>` | injects_into `@Inject at TAIL` | both | `EntityDataSerializersMixin.storeVanillaHandlers` |
| [[40-Interfaces/net.minecraft.network.syncher.EntityDataSerializers|EntityDataSerializers]] | `registerSerializer(Lnet/minecraft/network/syncher/EntityDataSerializer;)V` | injects_into `@Inject at HEAD` | both | `EntityDataSerializersMixin.onHeadRegister` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType|EntityType]] | `onlyOpCanSetNbt` | injects_into `@Inject at HEAD` | both | `EntityTypeMixin.onCanPotentiallyExecuteCommands` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType|EntityType]] | `trackDeltas` | injects_into `@Inject at HEAD` | both | `EntityTypeMixin.onAlwaysUpdateVelocity` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType_Builder|EntityType$Builder]] | `build` | injects_into `@Inject at RETURN` | both | `EntityTypeBuilderMixin.applyChildBuilders` |
| [[40-Interfaces/net.minecraft.world.entity.ai.attributes.DefaultAttributes|DefaultAttributes]] | `<clinit>*` | injects_into `@Inject at TAIL` | both | `DefaultAttributesMixin.injectAttributes` |
| [[40-Interfaces/net.minecraft.world.level.block.DetectorRailBlock|DetectorRailBlock]] | `getAnalogOutputSignal` | injects_into `@Inject at HEAD` | both | `DetectorRailBlockMixin.getCustomComparatorOutput` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntityType|BlockEntityType]] | `<init>` | injects_into `@Inject at RETURN` | both | `BlockEntityTypeMixin.mutableBlocks` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityType|FabricBlockEntityType]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder|FabricBlockEntityTypeBuilder]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.block.type.BlockSetTypeBuilder|BlockSetTypeBuilder]] (class, 18 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.block.type.WoodTypeBuilder|WoodTypeBuilder]] (class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry|FabricDefaultAttributeRegistry]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityDataRegistry|FabricEntityDataRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.entity.MinecartComparatorLogic|MinecartComparatorLogic]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.entity.MinecartComparatorLogicRegistry|MinecartComparatorLogicRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.object.builder.v1.world.poi.PoiHelper|PoiHelper]] (class, 2 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
