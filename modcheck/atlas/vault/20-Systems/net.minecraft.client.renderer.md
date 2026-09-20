---
type: "system"
package: "net.minecraft.client.renderer"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer

Analyst note: [[_authored/systems/net.minecraft.client.renderer|Rendering: level, entities, block and item models]]

1006 classes (715 top-level) across 39 packages in the processed jar; 51 changed by Loom processing; 87 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.client.renderer.BiomeColors|BiomeColors]] -- reads:3 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.GameRenderer|GameRenderer]] -- calls:11, injects_into:2, reads:2 -- by fabric-client-gametest-api-v1, fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] -- calls:1, injects_into:12, reads:3, wraps:4 -- by fabric-client-gametest-api-v1, fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.OrderedSubmitNodeCollector|OrderedSubmitNodeCollector]] -- calls:9 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.Panorama|Panorama]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.Rect2i|Rect2i]] -- calls:18 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]] -- reads:4 -- by fabric-creative-tab-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.renderer.Sheets|Sheets]] -- calls:31 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.SubmitNodeCollection|SubmitNodeCollection]] -- calls:1, reads:19, wraps:1 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.SubmitNodeCollector|SubmitNodeCollector]] -- calls:5 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockAndTintGetter|BlockAndTintGetter]] -- calls:17, reads:3 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelLighter|BlockModelLighter]] -- calls:2 -- by fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelLighter_Cache|BlockModelLighter$Cache]] -- calls:20 -- by fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]] -- calls:4, injects_into:5, reads:9 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockStateModelSet|BlockStateModelSet]] -- calls:4 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.FluidModel|FluidModel]] -- calls:3 -- by fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.FluidModel_Unbaked|FluidModel$Unbaked]] -- calls:1 -- by fabric-rendering-fluids-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.FluidRenderer|FluidRenderer]] -- calls:1, injects_into:2 -- by fabric-rendering-fluids-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.FluidStateModelSet|FluidStateModelSet]] -- calls:1, wraps:1 -- by fabric-rendering-fluids-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.ModelBlockRenderer|ModelBlockRenderer]] -- calls:2, injects_into:1, reads:2 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.MovingBlockRenderState|MovingBlockRenderState]] -- reads:4 -- by fabric-block-getter-api-v2, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockModelRotation|BlockModelRotation]] -- reads:2 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel|BlockStateModel]] -- calls:35 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel_Unbaked|BlockStateModel$Unbaked]] -- calls:2, reads:1, wraps:2 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel_UnbakedRoot|BlockStateModel$UnbakedRoot]] -- calls:3 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModelPart|BlockStateModelPart]] -- calls:7 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.ModelState|ModelState]] -- calls:14 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.SingleVariant|SingleVariant]] -- calls:1, reads:1 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.Variant|Variant]] -- reads:1 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.WeightedVariants|WeightedVariants]] -- reads:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.WeightedVariants_Unbaked|WeightedVariants$Unbaked]] -- calls:2 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel|MultiPartModel]] -- reads:3 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel_Selector|MultiPartModel$Selector]] -- calls:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel_SharedBakedState|MultiPartModel$SharedBakedState]] -- calls:3 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.model.BlockStateModelWrapper|BlockStateModelWrapper]] -- calls:1, reads:2, replaces:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.blockentity.BlockEntityRenderers|BlockEntityRenderers]] -- injects_into:1, reads:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.chunk.ChunkSectionLayer|ChunkSectionLayer]] -- calls:9, reads:8 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.chunk.RenderRegionCache|RenderRegionCache]] -- injects_into:2 -- by fabric-block-getter-api-v2
- [[40-Interfaces/net.minecraft.client.renderer.chunk.RenderSectionRegion|RenderSectionRegion]] -- reads:1 -- by fabric-block-getter-api-v2
- [[40-Interfaces/net.minecraft.client.renderer.chunk.SectionCompiler|SectionCompiler]] -- calls:1, injects_into:1, reads:2, wraps:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.debug.DebugRenderer|DebugRenderer]] -- injects_into:1, reads:1 -- by fabric-debug-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.ArmorModelSet|ArmorModelSet]] -- calls:2 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderDispatcher|EntityRenderDispatcher]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRendererProvider|EntityRendererProvider]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderers|EntityRenderers]] -- injects_into:1, reads:1, wraps:2 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.HumanoidMobRenderer|HumanoidMobRenderer]] -- wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.LivingEntityRenderer|LivingEntityRenderer]] -- wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.layers.CapeLayer|CapeLayer]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer|HumanoidArmorLayer]] -- injects_into:2 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.layers.RenderLayer|RenderLayer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]] -- injects_into:3, reads:4, wraps:3 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureFrameContext|FeatureFrameContext]] -- calls:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureRenderDispatcher|FeatureRenderDispatcher]] -- injects_into:1, reads:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureRendererMap|FeatureRendererMap]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureRendererType|FeatureRendererType]] -- calls:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer|MovingBlockFeatureRenderer]] -- injects_into:1, reads:1, wraps:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer_Submit|MovingBlockFeatureRenderer$Submit]] -- calls:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer|RenderTypeFeatureRenderer]] -- calls:3 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.feature.phase.FeatureRenderPhase|FeatureRenderPhase]] -- calls:3 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase|SimpleFeatureRenderPhase]] -- calls:5 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.submit.TranslucentSubmit|TranslucentSubmit]] -- calls:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.item.CuboidItemModelWrapper|CuboidItemModelWrapper]] -- injects_into:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.item.ItemModel|ItemModel]] -- calls:1 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.item.ItemModel_Unbaked|ItemModel$Unbaked]] -- calls:3 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState|ItemStackRenderState]] -- injects_into:3 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_FoilType|ItemStackRenderState$FoilType]] -- calls:7, reads:6 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_LayerRenderState|ItemStackRenderState$LayerRenderState]] -- calls:1, injects_into:2, reads:1, wraps:1 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.rendertype.RenderType|RenderType]] -- calls:8 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.rendertype.RenderTypes|RenderTypes]] -- calls:3 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.OptionsRenderState|OptionsRenderState]] -- reads:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.gui.GuiRenderState|GuiRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState|PictureInPictureRenderState]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.BlockBreakingRenderState|BlockBreakingRenderState]] -- calls:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.CameraRenderState|CameraRenderState]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState|FirstPersonHandsAndItemsRenderState]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.LevelRenderState|LevelRenderState]] -- injects_into:1, reads:2 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.ParticlesRenderState|ParticlesRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.PlayerRenderState|PlayerRenderState]] -- injects_into:1, reads:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.SkyRenderState|SkyRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.WeatherRenderState|WeatherRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.WorldBorderRenderState|WorldBorderRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.texture.OverlayTexture|OverlayTexture]] -- reads:5 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.texture.SpriteContents|SpriteContents]] -- calls:7 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.texture.SpriteLoader_Preparations|SpriteLoader$Preparations]] -- calls:3, reads:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.texture.TextureAtlas|TextureAtlas]] -- injects_into:1, reads:8 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.texture.TextureAtlasSprite|TextureAtlasSprite]] -- calls:46 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1, fabric-renderer-indigo

## Declared inventory

### `net.minecraft.client.renderer` (56 top-level)

`BindGroupLayouts`, [[40-Interfaces/net.minecraft.client.renderer.BiomeColors|BiomeColors]], `CloudRenderer`, `CubeMap`, `DebugCrosshairRenderer`, `DynamicGpuData`, `DynamicGpuDataStorage`, `DynamicGpuDataStorageMapped`, `DynamicGpuDataStorageNonMapped`, `EndFlashState`, `FaceInfo`, `FirstPersonHandsAndItemsRenderer`, [[40-Interfaces/net.minecraft.client.renderer.GameRenderer|GameRenderer]], `GlobalSettingsUniform`, `GpuWarnlistManager`, `LevelEventHandler`, [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]], `LevelTargetBundle`, `Lightmap`, `LightmapRenderStateExtractor`, `MapRenderer`, `MappableRingBuffer`, `MultiblockChestResources`, `Octree`, [[40-Interfaces/net.minecraft.client.renderer.OrderedSubmitNodeCollector|OrderedSubmitNodeCollector]], [[40-Interfaces/net.minecraft.client.renderer.Panorama|Panorama]], `PlayerSkinRenderCache`, `PostChain`, `PostChainConfig`, `PostPass`, `Projection`, `ProjectionMatrixBuffer`, [[40-Interfaces/net.minecraft.client.renderer.Rect2i|Rect2i]], `RenderBuffers`, [[40-Interfaces/net.minecraft.client.renderer.RenderPipelines|RenderPipelines]], `RunningTrimmedMean`, `ScreenEffectRenderer`, `SectionBufferBuilderPack`, `SectionBufferBuilderPool`, `SectionOcclusionGraph`, `ShaderDefines`, `ShaderManager`, [[40-Interfaces/net.minecraft.client.renderer.Sheets|Sheets]], `SkyRenderer`, `SpriteCoordinateExpander`, `SpriteMapper`, `StagedVertexBuffer`, [[40-Interfaces/net.minecraft.client.renderer.SubmitNodeCollection|SubmitNodeCollection]], [[40-Interfaces/net.minecraft.client.renderer.SubmitNodeCollector|SubmitNodeCollector]], `SubmitNodeStorage`, `UiLightmap`, `UniformValue`, `ViewArea`, `WeatherEffectRenderer`, `WorldBorderRenderer`, `package-info`

### `net.minecraft.client.renderer.block` (16 top-level)

[[40-Interfaces/net.minecraft.client.renderer.block.BlockAndTintGetter|BlockAndTintGetter]], [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelLighter|BlockModelLighter]], [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]], `BlockModelResolver`, `BlockModelSet`, `BlockQuadOutput`, [[40-Interfaces/net.minecraft.client.renderer.block.BlockStateModelSet|BlockStateModelSet]], [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]], [[40-Interfaces/net.minecraft.client.renderer.block.FluidModel|FluidModel]], [[40-Interfaces/net.minecraft.client.renderer.block.FluidRenderer|FluidRenderer]], [[40-Interfaces/net.minecraft.client.renderer.block.FluidStateModelSet|FluidStateModelSet]], `LoadedBlockModels`, [[40-Interfaces/net.minecraft.client.renderer.block.ModelBlockRenderer|ModelBlockRenderer]], [[40-Interfaces/net.minecraft.client.renderer.block.MovingBlockRenderState|MovingBlockRenderState]], `SelectBlockModel`, `package-info`

### `net.minecraft.client.renderer.block.dispatch` (11 top-level)

[[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockModelRotation|BlockModelRotation]], [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel|BlockStateModel]], `BlockStateModelDispatcher`, [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModelPart|BlockStateModelPart]], [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.ModelState|ModelState]], [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.SingleVariant|SingleVariant]], [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.Variant|Variant]], `VariantMutator`, `VariantSelector`, [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.WeightedVariants|WeightedVariants]], `package-info`

### `net.minecraft.client.renderer.block.dispatch.multipart` (6 top-level)

`CombinedCondition`, `Condition`, `KeyValueCondition`, [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel|MultiPartModel]], `Selector`, `package-info`

### `net.minecraft.client.renderer.block.model` (8 top-level)

`BlockDisplayContext`, `BlockModel`, [[40-Interfaces/net.minecraft.client.renderer.block.model.BlockStateModelWrapper|BlockStateModelWrapper]], `CompositeBlockModel`, `ConditionalBlockModel`, `EmptyBlockModel`, `SpecialBlockModelWrapper`, `package-info`

### `net.minecraft.client.renderer.block.model.properties` (1 top-level)

`package-info`

### `net.minecraft.client.renderer.block.model.properties.conditional` (3 top-level)

`ConditionalBlockModelProperty`, `IsXmas`, `package-info`

### `net.minecraft.client.renderer.block.model.properties.select` (3 top-level)

`DisplayContext`, `SelectBlockModelProperty`, `package-info`

### `net.minecraft.client.renderer.blockentity` (33 top-level)

`AbstractEndPortalRenderer`, `AbstractSignRenderer`, `BannerRenderer`, `BeaconRenderer`, `BellRenderer`, `BlockEntityRenderDispatcher`, `BlockEntityRenderer`, `BlockEntityRendererProvider`, [[40-Interfaces/net.minecraft.client.renderer.blockentity.BlockEntityRenderers|BlockEntityRenderers]], `BlockEntityWithBoundingBoxRenderer`, `BrightnessCombiner`, `BrushableBlockRenderer`, `CampfireRenderer`, `ChestRenderer`, `ConduitRenderer`, `CopperGolemStatueBlockRenderer`, `DecoratedPotRenderer`, `EnchantTableRenderer`, `HangingSignRenderer`, `LecternRenderer`, `PistonHeadRenderer`, `ShelfRenderer`, `ShulkerBoxRenderer`, `SkullBlockRenderer`, `SpawnerRenderer`, `StandingSignRenderer`, `TestInstanceRenderer`, `TheEndGatewayRenderer`, `TheEndPortalRenderer`, `TrialSpawnerRenderer`, `VaultRenderer`, `WallAndGroundTransformations`, `package-info`

### `net.minecraft.client.renderer.blockentity.state` (27 top-level)

`BannerRenderState`, `BeaconRenderState`, `BedRenderState`, `BellRenderState`, `BlockEntityRenderState`, `BlockEntityWithBoundingBoxRenderState`, `BrushableBlockRenderState`, `CampfireRenderState`, `ChestRenderState`, `ConduitRenderState`, `CopperGolemStatueRenderState`, `DecoratedPotRenderState`, `EnchantTableRenderState`, `EndGatewayRenderState`, `EndPortalRenderState`, `HangingSignRenderState`, `LecternRenderState`, `PistonHeadRenderState`, `ShelfRenderState`, `ShulkerBoxRenderState`, `SignRenderState`, `SkullBlockRenderState`, `SpawnerRenderState`, `StandingSignRenderState`, `TestInstanceRenderState`, `VaultRenderState`, `package-info`

### `net.minecraft.client.renderer.chunk` (15 top-level)

[[40-Interfaces/net.minecraft.client.renderer.chunk.ChunkSectionLayer|ChunkSectionLayer]], `ChunkSectionLayerGroup`, `ChunkSectionsToRender`, `CompiledSectionMesh`, [[40-Interfaces/net.minecraft.client.renderer.chunk.RenderRegionCache|RenderRegionCache]], [[40-Interfaces/net.minecraft.client.renderer.chunk.RenderSectionRegion|RenderSectionRegion]], [[40-Interfaces/net.minecraft.client.renderer.chunk.SectionCompiler|SectionCompiler]], `SectionCopy`, `SectionMesh`, `SectionRenderDispatcher`, `SectionTaskDynamicQueue`, `TranslucencyPointOfView`, `VisGraph`, `VisibilitySet`, `package-info`

### `net.minecraft.client.renderer.culling` (2 top-level)

`Frustum`, `package-info`

### `net.minecraft.client.renderer.debug` (28 top-level)

`BeeDebugRenderer`, `BrainDebugRenderer`, `BreezeDebugRenderer`, `ChunkBorderRenderer`, `ChunkCullingDebugRenderer`, `ChunkDebugRenderer`, `CollisionBoxRenderer`, [[40-Interfaces/net.minecraft.client.renderer.debug.DebugRenderer|DebugRenderer]], `EntityBlockIntersectionDebugRenderer`, `EntityHitboxDebugRenderer`, `GameEventListenerRenderer`, `GameTestBlockHighlightRenderer`, `GoalSelectorDebugRenderer`, `HeightMapRenderer`, `LightDebugRenderer`, `LightSectionDebugRenderer`, `NeighborsUpdateRenderer`, `OctreeDebugRenderer`, `PathfindingRenderer`, `PoiDebugRenderer`, `RaidDebugRenderer`, `RedstoneWireOrientationsRenderer`, `SolidFaceRenderer`, `StructureRenderer`, `SupportBlockRenderer`, `VillageSectionsDebugRenderer`, `WaterDebugRenderer`, `package-info`

### `net.minecraft.client.renderer.entity` (136 top-level)

`AbstractBoatRenderer`, `AbstractCubeMobRenderer`, `AbstractHoglinRenderer`, `AbstractHorseRenderer`, `AbstractMinecartRenderer`, `AbstractSkeletonRenderer`, `AbstractZombieRenderer`, `AgeableMobRenderer`, `AllayRenderer`, `ArmadilloRenderer`, [[40-Interfaces/net.minecraft.client.renderer.entity.ArmorModelSet|ArmorModelSet]], `ArmorStandRenderer`, `ArrowRenderer`, `AxolotlRenderer`, `BatRenderer`, `BeeRenderer`, `BlazeRenderer`, `BoatRenderer`, `BoggedRenderer`, `BreezeRenderer`, `CamelHuskRenderer`, `CamelRenderer`, `CatRenderer`, `CaveSpiderRenderer`, `ChickenRenderer`, `CodRenderer`, `CopperGolemRenderer`, `CowRenderer`, `CreakingRenderer`, `CreeperRenderer`, `CushionRenderer`, `DisplayRenderer`, `DolphinRenderer`, `DonkeyRenderer`, `DragonFireballRenderer`, `DrownedRenderer`, `ElderGuardianRenderer`, `EndCrystalRenderer`, `EnderDragonRenderer`, `EndermanRenderer`, `EndermiteRenderer`, [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderDispatcher|EntityRenderDispatcher]], `EntityRenderer`, [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRendererProvider|EntityRendererProvider]], [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderers|EntityRenderers]], `EvokerFangsRenderer`, `EvokerRenderer`, `ExperienceOrbRenderer`, `FallingBlockRenderer`, `FireworkEntityRenderer`, `FishingHookRenderer`, `FoxRenderer`, `FrogRenderer`, `GhastRenderer`, `GiantMobRenderer`, `GlowSquidRenderer`, `GoatRenderer`, `GuardianRenderer`, `HappyGhastRenderer`, `HoglinRenderer`, `HorseRenderer`, [[40-Interfaces/net.minecraft.client.renderer.entity.HumanoidMobRenderer|HumanoidMobRenderer]], `HuskRenderer`, `IllagerRenderer`, `IllusionerRenderer`, `IronGolemRenderer`, `ItemEntityRenderer`, `ItemFrameRenderer`, `LeashKnotRenderer`, `LightningBoltRenderer`, [[40-Interfaces/net.minecraft.client.renderer.entity.LivingEntityRenderer|LivingEntityRenderer]], `LlamaRenderer`, `LlamaSpitRenderer`, `MagmaCubeRenderer`, `MinecartRenderer`, `MobRenderer`, `MushroomCowRenderer`, `NautilusRenderer`, `NoopRenderer`, `OcelotRenderer`, `OminousItemSpawnerRenderer`, `PaintingRenderer`, `PandaRenderer`, `ParchedRenderer`, `ParrotRenderer`, `PhantomRenderer`, `PigRenderer`, `PiglinRenderer`, `PillagerRenderer`, `PolarBearRenderer`, `PufferfishRenderer`, `RabbitRenderer`, `RaftRenderer`, `RavagerRenderer`, `RenderLayerParent`, `SalmonRenderer`, `SheepRenderer`, `ShulkerBulletRenderer`, `ShulkerRenderer`, `SilverfishRenderer`, `SkeletonRenderer`, `SlimeRenderer`, `SnifferRenderer`, `SnowGolemRenderer`, `SpectralArrowRenderer`, `SpiderRenderer`, `SquidRenderer`, `StrayRenderer`, `StriderRenderer`, `SulfurCubeRenderer`, `TadpoleRenderer`, `ThrownItemRenderer`, `ThrownTridentRenderer`, `TippableArrowRenderer`, `TntMinecartRenderer`, `TntRenderer`, `TropicalFishRenderer`, `TurtleRenderer`, `UndeadHorseRenderer`, `VexRenderer`, `VillagerRenderer`, `VindicatorRenderer`, `WanderingTraderRenderer`, `WardenRenderer`, `WindChargeRenderer`, `WitchRenderer`, `WitherBossRenderer`, `WitherSkeletonRenderer`, `WitherSkullRenderer`, `WolfRenderer`, `ZoglinRenderer`, `ZombieNautilusRenderer`, `ZombieRenderer`, `ZombieVillagerRenderer`, `ZombifiedPiglinRenderer`, `package-info`

### `net.minecraft.client.renderer.entity.layers` (51 top-level)

`ArrowLayer`, `BeeStingerLayer`, `BlockDecorationLayer`, `BreezeEyesLayer`, `BreezeWindLayer`, [[40-Interfaces/net.minecraft.client.renderer.entity.layers.CapeLayer|CapeLayer]], `CarriedBlockLayer`, `CatCollarLayer`, `CreeperPowerLayer`, `CrossedArmsItemLayer`, `CustomHeadLayer`, `Deadmau5EarsLayer`, `DolphinCarryingItemLayer`, `DrownedOuterLayer`, `EnderEyesLayer`, `EnergySwirlLayer`, `EquipmentLayerRenderer`, `EyesLayer`, `FoxHeldItemLayer`, `HorseMarkingLayer`, [[40-Interfaces/net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer|HumanoidArmorLayer]], `IronGolemCrackinessLayer`, `IronGolemFlowerLayer`, `ItemInHandLayer`, `LivingEntityEmissiveLayer`, `LlamaDecorLayer`, `MushroomCowMushroomLayer`, `PandaHoldsItemLayer`, `ParrotOnShoulderLayer`, `PhantomEyesLayer`, `PlayerItemInHandLayer`, [[40-Interfaces/net.minecraft.client.renderer.entity.layers.RenderLayer|RenderLayer]], `RopesLayer`, `SheepWoolLayer`, `SheepWoolUndercoatLayer`, `SimpleEquipmentLayer`, `SkeletonClothingLayer`, `SlimeOuterLayer`, `SnowGolemHeadLayer`, `SpiderEyesLayer`, `SpinAttackEffectLayer`, `StuckInBodyLayer`, `SulfurCubeInnerLayer`, `TropicalFishPatternLayer`, `VillagerProfessionLayer`, `WingsLayer`, `WitchItemLayer`, `WitherArmorLayer`, `WolfArmorLayer`, `WolfCollarLayer`, `package-info`

### `net.minecraft.client.renderer.entity.player` (2 top-level)

`AvatarRenderer`, `package-info`

### `net.minecraft.client.renderer.entity.state` (103 top-level)

`AllayRenderState`, `ArmadilloRenderState`, `ArmedEntityRenderState`, `ArmorStandRenderState`, `ArrowRenderState`, `AvatarRenderState`, `AxolotlRenderState`, `BatRenderState`, `BeeRenderState`, `BlockDisplayEntityRenderState`, `BoatRenderState`, `BoggedRenderState`, `BreezeRenderState`, `CamelRenderState`, `CatRenderState`, `ChickenRenderState`, `CopperGolemRenderState`, `CowRenderState`, `CreakingRenderState`, `CreeperRenderState`, `CushionRenderState`, `DisplayEntityRenderState`, `DolphinRenderState`, `DonkeyRenderState`, `EndCrystalRenderState`, `EnderDragonRenderState`, `EndermanRenderState`, `EntityRenderState`, `EquineRenderState`, `EvokerFangsRenderState`, `EvokerRenderState`, `ExperienceOrbRenderState`, `FallingBlockRenderState`, `FelineRenderState`, `FireworkRocketRenderState`, `FishingHookRenderState`, `FoxRenderState`, `FrogRenderState`, `GhastRenderState`, `GoatRenderState`, `GuardianRenderState`, `HappyGhastRenderState`, `HitboxRenderState`, `HoglinRenderState`, `HoldingEntityRenderState`, `HorseRenderState`, `HumanoidRenderState`, `IllagerRenderState`, `IllusionerRenderState`, `IronGolemRenderState`, `ItemClusterRenderState`, `ItemDisplayEntityRenderState`, `ItemEntityRenderState`, `ItemFrameRenderState`, `LightningBoltRenderState`, `LivingEntityRenderState`, `LlamaRenderState`, `LlamaSpitRenderState`, `MinecartRenderState`, `MinecartTntRenderState`, `MushroomCowRenderState`, `NautilusRenderState`, `PaintingRenderState`, `PandaRenderState`, `ParrotRenderState`, `PhantomRenderState`, `PigRenderState`, `PiglinRenderState`, `PolarBearRenderState`, `PufferfishRenderState`, `RabbitRenderState`, `RavagerRenderState`, `SalmonRenderState`, `SheepRenderState`, `ShulkerBulletRenderState`, `ShulkerRenderState`, `SkeletonRenderState`, `SlimeRenderState`, `SnifferRenderState`, `SnowGolemRenderState`, `SquidRenderState`, `StriderRenderState`, `SulfurCubeRenderState`, `TextDisplayEntityRenderState`, `ThrownItemRenderState`, `ThrownTridentRenderState`, `TippableArrowRenderState`, `TntRenderState`, `TropicalFishRenderState`, `TurtleRenderState`, `UndeadRenderState`, `VexRenderState`, `VillagerDataHolderRenderState`, `VillagerRenderState`, `WardenRenderState`, `WitchRenderState`, `WitherRenderState`, `WitherSkullRenderState`, `WolfRenderState`, `ZombieRenderState`, `ZombieVillagerRenderState`, `ZombifiedPiglinRenderState`, `package-info`

### `net.minecraft.client.renderer.extract` (3 top-level)

[[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]], `TransientBlock`, `package-info`

### `net.minecraft.client.renderer.feature` (19 top-level)

`BlockModelFeatureRenderer`, `CustomFeatureRenderer`, [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureFrameContext|FeatureFrameContext]], [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureRenderDispatcher|FeatureRenderDispatcher]], `FeatureRenderer`, [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureRendererMap|FeatureRendererMap]], [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureRendererType|FeatureRendererType]], `FlameFeatureRenderer`, `GizmoFeatureRenderer`, `ItemFeatureRenderer`, `LeashFeatureRenderer`, `ModelFeatureRenderer`, [[40-Interfaces/net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer|MovingBlockFeatureRenderer]], `QuadParticleFeatureRenderer`, [[40-Interfaces/net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer|RenderTypeFeatureRenderer]], `ShadowFeatureRenderer`, `ShapeOutlineFeatureRenderer`, `TextFeatureRenderer`, `package-info`

### `net.minecraft.client.renderer.feature.phase` (4 top-level)

[[40-Interfaces/net.minecraft.client.renderer.feature.phase.FeatureRenderPhase|FeatureRenderPhase]], [[40-Interfaces/net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase|SimpleFeatureRenderPhase]], `TranslucentFeatureRenderPhase`, `package-info`

### `net.minecraft.client.renderer.feature.submit` (4 top-level)

`BatchableSubmit`, `SubmitNode`, [[40-Interfaces/net.minecraft.client.renderer.feature.submit.TranslucentSubmit|TranslucentSubmit]], `package-info`

### `net.minecraft.client.renderer.fog` (3 top-level)

`FogData`, `FogRenderer`, `package-info`

### `net.minecraft.client.renderer.fog.environment` (9 top-level)

`AtmosphericFogEnvironment`, `BlindnessFogEnvironment`, `DarknessFogEnvironment`, `FogEnvironment`, `LavaFogEnvironment`, `MobEffectFogEnvironment`, `PowderedSnowFogEnvironment`, `WaterFogEnvironment`, `package-info`

### `net.minecraft.client.renderer.gizmos` (2 top-level)

`DrawableGizmoPrimitives`, `package-info`

### `net.minecraft.client.renderer.item` (17 top-level)

`BundleSelectedItemSpecialRenderer`, `ClientItem`, `CompositeModel`, `ConditionalItemModel`, [[40-Interfaces/net.minecraft.client.renderer.item.CuboidItemModelWrapper|CuboidItemModelWrapper]], `EmptyModel`, [[40-Interfaces/net.minecraft.client.renderer.item.ItemModel|ItemModel]], `ItemModelResolver`, `ItemModels`, [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState|ItemStackRenderState]], `MissingItemModel`, `ModelRenderProperties`, `RangeSelectItemModel`, `SelectItemModel`, `SpecialModelWrapper`, `TrackingItemStackRenderState`, `package-info`

### `net.minecraft.client.renderer.item.properties` (1 top-level)

`package-info`

### `net.minecraft.client.renderer.item.properties.conditional` (17 top-level)

`Broken`, `BundleHasSelectedItem`, `ComponentMatches`, `ConditionalItemModelProperties`, `ConditionalItemModelProperty`, `CustomModelDataProperty`, `Damaged`, `ExtendedView`, `FishingRodCast`, `HasComponent`, `IsCarried`, `IsKeybindDown`, `IsSelected`, `IsUsingItem`, `IsViewEntity`, `ItemModelPropertyTest`, `package-info`

### `net.minecraft.client.renderer.item.properties.numeric` (15 top-level)

`BundleFullness`, `CompassAngle`, `CompassAngleState`, `Cooldown`, `Count`, `CrossbowPull`, `CustomModelDataProperty`, `Damage`, `NeedleDirectionHelper`, `RangeSelectItemModelProperties`, `RangeSelectItemModelProperty`, `Time`, `UseCycle`, `UseDuration`, `package-info`

### `net.minecraft.client.renderer.item.properties.select` (13 top-level)

`Charge`, `ComponentContents`, `ContextDimension`, `ContextEntityType`, `CustomModelDataProperty`, `DisplayContext`, `ItemBlockState`, `LocalTime`, `MainHand`, `SelectItemModelProperties`, `SelectItemModelProperty`, `TrimMaterialProperty`, `package-info`

### `net.minecraft.client.renderer.oit` (4 top-level)

`OitPipelineSet`, `OitRenderPassProvider`, `OitStage`, `package-info`

### `net.minecraft.client.renderer.rendertype` (7 top-level)

`LayeringTransform`, `PreparedRenderType`, `RenderSetup`, [[40-Interfaces/net.minecraft.client.renderer.rendertype.RenderType|RenderType]], [[40-Interfaces/net.minecraft.client.renderer.rendertype.RenderTypes|RenderTypes]], `TextureTransform`, `package-info`

### `net.minecraft.client.renderer.special` (17 top-level)

`BannerSpecialRenderer`, `BellSpecialRenderer`, `BookSpecialRenderer`, `ChestSpecialRenderer`, `ConduitSpecialRenderer`, `CopperGolemStatueSpecialRenderer`, `DecoratedPotSpecialRenderer`, `EndCubeSpecialRenderer`, `NoDataSpecialModelRenderer`, `PlayerHeadSpecialRenderer`, `ShieldSpecialRenderer`, `ShulkerBoxSpecialRenderer`, `SkullSpecialRenderer`, `SpecialModelRenderer`, `SpecialModelRenderers`, `TridentSpecialRenderer`, `package-info`

### `net.minecraft.client.renderer.state` (6 top-level)

`GameRenderState`, `LightmapRenderState`, `MapRenderState`, [[40-Interfaces/net.minecraft.client.renderer.state.OptionsRenderState|OptionsRenderState]], `WindowRenderState`, `package-info`

### `net.minecraft.client.renderer.state.gui` (11 top-level)

`BlitRenderState`, `ColoredRectangleRenderState`, `GlyphRenderState`, `GuiElementRenderState`, `GuiItemRenderState`, [[40-Interfaces/net.minecraft.client.renderer.state.gui.GuiRenderState|GuiRenderState]], `GuiTextRenderState`, `PanoramaRenderState`, `ScreenArea`, `TiledBlitRenderState`, `package-info`

### `net.minecraft.client.renderer.state.gui.pip` (8 top-level)

`GuiBannerResultRenderState`, `GuiBookModelRenderState`, `GuiEntityRenderState`, `GuiProfilerChartRenderState`, `GuiSkinRenderState`, `OversizedItemRenderState`, [[40-Interfaces/net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState|PictureInPictureRenderState]], `package-info`

### `net.minecraft.client.renderer.state.level` (17 top-level)

[[40-Interfaces/net.minecraft.client.renderer.state.level.BlockBreakingRenderState|BlockBreakingRenderState]], `BlockOutlineRenderState`, `CameraEntityRenderState`, [[40-Interfaces/net.minecraft.client.renderer.state.level.CameraRenderState|CameraRenderState]], `ChunkLoadingRenderState`, [[40-Interfaces/net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState|FirstPersonHandsAndItemsRenderState]], [[40-Interfaces/net.minecraft.client.renderer.state.level.LevelRenderState|LevelRenderState]], `ParticleGroupRenderState`, [[40-Interfaces/net.minecraft.client.renderer.state.level.ParticlesRenderState|ParticlesRenderState]], [[40-Interfaces/net.minecraft.client.renderer.state.level.PlayerRenderState|PlayerRenderState]], `QuadParticleRenderState`, `SectionUpdateRenderState`, [[40-Interfaces/net.minecraft.client.renderer.state.level.SkyRenderState|SkyRenderState]], `TransientBlockRenderState`, [[40-Interfaces/net.minecraft.client.renderer.state.level.WeatherRenderState|WeatherRenderState]], [[40-Interfaces/net.minecraft.client.renderer.state.level.WorldBorderRenderState|WorldBorderRenderState]], `package-info`

### `net.minecraft.client.renderer.texture` (25 top-level)

`AbstractTexture`, `CubeMapTexture`, `Dumpable`, `DynamicAtlasTree`, `DynamicAtlasTreeSlot`, `DynamicTexture`, `MipmapGenerator`, `MipmapStrategy`, `MipmappedTexture`, `MissingTextureAtlasSprite`, [[40-Interfaces/net.minecraft.client.renderer.texture.OverlayTexture|OverlayTexture]], `ReloadableTexture`, `SimpleTexture`, `SkinTextureDownloader`, [[40-Interfaces/net.minecraft.client.renderer.texture.SpriteContents|SpriteContents]], `SpriteLoader`, `Stitcher`, `StitcherException`, [[40-Interfaces/net.minecraft.client.renderer.texture.TextureAtlas|TextureAtlas]], [[40-Interfaces/net.minecraft.client.renderer.texture.TextureAtlasSprite|TextureAtlasSprite]], `TextureContents`, `TextureManager`, `TickableTexture`, `UvMapping`, `package-info`

### `net.minecraft.client.renderer.texture.atlas` (5 top-level)

`SpriteResourceLoader`, `SpriteSource`, `SpriteSourceList`, `SpriteSources`, `package-info`

### `net.minecraft.client.renderer.texture.atlas.sources` (7 top-level)

`DirectoryLister`, `LazyLoadedImage`, `PalettedPermutations`, `SingleFile`, `SourceFilter`, `Unstitcher`, `package-info`

