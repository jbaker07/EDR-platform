---
type: "interface"
fqcn: "net.minecraft.client.gui.render.GuiRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.render.GuiRenderer

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Lnet/minecraft/client/renderer/state/gui/GuiRenderState;Lnet/minecraf` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `addElementToMesh` | `(Lnet/minecraft/client/renderer/state/gui/GuiElementRenderState;)V` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `close` | `()V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `preparePictureInPicture` | `()V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `preparePictureInPicture` | `()V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `preparePictureInPictureState` | `(Lnet/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderSt` | name_only | @ModifyVariable at ['STORE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `pictureInPictureRenderers` | `Ljava/util/Map;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |

## Declared members (27 fields, 31 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final MAX_GUI_Z : F
public static final MIN_GUI_Z : F
private static final GUI_Z_NEAR : F
public static final GUI_3D_Z_FAR : I
public static final GUI_3D_Z_NEAR : I
public static final DEFAULT_ITEM_SIZE : I
public static final CLEAR_COLOR : Lorg/joml/Vector4fc;
private static final SCISSOR_COMPARATOR : Ljava/util/Comparator;
private static final TEXTURE_COMPARATOR : Ljava/util/Comparator;
private static final ELEMENT_SORT_COMPARATOR : Ljava/util/Comparator;
private final oversizedItemRenderers : Ljava/util/Map;
private final renderState : Lnet/minecraft/client/renderer/state/gui/GuiRenderState;
private final draws : Ljava/util/List;
private final vertexBuffer : Lnet/minecraft/client/renderer/StagedVertexBuffer;
private firstDrawIndexAfterBlur : I
private final guiProjection : Lnet/minecraft/client/renderer/Projection;
private final guiProjectionMatrixBuffer : Lnet/minecraft/client/renderer/ProjectionMatrixBuffer;
private final featureRenderDispatcher : Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher;
private final pictureInPictureRenderers : Ljava/util/Map;
private itemAtlas : Lnet/minecraft/client/gui/render/GuiItemAtlas;
private cachedGuiScale : I
private final cubeMap : Lnet/minecraft/client/renderer/CubeMap;
private previousScissorArea : Lnet/minecraft/client/gui/navigation/ScreenRectangle;
private previousPipeline : Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;
private previousTextureSetup : Lnet/minecraft/client/gui/render/TextureSetup;
private previousDraw : Lnet/minecraft/client/renderer/StagedVertexBuffer$Draw;
public <init>(Lnet/minecraft/client/renderer/state/gui/GuiRenderState;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher;Ljava/util/List;)V
public endFrame()V
public render()V
private clearUnusedOversizedItemRenderers()V
private prepare()V
private addElementsToMeshes(Lnet/minecraft/client/renderer/state/gui/GuiRenderState$TraverseRange;)V
private draw()V
private executeDrawRange(Ljava/util/function/Supplier;Lcom/mojang/blaze3d/pipeline/RenderTarget;Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;II)V
private addElementToMesh(Lnet/minecraft/client/renderer/state/gui/GuiElementRenderState;)V
private prepareText()V
private prepareItemElements()V
private preparePictureInPicture()V
private preparePictureInPictureState(Lnet/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState;I)V
private submitBlitFromItemAtlas(Lnet/minecraft/client/renderer/state/gui/GuiItemRenderState;Lnet/minecraft/client/gui/render/GuiItemAtlas$SlotView;)V
private prepareItemAtlas(Ljava/util/Set;I)Lnet/minecraft/client/gui/render/GuiItemAtlas;
private getGuiScaleInvalidatingItemAtlasIfChanged()I
private invalidateItemAtlas()V
private executeDraw(Lnet/minecraft/client/gui/render/GuiRenderer$Draw;Lcom/mojang/renderpearl/api/commands/RenderPass;)V
private scissorChanged(Lnet/minecraft/client/gui/navigation/ScreenRectangle;Lnet/minecraft/client/gui/navigation/ScreenRectangle;)Z
private enableScissor(Lnet/minecraft/client/gui/navigation/ScreenRectangle;Lcom/mojang/renderpearl/api/commands/RenderPass;)V
public registerPanoramaTextures(Lnet/minecraft/client/renderer/texture/TextureManager;)V
public close()V
private synthetic lambda$preparePictureInPicture$0(ILnet/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState;)V
private synthetic lambda$prepareItemElements$1(ILnet/minecraft/client/renderer/state/gui/GuiItemRenderState;)V
private static synthetic lambda$prepareItemElements$2(Ljava/lang/Object;)Lnet/minecraft/client/gui/render/pip/OversizedItemRenderer;
private synthetic lambda$prepareItemElements$0(Lorg/apache/commons/lang3/mutable/MutableBoolean;Lnet/minecraft/client/gui/render/GuiItemAtlas;Lnet/minecraft/client/renderer/state/gui/GuiItemRenderState;)V
private synthetic lambda$prepareText$0(Lnet/minecraft/client/renderer/state/gui/GuiTextRenderState;)V
private static synthetic lambda$draw$1()Ljava/lang/String;
private static synthetic lambda$draw$0()Ljava/lang/String;
private static synthetic lambda$new$0()Ljava/lang/String;
static <clinit>()V
```
