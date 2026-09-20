---
type: "interface"
fqcn: "net.minecraft.client.gui.render.pip.PictureInPictureRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.render.pip.PictureInPictureRenderer

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `close` | `()V` | exact | invokevirtual@53 in `PictureInPictureRendererPool.cleanUpUnusedRenderers` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getRenderStateClass` | `()Ljava/lang/Class;` | exact | invokevirtual@58 in `PictureInPictureRendererRegistryImpl.onReady` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getRenderStateClass` | `()Ljava/lang/Class;` | exact | invokevirtual@74 in `PictureInPictureRendererRegistryImpl.onReady` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getRenderStateClass` | `()Ljava/lang/Class;` | exact | invokevirtual@18 in `GuiRendererMixin.substituteSpecialElementRenderer` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (7 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private texture : Lcom/mojang/renderpearl/api/textures/GpuTexture;
private textureView : Lcom/mojang/renderpearl/api/textures/GpuTextureView;
private depthTexture : Lcom/mojang/renderpearl/api/textures/GpuTexture;
private depthTextureView : Lcom/mojang/renderpearl/api/textures/GpuTextureView;
private final projection : Lnet/minecraft/client/renderer/Projection;
private final projectionMatrixBuffer : Lnet/minecraft/client/renderer/ProjectionMatrixBuffer;
private final submitNodeStorage : Lnet/minecraft/client/renderer/SubmitNodeStorage;
public <init>()V
public prepare(Lnet/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState;Lnet/minecraft/client/renderer/state/gui/GuiRenderState;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher;I)V
protected blitTexture(Lnet/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState;Lnet/minecraft/client/renderer/state/gui/GuiRenderState;)V
private prepareTexturesAndProjection(ZII)V
protected textureIsReadyToBlit(Lnet/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState;)Z
protected getTranslateY(II)F
public close()V
public abstract getRenderStateClass()Ljava/lang/Class;
protected abstract renderToTexture(Lnet/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;)V
protected abstract getTextureLabel()Ljava/lang/String;
private synthetic lambda$prepareTexturesAndProjection$1()Ljava/lang/String;
private synthetic lambda$prepareTexturesAndProjection$0()Ljava/lang/String;
private static synthetic lambda$prepare$0()Ljava/lang/String;
```
