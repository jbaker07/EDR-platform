---
type: "interface"
fqcn: "net.minecraft.client.gui.render.GuiRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.render.GuiRenderer

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `close` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `preparePictureInPicture` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `preparePictureInPicture` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `preparePictureInPictureState` | `@ModifyVariable at STORE` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (58, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.render.GuiRenderer implements java.lang.AutoCloseable {
    private static final org.slf4j.Logger LOGGER;
    private static final float MAX_GUI_Z;
    public static final float MIN_GUI_Z;
    private static final float GUI_Z_NEAR;
    public static final int GUI_3D_Z_FAR;
    public static final int GUI_3D_Z_NEAR;
    public static final int DEFAULT_ITEM_SIZE;
    public static final org.joml.Vector4fc CLEAR_COLOR;
    private static final java.util.Comparator<net.minecraft.client.gui.navigation.ScreenRectangle> SCISSOR_COMPARATOR;
    private static final java.util.Comparator<net.minecraft.client.gui.render.TextureSetup> TEXTURE_COMPARATOR;
    private static final java.util.Comparator<net.minecraft.client.renderer.state.gui.GuiElementRenderState> ELEMENT_SORT_COMPARATOR;
    private final java.util.Map<java.lang.Object, net.minecraft.client.gui.render.pip.OversizedItemRenderer> oversizedItemRenderers;
    private final net.minecraft.client.renderer.state.gui.GuiRenderState renderState;
    private final java.util.List<net.minecraft.client.gui.render.GuiRenderer$Draw> draws;
    private final net.minecraft.client.renderer.StagedVertexBuffer vertexBuffer;
    private int firstDrawIndexAfterBlur;
    private final net.minecraft.client.renderer.Projection guiProjection;
    private final net.minecraft.client.renderer.ProjectionMatrixBuffer guiProjectionMatrixBuffer;
    private final net.minecraft.client.renderer.feature.FeatureRenderDispatcher featureRenderDispatcher;
    private final java.util.Map<java.lang.Class<? extends net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState>, net.minecraft.client.gui.render.pip.PictureInPictureRenderer<?>> pictureInPictureRenderers;
    private net.minecraft.client.gui.render.GuiItemAtlas itemAtlas;
    private int cachedGuiScale;
    private final net.minecraft.client.renderer.CubeMap cubeMap;
    private net.minecraft.client.gui.navigation.ScreenRectangle previousScissorArea;
    private com.mojang.renderpearl.api.pipeline.RenderPipeline previousPipeline;
    private net.minecraft.client.gui.render.TextureSetup previousTextureSetup;
    private net.minecraft.client.renderer.StagedVertexBuffer$Draw previousDraw;
    public net.minecraft.client.gui.render.GuiRenderer(net.minecraft.client.renderer.state.gui.GuiRenderState, net.minecraft.client.renderer.feature.FeatureRenderDispatcher, java.util.List<net.minecraft.client.gui.render.pip.PictureInPictureRenderer<?>>);
    public void endFrame();
    public void render();
    private void clearUnusedOversizedItemRenderers();
    private void prepare();
    private void addElementsToMeshes(net.minecraft.client.renderer.state.gui.GuiRenderState$TraverseRange);
    private void draw();
    private void executeDrawRange(java.util.function.Supplier<java.lang.String>, com.mojang.blaze3d.pipeline.RenderTarget, com.mojang.renderpearl.api.buffers.GpuBufferSlice, int, int);
    private void addElementToMesh(net.minecraft.client.renderer.state.gui.GuiElementRenderState);
    private void prepareText();
    private void prepareItemElements();
    private void preparePictureInPicture();
    private <T extends net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState> void preparePictureInPictureState(T, int);
    private void submitBlitFromItemAtlas(net.minecraft.client.renderer.state.gui.GuiItemRenderState, net.minecraft.client.gui.render.GuiItemAtlas$SlotView);
    private net.minecraft.client.gui.render.GuiItemAtlas prepareItemAtlas(java.util.Set<java.lang.Object>, int);
    private int getGuiScaleInvalidatingItemAtlasIfChanged();
    private void invalidateItemAtlas();
    private void executeDraw(net.minecraft.client.gui.render.GuiRenderer$Draw, com.mojang.renderpearl.api.commands.RenderPass);
    private boolean scissorChanged(net.minecraft.client.gui.navigation.ScreenRectangle, net.minecraft.client.gui.navigation.ScreenRectangle);
    private void enableScissor(net.minecraft.client.gui.navigation.ScreenRectangle, com.mojang.renderpearl.api.commands.RenderPass);
    public void registerPanoramaTextures(net.minecraft.client.renderer.texture.TextureManager);
    public void close();
    private void lambda$preparePictureInPicture$0(int, net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState);
    private void lambda$prepareItemElements$1(int, net.minecraft.client.renderer.state.gui.GuiItemRenderState);
    private static net.minecraft.client.gui.render.pip.OversizedItemRenderer lambda$prepareItemElements$2(java.lang.Object);
    private void lambda$prepareItemElements$0(org.apache.commons.lang3.mutable.MutableBoolean, net.minecraft.client.gui.render.GuiItemAtlas, net.minecraft.client.renderer.state.gui.GuiItemRenderState);
    private void lambda$prepareText$0(net.minecraft.client.renderer.state.gui.GuiTextRenderState);
    private static java.lang.String lambda$draw$1();
    private static java.lang.String lambda$draw$0();
    private static java.lang.String lambda$new$0();
    static {};
}
```
