---
type: "mechanism"
module: "fabric-renderer-api-v1"
version: "17.0.15+79385d0b5d"
sha256: "2e4aaeb20f8615e8176f30bc82616cee25f8727323bfef7a70d5a6555ae8e9ee"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-renderer-api-v1

**Version** `17.0.15+79385d0b5d` -- **artifact sha256** `2e4aaeb20f8615e8176f30bc82616cee25f8727323bfef7a70d5a6555ae8e9ee`

## Declared (fabric.mod.json)

- environment: `client`
- depends: `{"fabricloader": ">=0.19.3", "minecraft": ">=26.1-rc.2", "fabric-api-base": "*", "fabric-rendering-v1": "*", "fabric-transitive-access-wideners-v1": "*"}`
- entrypoints: `{"client": ["net.fabricmc.fabric.impl.client.renderer.DebugOverlayClient"]}`
- mixin configs: `["fabric-renderer-api-v1.mixins.json"]`
- access widener: `fabric-renderer-api-v1.classtweaker`
- mixin classes: 26 found by annotation, 26 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.particle.BlockMarker|BlockMarker]].`<init>` | `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDLnet/minecraft/world/level/block/state/BlockState;)V` | exact | @Redirect | INVOKE `Lnet/minecraft/client/renderer/block/BlockStateModelSet;getParticleMaterial(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;` (exact) | client | 1000 (default) | `BlockMarkerMixin.getParticleMaterialProxy` |
| [[40-Interfaces/net.minecraft.client.particle.TerrainParticle|TerrainParticle]].`<init>` | `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V` | exact | @Redirect | INVOKE `Lnet/minecraft/client/renderer/block/BlockStateModelSet;getParticleMaterial(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;` (exact) | client | 1000 (default) | `TerrainParticleMixin.getParticleIconProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]].`submitBlockDestroyAnimation` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V` | exact | @Inject | INVOKE `Lnet/minecraft/util/RandomSource;createThreadLocalInstance()Lnet/minecraft/util/RandomSource;` (exact) | client | 1000 (default) | `LevelRendererMixin.beforeCreateRandom` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]].`submitBlockDestroyAnimation` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V` | exact | @Redirect | INVOKE `Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;collectParts(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V` (exact) | client | 1000 (default) | `LevelRendererMixin.cancelCollectParts` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]].`submitBlockDestroyAnimation` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V` | exact | @Redirect | INVOKE `Lnet/minecraft/client/renderer/SubmitNodeCollector;submitBreakingBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/List;IZ)V` (inherited_exact) | client | 1000 (default) | `LevelRendererMixin.submitBreakingBlockModelProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.SubmitNodeCollection|SubmitNodeCollection]].`submitMovingBlock` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/block/MovingBlockRenderState;I)V` | name_only | @Redirect | INVOKE `Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;hasMaterialFlag(I)Z` (exact) | client | 1000 (default) | `SubmitNodeCollectionMixin.hasMaterialFlagProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]].`clear` | `()V` | exact | @Inject | RETURN | client | 1000 (default) | `BlockModelRenderStateMixin.onReturnClear` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]].`isEmpty` | `()Z` | exact | @ModifyReturnValue | RETURN | client | 1000 (default) | `BlockModelRenderStateMixin.modifyIsEmpty` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]].`setupModel` | `(Lorg/joml/Matrix4fc;Z)Ljava/util/List;` | name_only | @Inject | RETURN | client | 1000 (default) | `BlockModelRenderStateMixin.onReturnSetupModel` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]].`submitModel` | `(Lnet/minecraft/client/renderer/rendertype/RenderType;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V` | name_only | @Inject | HEAD | client | 1000 (default) | `BlockModelRenderStateMixin.submitMesh` |
| [[40-Interfaces/net.minecraft.client.renderer.block.model.BlockStateModelWrapper|BlockStateModelWrapper]].`update` | `(Lnet/minecraft/client/renderer/block/BlockModelRenderState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/model/BlockDisplayContext;J)V` | exact | @Overwrite | - | client | 1000 (default) | `BlockStateModelWrapperMixin.update` |
| [[40-Interfaces/net.minecraft.client.renderer.chunk.SectionCompiler|SectionCompiler]].`compile` | `(Lnet/minecraft/core/SectionPos;Lnet/minecraft/client/renderer/chunk/RenderSectionRegion;Lcom/mojang/blaze3d/vertex/VertexSorting;Lnet/minecraft/client/renderer/SectionBufferBuilderPack;)Lnet/minecraft/client/renderer/chunk/SectionCompiler$Results;` | name_only | @Inject | INVOKE `Lnet/minecraft/core/BlockPos;betweenClosed(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;)Ljava/lang/Iterable;` (exact) | client | 1000 (default) | `SectionCompilerMixin.beforeLoopCompile` |
| [[40-Interfaces/net.minecraft.client.renderer.chunk.SectionCompiler|SectionCompiler]].`compile` | `(Lnet/minecraft/core/SectionPos;Lnet/minecraft/client/renderer/chunk/RenderSectionRegion;Lcom/mojang/blaze3d/vertex/VertexSorting;Lnet/minecraft/client/renderer/SectionBufferBuilderPack;)Lnet/minecraft/client/renderer/chunk/SectionCompiler$Results;` | name_only | @Redirect | INVOKE `net/minecraft/client/renderer/block/ModelBlockRenderer.tesselateBlock(Lnet/minecraft/client/renderer/block/BlockQuadOutput;FFFLnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;J)V` (exact) | client | 1000 (default) | `SectionCompilerMixin.tesselateBlockProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]].`extractBlockOutline` | `(Lnet/minecraft/client/Camera;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V` | name_only | @Redirect | INVOKE `Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;hasMaterialFlag(I)Z` (exact) | client | 1000 (default) | `LevelExtractorMixin.hasMaterialFlagProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]].`extractPlayerState` | `(Lnet/minecraft/client/Camera;Lnet/minecraft/client/DeltaTracker;FLnet/minecraft/client/renderer/state/level/PlayerRenderState;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/renderer/block/BlockStateModelSet;getParticleMaterial(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;` (exact) | client | 1000 (default) | `LevelExtractorMixin.getParticleMaterialProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]].`getViewBlockingState` | `(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/client/renderer/culling/Frustum;)Lnet/minecraft/world/level/block/state/BlockState;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/level/block/state/BlockState;isViewBlocking(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/AABB;)Z` (inherited_exact) | client | 1000 (default) | `LevelExtractorMixin.captureViewBlockingPosition` |
| [[40-Interfaces/net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer|MovingBlockFeatureRenderer]].`buildGroup` | `(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;Ljava/util/List;)V` | name_only | @Inject | INVOKE `net/minecraft/client/renderer/block/ModelBlockRenderer.<init>(ZZLnet/minecraft/client/color/block/BlockColors;)V` (exact) | client | 1000 (default) | `MovingBlockFeatureRendererMixin.beforeInitBlockRenderer` |
| [[40-Interfaces/net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer|MovingBlockFeatureRenderer]].`buildGroup` | `(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;Ljava/util/List;)V` | name_only | @Redirect | INVOKE `net/minecraft/client/renderer/block/ModelBlockRenderer.tesselateBlock(Lnet/minecraft/client/renderer/block/BlockQuadOutput;FFFLnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;J)V` (exact) | client | 1000 (default) | `MovingBlockFeatureRendererMixin.tesselateBlockProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.item.CuboidItemModelWrapper|CuboidItemModelWrapper]].`<init>` | `(Ljava/util/List;Lnet/minecraft/client/resources/model/geometry/QuadCollection;Lnet/minecraft/client/renderer/item/ModelRenderProperties;Lorg/joml/Matrix4fc;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `CuboidItemModelWrapperMixin.onReturnInit` |
| [[40-Interfaces/net.minecraft.client.renderer.item.CuboidItemModelWrapper|CuboidItemModelWrapper]].`update` | `(Lnet/minecraft/client/renderer/item/ItemStackRenderState;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/client/renderer/item/ItemModelResolver;Lnet/minecraft/world/item/ItemDisplayContext;Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/world/entity/ItemOwner;I)V` | name_only | @Inject | RETURN | client | 1000 (default) | `CuboidItemModelWrapperMixin.onReturnUpdate` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState|ItemStackRenderState]].`visitExtents` | `(Ljava/util/function/Consumer;)V` | exact | @Inject | NEW `com/mojang/blaze3d/vertex/PoseStack$Pose` (exact) | client | 1000 (default) | `ItemStackRenderStateMixin.afterInitVecLoad` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState|ItemStackRenderState]].`visitExtents` | `(Ljava/util/function/Consumer;)V` | exact | @Inject | INVOKE `Lcom/mojang/blaze3d/vertex/PoseStack$Pose;setIdentity()V` (exact) | client | 1000 (default) | `ItemStackRenderStateMixin.afterLayerLoad` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_LayerRenderState|ItemStackRenderState$LayerRenderState]].`clear` | `()V` | exact | @Inject | RETURN | client | 1000 (default) | `ItemStackRenderStateLayerRenderStateMixin.onReturnClear` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_LayerRenderState|ItemStackRenderState$LayerRenderState]].`submit` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V` | name_only | @Redirect | INVOKE `Lnet/minecraft/client/renderer/SubmitNodeCollector;submitItem(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/item/ItemDisplayContext;III[ILnet/minecraft/client/resources/model/geometry/ItemQuads;Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;)V` (inherited_exact) | client | 1000 (default) | `ItemStackRenderStateLayerRenderStateMixin.submitItemProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.texture.TextureAtlas|TextureAtlas]].`upload` | `(Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `TextureAtlasMixin.uploadHook` |
| [[40-Interfaces/net.minecraft.client.resources.model.SimpleModelWrapper|SimpleModelWrapper]].`bake` | `(Lnet/minecraft/client/resources/model/ModelBaker;Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/renderer/block/dispatch/ModelState;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModelPart;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/resources/model/SimpleModelWrapper;findNonBlockSprites(Lnet/minecraft/client/resources/model/geometry/QuadCollection;)Lcom/google/common/collect/Multimap;` (exact) | client | 1000 (default) | `SimpleModelWrapperMixin.storeModelBakery` |
| [[40-Interfaces/net.minecraft.client.resources.model.SimpleModelWrapper|SimpleModelWrapper]].`findNonBlockSprites` | `(Lnet/minecraft/client/resources/model/geometry/QuadCollection;)Lcom/google/common/collect/Multimap;` | name_only | @Inject | INVOKE `Lnet/minecraft/client/resources/model/geometry/QuadCollection;getAll()Ljava/util/List;` (exact) | client | 1000 (default) | `SimpleModelWrapperMixin.analyzeMesh` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.Renderer|Renderer]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.Mesh|Mesh]] (interface, 0 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.MeshView|MeshView]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableMesh|MutableMesh]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView|MutableQuadView]] (interface, 47 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadAtlas|QuadAtlas]] (enum, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadEmitter|QuadEmitter]] (interface, 78 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadTransform|QuadTransform]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadView|QuadView]] (interface, 35 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.ShadeMode|ShadeMode]] (enum, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.FabricBlockStateModel|FabricBlockStateModel]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.FabricBlockStateModelPart|FabricBlockStateModelPart]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.FabricBlockStateModelSet|FabricBlockStateModelSet]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.MeshQuadCollection|MeshQuadCollection]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.ModelHelper|ModelHelper]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.ModelStateHelper|ModelStateHelper]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.AltModelBlockRenderer|AltModelBlockRenderer]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.ChunkSectionLayerHelper|ChunkSectionLayerHelper]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.ExtraLightCoordsUtil|ExtraLightCoordsUtil]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.FabricBlockModelRenderState|FabricBlockModelRenderState]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.FabricLayerRenderState|FabricLayerRenderState]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.FabricOrderedSubmitNodeCollector|FabricOrderedSubmitNodeCollector]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.submit.ExtendedBlockModelSubmit|ExtendedBlockModelSubmit]] (record, 16 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.submit.ExtendedItemSubmit|ExtendedItemSubmit]] (record, 16 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.FabricMaterialBaker|FabricMaterialBaker]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.FabricPreparations|FabricPreparations]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.FabricTextureAtlas|FabricTextureAtlas]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.SpriteFinder|SpriteFinder]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.SpriteFinderGetter|SpriteFinderGetter]] (interface, 1 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
