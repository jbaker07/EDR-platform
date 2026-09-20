---
type: "system"
package: "net.minecraft.world.level"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level

Analyst note: [[_authored/systems/net.minecraft.world.level|Level, blocks, block entities, chunks and world generation]]

2176 classes in the jar. Hooked types: 106

- [[40-Interfaces/net.minecraft.world.level.CardinalLighting|CardinalLighting]] -- calls:2 -- by fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.world.level.ChunkPos|ChunkPos]] -- calls:5 -- by fabric-data-attachment-api-v1, fabric-lifecycle-events-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.world.level.CollisionGetter|CollisionGetter]] -- calls:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.level.DataPackConfig|DataPackConfig]] -- calls:3 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.world.level.GameType|GameType]] -- reads:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.world.level.ItemLike|ItemLike]] -- calls:3 -- by fabric-api-lookup-api-v1, fabric-data-generation-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.world.level.Level|Level]] -- calls:29 -- by fabric-api-lookup-api-v1, fabric-block-api-v1, fabric-data-attachment-api-v1, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-permission-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.LevelAccessor|LevelAccessor]] -- calls:2 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.WorldDataConfiguration|WorldDataConfiguration]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.world.level.biome.BiomeGenerationSettings|BiomeGenerationSettings]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.BiomeSource|BiomeSource]] -- calls:2, wraps:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.Climate_ParameterList|Climate$ParameterList]] -- calls:2 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.FeatureSorter|FeatureSorter]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList_Preset|MultiNoiseBiomeSourceParameterList$Preset]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList_Preset_1|MultiNoiseBiomeSourceParameterList$Preset$1]] -- injects_into:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]] -- injects_into:5 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.AbstractBedBlock|AbstractBedBlock]] -- calls:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.level.block.BedBlock|BedBlock]] -- reads:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.level.block.Block|Block]] -- calls:12, reads:1 -- by fabric-api-lookup-api-v1, fabric-block-api-v1, fabric-content-registries-v0, fabric-entity-events-v1, fabric-model-loading-api-v1, fabric-registry-sync-v0, fabric-renderer-indigo, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] -- injects_into:1, reads:8 -- by fabric-block-api-v1, fabric-entity-events-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.level.block.ChestBlock|ChestBlock]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.ColorCollection|ColorCollection]] -- calls:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.level.block.CrafterBlock|CrafterBlock]] -- injects_into:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.DetectorRailBlock|DetectorRailBlock]] -- injects_into:1, reads:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.DispenserBlock|DispenserBlock]] -- reads:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.DropperBlock|DropperBlock]] -- injects_into:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.FireBlock|FireBlock]] -- injects_into:3 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.LadderBlock|LadderBlock]] -- reads:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.LiquidBlock|LiquidBlock]] -- injects_into:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.TrapDoorBlock|TrapDoorBlock]] -- reads:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopper|WeatheringCopper]] -- injects_into:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopperCollection|WeatheringCopperCollection]] -- calls:2 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.WeatheringCopperCollection_ByState|WeatheringCopperCollection$ByState]] -- calls:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity|AbstractFurnaceBlockEntity]] -- injects_into:2, wraps:1 -- by fabric-item-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.BaseContainerBlockEntity|BaseContainerBlockEntity]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntity|BlockEntity]] -- calls:10, injects_into:2 -- by fabric-api-lookup-api-v1, fabric-block-getter-api-v2, fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntityType|BlockEntityType]] -- calls:3, injects_into:1 -- by fabric-api-lookup-api-v1, fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.BrewingStandBlockEntity|BrewingStandBlockEntity]] -- wraps:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.ChestBlockEntity|ChestBlockEntity]] -- calls:3 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity|ChiseledBookShelfBlockEntity]] -- injects_into:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.DispenserBlockEntity|DispenserBlockEntity]] -- calls:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.Hopper|Hopper]] -- calls:3 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.HopperBlockEntity|HopperBlockEntity]] -- injects_into:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.JukeboxBlockEntity|JukeboxBlockEntity]] -- injects_into:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity|ShulkerBoxBlockEntity]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.state.BlockBehaviour_BlockStateBase|BlockBehaviour$BlockStateBase]] -- injects_into:2 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.world.level.block.state.BlockState|BlockState]] -- calls:41 -- by fabric-api-lookup-api-v1, fabric-block-api-v1, fabric-content-registries-v0, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-object-builder-api-v1, fabric-particles-v1, fabric-registry-sync-v0, fabric-renderer-api-v1, fabric-renderer-indigo, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.block.state.StateDefinition|StateDefinition]] -- calls:5 -- by fabric-content-registries-v0, fabric-model-loading-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.level.block.state.StateHolder|StateHolder]] -- calls:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.state.properties.BlockStateProperties|BlockStateProperties]] -- reads:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.block.state.properties.WoodType|WoodType]] -- calls:2 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.ChunkAccess|ChunkAccess]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.ChunkGenerator|ChunkGenerator]] -- calls:3 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]] -- calls:7, injects_into:5, wraps:2 -- by fabric-block-getter-api-v2, fabric-data-attachment-api-v1, fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunkSection|LevelChunkSection]] -- wraps:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunkSection_1BlockCounter|LevelChunkSection$1BlockCounter]] -- wraps:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatus|ChunkStatus]] -- reads:2 -- by fabric-data-attachment-api-v1, fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatusTasks|ChunkStatusTasks]] -- injects_into:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.status.WorldGenContext|WorldGenContext]] -- calls:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]] -- injects_into:4, wraps:1 -- by fabric-data-attachment-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.level.dimension.DimensionType|DimensionType]] -- calls:1 -- by fabric-dimensions-v1
- [[40-Interfaces/net.minecraft.world.level.dimension.LevelStem|LevelStem]] -- calls:2, reads:1 -- by fabric-biome-api-v1, fabric-dimensions-v1
- [[40-Interfaces/net.minecraft.world.level.entity.PersistentEntitySectionManager|PersistentEntitySectionManager]] -- injects_into:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.level.gameevent.GameEvent_Context|GameEvent$Context]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.gamerules.GameRule|GameRule]] -- calls:8 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.world.level.gamerules.GameRuleMap|GameRuleMap]] -- reads:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.gamerules.GameRules|GameRules]] -- calls:6 -- by fabric-client-gametest-api-v1, fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.LegacyRandomSource|LegacyRandomSource]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.RandomState|RandomState]] -- injects_into:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.WorldDimensions|WorldDimensions]] -- injects_into:1, wraps:1 -- by fabric-dimensions-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.WorldGenSettings|WorldGenSettings]] -- reads:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.WorldgenRandom|WorldgenRandom]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.blockpredicates.BlockPredicate|BlockPredicate]] -- calls:3 -- by fabric-content-registries-v0, fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.BlockStateProvider|BlockStateProvider]] -- calls:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.CopyPropertiesProvider|CopyPropertiesProvider]] -- calls:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider|RuleBasedStateProvider]] -- calls:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.feature.stateproviders.RuleBasedStateProvider_Builder|RuleBasedStateProvider$Builder]] -- calls:2 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.structure.Structure|Structure]] -- calls:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager|StructureTemplateManager]] -- injects_into:1 -- by fabric-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.levelgen.synth.PerlinNoise|PerlinNoise]] -- calls:2 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.level.material.FlowingFluid|FlowingFluid]] -- calls:1, injects_into:1 -- by fabric-block-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.material.Fluid|Fluid]] -- calls:6, injects_into:1, reads:1 -- by fabric-block-api-v1, fabric-registry-sync-v0, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.material.FluidState|FluidState]] -- calls:4 -- by fabric-block-api-v1, fabric-content-registries-v0, fabric-rendering-fluids-v1
- [[40-Interfaces/net.minecraft.world.level.material.Fluids|Fluids]] -- reads:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.level.material.LavaFluid|LavaFluid]] -- injects_into:1 -- by fabric-block-api-v1
- [[40-Interfaces/net.minecraft.world.level.pathfinder.PathfindingContext|PathfindingContext]] -- injects_into:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.pathfinder.WalkNodeEvaluator|WalkNodeEvaluator]] -- injects_into:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.level.saveddata.SavedData|SavedData]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.saveddata.SavedDataType|SavedDataType]] -- calls:2 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.LevelData_RespawnData|LevelData$RespawnData]] -- calls:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.level.storage.LevelStorageSource|LevelStorageSource]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.LevelStorageSource_LevelStorageAccess|LevelStorageSource$LevelStorageAccess]] -- calls:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.world.level.storage.PrimaryLevelData|PrimaryLevelData]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.SavedDataStorage|SavedDataStorage]] -- calls:6 -- by fabric-client-gametest-api-v1, fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.TagValueInput|TagValueInput]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.TagValueOutput|TagValueOutput]] -- calls:2 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.ValueInput|ValueInput]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.ValueOutput|ValueOutput]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootContext|LootContext]] -- calls:1 -- by fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootContext_Builder|LootContext$Builder]] -- calls:2 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootParams_Builder|LootParams$Builder]] -- calls:3 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootPool_Builder|LootPool$Builder]] -- calls:3 -- by fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.world.level.storage.loot.LootTable_Builder|LootTable$Builder]] -- calls:4 -- by fabric-data-generation-api-v1, fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.world.level.storage.loot.functions.EnchantRandomlyFunction|EnchantRandomlyFunction]] -- wraps:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.world.level.storage.loot.providers.number.ints.ResolvableInt|ResolvableInt]] -- calls:1 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.level.validation.DirectoryValidator|DirectoryValidator]] -- calls:1 -- by fabric-resource-loader-v1
