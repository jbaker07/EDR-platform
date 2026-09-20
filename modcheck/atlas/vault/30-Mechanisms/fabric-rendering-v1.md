---
type: "mechanism"
module: "fabric-rendering-v1"
version: "27.0.14+901a437c5d"
sha256: "749427999b4845b129683b1db268a04b524abb6ab351dcaf67cda9a3ab56b5c0"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-rendering-v1

**Version** `27.0.14+901a437c5d` -- **artifact sha256** `749427999b4845b129683b1db268a04b524abb6ab351dcaf67cda9a3ab56b5c0`

## Declared (fabric.mod.json)

- environment: `client`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-lifecycle-events-v1": "*"}`
- entrypoints: `{"client": ["net.fabricmc.fabric.impl.client.rendering.hud.HudStatusBarHeightRegistryImpl"]}`
- mixin configs: `["fabric-rendering-v1.mixins.json"]`
- access widener: `fabric-rendering-v1.classtweaker`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.BuiltInBlockModelsCallback.EVENT|BuiltInBlockModelsCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.ClientTooltipComponentCallback.EVENT|ClientTooltipComponentCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.ExtractItemDecorationsCallback.EVENT|ExtractItemDecorationsCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.InvalidateRenderStateCallback.EVENT|InvalidateRenderStateCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.LivingEntityFeatureRenderEvents.ALLOW_CAPE_RENDER|LivingEntityFeatureRenderEvents.ALLOW_CAPE_RENDER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback.EVENT|LivingEntityRenderLayerRegistrationCallback.EVENT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelExtractionEvents.AFTER_BLOCK_OUTLINE_EXTRACTION|LevelExtractionEvents.AFTER_BLOCK_OUTLINE_EXTRACTION]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelExtractionEvents.END_EXTRACTION|LevelExtractionEvents.END_EXTRACTION]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_BLOCK_OUTLINE_EXTRACTION|LevelRenderEvents.AFTER_BLOCK_OUTLINE_EXTRACTION]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_OPAQUE_TERRAIN|LevelRenderEvents.AFTER_OPAQUE_TERRAIN]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_SOLID_FEATURES|LevelRenderEvents.AFTER_SOLID_FEATURES]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_TRANSLUCENT_FEATURES|LevelRenderEvents.AFTER_TRANSLUCENT_FEATURES]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_TRANSLUCENT_TERRAIN|LevelRenderEvents.AFTER_TRANSLUCENT_TERRAIN]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.BEFORE_BLOCK_OUTLINE|LevelRenderEvents.BEFORE_BLOCK_OUTLINE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.BEFORE_GIZMOS|LevelRenderEvents.BEFORE_GIZMOS]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.BEFORE_TRANSLUCENT_TERRAIN|LevelRenderEvents.BEFORE_TRANSLUCENT_TERRAIN]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.COLLECT_SUBMITS|LevelRenderEvents.COLLECT_SUBMITS]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.END_EXTRACTION|LevelRenderEvents.END_EXTRACTION]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.END_MAIN|LevelRenderEvents.END_MAIN]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.START_MAIN|LevelRenderEvents.START_MAIN]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/com.mojang.renderpearl.api.pipeline.RenderPipeline_Builder|RenderPipeline$Builder]] | `withSnippet` | injects_into `@Inject at TAIL` | client | `RenderPipelineBuilderMixin.copyUsePipelineDrawModeForGuiFromSnippet` |
| [[40-Interfaces/net.minecraft.client.color.block.BlockColors|BlockColors]] | `createDefault` | injects_into `@Inject at RETURN` | client | `BlockColorsMixin.create` |
| [[40-Interfaces/net.minecraft.client.gui.GuiGraphicsExtractor|GuiGraphicsExtractor]] | `itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V` | injects_into `@Inject at RETURN` | client | `GuiGraphicsExtractorMixin.drawStackOverlay` |
| [[40-Interfaces/net.minecraft.client.gui.render.GuiRenderer|GuiRenderer]] | `<init>` | injects_into `@Inject at RETURN` | client | `GuiRendererMixin.mutableSpecialElementRenderers` |
| [[40-Interfaces/net.minecraft.client.gui.render.GuiRenderer|GuiRenderer]] | `close` | injects_into `@Inject at RETURN` | client | `GuiRendererMixin.closeRendererPools` |
| [[40-Interfaces/net.minecraft.client.gui.render.GuiRenderer|GuiRenderer]] | `preparePictureInPicture` | injects_into `@Inject at HEAD` | client | `GuiRendererMixin.prePrepareSpecialElements` |
| [[40-Interfaces/net.minecraft.client.gui.render.GuiRenderer|GuiRenderer]] | `preparePictureInPicture` | injects_into `@Inject at RETURN` | client | `GuiRendererMixin.postPrepareSpecialElements` |
| [[40-Interfaces/net.minecraft.client.gui.render.GuiRenderer|GuiRenderer]] | `preparePictureInPictureState` | injects_into `@ModifyVariable at STORE` | client | `GuiRendererMixin.substituteSpecialElementRenderer` |
| [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementTab|AdvancementTab]] | `extractContents` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;extractConnectivity(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIZ)V` | client | `AdvancementTabMixin.extractAdvancementBackground` |
| [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementWidget|AdvancementWidget]] | `extractHover` | injects_into `@Inject at INVOKE Ljava/util/List;isEmpty()Z` | client | `AdvancementWidgetMixin.captureExtractTooltip` |
| [[40-Interfaces/net.minecraft.client.gui.screens.debug.DebugOptionsScreen_OptionList|DebugOptionsScreen$OptionList]] | `lambda$static$0` | wraps `@Redirect at INVOKE Lnet/minecraft/resources/Identifier;compareTo(Lnet/minecraft/resources/Identifier;)I` | client | `DebugOptionsScreenOptionListMixin.sort` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent|ClientTooltipComponent]] | `create(Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;` | injects_into `@Inject at HEAD` | client | `ClientTooltipComponentMixin.convertCustomTooltipComponent` |
| [[40-Interfaces/net.minecraft.client.model.Model|Model]] | `<init>` | injects_into `@Inject at TAIL` | client | `ModelMixin.fillChildPartMap` |
| [[40-Interfaces/net.minecraft.client.model.geom.LayerDefinitions|LayerDefinitions]] | `createRoots` | injects_into `@Inject at INVOKE Lcom/google/common/collect/ImmutableMap$Builder;build()Lcom/google/common/collect/ImmutableMap;` | client | `LayerDefinitionsMixin.registerExtraModelData` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel|ClientLevel]] | `clearTintCaches()V` | injects_into `@Inject at RETURN` | client | `ClientLevelMixin.onReloadColor` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel|ClientLevel]] | `onChunkLoaded(Lnet/minecraft/world/level/ChunkPos;)V` | injects_into `@Inject at RETURN` | client | `ClientLevelMixin.onResetChunkColor` |
| [[40-Interfaces/net.minecraft.client.renderer.GameRenderer|GameRenderer]] | `<init>` | injects_into `@Inject at RETURN` | client | `GameRendererMixin.guiRendererReady` |
| [[40-Interfaces/net.minecraft.client.renderer.GameRenderer|GameRenderer]] | `extract` | injects_into `@Inject at HEAD` | client | `GameRendererMixin.beforeExtract` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `executeClassicTransparency` | injects_into `@Inject at INVOKE Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;executeTranslucent(Lcom/mojang/renderpearl/api/commands/RenderPass;)V` | client | `LevelRendererMixin.afterRenderClassicTranslucentFeatures` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `executeOit` | injects_into `@Inject at RETURN` | client | `LevelRendererMixin.afterRenderOitTranslucentFeatures` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `executeSolid` | injects_into `@Inject at INVOKE Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;executeSolid(Lcom/mojang/renderpearl/api/commands/RenderPass;)V` | client | `LevelRendererMixin.afterRenderSolidFeatures` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `lambda$addMainPass$0` | injects_into `@Inject at RETURN` | client | `LevelRendererMixin.endMainRender` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `prepareChunkRenders` | injects_into `@Inject at RETURN` | client | `LevelRendererMixin.prepareChunkRenders` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `prepareChunkRendersIndirect` | injects_into `@Inject at RETURN` | client | `LevelRendererMixin.prepareChunkRenders` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `render` | injects_into `@Inject at HEAD` | client | `LevelRendererMixin.beforeRender` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `submitBlockOutline` | injects_into `@Inject at FIELD Lnet/minecraft/client/renderer/state/level/CameraRenderState;pos:Lnet/minecraft/world/phys/Vec3;` | client | `LevelRendererMixin.beforeRenderBlockOutline` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `submitFeatures` | injects_into `@Inject at RETURN` | client | `LevelRendererMixin.afterCollectSubmits` |
| [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]] | `submitFeatures` | injects_into `@Inject at INVOKE Lnet/minecraft/client/renderer/LevelRenderer;finalizeGizmoCollection()V` | client | `LevelRendererMixin.beforeCollectGizmos` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BlockModelRenderState|BlockModelRenderState]] | `clear` | injects_into `@Inject at TAIL` | client | `BlockModelRenderStateMixin.clearExtraRenderData` |
| [[40-Interfaces/net.minecraft.client.renderer.block.BuiltInBlockModels|BuiltInBlockModels]] | `createBlockModels` | injects_into `@Inject at INVOKE Lnet/minecraft/client/renderer/block/BuiltInBlockModels$Builder;build()Ljava/util/Map;` | client | `BuiltInBlockModelsMixin.createBlockModels` |
| [[40-Interfaces/net.minecraft.client.renderer.block.ModelBlockRenderer|ModelBlockRenderer]] | `computeTintColor(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;I)I` | injects_into `@Inject at FIELD Lnet/minecraft/client/renderer/block/ModelBlockRenderer;tintSourcesInitialized:Z` | client | `ModelBlockRendererMixin.injectFactoryTintCacheLoading` |
| [[40-Interfaces/net.minecraft.client.renderer.blockentity.BlockEntityRenderers|BlockEntityRenderers]] | `<clinit>*` | injects_into `@Inject at RETURN` | client | `BlockEntityRenderersMixin.init` |
| [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderDispatcher|EntityRenderDispatcher]] | `onResourceManagerReload` | injects_into `@Inject at TAIL` | client | `EntityRenderDispatcherMixin.createArmorRenderers` |
| [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderers|EntityRenderers]] | `<clinit>*` | injects_into `@Inject at RETURN` | client | `EntityRenderersMixin.onRegisterRenderers` |
| [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderers|EntityRenderers]] | `lambda$createEntityRenderers$0` | wraps `@Redirect at INVOKE Lnet/minecraft/client/renderer/entity/EntityRendererProvider;create(Lnet/minecraft/client/renderer/entity/EntityRendererProvider$Context;)Lnet/minecraft/client/renderer/entity/EntityRenderer;` | client | `EntityRenderersMixin.createEntityRenderer` |
| [[40-Interfaces/net.minecraft.client.renderer.entity.layers.CapeLayer|CapeLayer]] | `submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/AvatarRenderState;FF)V` | injects_into `@Inject at HEAD` | client | `CapeLayerMixin.injectCapeRenderCheck` |
| [[40-Interfaces/net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer|HumanoidArmorLayer]] | `renderArmorPiece` | injects_into `@Inject at HEAD` | client | `HumanoidArmorLayerMixin.renderArmor` |
| [[40-Interfaces/net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer|HumanoidArmorLayer]] | `submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/HumanoidRenderState;FF)V` | injects_into `@Inject at HEAD` | client | `HumanoidArmorLayerMixin.render` |
| [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]] | `allChanged` | injects_into `@Inject at HEAD` | client | `LevelExtractorMixin.onReload` |
| [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]] | `extract` | injects_into `@Inject at RETURN` | client | `LevelExtractorMixin.afterExtractLevel` |
| [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]] | `extractBlockOutline` | injects_into `@Inject at RETURN` | client | `LevelExtractorMixin.afterBlockOutlineExtraction` |
| [[40-Interfaces/net.minecraft.client.renderer.feature.FeatureRenderDispatcher|FeatureRenderDispatcher]] | `<init>` | injects_into `@Inject at RETURN` | client | `FeatureRenderDispatcherMixin.registerExtendedFeatureRenderers` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState|ItemStackRenderState]] | `clear` | injects_into `@Inject at TAIL` | client | `ItemStackRenderStateMixin.clearExtraRenderData` |
| [[40-Interfaces/net.minecraft.client.renderer.item.ItemStackRenderState_LayerRenderState|ItemStackRenderState$LayerRenderState]] | `clear` | injects_into `@Inject at TAIL` | client | `ItemStackRenderStateLayerRenderStateMixin.clearExtraRenderData` |
| [[40-Interfaces/net.minecraft.client.renderer.state.gui.GuiRenderState|GuiRenderState]] | `reset` | injects_into `@Inject at TAIL` | client | `GuiRenderStateMixin.clearExtraRenderData` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.LevelRenderState|LevelRenderState]] | `reset` | injects_into `@Inject at TAIL` | client | `LevelRenderStateMixin.clearExtraRenderData` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.ParticlesRenderState|ParticlesRenderState]] | `reset` | injects_into `@Inject at TAIL` | client | `ParticlesRenderStateMixin.clearExtraRenderData` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.PlayerRenderState|PlayerRenderState]] | `reset` | injects_into `@Inject at TAIL` | client | `PlayerRenderStateMixin.clearExtraRenderData` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.SkyRenderState|SkyRenderState]] | `reset` | injects_into `@Inject at TAIL` | client | `SkyRenderStateMixin.clearExtraRenderData` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.WeatherRenderState|WeatherRenderState]] | `reset` | injects_into `@Inject at TAIL` | client | `WeatherRenderStateMixin.clearExtraRenderData` |
| [[40-Interfaces/net.minecraft.client.renderer.state.level.WorldBorderRenderState|WorldBorderRenderState]] | `reset` | injects_into `@Inject at TAIL` | client | `WorldBorderRenderStateMixin.clearExtraRenderData` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.ArmorRenderer|ArmorRenderer]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.AtlasRegistry|AtlasRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.BlockColorRegistry|BlockColorRegistry]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.BlockEntityRendererRegistry|BlockEntityRendererRegistry]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.BlockTintsFactory|BlockTintsFactory]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.BuiltInBlockModelsCallback|BuiltInBlockModelsCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.ClientTooltipComponentCallback|ClientTooltipComponentCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.ColorResolverRegistry|ColorResolverRegistry]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.EntityRendererRegistry|EntityRendererRegistry]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.ExtractItemDecorationsCallback|ExtractItemDecorationsCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.FabricModel|FabricModel]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.FabricOrderedSubmitNodeCollector|FabricOrderedSubmitNodeCollector]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.FabricRenderPipeline|FabricRenderPipeline]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.FabricRenderState|FabricRenderState]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.FeatureRendererRegistry|FeatureRendererRegistry]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.InvalidateRenderStateCallback|InvalidateRenderStateCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.LivingEntityFeatureRenderEvents|LivingEntityFeatureRenderEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback|LivingEntityRenderLayerRegistrationCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.ModelLayerRegistry|ModelLayerRegistry]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.PictureInPictureRendererRegistry|PictureInPictureRendererRegistry]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.RenderStateDataKey|RenderStateDataKey]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.SpriteSourceRegistry|SpriteSourceRegistry]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.SubmitRenderPhase|SubmitRenderPhase]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.SubmitRenderPhases|SubmitRenderPhases]] (class, 16 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.TransformCopyingModel|TransformCopyingModel]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.advancement.AdvancementRenderContext|AdvancementRenderContext]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.advancement.AdvancementRenderer|AdvancementRenderer]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.hud.HudElement|HudElement]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry|HudElementRegistry]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.hud.HudStatusBarHeightRegistry|HudStatusBarHeightRegistry]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.hud.StatusBarHeightProvider|StatusBarHeightProvider]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.hud.VanillaHudElements|VanillaHudElements]] (class, 24 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.level.AbstractLevelRenderContext|AbstractLevelRenderContext]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.level.LevelExtractionContext|LevelExtractionContext]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.level.LevelExtractionEvents|LevelExtractionEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderContext|LevelRenderContext]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents|LevelRenderEvents]] (class, 13 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.rendering.v1.level.LevelTerrainRenderContext|LevelTerrainRenderContext]] (interface, 1 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
