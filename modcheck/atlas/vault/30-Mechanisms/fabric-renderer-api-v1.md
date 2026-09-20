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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.particle.BlockMarker|BlockMarker]] | `<init>(Lnet/minecraft/client/multiplayer/ClientLevel;DDDLnet/minecraft/world/level/block/state/BlockState;)V` | wraps `@Redirect at INVOKE Lnet/minecraft/client/renderer/block/BlockStateModelSet;getParticleMaterial(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;` | client | `BlockMarkerMixin.getParticleMaterialProxy` |
| [[40-Interfaces/net.minecraft.client.particle.TerrainParticle|TerrainParticle]] | `<init>(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V` | wraps `@Redirect at INVOKE Lnet/minecraft/client/renderer/block/BlockStateModelSet;getParticleMaterial(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;` | client | `TerrainParticleMixin.getParticleIconProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `submitBlockDestroyAnimation(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V` | injects_into `@Inject at INVOKE Lnet/minecraft/util/RandomSource;createThreadLocalInstance()Lnet/minecraft/util/RandomSource;` | client | `LevelRendererMixin.beforeCreateRandom` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `submitBlockDestroyAnimation(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V` | wraps `@Redirect at INVOKE Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;collectParts(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V` | client | `LevelRendererMixin.cancelCollectParts` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `submitBlockDestroyAnimation(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/LevelRenderState;)V` | wraps `@Redirect at INVOKE Lnet/minecraft/client/renderer/SubmitNodeCollector;submitBreakingBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/List;IZ)V` | client | `LevelRendererMixin.submitBreakingBlockModelProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.SubmitNodeCollection|SubmitNodeCollection]] | `submitMovingBlock` | wraps `@Redirect at INVOKE Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;hasMaterialFlag(I)Z` | client | `SubmitNodeCollectionMixin.hasMaterialFlagProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]] | `clear()V` | injects_into `@Inject at RETURN` | client | `BlockModelRenderStateMixin.onReturnClear` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]] | `setupModel` | injects_into `@Inject at RETURN` | client | `BlockModelRenderStateMixin.onReturnSetupModel` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]] | `submitModel` | injects_into `@Inject at HEAD` | client | `BlockModelRenderStateMixin.submitMesh` |
| [[40-Interfaces/net.minecraft.client.renderer.chunk.SectionCompiler|SectionCompiler]] | `compile` | injects_into `@Inject at INVOKE Lnet/minecraft/core/BlockPos;betweenClosed(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;)Ljava/lang/Iterable;` | client | `SectionCompilerMixin.beforeLoopCompile` |
| [[40-Interfaces/net.minecraft.client.renderer.chunk.SectionCompiler|SectionCompiler]] | `compile` | wraps `@Redirect at INVOKE net/minecraft/client/renderer/block/ModelBlockRenderer.tesselateBlock(Lnet/minecraft/client/renderer/block/BlockQuadOutput;FFFLnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;J)V` | client | `SectionCompilerMixin.tesselateBlockProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]] | `extractBlockOutline` | wraps `@Redirect at INVOKE Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;hasMaterialFlag(I)Z` | client | `LevelExtractorMixin.hasMaterialFlagProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer|MovingBlockFeatureRenderer]] | `buildGroup` | injects_into `@Inject at INVOKE net/minecraft/client/renderer/block/ModelBlockRenderer.<init>(ZZLnet/minecraft/client/color/block/BlockColors;)V` | client | `MovingBlockFeatureRendererMixin.beforeInitBlockRenderer` |
| [[40-Interfaces/net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer|MovingBlockFeatureRenderer]] | `buildGroup` | wraps `@Redirect at INVOKE net/minecraft/client/renderer/block/ModelBlockRenderer.tesselateBlock(Lnet/minecraft/client/renderer/block/BlockQuadOutput;FFFLnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;J)V` | client | `MovingBlockFeatureRendererMixin.tesselateBlockProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.item.CuboidItemModelWrapper|CuboidItemModelWrapper]] | `<init>` | injects_into `@Inject at RETURN` | client | `CuboidItemModelWrapperMixin.onReturnInit` |
| [[40-Interfaces/net.minecraft.client.renderer.item.CuboidItemModelWrapper|CuboidItemModelWrapper]] | `update` | injects_into `@Inject at RETURN` | client | `CuboidItemModelWrapperMixin.onReturnUpdate` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState|ItemStackRenderState]] | `visitExtents(Ljava/util/function/Consumer;)V` | injects_into `@Inject at NEW com/mojang/blaze3d/vertex/PoseStack$Pose` | client | `ItemStackRenderStateMixin.afterInitVecLoad` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState|ItemStackRenderState]] | `visitExtents(Ljava/util/function/Consumer;)V` | injects_into `@Inject at INVOKE Lcom/mojang/blaze3d/vertex/PoseStack$Pose;setIdentity()V` | client | `ItemStackRenderStateMixin.afterLayerLoad` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_LayerRenderState|ItemStackRenderState$LayerRenderState]] | `clear()V` | injects_into `@Inject at RETURN` | client | `ItemStackRenderStateLayerRenderStateMixin.onReturnClear` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_LayerRenderState|ItemStackRenderState$LayerRenderState]] | `submit` | wraps `@Redirect at INVOKE Lnet/minecraft/client/renderer/SubmitNodeCollector;submitItem(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/item/ItemDisplayContext;III[ILnet/minecraft/client/resources/model/geometry/ItemQuads;Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;)V` | client | `ItemStackRenderStateLayerRenderStateMixin.submitItemProxy` |
| [[40-Interfaces/net.minecraft.client.renderer.texture.TextureAtlas|TextureAtlas]] | `upload` | injects_into `@Inject at RETURN` | client | `TextureAtlasMixin.uploadHook` |
| [[40-Interfaces/net.minecraft.client.resources.model.SimpleModelWrapper|SimpleModelWrapper]] | `findNonBlockSprites` | injects_into `@Inject at INVOKE Lnet/minecraft/client/resources/model/geometry/QuadCollection;getAll()Ljava/util/List;` | client | `SimpleModelWrapperMixin.analyzeMesh` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.Renderer|Renderer]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.MeshView|MeshView]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableMesh|MutableMesh]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView|MutableQuadView]] (interface, 47 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadAtlas|QuadAtlas]] (class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadEmitter|QuadEmitter]] (interface, 78 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadTransform|QuadTransform]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadView|QuadView]] (interface, 35 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.mesh.ShadeMode|ShadeMode]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.FabricBlockStateModel|FabricBlockStateModel]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.FabricBlockStateModelPart|FabricBlockStateModelPart]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.FabricBlockStateModelSet|FabricBlockStateModelSet]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.MeshQuadCollection|MeshQuadCollection]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.ModelHelper|ModelHelper]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.model.ModelStateHelper|ModelStateHelper]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.AltModelBlockRenderer|AltModelBlockRenderer]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.ChunkSectionLayerHelper|ChunkSectionLayerHelper]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.ExtraLightCoordsUtil|ExtraLightCoordsUtil]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.FabricBlockModelRenderState|FabricBlockModelRenderState]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.FabricLayerRenderState|FabricLayerRenderState]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.FabricOrderedSubmitNodeCollector|FabricOrderedSubmitNodeCollector]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.submit.ExtendedBlockModelSubmit|ExtendedBlockModelSubmit]] (record, 17 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.render.submit.ExtendedItemSubmit|ExtendedItemSubmit]] (record, 17 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.FabricMaterialBaker|FabricMaterialBaker]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.FabricPreparations|FabricPreparations]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.FabricTextureAtlas|FabricTextureAtlas]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.SpriteFinder|SpriteFinder]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.renderer.v1.sprite.SpriteFinderGetter|SpriteFinderGetter]] (interface, 1 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
