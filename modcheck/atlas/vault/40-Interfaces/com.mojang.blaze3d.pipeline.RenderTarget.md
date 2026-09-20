---
type: "interface"
fqcn: "com.mojang.blaze3d.pipeline.RenderTarget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.pipeline.RenderTarget

System: [[20-Systems/com.mojang.blaze3d.pipeline|com.mojang.blaze3d.pipeline]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getColorTexture` | `()Lcom/mojang/renderpearl/api/textures/GpuTexture;` | exact | invokevirtual@16 in `GlCommandEncoderMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getColorTexture` | `()Lcom/mojang/renderpearl/api/textures/GpuTexture;` | exact | invokevirtual@16 in `VulkanGpuSurfaceMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (10 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static UNNAMED_RENDER_TARGETS : I
public width : I
public height : I
protected final label : Ljava/lang/String;
protected final colorFormat : Lcom/mojang/renderpearl/api/GpuFormat;
protected final depthFormat : Lcom/mojang/renderpearl/api/GpuFormat;
protected colorTexture : Lcom/mojang/renderpearl/api/textures/GpuTexture;
protected colorTextureView : Lcom/mojang/renderpearl/api/textures/GpuTextureView;
protected depthTexture : Lcom/mojang/renderpearl/api/textures/GpuTexture;
protected depthTextureView : Lcom/mojang/renderpearl/api/textures/GpuTextureView;
public <init>(Ljava/lang/String;Lcom/mojang/renderpearl/api/GpuFormat;Lcom/mojang/renderpearl/api/GpuFormat;)V
public resize(II)V
public destroyBuffers()V
public copyDepthFrom(Lcom/mojang/blaze3d/pipeline/RenderTarget;)V
public copyColorFrom(Lcom/mojang/blaze3d/pipeline/RenderTarget;)V
public createBuffers(II)V
public blitAndBlendToTexture(Lcom/mojang/renderpearl/api/textures/GpuTextureView;Lcom/mojang/renderpearl/api/textures/GpuTextureView;)V
public getColorTexture()Lcom/mojang/renderpearl/api/textures/GpuTexture;
public getColorTextureView()Lcom/mojang/renderpearl/api/textures/GpuTextureView;
public getDepthTexture()Lcom/mojang/renderpearl/api/textures/GpuTexture;
public getDepthTextureView()Lcom/mojang/renderpearl/api/textures/GpuTextureView;
public hasDepth()Z
private static synthetic lambda$blitAndBlendToTexture$0()Ljava/lang/String;
private synthetic lambda$createBuffers$1()Ljava/lang/String;
private synthetic lambda$createBuffers$0()Ljava/lang/String;
static <clinit>()V
```
