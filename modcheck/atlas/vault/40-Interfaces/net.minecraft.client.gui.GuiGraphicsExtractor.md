---
type: "interface"
fqcn: "net.minecraft.client.gui.GuiGraphicsExtractor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.GuiGraphicsExtractor

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `blit` | `(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/re` | exact | invokevirtual@124 in `FabricCreativeGuiComponents$CreativeModeTabButton.extractContents` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `blit` | `(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/re` | exact | invokevirtual@45 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `blit` | `(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/re` | exact | invokevirtual@75 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `blit` | `(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/re` | exact | invokevirtual@111 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `disableScissor` | `()V` | exact | invokevirtual@115 in `OptimizedScrollableLayout$Container.extractWidgetRenderState` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `enableScissor` | `(IIII)V` | exact | invokevirtual@27 in `OptimizedScrollableLayout$Container.extractWidgetRenderState` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `fill` | `(IIIII)V` | exact | invokevirtual@61 in `PackTooltipComponent.extractImage` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `pose` | `()Lorg/joml/Matrix3x2fStack;` | exact | invokevirtual@44 in `HudStatusBarHeightRegistryImpl.lambda$replaceVanillaElement$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `pose` | `()Lorg/joml/Matrix3x2fStack;` | exact | invokevirtual@52 in `HudStatusBarHeightRegistryImpl.lambda$replaceVanillaElement$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `pose` | `()Lorg/joml/Matrix3x2fStack;` | exact | invokevirtual@77 in `HudStatusBarHeightRegistryImpl.lambda$replaceVanillaElement$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `setTooltipForNextFrame` | `(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;II` | exact | invokevirtual@159 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `setTooltipForNextFrame` | `(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;` | exact | invokevirtual@176 in `FabricCreativeGuiComponents$CreativeModeTabButton.extractContents` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `text` | `(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;` | exact | invokevirtual@27 in `PackTooltipComponent.extractText` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `text` | `(Lnet/minecraft/client/gui/Font;Lnet/minecraft/util/FormattedCharSeque` | exact | invokevirtual@124 in `PackTooltipComponent.extractText` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `itemDecorations` | `(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;II` | exact | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (14 fields, 103 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final EXTRA_SPACE_AFTER_FIRST_TOOLTIP_LINE : I
private final minecraft : Lnet/minecraft/client/Minecraft;
private final pose : Lorg/joml/Matrix3x2fStack;
public final scissorStack : Lnet/minecraft/client/gui/GuiGraphicsExtractor$ScissorStack;
private final sprites : Lnet/minecraft/client/resources/model/sprite/SpriteGetter;
private final guiSprites : Lnet/minecraft/client/renderer/texture/TextureAtlas;
public final guiRenderState : Lnet/minecraft/client/renderer/state/gui/GuiRenderState;
private pendingCursor : Lcom/mojang/blaze3d/platform/cursor/CursorType;
private final mouseX : I
private final mouseY : I
private deferredTooltip : Ljava/lang/Runnable;
private hoveredTextStyle : Lnet/minecraft/network/chat/Style;
private clickableTextStyle : Lnet/minecraft/network/chat/Style;
private preeditOverlay : Lnet/minecraft/client/gui/components/Renderable;
private <init>(Lnet/minecraft/client/Minecraft;Lorg/joml/Matrix3x2fStack;Lnet/minecraft/client/renderer/state/gui/GuiRenderState;II)V
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/renderer/state/gui/GuiRenderState;II)V
public requestCursor(Lcom/mojang/blaze3d/platform/cursor/CursorType;)V
public applyCursor(Lcom/mojang/blaze3d/platform/Window;)V
public guiWidth()I
public guiHeight()I
public pose()Lorg/joml/Matrix3x2fStack;
public nextStratum()V
public blurBeforeThisStratum()V
public enableScissor(IIII)V
public disableScissor()V
public containsPointInScissor(II)Z
public horizontalLine(IIII)V
public verticalLine(IIII)V
public fill(IIIII)V
public fill(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;IIIII)V
public fillGradient(IIIIII)V
public fill(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/client/gui/render/TextureSetup;IIII)V
public outline(IIIII)V
private innerFill(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/client/gui/render/TextureSetup;IIIIILjava/lang/Integer;)V
public textHighlight(IIIIZ)V
public text(Lnet/minecraft/client/gui/Font;Ljava/lang/String;III)V
public text(Lnet/minecraft/client/gui/Font;Ljava/lang/String;IIIZ)V
public text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/util/FormattedCharSequence;III)V
public text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/util/FormattedCharSequence;IIIZ)V
public text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;III)V
public text(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IIIZ)V
public centeredText(Lnet/minecraft/client/gui/Font;Ljava/lang/String;III)V
public centeredText(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;III)V
public centeredText(Lnet/minecraft/client/gui/Font;Lnet/minecraft/util/FormattedCharSequence;III)V
public textWithWordWrap(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/FormattedText;IIII)I
public textWithWordWrap(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/FormattedText;IIIIZ)I
public textWithBackdrop(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IIII)V
public blit(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIFFIIIII)V
public blit(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIFFIIII)V
public blit(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIFFIIIIII)V
public blit(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIFFIIIIIII)V
public blit(Lnet/minecraft/resources/Identifier;IIIIFFFF)V
public blit(Lcom/mojang/renderpearl/api/textures/GpuTextureView;Lcom/mojang/renderpearl/api/textures/GpuSampler;IIIIFFFF)V
public blitSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIII)V
public blitSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIIIF)V
public blitSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIIII)V
public blitSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIIIIIII)V
public blitSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIIIIIIII)V
public blitSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;IIII)V
public blitSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;IIIII)V
private blitSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;IIIIIIIII)V
private blitNineSlicedSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;Lnet/minecraft/client/resources/metadata/gui/GuiSpriteScaling$NineSlice;IIIII)V
private blitNineSliceInnerSegment(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/client/resources/metadata/gui/GuiSpriteScaling$NineSlice;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;IIIIIIIIIII)V
private blitTiledSprite(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;IIIIIIIIIII)V
private innerBlit(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lnet/minecraft/resources/Identifier;IIIIFFFFI)V
private innerBlit(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lcom/mojang/renderpearl/api/textures/GpuTextureView;Lcom/mojang/renderpearl/api/textures/GpuSampler;IIIIFFFFI)V
private innerTiledBlit(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lcom/mojang/renderpearl/api/textures/GpuTextureView;Lcom/mojang/renderpearl/api/textures/GpuSampler;IIIIIIFFFFI)V
private static getSpriteScaling(Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;)Lnet/minecraft/client/resources/metadata/gui/GuiSpriteScaling;
public item(Lnet/minecraft/world/item/ItemStack;II)V
public item(Lnet/minecraft/world/item/ItemStack;III)V
public item(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/ItemStack;III)V
private item(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;III)V
public fakeItem(Lnet/minecraft/world/item/ItemStack;II)V
public fakeItem(Lnet/minecraft/world/item/ItemStack;III)V
public itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;II)V
public itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V
private itemBar(Lnet/minecraft/world/item/ItemStack;II)V
private itemCount(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V
private itemCooldown(Lnet/minecraft/world/item/ItemStack;II)V
public map(Lnet/minecraft/client/renderer/state/MapRenderState;)V
public entity(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;FLorg/joml/Vector3fc;Lorg/joml/Quaternionfc;Lorg/joml/Quaternionfc;IIII)V
public skin(Lnet/minecraft/client/model/Model$Simple;Lnet/minecraft/resources/Identifier;FFFFIIII)V
public book(Lnet/minecraft/client/model/object/book/BookModel;Lnet/minecraft/resources/Identifier;FFFIIII)V
public bannerPattern(Lnet/minecraft/client/model/object/banner/BannerFlagModel;Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/world/level/block/entity/BannerPatternLayers;IIII)V
public profilerChart(Ljava/util/List;IIII)V
public setTooltipForNextFrame(Lnet/minecraft/network/chat/Component;II)V
public setTooltipForNextFrame(Ljava/util/List;II)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;II)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;II)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;IILnet/minecraft/resources/Identifier;)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;IILnet/minecraft/resources/Identifier;Z)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Ljava/util/Optional;Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipPositioner;IIZLnet/minecraft/resources/Identifier;)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;II)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;IILnet/minecraft/resources/Identifier;)V
public setComponentTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;II)V
public setComponentTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;IILnet/minecraft/resources/Identifier;)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;II)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;IILnet/minecraft/resources/Identifier;)V
public setTooltipForNextFrame(Lnet/minecraft/client/gui/Font;Ljava/util/List;Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipPositioner;IIZ)V
private setTooltipForNextFrameInternal(Lnet/minecraft/client/gui/Font;Ljava/util/List;IILnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipPositioner;Lnet/minecraft/resources/Identifier;Z)V
private setTooltipForNextFrameInternal(Lnet/minecraft/client/gui/Font;Ljava/util/List;IILnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipPositioner;Lnet/minecraft/resources/Identifier;ZZ)V
public tooltip(Lnet/minecraft/client/gui/Font;Ljava/util/List;IILnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipPositioner;Lnet/minecraft/resources/Identifier;Z)V
public setPreeditOverlay(Lnet/minecraft/client/gui/components/Renderable;)V
public extractDeferredElements(IIF)V
private componentHoverEffect(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Style;II)V
public getSprite(Lnet/minecraft/client/resources/model/sprite/SpriteId;)Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public textRendererForWidget(Lnet/minecraft/client/gui/components/AbstractWidget;Lnet/minecraft/client/gui/GuiGraphicsExtractor$HoveredTextEffects;)Lnet/minecraft/client/gui/ActiveTextCollector;
public textRenderer()Lnet/minecraft/client/gui/ActiveTextCollector;
public textRenderer(Lnet/minecraft/client/gui/GuiGraphicsExtractor$HoveredTextEffects;)Lnet/minecraft/client/gui/ActiveTextCollector;
public textRenderer(Lnet/minecraft/client/gui/GuiGraphicsExtractor$HoveredTextEffects;Ljava/util/function/Consumer;)Lnet/minecraft/client/gui/ActiveTextCollector;
private createDefaultTextParameters(F)Lnet/minecraft/client/gui/ActiveTextCollector$Parameters;
private synthetic lambda$setTooltipForNextFrameInternal$0(Lnet/minecraft/client/gui/Font;Ljava/util/List;IILnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipPositioner;Lnet/minecraft/resources/Identifier;Z)V
private static synthetic lambda$setTooltipForNextFrame$1(Ljava/util/List;Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)V
private static synthetic lambda$setTooltipForNextFrame$0(Ljava/util/List;Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)V
private static synthetic lambda$item$2(Lnet/minecraft/world/item/ItemStack;)Ljava/lang/String;
private static synthetic lambda$item$1(Lnet/minecraft/world/item/ItemStack;)Ljava/lang/String;
private static synthetic lambda$item$0(Lnet/minecraft/world/item/ItemStack;)Ljava/lang/String;
```
