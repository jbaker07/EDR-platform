---
type: "interface"
fqcn: "com.mojang.renderpearl.backend.opengl.GlCommandEncoder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.backend.opengl.GlCommandEncoder

System: [[20-Systems/com.mojang.renderpearl.backend|com.mojang.renderpearl.backend]]

`class` ; extends `java/lang/Object`; implements `com/mojang/renderpearl/backend/api/CommandEncoderBackend`, `com/mojang/renderpearl/util/UncheckedAutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `presentTexture` | `(JLcom/mojang/renderpearl/api/textures/GpuTextureView;II)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (13 fields, 28 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final MAX_SUBMITS_IN_FLIGHT : I
private static final NO_FENCE : J
private final device : Lcom/mojang/renderpearl/backend/opengl/GlDevice;
private final transientMemory : Lcom/mojang/renderpearl/backend/opengl/GlTransientMemory;
private final fences : [J
private currentSubmitIndex : J
private final readFbo : I
private final drawFbo : I
private lastPipeline : Lcom/mojang/renderpearl/backend/opengl/GlRenderPipeline;
private lastProgram : Lcom/mojang/renderpearl/backend/opengl/GlProgram;
private final renderPassColorTextures : Ljava/util/List;
static final synthetic $assertionsDisabled : Z
protected <init>(Lcom/mojang/renderpearl/backend/opengl/GlDevice;)V
public close()V
public currentSubmitIndex()J
public currentSubmitSlot()I
public submit()V
public awaitSubmit(JJ)Z
public transientMemory()Lcom/mojang/renderpearl/api/buffers/TransientMemory;
public createRenderPass(Lcom/mojang/renderpearl/api/commands/RenderPassDescriptor;)Lcom/mojang/renderpearl/backend/api/RenderPassBackend;
public clearColorTexture(Lcom/mojang/renderpearl/api/textures/GpuTexture;Lorg/joml/Vector4fc;)V
public clearColorAndDepthTextures(Lcom/mojang/renderpearl/api/textures/GpuTexture;Lorg/joml/Vector4fc;Lcom/mojang/renderpearl/api/textures/GpuTexture;D)V
public clearColorAndDepthTextures(Lcom/mojang/renderpearl/api/textures/GpuTexture;Lorg/joml/Vector4fc;Lcom/mojang/renderpearl/api/textures/GpuTexture;DIIIII)V
public clearDepthTexture(Lcom/mojang/renderpearl/api/textures/GpuTexture;D)V
public writeToBuffer(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;Ljava/nio/ByteBuffer;)V
public copyToBuffer(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;)V
public writeToTexture(Lcom/mojang/renderpearl/api/textures/GpuTexture;Ljava/nio/ByteBuffer;IIIIII)V
public copyBufferToTexture(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;IIIILcom/mojang/renderpearl/api/textures/GpuTexture;IIIIII)V
public copyTextureToBuffer(Lcom/mojang/renderpearl/api/textures/GpuTexture;Lcom/mojang/renderpearl/api/buffers/GpuBuffer;JLjava/lang/Runnable;I)V
public copyTextureToBuffer(Lcom/mojang/renderpearl/api/textures/GpuTexture;Lcom/mojang/renderpearl/api/buffers/GpuBuffer;JLjava/lang/Runnable;IIIII)V
public copyTextureToTexture(Lcom/mojang/renderpearl/api/textures/GpuTexture;Lcom/mojang/renderpearl/api/textures/GpuTexture;IIIIIII)V
 presentTexture(JLcom/mojang/renderpearl/api/textures/GpuTextureView;II)V
public createFence()Lcom/mojang/renderpearl/api/commands/GpuFence;
protected executeDraw(Lcom/mojang/renderpearl/backend/opengl/GlRenderPass;IIILcom/mojang/renderpearl/api/pipeline/IndexType;II)V
public executeDraws(Lcom/mojang/renderpearl/backend/opengl/GlRenderPass;Lcom/mojang/renderpearl/api/pipeline/IndexType;Lorg/lwjgl/PointerBuffer;Ljava/nio/IntBuffer;Ljava/nio/IntBuffer;I)V
protected executeDrawIndirect(Lcom/mojang/renderpearl/backend/opengl/GlRenderPass;Lcom/mojang/renderpearl/api/pipeline/IndexType;Lcom/mojang/renderpearl/backend/opengl/GlBuffer;JI)V
private setupDraw(Lcom/mojang/renderpearl/backend/opengl/GlRenderPass;)V
public submitRenderPass()V
public writeTimestamp(Lcom/mojang/renderpearl/api/commands/GpuQueryPool;I)V
static <clinit>()V
```
