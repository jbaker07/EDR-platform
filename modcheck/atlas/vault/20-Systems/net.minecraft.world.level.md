---
type: "system"
package: "net.minecraft.world.level"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level

Analyst note: [[_authored/systems/net.minecraft.world.level|Level, blocks, block entities, chunks and world generation]]

2176 classes (1418 top-level) across 75 packages in the processed jar; 142 changed by Loom processing; 143 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.world.level.BlockGetter|BlockGetter]] -- calls:1 -- by fabric-block-getter-api-v2
- [[40-Interfaces/net.minecraft.world.level.CardinalLighting|CardinalLighting]] -- calls:10 -- by fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.world.level.ChunkPos|ChunkPos]] -- calls:10 -- by fabric-data-attachment-api-v1, fabric-lifecycle-events-v1, fabric-networking-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.world.level.CollisionGetter|CollisionGetter]] -- calls:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.level.DataPackConfig|DataPackConfig]] -- calls:4, reads:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.world.level.GameType|GameType]] -- reads:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.world.level.ItemLike|ItemLike]] -- calls:13 -- by fabric-api-lookup-api-v1, fabric-content-registries-v0, fabric-creative-tab-api-v1, fabric-data-generation-api-v1, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.Level|Level]] -- calls:49 -- by fabric-api-lookup-api-v1, fabric-block-api-v1, fabric-data-attachment-api-v1, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-networking-api-v1, fabric-permission-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.LevelAccessor|LevelAccessor]] -- calls:2 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.LevelReader|LevelReader]] -- calls:1 -- by fabric-block-getter-api-v2
- [[40-Interfaces/net.minecraft.world.level.WorldDataConfiguration|WorldDataConfiguration]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.world.level.biome.Biome|Biome]] -- calls:9, reads:12, writes:7 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.Biome_ClimateSettings|Biome$ClimateSettings]] -- calls:16 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.BiomeGenerationSettings|BiomeGenerationSettings]] -- calls:3, reads:8, writes:6 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.BiomeSource|BiomeSource]] -- calls:2, wraps:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.BiomeSpecialEffects|BiomeSpecialEffects]] -- writes:5 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.Biomes|Biomes]] -- reads:19 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.Climate|Climate]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.Climate_ParameterList|Climate$ParameterList]] -- calls:2 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.Climate_TargetPoint|Climate$TargetPoint]] -- calls:6 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.FeatureSorter|FeatureSorter]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.MobSpawnSettings|MobSpawnSettings]] -- calls:3 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.MobSpawnSettings_Builder|MobSpawnSettings$Builder]] -- calls:4 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.MobSpawnSettings_MobSpawnCost|MobSpawnSettings$MobSpawnCost]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.MobSpawnSettings_SpawnerData|MobSpawnSettings$SpawnerData]] -- calls:3 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList_Preset|MultiNoiseBiomeSourceParameterList$Preset]] -- calls:1, reads:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList_Preset_1|MultiNoiseBiomeSourceParameterList$Preset$1]] -- injects_into:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]] -- injects_into:5, reads:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.AbstractBedBlock|AbstractBedBlock]] -- calls:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.level.block.BedBlock|BedBlock]] -- reads:3 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.level.block.Block|Block]] -- calls:21, reads:1 -- by fabric-api-lookup-api-v1, fabric-block-api-v1, fabric-content-registries-v0, fabric-data-generation-api-v1, fabric-entity-events-v1, fabric-model-loading-api-v1, fabric-object-builder-api-v1, fabric-registry-sync-v0, fabric-renderer-indigo, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] -- injects_into:1, reads:17 -- by fabric-block-api-v1, fabric-content-registries-v0, fabric-entity-events-v1, fabric-registry-sync-v0, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.ChestBlock|ChestBlock]] -- calls:2, reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.ColorCollection|ColorCollection]] -- calls:7 -- by fabric-convention-tags-v2, fabric-entity-events-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.ComposterBlock|ComposterBlock]] -- reads:5 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.CrafterBlock|CrafterBlock]] -- injects_into:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.DetectorRailBlock|DetectorRailBlock]] -- calls:1, injects_into:1, reads:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.DispenserBlock|DispenserBlock]] -- reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.DropperBlock|DropperBlock]] -- injects_into:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.FireBlock|FireBlock]] -- calls:2, injects_into:3 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.LadderBlock|LadderBlock]] -- reads:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.LayeredCauldronBlock|LayeredCauldronBlock]] -- reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.LiquidBlock|LiquidBlock]] -- injects_into:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.SoundType|SoundType]] -- reads:3 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.TrapDoorBlock|TrapDoorBlock]] -- reads:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopper|WeatheringCopper]] -- injects_into:1, reads:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopperCollection|WeatheringCopperCollection]] -- calls:2 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopperCollection_ByState|WeatheringCopperCollection$ByState]] -- calls:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity|AbstractFurnaceBlockEntity]] -- calls:1, injects_into:2, reads:3, wraps:1 -- by fabric-item-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.BannerBlockEntity|BannerBlockEntity]] -- injects_into:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.BaseContainerBlockEntity|BaseContainerBlockEntity]] -- calls:1, wraps:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntity|BlockEntity]] -- calls:18, injects_into:2, reads:2 -- by fabric-api-lookup-api-v1, fabric-block-getter-api-v2, fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-networking-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntityType|BlockEntityType]] -- calls:3, injects_into:1, reads:1 -- by fabric-api-lookup-api-v1, fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.BrewingStandBlockEntity|BrewingStandBlockEntity]] -- wraps:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.ChestBlockEntity|ChestBlockEntity]] -- calls:4 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity|ChiseledBookShelfBlockEntity]] -- calls:1, injects_into:1, reads:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.DispenserBlockEntity|DispenserBlockEntity]] -- calls:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.Hopper|Hopper]] -- calls:3 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.HopperBlockEntity|HopperBlockEntity]] -- injects_into:2, reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.JukeboxBlockEntity|JukeboxBlockEntity]] -- calls:1, injects_into:1, reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.ListBackedContainer|ListBackedContainer]] -- wraps:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity|ShulkerBoxBlockEntity]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.state.BlockBehaviour_BlockStateBase|BlockBehaviour$BlockStateBase]] -- calls:2, injects_into:2, reads:1 -- by fabric-content-registries-v0, fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.world.level.block.state.BlockBehaviour_Properties|BlockBehaviour$Properties]] -- reads:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.state.BlockState|BlockState]] -- calls:73 -- by fabric-api-lookup-api-v1, fabric-block-api-v1, fabric-content-registries-v0, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-model-loading-api-v1, fabric-object-builder-api-v1, fabric-particles-v1, fabric-registry-sync-v0, fabric-renderer-api-v1, fabric-renderer-indigo, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.state.StateDefinition|StateDefinition]] -- calls:7 -- by fabric-content-registries-v0, fabric-model-loading-api-v1, fabric-object-builder-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.level.block.state.StateHolder|StateHolder]] -- calls:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.state.properties.BlockSetType|BlockSetType]] -- calls:15 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.state.properties.BlockSetType_PressurePlateSensitivity|BlockSetType$PressurePlateSensitivity]] -- reads:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.state.properties.BlockStateProperties|BlockStateProperties]] -- reads:4 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.state.properties.ChestType|ChestType]] -- reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.state.properties.IntegerProperty|IntegerProperty]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.state.properties.WoodType|WoodType]] -- calls:10 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.ChunkAccess|ChunkAccess]] -- calls:4, reads:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.ChunkGenerator|ChunkGenerator]] -- calls:3, writes:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.ImposterProtoChunk|ImposterProtoChunk]] -- reads:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]] -- calls:14, injects_into:7, reads:1, wraps:2 -- by fabric-block-getter-api-v2, fabric-data-attachment-api-v1, fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunkSection|LevelChunkSection]] -- wraps:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunkSection_1BlockCounter|LevelChunkSection$1BlockCounter]] -- wraps:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatus|ChunkStatus]] -- reads:8 -- by fabric-client-gametest-api-v1, fabric-data-attachment-api-v1, fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatusTasks|ChunkStatusTasks]] -- injects_into:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.status.WorldGenContext|WorldGenContext]] -- calls:3 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]] -- injects_into:4, wraps:1 -- by fabric-data-attachment-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.level.dimension.DimensionType|DimensionType]] -- calls:1 -- by fabric-dimensions-v1
- [[40-Interfaces/net.minecraft.world.level.dimension.LevelStem|LevelStem]] -- calls:4, reads:4 -- by fabric-biome-api-v1, fabric-dimensions-v1
- [[40-Interfaces/net.minecraft.world.level.entity.PersistentEntitySectionManager|PersistentEntitySectionManager]] -- injects_into:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.gameevent.GameEvent|GameEvent]] -- reads:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.gameevent.GameEvent_Context|GameEvent$Context]] -- calls:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.gameevent.vibrations.VibrationSystem|VibrationSystem]] -- reads:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.gamerules.GameRule|GameRule]] -- calls:11, wraps:1 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.world.level.gamerules.GameRuleCategory|GameRuleCategory]] -- reads:1 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.world.level.gamerules.GameRuleMap|GameRuleMap]] -- reads:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.gamerules.GameRuleType|GameRuleType]] -- reads:3 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.world.level.gamerules.GameRules|GameRules]] -- calls:11, reads:4 -- by fabric-client-gametest-api-v1, fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.GenerationStep_Decoration|GenerationStep$Decoration]] -- calls:3 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.LegacyRandomSource|LegacyRandomSource]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.RandomState|RandomState]] -- injects_into:1, reads:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.WorldDimensions|WorldDimensions]] -- injects_into:1, wraps:2 -- by fabric-dimensions-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.WorldGenSettings|WorldGenSettings]] -- reads:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.WorldgenRandom|WorldgenRandom]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate|BlockPredicate]] -- calls:33 -- by fabric-content-registries-v0, fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider|BlockStateProvider]] -- calls:26 -- by fabric-content-registries-v0, fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider|CopyPropertiesProvider]] -- calls:3 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider|RuleBasedStateProvider]] -- calls:5 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider_Builder|RuleBasedStateProvider$Builder]] -- calls:10 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.placement.PlacedFeature|PlacedFeature]] -- calls:2 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.presets.WorldPresets|WorldPresets]] -- reads:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.structure.Structure|Structure]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate|StructureTemplate]] -- calls:2 -- by fabric-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager|StructureTemplateManager]] -- injects_into:1 -- by fabric-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.structure.templatesystem.loader.TemplateSource|TemplateSource]] -- calls:1 -- by fabric-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.synth.PerlinNoise|PerlinNoise]] -- calls:2 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.material.FlowingFluid|FlowingFluid]] -- calls:1, injects_into:1 -- by fabric-block-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.material.Fluid|Fluid]] -- calls:12, injects_into:1, reads:1 -- by fabric-block-api-v1, fabric-registry-sync-v0, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.material.FluidState|FluidState]] -- calls:6 -- by fabric-block-api-v1, fabric-content-registries-v0, fabric-rendering-fluids-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.material.Fluids|Fluids]] -- reads:17 -- by fabric-registry-sync-v0, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.material.LavaFluid|LavaFluid]] -- injects_into:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.pathfinder.PathfindingContext|PathfindingContext]] -- calls:2, injects_into:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.pathfinder.WalkNodeEvaluator|WalkNodeEvaluator]] -- injects_into:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.saveddata.SavedData|SavedData]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.saveddata.SavedDataType|SavedDataType]] -- calls:2 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.LevelData_RespawnData|LevelData$RespawnData]] -- calls:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.level.storage.LevelResource|LevelResource]] -- reads:2 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.level.storage.LevelStorageSource|LevelStorageSource]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.LevelStorageSource_LevelStorageAccess|LevelStorageSource$LevelStorageAccess]] -- calls:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.level.storage.PrimaryLevelData|PrimaryLevelData]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.SavedDataStorage|SavedDataStorage]] -- calls:8, wraps:1 -- by fabric-client-gametest-api-v1, fabric-data-attachment-api-v1, fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.TagValueInput|TagValueInput]] -- calls:2, reads:1 -- by fabric-data-attachment-api-v1, fabric-serialization-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.TagValueOutput|TagValueOutput]] -- calls:4, reads:1 -- by fabric-data-attachment-api-v1, fabric-serialization-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.ValueInput|ValueInput]] -- calls:7 -- by fabric-data-attachment-api-v1, fabric-serialization-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.ValueOutput|ValueOutput]] -- calls:5 -- by fabric-data-attachment-api-v1, fabric-serialization-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootContext|LootContext]] -- calls:1 -- by fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootContext_Builder|LootContext$Builder]] -- calls:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootParams_Builder|LootParams$Builder]] -- calls:4 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool|LootPool]] -- calls:1 -- by fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool_Builder|LootPool$Builder]] -- calls:6, reads:1 -- by fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootTable|LootTable]] -- calls:2, reads:1, wraps:1 -- by fabric-data-generation-api-v1, fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootTable_Builder|LootTable$Builder]] -- calls:6, reads:1 -- by fabric-data-generation-api-v1, fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction|EnchantRandomlyFunction]] -- wraps:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.parameters.LootContextParamSets|LootContextParamSets]] -- reads:3 -- by fabric-data-generation-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.parameters.LootContextParams|LootContextParams]] -- reads:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt|ResolvableInt]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.validation.DirectoryValidator|DirectoryValidator]] -- calls:1 -- by fabric-resource-loader-v1

## Declared inventory

### `net.minecraft.world.level` (54 top-level)

`BaseCommandBlock`, `BaseSpawner`, `BlockAndLightGetter`, `BlockCollisions`, `BlockEventData`, [[40-Interfaces/net.minecraft.world.level.BlockGetter|BlockGetter]], [[40-Interfaces/net.minecraft.world.level.CardinalLighting|CardinalLighting]], [[40-Interfaces/net.minecraft.world.level.ChunkPos|ChunkPos]], `ClipBlockStateContext`, `ClipContext`, [[40-Interfaces/net.minecraft.world.level.CollisionGetter|CollisionGetter]], `ColorMapColorUtil`, `ColorResolver`, `CommonLevelAccessor`, `CustomSpawner`, [[40-Interfaces/net.minecraft.world.level.DataPackConfig|DataPackConfig]], `DryFoliageColor`, `EmptyBlockGetter`, `EmptyStructureManager`, `EntityBasedExplosionDamageCalculator`, `EntityGetter`, `Explosion`, `ExplosionDamageCalculator`, `FoliageColor`, [[40-Interfaces/net.minecraft.world.level.GameType|GameType]], `GrassColor`, [[40-Interfaces/net.minecraft.world.level.ItemLike|ItemLike]], [[40-Interfaces/net.minecraft.world.level.Level|Level]], [[40-Interfaces/net.minecraft.world.level.LevelAccessor|LevelAccessor]], `LevelHeightAccessor`, [[40-Interfaces/net.minecraft.world.level.LevelReader|LevelReader]], `LevelSettings`, `LevelSimulatedRW`, `LevelSimulatedReader`, `LevelWriter`, `LightLayer`, `LocalMobCapCalculator`, `MoonPhase`, `NaturalSpawner`, `NoiseColumn`, `PathNavigationRegion`, `PotentialCalculator`, `ScheduledTickAccess`, `ServerExplosion`, `ServerLevelAccessor`, `SignalGetter`, `SimpleExplosionDamageCalculator`, `SpawnData`, `Spawner`, `StructureManager`, `TicketStorage`, [[40-Interfaces/net.minecraft.world.level.WorldDataConfiguration|WorldDataConfiguration]], `WorldGenLevel`, `package-info`

### `net.minecraft.world.level.biome` (19 top-level)

[[40-Interfaces/net.minecraft.world.level.biome.Biome|Biome]], [[40-Interfaces/net.minecraft.world.level.biome.BiomeGenerationSettings|BiomeGenerationSettings]], `BiomeManager`, `BiomeResolver`, [[40-Interfaces/net.minecraft.world.level.biome.BiomeSource|BiomeSource]], `BiomeSources`, [[40-Interfaces/net.minecraft.world.level.biome.BiomeSpecialEffects|BiomeSpecialEffects]], [[40-Interfaces/net.minecraft.world.level.biome.Biomes|Biomes]], `CheckerboardColumnBiomeSource`, [[40-Interfaces/net.minecraft.world.level.biome.Climate|Climate]], [[40-Interfaces/net.minecraft.world.level.biome.FeatureSorter|FeatureSorter]], `FixedBiomeSource`, [[40-Interfaces/net.minecraft.world.level.biome.MobSpawnSettings|MobSpawnSettings]], `MultiNoiseBiomeSource`, `MultiNoiseBiomeSourceParameterList`, `MultiNoiseBiomeSourceParameterLists`, `OverworldBiomeBuilder`, [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]], `package-info`

### `net.minecraft.world.level.block` (330 top-level)

`AbstractBannerBlock`, [[40-Interfaces/net.minecraft.world.level.block.AbstractBedBlock|AbstractBedBlock]], `AbstractCandleBlock`, `AbstractCauldronBlock`, `AbstractChestBlock`, `AbstractFurnaceBlock`, `AbstractSkullBlock`, `AirBlock`, `AmethystBlock`, `AmethystClusterBlock`, `AnvilBlock`, `AttachedStemBlock`, `AzaleaBlock`, `BambooSaplingBlock`, `BambooStalkBlock`, `BannerBlock`, `BarrelBlock`, `BarrierBlock`, `BaseCoralFanBlock`, `BaseCoralPlantBlock`, `BaseCoralPlantTypeBlock`, `BaseCoralWallFanBlock`, `BaseEntityBlock`, `BaseFireBlock`, `BasePressurePlateBlock`, `BaseRailBlock`, `BaseTorchBlock`, `BeaconBeamBlock`, `BeaconBlock`, [[40-Interfaces/net.minecraft.world.level.block.BedBlock|BedBlock]], `BeehiveBlock`, `BeetrootBlock`, `BellBlock`, `BigDripleafBlock`, `BigDripleafStemBlock`, `BlastFurnaceBlock`, [[40-Interfaces/net.minecraft.world.level.block.Block|Block]], [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]], `BonemealSource`, `BonemealableBlock`, `BonemealableFeaturePlacerBlock`, `BrewingStandBlock`, `BrushableBlock`, `BubbleColumnBlock`, `BucketPickup`, `BuddingAmethystBlock`, `BushBlock`, `ButtonBlock`, `CactusBlock`, `CactusFlowerBlock`, `CakeBlock`, `CalibratedSculkSensorBlock`, `CampfireBlock`, `CandleBlock`, `CandleCakeBlock`, `CarpetBlock`, `CarrotBlock`, `CartographyTableBlock`, `CarvedPumpkinBlock`, `CauldronBlock`, `CaveVines`, `CaveVinesBlock`, `CaveVinesPlantBlock`, `CeilingHangingSignBlock`, `ChainBlock`, `ChangeOverTimeBlock`, [[40-Interfaces/net.minecraft.world.level.block.ChestBlock|ChestBlock]], `ChiseledBookShelfBlock`, `ChorusFlowerBlock`, `ChorusPlantBlock`, `CocoaBlock`, [[40-Interfaces/net.minecraft.world.level.block.ColorCollection|ColorCollection]], `ColoredFallingBlock`, `CommandBlock`, `ComparatorBlock`, [[40-Interfaces/net.minecraft.world.level.block.ComposterBlock|ComposterBlock]], `ConcretePowderBlock`, `ConduitBlock`, `CopperBulbBlock`, `CopperChestBlock`, `CopperGolemStatueBlock`, `CoralBlock`, `CoralFanBlock`, `CoralPlantBlock`, `CoralWallFanBlock`, [[40-Interfaces/net.minecraft.world.level.block.CrafterBlock|CrafterBlock]], `CraftingTableBlock`, `CreakingHeartBlock`, `CropBlock`, `CrossCollisionBlock`, `CryingObsidianBlock`, `DaylightDetectorBlock`, `DecoratedPotBlock`, [[40-Interfaces/net.minecraft.world.level.block.DetectorRailBlock|DetectorRailBlock]], `DiodeBlock`, `DirectionalBlock`, [[40-Interfaces/net.minecraft.world.level.block.DispenserBlock|DispenserBlock]], `DoorBlock`, `DoubleBlockCombiner`, `DoublePlantBlock`, `DragonEggBlock`, `DriedGhastBlock`, `DropExperienceBlock`, [[40-Interfaces/net.minecraft.world.level.block.DropperBlock|DropperBlock]], `DryVegetationBlock`, `EnchantingTableBlock`, `EndGatewayBlock`, `EndPortalBlock`, `EndPortalFrameBlock`, `EndRodBlock`, `EnderChestBlock`, `EntityBlock`, `EyeblossomBlock`, `FaceAttachedHorizontalDirectionalBlock`, `Fallable`, `FallingBlock`, `FallingParticlesLeavesBlock`, `FarmlandBlock`, `FenceBlock`, `FenceGateBlock`, [[40-Interfaces/net.minecraft.world.level.block.FireBlock|FireBlock]], `FireflyBushBlock`, `FlowerBedBlock`, `FlowerBlock`, `FlowerPotBlock`, `FrogspawnBlock`, `FrostedIceBlock`, `FurnaceBlock`, `GameMasterBlock`, `GlazedTerracottaBlock`, `GlowLichenBlock`, `GrassBlock`, `GrindstoneBlock`, `GrowingPlantBlock`, `GrowingPlantBodyBlock`, `GrowingPlantHeadBlock`, `HalfTransparentBlock`, `HangingMossBlock`, `HangingRootsBlock`, `HangingSignBlock`, `HayBlock`, `HeavyCoreBlock`, `HoneyBlock`, `HopperBlock`, `HorizontalDirectionalBlock`, `HugeMushroomBlock`, `IceBlock`, `InfestedBlock`, `InfestedRotatedPillarBlock`, `IronBarsBlock`, `JigsawBlock`, `JukeboxBlock`, `KelpBlock`, `KelpPlantBlock`, [[40-Interfaces/net.minecraft.world.level.block.LadderBlock|LadderBlock]], `LanternBlock`, `LavaCauldronBlock`, [[40-Interfaces/net.minecraft.world.level.block.LayeredCauldronBlock|LayeredCauldronBlock]], `LeafLitterBlock`, `LeavesBlock`, `LecternBlock`, `LevelEvent`, `LeverBlock`, `LightBlock`, `LightningRodBlock`, `LilyPadBlock`, [[40-Interfaces/net.minecraft.world.level.block.LiquidBlock|LiquidBlock]], `LiquidBlockContainer`, `LoomBlock`, `MagmaBlock`, `MangroveLeavesBlock`, `MangrovePropaguleBlock`, `MangroveRootsBlock`, `Mirror`, `MossyCarpetBlock`, `MudBlock`, `MultifaceBlock`, `MultifaceSpreadeableBlock`, `MultifaceSpreader`, `MushroomBlock`, `MyceliumBlock`, `NetherFungusBlock`, `NetherPortalBlock`, `NetherRootsBlock`, `NetherSproutsBlock`, `NetherVines`, `NetherWartBlock`, `NetherrackBlock`, `NoteBlock`, `NyliumBlock`, `ObserverBlock`, `PathBlock`, `PiglinWallSkullBlock`, `PipeBlock`, `PitcherCropBlock`, `PlainSignBlock`, `PlayerHeadBlock`, `PlayerWallHeadBlock`, `PointedDripstoneBlock`, `Portal`, `PotatoBlock`, `PotentSulfurBlock`, `PowderSnowBlock`, `PoweredBlock`, `PoweredRailBlock`, `PressurePlateBlock`, `PumpkinBlock`, `RailBlock`, `RailState`, `RedStoneOreBlock`, `RedstoneLampBlock`, `RedstoneTorchBlock`, `RedstoneWallTorchBlock`, `RedstoneWireBlock`, `RenderShape`, `RepeaterBlock`, `RespawnAnchorBlock`, `RodBlock`, `RootedDirtBlock`, `RotatedPillarBlock`, `Rotation`, `SandBlock`, `SaplingBlock`, `ScaffoldingBlock`, `SculkBehaviour`, `SculkBlock`, `SculkCatalystBlock`, `SculkSensorBlock`, `SculkShriekerBlock`, `SculkSpreader`, `SculkVeinBlock`, `SeaPickleBlock`, `SeagrassBlock`, `SegmentableBlock`, `SelectableSlotContainer`, `ShelfBlock`, `ShelfMushroomBlock`, `ShortDryGrassBlock`, `ShulkerBoxBlock`, `SideChainPartBlock`, `SignBlock`, `SimpleWaterloggedBlock`, `SkullBlock`, `SlabBlock`, `SlimeBlock`, `SmallDripleafBlock`, `SmithingTableBlock`, `SmokerBlock`, `SnifferEggBlock`, `SnowLayerBlock`, `SnowyBlock`, `SoulFireBlock`, `SoulSandBlock`, [[40-Interfaces/net.minecraft.world.level.block.SoundType|SoundType]], `SpawnerBlock`, `SpeleothemBlock`, `SpongeBlock`, `SporeBlossomBlock`, `SpreadingSnowyBlock`, `StainedGlassBlock`, `StainedGlassPaneBlock`, `StairBlock`, `StandingSignBlock`, `StemBlock`, `StonecutterBlock`, `StrawBedBlock`, `StructureBlock`, `StructureVoidBlock`, `SugarCaneBlock`, `SulfurSpikeBlock`, `SupportType`, `SuspiciousEffectHolder`, `SweetBerryBushBlock`, `TallDryGrassBlock`, `TallFlowerBlock`, `TallGrassBlock`, `TallSeagrassBlock`, `TargetBlock`, `TestBlock`, `TestInstanceBlock`, `TintedGlassBlock`, `TintedParticleLeavesBlock`, `TntBlock`, `TorchBlock`, `TorchflowerCropBlock`, `TransparentBlock`, [[40-Interfaces/net.minecraft.world.level.block.TrapDoorBlock|TrapDoorBlock]], `TrappedChestBlock`, `TrialSpawnerBlock`, `TripWireBlock`, `TripWireHookBlock`, `TurtleEggBlock`, `TwistingVinesBlock`, `TwistingVinesPlantBlock`, `UntintedParticleLeavesBlock`, `VaultBlock`, `VegetationBlock`, `VineBlock`, `WallBannerBlock`, `WallBlock`, `WallHangingSignBlock`, `WallSignBlock`, `WallSkullBlock`, `WallTorchBlock`, `WaterloggedTransparentBlock`, [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopper|WeatheringCopper]], `WeatheringCopperBarsBlock`, `WeatheringCopperBulbBlock`, `WeatheringCopperChainBlock`, `WeatheringCopperChestBlock`, [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopperCollection|WeatheringCopperCollection]], `WeatheringCopperDoorBlock`, `WeatheringCopperFullBlock`, `WeatheringCopperGolemStatueBlock`, `WeatheringCopperGrateBlock`, `WeatheringCopperSlabBlock`, `WeatheringCopperStairBlock`, `WeatheringCopperTrapDoorBlock`, `WeatheringLanternBlock`, `WeatheringLightningRodBlock`, `WebBlock`, `WeepingVinesBlock`, `WeepingVinesPlantBlock`, `WeightedPressurePlateBlock`, `WetSpongeBlock`, `WitherRoseBlock`, `WitherSkullBlock`, `WitherWallSkullBlock`, `WoolCarpetBlock`, `package-info`

### `net.minecraft.world.level.block.entity` (72 top-level)

[[40-Interfaces/net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity|AbstractFurnaceBlockEntity]], [[40-Interfaces/net.minecraft.world.level.block.entity.BannerBlockEntity|BannerBlockEntity]], `BannerPattern`, `BannerPatternLayers`, `BannerPatterns`, `BarrelBlockEntity`, [[40-Interfaces/net.minecraft.world.level.block.entity.BaseContainerBlockEntity|BaseContainerBlockEntity]], `BeaconBeamOwner`, `BeaconBlockEntity`, `BeehiveBlockEntity`, `BellBlockEntity`, `BlastFurnaceBlockEntity`, [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntity|BlockEntity]], `BlockEntityTicker`, [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntityType|BlockEntityType]], `BlockEntityTypeIds`, `BlockEntityTypes`, `BoundingBoxRenderable`, [[40-Interfaces/net.minecraft.world.level.block.entity.BrewingStandBlockEntity|BrewingStandBlockEntity]], `BrushableBlockEntity`, `CalibratedSculkSensorBlockEntity`, `CampfireBlockEntity`, [[40-Interfaces/net.minecraft.world.level.block.entity.ChestBlockEntity|ChestBlockEntity]], `ChestLidController`, [[40-Interfaces/net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity|ChiseledBookShelfBlockEntity]], `CommandBlockEntity`, `ComparatorBlockEntity`, `ConduitBlockEntity`, `ContainerOpenersCounter`, `CopperGolemStatueBlockEntity`, `CrafterBlockEntity`, `CreakingHeartBlockEntity`, `DaylightDetectorBlockEntity`, `DecoratedPotBlockEntity`, `DecoratedPotPattern`, `DecoratedPotPatterns`, [[40-Interfaces/net.minecraft.world.level.block.entity.DispenserBlockEntity|DispenserBlockEntity]], `DropperBlockEntity`, `EnchantingTableBlockEntity`, `EnderChestBlockEntity`, `FurnaceBlockEntity`, `HangingSignBlockEntity`, [[40-Interfaces/net.minecraft.world.level.block.entity.Hopper|Hopper]], [[40-Interfaces/net.minecraft.world.level.block.entity.HopperBlockEntity|HopperBlockEntity]], `JigsawBlockEntity`, [[40-Interfaces/net.minecraft.world.level.block.entity.JukeboxBlockEntity|JukeboxBlockEntity]], `LecternBlockEntity`, `LidBlockEntity`, [[40-Interfaces/net.minecraft.world.level.block.entity.ListBackedContainer|ListBackedContainer]], `PotDecorations`, `PotentSulfurBlockEntity`, `RandomizableContainerBlockEntity`, `SculkCatalystBlockEntity`, `SculkSensorBlockEntity`, `SculkShriekerBlockEntity`, `ShelfBlockEntity`, [[40-Interfaces/net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity|ShulkerBoxBlockEntity]], `SignBlockEntity`, `SignText`, `SignTextSlot`, `SkullBlockEntity`, `SmokerBlockEntity`, `SpawnerBlockEntity`, `StructureBlockEntity`, `TestBlockEntity`, `TestInstanceBlockEntity`, `TheEndGatewayBlockEntity`, `TheEndPortalBlockEntity`, `TickingBlockEntity`, `TrappedChestBlockEntity`, `TrialSpawnerBlockEntity`, `package-info`

### `net.minecraft.world.level.block.entity.trialspawner` (7 top-level)

`PlayerDetector`, `TrialSpawner`, `TrialSpawnerConfig`, `TrialSpawnerConfigs`, `TrialSpawnerState`, `TrialSpawnerStateData`, `package-info`

### `net.minecraft.world.level.block.entity.vault` (7 top-level)

`VaultBlockEntity`, `VaultClientData`, `VaultConfig`, `VaultServerData`, `VaultSharedData`, `VaultState`, `package-info`

### `net.minecraft.world.level.block.grower` (2 top-level)

`TreeGrower`, `package-info`

### `net.minecraft.world.level.block.piston` (7 top-level)

`MovingPistonBlock`, `PistonBaseBlock`, `PistonHeadBlock`, `PistonMath`, `PistonMovingBlockEntity`, `PistonStructureResolver`, `package-info`

### `net.minecraft.world.level.block.sounds` (3 top-level)

`AmbientDesertBlockSoundsPlayer`, `AmbientLeavesBlockSoundPlayer`, `package-info`

### `net.minecraft.world.level.block.state` (6 top-level)

`BlockBehaviour`, [[40-Interfaces/net.minecraft.world.level.block.state.BlockState|BlockState]], `SolidDebugger`, [[40-Interfaces/net.minecraft.world.level.block.state.StateDefinition|StateDefinition]], [[40-Interfaces/net.minecraft.world.level.block.state.StateHolder|StateHolder]], `package-info`

### `net.minecraft.world.level.block.state.pattern` (4 top-level)

`BlockInWorld`, `BlockPattern`, `BlockPatternBuilder`, `package-info`

### `net.minecraft.world.level.block.state.predicate` (3 top-level)

`BlockPredicate`, `BlockStatePredicate`, `package-info`

### `net.minecraft.world.level.block.state.properties` (33 top-level)

`AttachFace`, `BambooLeaves`, `BedPart`, `BellAttachType`, [[40-Interfaces/net.minecraft.world.level.block.state.properties.BlockSetType|BlockSetType]], [[40-Interfaces/net.minecraft.world.level.block.state.properties.BlockStateProperties|BlockStateProperties]], `BooleanProperty`, [[40-Interfaces/net.minecraft.world.level.block.state.properties.ChestType|ChestType]], `ComparatorMode`, `CreakingHeartState`, `DoorHingeSide`, `DoubleBlockHalf`, `EnumProperty`, `Half`, [[40-Interfaces/net.minecraft.world.level.block.state.properties.IntegerProperty|IntegerProperty]], `NoteBlockInstrument`, `PistonType`, `PotentSulfurState`, `Property`, `RailShape`, `RedstoneSide`, `RotationSegment`, `SculkSensorPhase`, `SideChainPart`, `SlabType`, `SpeleothemThickness`, `StairsShape`, `StructureMode`, `TestBlockMode`, `Tilt`, `WallSide`, [[40-Interfaces/net.minecraft.world.level.block.state.properties.WoodType|WoodType]], `package-info`

### `net.minecraft.world.level.blockscan` (7 top-level)

`BlockMatcher`, `BlockScanUtils`, `BlockStateConsumer`, `BoxBlockMatcher`, `FilteredSectionCache`, `OrderedBlockMatcher`, `package-info`

### `net.minecraft.world.level.border` (4 top-level)

`BorderChangeListener`, `BorderStatus`, `WorldBorder`, `package-info`

### `net.minecraft.world.level.chunk` (32 top-level)

`BlockColumn`, `BulkSectionAccess`, `CarverOutput`, `CarvingMask`, [[40-Interfaces/net.minecraft.world.level.chunk.ChunkAccess|ChunkAccess]], [[40-Interfaces/net.minecraft.world.level.chunk.ChunkGenerator|ChunkGenerator]], `ChunkGeneratorStructureState`, `ChunkGenerators`, `ChunkSource`, `Configuration`, `DataLayer`, `EmptyLevelChunk`, `GlobalPalette`, `HashMapPalette`, [[40-Interfaces/net.minecraft.world.level.chunk.ImposterProtoChunk|ImposterProtoChunk]], [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]], [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunkSection|LevelChunkSection]], `LightChunk`, `LightChunkGetter`, `LinearPalette`, `MissingPaletteEntryException`, `Palette`, `PaletteResize`, `PalettedContainer`, `PalettedContainerFactory`, `PalettedContainerRO`, `ProtoChunk`, `SingleValuePalette`, `Strategy`, `StructureAccess`, `UpgradeData`, `package-info`

### `net.minecraft.world.level.chunk.status` (9 top-level)

`ChunkDependencies`, `ChunkPyramid`, [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatus|ChunkStatus]], `ChunkStatusTask`, [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatusTasks|ChunkStatusTasks]], `ChunkStep`, `ChunkType`, [[40-Interfaces/net.minecraft.world.level.chunk.status.WorldGenContext|WorldGenContext]], `package-info`

### `net.minecraft.world.level.chunk.storage` (14 top-level)

`ChunkIOErrorReporter`, `ChunkScanAccess`, `EntityStorage`, `IOWorker`, `RecreatingSimpleRegionStorage`, `RegionBitmap`, `RegionFile`, `RegionFileStorage`, `RegionFileVersion`, `RegionStorageInfo`, `SectionStorage`, [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]], `SimpleRegionStorage`, `package-info`

### `net.minecraft.world.level.dimension` (5 top-level)

`BuiltinDimensionTypes`, `DimensionDefaults`, [[40-Interfaces/net.minecraft.world.level.dimension.DimensionType|DimensionType]], [[40-Interfaces/net.minecraft.world.level.dimension.LevelStem|LevelStem]], `package-info`

### `net.minecraft.world.level.dimension.end` (3 top-level)

`DragonRespawnStage`, `EnderDragonFight`, `package-info`

### `net.minecraft.world.level.entity` (19 top-level)

`ChunkEntities`, `ChunkStatusUpdateListener`, `EntityAccess`, `EntityInLevelCallback`, `EntityLookup`, `EntityPersistentStorage`, `EntitySection`, `EntitySectionStorage`, `EntityTickList`, `EntityTypeTest`, `LevelCallback`, `LevelEntityGetter`, `LevelEntityGetterAdapter`, [[40-Interfaces/net.minecraft.world.level.entity.PersistentEntitySectionManager|PersistentEntitySectionManager]], `TransientEntitySectionManager`, `UUIDLookup`, `UniquelyIdentifyable`, `Visibility`, `package-info`

### `net.minecraft.world.level.gameevent` (11 top-level)

`BlockPositionSource`, `DynamicGameEventListener`, `EntityPositionSource`, `EuclideanGameEventListenerRegistry`, [[40-Interfaces/net.minecraft.world.level.gameevent.GameEvent|GameEvent]], `GameEventDispatcher`, `GameEventListener`, `GameEventListenerRegistry`, `PositionSource`, `PositionSourceType`, `package-info`

### `net.minecraft.world.level.gameevent.vibrations` (4 top-level)

`VibrationInfo`, `VibrationSelector`, [[40-Interfaces/net.minecraft.world.level.gameevent.vibrations.VibrationSystem|VibrationSystem]], `package-info`

### `net.minecraft.world.level.gamerules` (7 top-level)

[[40-Interfaces/net.minecraft.world.level.gamerules.GameRule|GameRule]], [[40-Interfaces/net.minecraft.world.level.gamerules.GameRuleCategory|GameRuleCategory]], [[40-Interfaces/net.minecraft.world.level.gamerules.GameRuleMap|GameRuleMap]], [[40-Interfaces/net.minecraft.world.level.gamerules.GameRuleType|GameRuleType]], `GameRuleTypeVisitor`, [[40-Interfaces/net.minecraft.world.level.gamerules.GameRules|GameRules]], `package-info`

### `net.minecraft.world.level.levelgen` (41 top-level)

`Aquifer`, `Beardifier`, `BelowZeroRetrogen`, `BitRandomSource`, `Column`, `DebugLevelSource`, `Density`, `FlatLevelSource`, `GenerationStep`, `GeodeBlockSettings`, `GeodeCrackSettings`, `GeodeLayerSettings`, `Heightmap`, [[40-Interfaces/net.minecraft.world.level.levelgen.LegacyRandomSource|LegacyRandomSource]], `MarsagliaPolarGaussian`, `NoiseBasedChunkGenerator`, `NoiseChunk`, `NoiseGeneratorSettings`, `NoiseRouter`, `NoiseRouterData`, `NoiseSettings`, `NoiseSpawnFinder`, `Noises`, `OverworldFunctionSet`, `PatrolSpawner`, `PhantomSpawner`, `PositionalRandomFactory`, [[40-Interfaces/net.minecraft.world.level.levelgen.RandomState|RandomState]], `RandomSupport`, `SingleThreadedRandomSource`, `SpawnTargetPoint`, `ThreadSafeLegacyRandomSource`, `VerticalAnchor`, [[40-Interfaces/net.minecraft.world.level.levelgen.WorldDimensions|WorldDimensions]], [[40-Interfaces/net.minecraft.world.level.levelgen.WorldGenSettings|WorldGenSettings]], `WorldGenerationContext`, `WorldOptions`, [[40-Interfaces/net.minecraft.world.level.levelgen.WorldgenRandom|WorldgenRandom]], `Xoroshiro128PlusPlus`, `XoroshiroRandomSource`, `package-info`

### `net.minecraft.world.level.levelgen.blending` (3 top-level)

`Blender`, `BlendingData`, `package-info`

### `net.minecraft.world.level.levelgen.blockpredicates` (21 top-level)

`AllOfPredicate`, `AnyOfPredicate`, [[40-Interfaces/net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate|BlockPredicate]], `BlockPredicateType`, `CombiningPredicate`, `HasSturdyFacePredicate`, `HeightRangePredicate`, `InsideWorldBoundsPredicate`, `MatchingBiomesPredicate`, `MatchingBlockTagPredicate`, `MatchingBlocksPredicate`, `MatchingFluidsPredicate`, `NotPredicate`, `ReplaceablePredicate`, `SolidPredicate`, `StateTestingPredicate`, `TrueBlockPredicate`, `UnobstructedPredicate`, `VolumeMatchPredicate`, `WouldSurvivePredicate`, `package-info`

### `net.minecraft.world.level.levelgen.carver` (5 top-level)

`CanyonWorldCarver`, `CaveWorldCarver`, `WorldCarver`, `WorldCarverTypes`, `package-info`

### `net.minecraft.world.level.levelgen.densityfunction` (18 top-level)

`CacheId`, `CachingDensitySampler`, `ContextBoundSampler`, `DensityBuffer`, `DensityBufferArena`, `DensityBufferPool`, `DensityFunction`, `DensityFunctionCompiler`, `DensityFunctions`, `DensitySampler`, `DensitySamplerSet`, `DensityVolume`, `DfRewriteRule`, `DistanceMetric`, `SamplerContext`, `ScopedDensityBuffer`, `TilingMode`, `package-info`

### `net.minecraft.world.level.levelgen.densityfunction.generator` (8 top-level)

`ConstantFunction`, `DistanceToPointFunction`, `EndIslandFunction`, `GradientFunction`, `NoiseFunction`, `ShiftNoiseFunction`, `SimpleDensityFunction`, `package-info`

### `net.minecraft.world.level.levelgen.densityfunction.op` (15 top-level)

`BinaryFunction`, `BlendDensityFunction`, `CacheFunction`, `ClampFunction`, `FindTopSurfaceFunction`, `InterpolatedFunction`, `IntervalSelectFunction`, `LerpFunction`, `PowFunction`, `RangeChoiceFunction`, `RoundFunction`, `SliceFunction`, `SplineFunction`, `UnaryFunction`, `package-info`

### `net.minecraft.world.level.levelgen.feature` (68 top-level)

`AbstractHugeMushroomFeature`, `AbstractOreFeature`, `BambooFeature`, `BlockBlobFeature`, `BlockColumnFeature`, `BlockPileFeature`, `BlockReplacement`, `BlueIceFeature`, `BonusChestFeature`, `ChorusPlantFeature`, `CoralClawFeature`, `CoralTreeFeature`, `CuboidPlacement`, `DeltaFeature`, `DiskFeature`, `EndGatewayFeature`, `EndIslandFeature`, `EndPlatformFeature`, `EndPodiumFeature`, `EndSpikeFeature`, `FallenTreeFeature`, `Feature`, `FeatureCountTracker`, `FeatureTypes`, `FillLayerFeature`, `FossilFeature`, `GeodeFeature`, `HugeBrownMushroomFeature`, `HugeFungusFeature`, `HugeRedMushroomFeature`, `IcebergFeature`, `LakeFeature`, `LargeDripstoneFeature`, `MonsterRoomFeature`, `MultifaceGrowthFeature`, `NoOpFeature`, `OreFeature`, `OverlayFeature`, `ProjectedRandomPatchySquare`, `RandomBooleanSelectorFeature`, `RandomNeighborSpreadFeature`, `RandomSelectorFeature`, `ReplaceBlobsFeature`, `ReplaceBlockFeature`, `RootSystemFeature`, `ScatteredOreFeature`, `SculkPatchFeature`, `SequenceFeature`, `SimpleBlockFeature`, `SimpleRandomSelectorFeature`, `SingleBlockPillarFeature`, `SnowAndFreezeFeature`, `SpeleothemClusterFeature`, `SpeleothemFeature`, `SpeleothemUtils`, `SpikeFeature`, `SpringFeature`, `SteppedColumnClusterFeature`, `TemplateFeature`, `TreeFeature`, `UnderwaterMagmaFeature`, `VegetationPatchFeature`, `VinesFeature`, `VoidStartPlatformFeature`, `WaterloggedVegetationPatchFeature`, `WeightedPlacedFeature`, `WeightedRandomSelectorFeature`, `package-info`

### `net.minecraft.world.level.levelgen.feature.featuresize` (5 top-level)

`FeatureSize`, `FeatureSizeType`, `ThreeLayersFeatureSize`, `TwoLayersFeatureSize`, `package-info`

### `net.minecraft.world.level.levelgen.feature.foliageplacers` (15 top-level)

`AcaciaFoliagePlacer`, `BlobFoliagePlacer`, `BushFoliagePlacer`, `CherryFoliagePlacer`, `DarkOakFoliagePlacer`, `FancyFoliagePlacer`, `FoliagePlacer`, `FoliagePlacerType`, `MegaJungleFoliagePlacer`, `MegaPineFoliagePlacer`, `PineFoliagePlacer`, `PoplarFoliagePlacer`, `RandomSpreadFoliagePlacer`, `SpruceFoliagePlacer`, `package-info`

### `net.minecraft.world.level.levelgen.feature.rootplacers` (6 top-level)

`AboveRootPlacement`, `MangroveRootPlacement`, `MangroveRootPlacer`, `RootPlacer`, `RootPlacerType`, `package-info`

### `net.minecraft.world.level.levelgen.feature.stateproviders` (14 top-level)

[[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider|BlockStateProvider]], `BlockStateProviderTypes`, [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider|CopyPropertiesProvider]], `DualNoiseProvider`, `NoiseBasedStateProvider`, `NoiseProvider`, `NoiseThresholdProvider`, `RandomBlockProvider`, `RandomizedIntStateProvider`, `RotatedBlockProvider`, [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider|RuleBasedStateProvider]], `SimpleStateProvider`, `WeightedStateProvider`, `package-info`

### `net.minecraft.world.level.levelgen.feature.treedecorators` (14 top-level)

`AlterGroundDecorator`, `AttachedToLeavesDecorator`, `AttachedToLogsDecorator`, `BeehiveDecorator`, `CocoaDecorator`, `CreakingHeartDecorator`, `LeaveVineDecorator`, `PaleMossDecorator`, `PlaceOnGroundDecorator`, `ShelfMushroomDecorator`, `TreeDecorator`, `TreeDecoratorType`, `TrunkVineDecorator`, `package-info`

### `net.minecraft.world.level.levelgen.feature.trunkplacers` (13 top-level)

`BendingTrunkPlacer`, `CherryTrunkPlacer`, `DarkOakTrunkPlacer`, `FancyTrunkPlacer`, `ForkingTrunkPlacer`, `GiantTrunkPlacer`, `MegaJungleTrunkPlacer`, `PoplarTrunkPlacer`, `StraightTrunkPlacer`, `TrunkPlacer`, `TrunkPlacerType`, `UpwardsBranchingTrunkPlacer`, `package-info`

### `net.minecraft.world.level.levelgen.flat` (5 top-level)

`FlatLayerInfo`, `FlatLevelGeneratorPreset`, `FlatLevelGeneratorPresets`, `FlatLevelGeneratorSettings`, `package-info`

### `net.minecraft.world.level.levelgen.heightproviders` (9 top-level)

`BiasedToBottomHeight`, `ConstantHeight`, `HeightProvider`, `HeightProviderType`, `TrapezoidHeight`, `UniformHeight`, `VeryBiasedToBottomHeight`, `WeightedListHeight`, `package-info`

### `net.minecraft.world.level.levelgen.material` (4 top-level)

`MaterialRuleContext`, `MaterialRules`, `MaterialSystem`, `package-info`

### `net.minecraft.world.level.levelgen.material.condition` (14 top-level)

`AbovePreliminarySurfaceCondition`, `BiomeCondition`, `ConditionEvaluator`, `HoleCondition`, `MaterialCondition`, `NoiseThresholdCondition`, `NotCondition`, `SteepCondition`, `StoneDepthCondition`, `TemperatureCondition`, `VerticalGradientCondition`, `WaterCondition`, `YCondition`, `package-info`

### `net.minecraft.world.level.levelgen.material.rule` (8 top-level)

`BandlandsRule`, `BlockRule`, `ConditionRule`, `MaterialRule`, `OreVeinRule`, `RuleEvaluator`, `SequenceRule`, `package-info`

### `net.minecraft.world.level.levelgen.placement` (26 top-level)

`BiomeFilter`, `BlockPredicateFilter`, `CaveSurface`, `CountOnEveryLayerPlacement`, `CountPlacement`, `EnvironmentScanPlacement`, `FeaturePlacer`, `FixedPlacement`, `HeightRangePlacement`, `HeightmapPlacement`, `InSquarePlacement`, `NoiseBasedCountPlacement`, `NoiseThresholdCountPlacement`, `OffsetPlacement`, [[40-Interfaces/net.minecraft.world.level.levelgen.placement.PlacedFeature|PlacedFeature]], `PlacementContext`, `PlacementFilter`, `PlacementModifier`, `PlacementModifierTypes`, `RandomChancePlacement`, `RandomlySelectedPlacement`, `RarityFilter`, `RepeatingPlacement`, `SurfaceRelativeThresholdFilter`, `SurfaceWaterDepthFilter`, `package-info`

### `net.minecraft.world.level.levelgen.presets` (3 top-level)

`WorldPreset`, [[40-Interfaces/net.minecraft.world.level.levelgen.presets.WorldPresets|WorldPresets]], `package-info`

### `net.minecraft.world.level.levelgen.structure` (18 top-level)

`BoundingBox`, `BuiltinStructureSets`, `BuiltinStructures`, `PoolElementStructurePiece`, `PostPlacementProcessor`, `ScatteredFeaturePiece`, `SinglePieceStructure`, [[40-Interfaces/net.minecraft.world.level.levelgen.structure.Structure|Structure]], `StructureCheck`, `StructureCheckResult`, `StructurePiece`, `StructureSet`, `StructureSpawnOverride`, `StructureStart`, `StructureType`, `TemplateStructurePiece`, `TerrainAdjustment`, `package-info`

### `net.minecraft.world.level.levelgen.structure.pieces` (5 top-level)

`PiecesContainer`, `StructurePieceSerializationContext`, `StructurePieceType`, `StructurePiecesBuilder`, `package-info`

### `net.minecraft.world.level.levelgen.structure.placement` (8 top-level)

`AbstractSpreadingStructurePlacement`, `ConcentricRingsStructurePlacement`, `DimensionOriginStructurePlacement`, `RandomSpreadStructurePlacement`, `RandomSpreadType`, `StructurePlacement`, `StructurePlacements`, `package-info`

### `net.minecraft.world.level.levelgen.structure.pools` (12 top-level)

`DimensionPadding`, `EmptyPoolElement`, `FeaturePoolElement`, `JigsawJunction`, `JigsawPlacement`, `LegacySinglePoolElement`, `ListPoolElement`, `SinglePoolElement`, `StructurePoolElement`, `StructurePoolElementType`, `StructureTemplatePool`, `package-info`

### `net.minecraft.world.level.levelgen.structure.pools.alias` (7 top-level)

`DirectPoolAlias`, `PoolAliasBinding`, `PoolAliasBindings`, `PoolAliasLookup`, `RandomGroupPoolAlias`, `RandomPoolAlias`, `package-info`

### `net.minecraft.world.level.levelgen.structure.structures` (32 top-level)

`BuriedTreasurePieces`, `BuriedTreasureStructure`, `DesertPyramidPiece`, `DesertPyramidStructure`, `EndCityPieces`, `EndCityStructure`, `IglooPieces`, `IglooStructure`, `JigsawStructure`, `JungleTemplePiece`, `JungleTempleStructure`, `MineshaftPieces`, `MineshaftStructure`, `NetherFortressPieces`, `NetherFortressStructure`, `NetherFossilPieces`, `NetherFossilStructure`, `OceanMonumentPieces`, `OceanMonumentStructure`, `OceanRuinPieces`, `OceanRuinStructure`, `RuinedPortalPiece`, `RuinedPortalStructure`, `ShipwreckPieces`, `ShipwreckStructure`, `StrongholdPieces`, `StrongholdStructure`, `SwampHutPiece`, `SwampHutStructure`, `WoodlandMansionPieces`, `WoodlandMansionStructure`, `package-info`

### `net.minecraft.world.level.levelgen.structure.templatesystem` (38 top-level)

`AllOfRuleTest`, `AlwaysTrueTest`, `AnyOfRuleTest`, `AxisAlignedLinearPosTest`, `BlackstoneReplaceProcessor`, `BlockAgeProcessor`, `BlockIgnoreProcessor`, `BlockMatchTest`, `BlockRotProcessor`, `BlockStateMatchTest`, `CappedProcessor`, `GravityProcessor`, `HeightMatchTest`, `JigsawReplacementProcessor`, `LavaSubmergedBlockProcessor`, `LinearPosTest`, `LiquidSettings`, `NopProcessor`, `NotRuleTest`, `PosAlwaysTrueTest`, `PosRuleTest`, `PosRuleTestType`, `ProcessorRule`, `ProtectedBlockProcessor`, `RandomBlockMatchTest`, `RandomBlockStateMatchTest`, `RuleProcessor`, `RuleTest`, `RuleTestType`, `StructurePlaceSettings`, `StructureProcessor`, `StructureProcessorList`, `StructureProcessorType`, `StructureProcessorTypes`, [[40-Interfaces/net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplate|StructureTemplate]], [[40-Interfaces/net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager|StructureTemplateManager]], `TagMatchTest`, `package-info`

### `net.minecraft.world.level.levelgen.structure.templatesystem.loader` (5 top-level)

`DirectoryTemplateSource`, `ResourceManagerTemplateSource`, `TemplatePathFactory`, [[40-Interfaces/net.minecraft.world.level.levelgen.structure.templatesystem.loader.TemplateSource|TemplateSource]], `package-info`

### `net.minecraft.world.level.levelgen.structure.templatesystem.rule.blockentity` (7 top-level)

`AppendLoot`, `AppendStatic`, `Clear`, `Passthrough`, `RuleBlockEntityModifier`, `RuleBlockEntityModifierType`, `package-info`

### `net.minecraft.world.level.levelgen.synth` (11 top-level)

`BlendedNoise`, `GradientNoise`, `LegacyFbmInitializer`, `Noise`, `NoiseStack`, `NoiseUtils`, `NormalNoise`, [[40-Interfaces/net.minecraft.world.level.levelgen.synth.PerlinNoise|PerlinNoise]], `SimplexNoise`, `SmearedPerlinNoise`, `package-info`

### `net.minecraft.world.level.lighting` (15 top-level)

`BlockLightEngine`, `BlockLightSectionStorage`, `ChunkSkyLightSources`, `DataLayerStorageMap`, `DynamicGraphMinFixedPoint`, `LayerLightEventListener`, `LayerLightSectionStorage`, `LevelLightEngine`, `LeveledPriorityQueue`, `LightEngine`, `LightEventListener`, `SkyLightEngine`, `SkyLightSectionStorage`, `SpatialLongSet`, `package-info`

### `net.minecraft.world.level.material` (12 top-level)

`EmptyFluid`, [[40-Interfaces/net.minecraft.world.level.material.FlowingFluid|FlowingFluid]], [[40-Interfaces/net.minecraft.world.level.material.Fluid|Fluid]], `FluidIds`, [[40-Interfaces/net.minecraft.world.level.material.FluidState|FluidState]], [[40-Interfaces/net.minecraft.world.level.material.Fluids|Fluids]], `FogType`, [[40-Interfaces/net.minecraft.world.level.material.LavaFluid|LavaFluid]], `MapColor`, `PushReaction`, `WaterFluid`, `package-info`

### `net.minecraft.world.level.pathfinder` (15 top-level)

`AmphibiousNodeEvaluator`, `BinaryHeap`, `FlyNodeEvaluator`, `Node`, `NodeEvaluator`, `Path`, `PathComputationType`, `PathFinder`, `PathType`, `PathTypeCache`, [[40-Interfaces/net.minecraft.world.level.pathfinder.PathfindingContext|PathfindingContext]], `SwimNodeEvaluator`, `Target`, [[40-Interfaces/net.minecraft.world.level.pathfinder.WalkNodeEvaluator|WalkNodeEvaluator]], `package-info`

### `net.minecraft.world.level.portal` (4 top-level)

`PortalForcer`, `PortalShape`, `TeleportTransition`, `package-info`

### `net.minecraft.world.level.redstone` (10 top-level)

`CollectingNeighborUpdater`, `DefaultRedstoneWireEvaluator`, `ExperimentalRedstoneUtils`, `ExperimentalRedstoneWireEvaluator`, `InstantNeighborUpdater`, `NeighborUpdater`, `Orientation`, `Redstone`, `RedstoneWireEvaluator`, `package-info`

### `net.minecraft.world.level.saveddata` (5 top-level)

[[40-Interfaces/net.minecraft.world.level.saveddata.SavedData|SavedData]], [[40-Interfaces/net.minecraft.world.level.saveddata.SavedDataType|SavedDataType]], `WanderingTraderData`, `WeatherData`, `package-info`

### `net.minecraft.world.level.saveddata.maps` (9 top-level)

`MapBanner`, `MapDecoration`, `MapDecorationType`, `MapDecorationTypes`, `MapFrame`, `MapId`, `MapIndex`, `MapItemSavedData`, `package-info`

### `net.minecraft.world.level.storage` (23 top-level)

`CommandStorage`, `DataVersion`, `DerivedLevelData`, `FileNameDateFormatter`, `LevelData`, `LevelDataAndDimensions`, [[40-Interfaces/net.minecraft.world.level.storage.LevelResource|LevelResource]], `LevelStorageException`, [[40-Interfaces/net.minecraft.world.level.storage.LevelStorageSource|LevelStorageSource]], `LevelSummary`, `LevelVersion`, `PlayerDataStorage`, [[40-Interfaces/net.minecraft.world.level.storage.PrimaryLevelData|PrimaryLevelData]], [[40-Interfaces/net.minecraft.world.level.storage.SavedDataStorage|SavedDataStorage]], `ServerLevelData`, [[40-Interfaces/net.minecraft.world.level.storage.TagValueInput|TagValueInput]], [[40-Interfaces/net.minecraft.world.level.storage.TagValueOutput|TagValueOutput]], [[40-Interfaces/net.minecraft.world.level.storage.ValueInput|ValueInput]], `ValueInputContextHelper`, [[40-Interfaces/net.minecraft.world.level.storage.ValueOutput|ValueOutput]], `WorldData`, `WritableLevelData`, `package-info`

### `net.minecraft.world.level.storage.loot` (17 top-level)

`BuiltInLootTables`, `ContainerComponentManipulator`, `ContainerComponentManipulators`, `FloatRangePredicate`, `IntLimit`, `IntRangePredicate`, [[40-Interfaces/net.minecraft.world.level.storage.loot.LootContext|LootContext]], `LootContextArg`, `LootContextUser`, `LootDataType`, `LootParams`, [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool|LootPool]], [[40-Interfaces/net.minecraft.world.level.storage.loot.LootTable|LootTable]], `Validatable`, `ValidationContext`, `ValidationContextSource`, `package-info`

### `net.minecraft.world.level.storage.loot.entries` (18 top-level)

`AlternativesEntry`, `ComposableEntryContainer`, `CompositeEntryBase`, `DynamicLoot`, `EmptyLootItem`, `EntryGroup`, `ExpandableContainerBase`, `LootItem`, `LootPoolEntries`, `LootPoolEntry`, `LootPoolEntryContainer`, `NestedLootTable`, `SequentialEntry`, `SingleEntryContainerBase`, `SlotLoot`, `TagEntry`, `UniformContainerBase`, `package-info`

### `net.minecraft.world.level.storage.loot.functions` (48 top-level)

`ApplyBonusCount`, `ApplyExplosionDecay`, `CopyBlockState`, `CopyComponentsFunction`, `CopyCustomDataFunction`, `CopyNameFunction`, `DiscardItem`, [[40-Interfaces/net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction|EnchantRandomlyFunction]], `EnchantWithLevelsFunction`, `EnchantedCountIncreaseFunction`, `ExplorationMapFunction`, `FillPlayerHead`, `FilteredFunction`, `FunctionUserBuilder`, `LimitCount`, `ListOperation`, `LootItemConditionalFunction`, `LootItemFunction`, `LootItemFunctions`, `ModifyContainerContents`, `SequenceFunction`, `SetAttributesFunction`, `SetBannerPatternFunction`, `SetBookCoverFunction`, `SetComponentsFunction`, `SetContainerContents`, `SetContainerLootTable`, `SetCustomDataFunction`, `SetCustomModelDataFunction`, `SetEnchantmentsFunction`, `SetFireworkExplosionFunction`, `SetFireworksFunction`, `SetInstrumentFunction`, `SetItemCountFunction`, `SetItemDamageFunction`, `SetItemFunction`, `SetLoreFunction`, `SetNameFunction`, `SetOminousBottleAmplifierFunction`, `SetPotionFunction`, `SetRandomDyesFunction`, `SetRandomPotionFunction`, `SetStewEffectFunction`, `SetWritableBookPagesFunction`, `SetWrittenBookPagesFunction`, `SmeltItemFunction`, `ToggleTooltips`, `package-info`

### `net.minecraft.world.level.storage.loot.parameters` (3 top-level)

[[40-Interfaces/net.minecraft.world.level.storage.loot.parameters.LootContextParamSets|LootContextParamSets]], [[40-Interfaces/net.minecraft.world.level.storage.loot.parameters.LootContextParams|LootContextParams]], `package-info`

### `net.minecraft.world.level.storage.loot.predicates` (26 top-level)

`AllOfCondition`, `AnyOfCondition`, `BonusLevelTableCondition`, `CompositeLootItemCondition`, `ConditionUserBuilder`, `DamageSourceCondition`, `EnchantmentActiveCheck`, `EntityHasScoreCondition`, `EnvironmentAttributeCheck`, `ExplosionCondition`, `FloatValueCheck`, `IntValueCheck`, `InvertedLootItemCondition`, `LocationCheck`, `LootItemCondition`, `LootItemConditionTypes`, `LootItemEntityPropertyCondition`, `LootItemKilledByPlayerCondition`, `LootItemRandomChanceCondition`, `LootItemRandomChanceWithEnchantedBonusCondition`, `LootPredicates`, `MatchBlock`, `MatchTool`, `TimeCheck`, `WeatherCheck`, `package-info`

### `net.minecraft.world.level.storage.loot.providers.nbt` (5 top-level)

`ContextNbtProvider`, `NbtProvider`, `NbtProviders`, `StorageNbtProvider`, `package-info`

### `net.minecraft.world.level.storage.loot.providers.number` (11 top-level)

`AggregateProvider`, `BinaryProvider`, `ConditionalProvider`, `DispatcherProvider`, `DistributionProvider`, `EnvironmentAttributeProvider`, `PowerProvider`, `RangeProvider`, `StoredNumberAccess`, `UnaryProvider`, `package-info`

### `net.minecraft.world.level.storage.loot.providers.number.floats` (33 top-level)

`Absolute`, `Average`, `Ceiling`, `ConditionalValue`, `ConstantValue`, `ContextFloatProvider`, `ContextFloatProviderTypes`, `ContextFloatProviders`, `Cosine`, `Difference`, `EnchantmentLevelProvider`, `EnvironmentAttributeValue`, `Floor`, `FromInt`, `Length`, `Maximum`, `Minimum`, `Modulus`, `Negate`, `NumberDispatcher`, `Power`, `Product`, `Quotient`, `ResolvableFloat`, `Round`, `Sine`, `SquareRoot`, `StorageValue`, `Sum`, `Truncate`, `UniformGenerator`, `WeightedListValue`, `package-info`

### `net.minecraft.world.level.storage.loot.providers.number.ints` (28 top-level)

`Absolute`, `Average`, `BinomialDistributionGenerator`, `ConditionalValue`, `ConstantValue`, `ContextIntProvider`, `ContextIntProviderTypes`, `ContextIntProviders`, `Difference`, `EnvironmentAttributeValue`, `FloorModulus`, `FloorQuotient`, `FromFloat`, `Maximum`, `Minimum`, `Modulus`, `Negate`, `NumberDispatcher`, `Power`, `Product`, `Quotient`, [[40-Interfaces/net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt|ResolvableInt]], `ScoreboardValue`, `StorageValue`, `Sum`, `UniformGenerator`, `WeightedListValue`, `package-info`

### `net.minecraft.world.level.storage.loot.providers.score` (5 top-level)

`ContextScoreboardNameProvider`, `FixedScoreboardNameProvider`, `ScoreboardNameProvider`, `ScoreboardNameProviders`, `package-info`

### `net.minecraft.world.level.timers` (6 top-level)

`FunctionCallback`, `FunctionTagCallback`, `TimerCallback`, `TimerCallbacks`, `TimerQueue`, `package-info`

### `net.minecraft.world.level.validation` (5 top-level)

`ContentValidationException`, [[40-Interfaces/net.minecraft.world.level.validation.DirectoryValidator|DirectoryValidator]], `ForbiddenSymlinkInfo`, `PathAllowList`, `package-info`

