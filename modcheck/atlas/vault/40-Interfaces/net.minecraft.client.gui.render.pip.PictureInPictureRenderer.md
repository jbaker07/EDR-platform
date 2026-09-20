---
type: "interface"
fqcn: "net.minecraft.client.gui.render.pip.PictureInPictureRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.render.pip.PictureInPictureRenderer

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `close()V` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getRenderStateClass()Ljava/lang/Class;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getRenderStateClass()Ljava/lang/Class;` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.gui.render.pip.PictureInPictureRenderer<T extends net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState> implements java.lang.AutoCloseable {
    private com.mojang.renderpearl.api.textures.GpuTexture texture;
    private com.mojang.renderpearl.api.textures.GpuTextureView textureView;
    private com.mojang.renderpearl.api.textures.GpuTexture depthTexture;
    private com.mojang.renderpearl.api.textures.GpuTextureView depthTextureView;
    private final net.minecraft.client.renderer.Projection projection;
    private final net.minecraft.client.renderer.ProjectionMatrixBuffer projectionMatrixBuffer;
    private final net.minecraft.client.renderer.SubmitNodeStorage submitNodeStorage;
    public net.minecraft.client.gui.render.pip.PictureInPictureRenderer();
    public void prepare(T, net.minecraft.client.renderer.state.gui.GuiRenderState, net.minecraft.client.renderer.feature.FeatureRenderDispatcher, int);
    protected void blitTexture(T, net.minecraft.client.renderer.state.gui.GuiRenderState);
    private void prepareTexturesAndProjection(boolean, int, int);
    protected boolean textureIsReadyToBlit(T);
    protected float getTranslateY(int, int);
    public void close();
    public abstract java.lang.Class<T> getRenderStateClass();
    protected abstract void renderToTexture(T, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector);
    protected abstract java.lang.String getTextureLabel();
    private java.lang.String lambda$prepareTexturesAndProjection$1();
    private java.lang.String lambda$prepareTexturesAndProjection$0();
    private static java.lang.String lambda$prepare$0();
}
```
