---
type: "system"
package: "net.minecraft.client.renderer"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer

Analyst note: [[_authored/systems/net.minecraft.client.renderer|Rendering: level, entities, block and item models]]

1006 classes in the jar. Hooked types: 66

- [[40-Interfaces/net.minecraft.client.renderer.GameRenderer|GameRenderer]] -- calls:10, injects_into:2 -- by fabric-client-gametest-api-v1, fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] -- calls:1, injects_into:11, wraps:2 -- by fabric-client-gametest-api-v1, fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.OrderedSubmitNodeCollector|OrderedSubmitNodeCollector]] -- calls:4 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.Panorama|Panorama]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.Rect2i|Rect2i]] -- calls:6 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.Sheets|Sheets]] -- calls:17 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.SubmitNodeCollection|SubmitNodeCollection]] -- wraps:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.SubmitNodeCollector|SubmitNodeCollector]] -- calls:4 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockAndTintGetter|BlockAndTintGetter]] -- calls:4, reads:2 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelLighter|BlockModelLighter]] -- calls:2 -- by fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelLighter_Cache|BlockModelLighter$Cache]] -- calls:3 -- by fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]] -- calls:2, injects_into:4, reads:2 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.BlockStateModelSet|BlockStateModelSet]] -- calls:3 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.FluidModel_Unbaked|FluidModel$Unbaked]] -- calls:1 -- by fabric-rendering-fluids-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.FluidRenderer|FluidRenderer]] -- calls:1, injects_into:1 -- by fabric-rendering-fluids-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.ModelBlockRenderer|ModelBlockRenderer]] -- calls:1, injects_into:1 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.MovingBlockRenderState|MovingBlockRenderState]] -- reads:3 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel|BlockStateModel]] -- calls:20 -- by fabric-model-loading-api-v1, fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel_Unbaked|BlockStateModel$Unbaked]] -- wraps:2 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModelPart|BlockStateModelPart]] -- calls:5 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.WeightedVariants_Unbaked|WeightedVariants$Unbaked]] -- calls:2 -- by fabric-model-loading-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel_Selector|MultiPartModel$Selector]] -- calls:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel_SharedBakedState|MultiPartModel$SharedBakedState]] -- calls:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.blockentity.BlockEntityRenderers|BlockEntityRenderers]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.chunk.ChunkSectionLayer|ChunkSectionLayer]] -- calls:2, reads:1 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.chunk.RenderRegionCache|RenderRegionCache]] -- injects_into:2 -- by fabric-block-getter-api-v2
- [[40-Interfaces/net.minecraft.client.renderer.chunk.SectionCompiler|SectionCompiler]] -- injects_into:1, wraps:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.debug.DebugRenderer|DebugRenderer]] -- injects_into:1 -- by fabric-debug-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.ArmorModelSet|ArmorModelSet]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderDispatcher|EntityRenderDispatcher]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRendererProvider|EntityRendererProvider]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderers|EntityRenderers]] -- injects_into:1, wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.layers.CapeLayer|CapeLayer]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer|HumanoidArmorLayer]] -- injects_into:2 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.entity.layers.RenderLayer|RenderLayer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]] -- injects_into:3, wraps:1 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureFrameContext|FeatureFrameContext]] -- calls:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureRenderDispatcher|FeatureRenderDispatcher]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer|MovingBlockFeatureRenderer]] -- injects_into:1, wraps:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer_Submit|MovingBlockFeatureRenderer$Submit]] -- calls:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer|RenderTypeFeatureRenderer]] -- calls:3 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.feature.phase.FeatureRenderPhase|FeatureRenderPhase]] -- calls:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase|SimpleFeatureRenderPhase]] -- calls:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.item.CuboidItemModelWrapper|CuboidItemModelWrapper]] -- injects_into:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState|ItemStackRenderState]] -- injects_into:3 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_FoilType|ItemStackRenderState$FoilType]] -- calls:3, reads:1 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_LayerRenderState|ItemStackRenderState$LayerRenderState]] -- calls:1, injects_into:2, reads:1, wraps:1 -- by fabric-renderer-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.rendertype.RenderType|RenderType]] -- calls:4 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.state.OptionsRenderState|OptionsRenderState]] -- reads:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.gui.GuiRenderState|GuiRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState|PictureInPictureRenderState]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.BlockBreakingRenderState|BlockBreakingRenderState]] -- calls:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.CameraRenderState|CameraRenderState]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.FirstPersonHandsAndItemsRenderState|FirstPersonHandsAndItemsRenderState]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.LevelRenderState|LevelRenderState]] -- injects_into:1, reads:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.ParticlesRenderState|ParticlesRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.PlayerRenderState|PlayerRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.SkyRenderState|SkyRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.WeatherRenderState|WeatherRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.state.level.WorldBorderRenderState|WorldBorderRenderState]] -- injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.renderer.texture.OverlayTexture|OverlayTexture]] -- reads:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.texture.SpriteContents|SpriteContents]] -- calls:2 -- by fabric-renderer-api-v1, fabric-renderer-indigo
- [[40-Interfaces/net.minecraft.client.renderer.texture.SpriteLoader_Preparations|SpriteLoader$Preparations]] -- calls:2 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.texture.TextureAtlas|TextureAtlas]] -- injects_into:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.renderer.texture.TextureAtlasSprite|TextureAtlasSprite]] -- calls:8 -- by fabric-renderer-api-v1, fabric-renderer-indigo
