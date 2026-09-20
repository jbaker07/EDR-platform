---
type: "interface"
fqcn: "com.mojang.blaze3d.systems.RenderSystem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.systems.RenderSystem

System: [[20-Systems/com.mojang.blaze3d.systems|com.mojang.blaze3d.systems]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getDevice` | `()Lcom/mojang/renderpearl/api/device/GpuDevice;` | exact | invokestatic@32 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (27 fields, 48 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final DEFAULT_DEPTH_CLEAR_VALUE : D
public static final MINIMUM_ATLAS_TEXTURE_SIZE : I
public static final PROJECTION_MATRIX_UBO_SIZE : I
private static renderThread : Ljava/lang/Thread;
private static DEVICE : Lcom/mojang/renderpearl/api/device/GpuDevice;
private static BACKEND : Lcom/mojang/renderpearl/api/device/GpuBackend;
private static final sharedSequential : Lcom/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer;
private static final sharedSequentialQuad : Lcom/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer;
private static final sharedSequentialLines : Lcom/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer;
private static projectionType : Lcom/mojang/blaze3d/ProjectionType;
private static savedProjectionType : Lcom/mojang/blaze3d/ProjectionType;
private static final modelViewStack : Lorg/joml/Matrix4fStack;
private static shaderFog : Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;
private static shaderLightDirections : Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;
private static projectionMatrixBuffer : Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;
private static savedProjectionMatrixBuffer : Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;
private static final pollEventsWaitStart : Ljava/util/concurrent/atomic/AtomicLong;
private static final pollingEvents : Ljava/util/concurrent/atomic/AtomicBoolean;
private static final PENDING_FENCES : Lnet/minecraft/util/ArrayListDeque;
public static isRenderingLevel : Z
private static globalSettingsUniform : Lcom/mojang/renderpearl/api/buffers/GpuBuffer;
private static dynamicGpuData : Lnet/minecraft/client/renderer/DynamicGpuData;
private static final scissorStateForRenderTypeDraws : Lcom/mojang/blaze3d/systems/ScissorState;
private static final samplerCache : Lcom/mojang/blaze3d/systems/SamplerCache;
private static fallbackPipelineCache : Lcom/mojang/blaze3d/pipeline/PipelineCache;
private static currentPipelineCache : Lcom/mojang/blaze3d/pipeline/PipelineCache;
public <init>()V
public static getSamplerCache()Lcom/mojang/blaze3d/systems/SamplerCache;
public static setFallbackPipelineCache(Lcom/mojang/blaze3d/pipeline/PipelineCache;)V
public static setCurrentPipelineCache(Lcom/mojang/blaze3d/pipeline/PipelineCache;)Lcom/mojang/blaze3d/pipeline/PipelineCache;
public static getCompiledPipelineNullable(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;)Lcom/mojang/renderpearl/api/pipeline/CompiledRenderPipeline;
public static getCompiledPipeline(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;)Lcom/mojang/renderpearl/api/pipeline/CompiledRenderPipeline;
public static initRenderThread()V
public static isOnRenderThread()Z
public static assertOnRenderThread()V
private static constructThreadException()Ljava/lang/IllegalStateException;
public static pollEvents(Lcom/mojang/blaze3d/platform/SDLEventHandler;)V
public static isFrozenAtPollEvents()Z
public static pumpEvents(Lcom/mojang/blaze3d/platform/SDLEventHandler;)V
public static setShaderFog(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;)V
public static getShaderFog()Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;
public static setShaderLights(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;)V
public static getShaderLights()Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;
public static enableScissorForRenderTypeDraws(IIII)V
public static disableScissorForRenderTypeDraws()V
public static getScissorStateForRenderTypeDraws()Lcom/mojang/blaze3d/systems/ScissorState;
public static getBackendDescription()Ljava/lang/String;
public static initBackendSystem()Lnet/minecraft/util/TimeSource$NanoTimeSource;
public static initRenderer(Lcom/mojang/renderpearl/api/device/GpuDevice;)V
public static shutdownRenderer()V
public static trackBackendLibraryForShutdown(Lcom/mojang/renderpearl/api/device/GpuBackend;)V
public static unloadTrackedBackendLibrary()V
public static setupDefaultState()V
public static setProjectionMatrix(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;Lcom/mojang/blaze3d/ProjectionType;)V
public static backupProjectionMatrix()V
public static restoreProjectionMatrix()V
public static getProjectionMatrixBuffer()Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;
public static getModelViewMatrixCopy()Lorg/joml/Matrix4f;
public static getModelViewStack()Lorg/joml/Matrix4fStack;
public static getSequentialBuffer(Lcom/mojang/renderpearl/api/pipeline/PrimitiveTopology;)Lcom/mojang/blaze3d/systems/RenderSystem$AutoStorageIndexBuffer;
public static setGlobalSettingsUniform(Lcom/mojang/renderpearl/api/buffers/GpuBuffer;)V
public static getGlobalSettingsUniform()Lcom/mojang/renderpearl/api/buffers/GpuBuffer;
public static getProjectionType()Lcom/mojang/blaze3d/ProjectionType;
public static queueFencedTask(Ljava/lang/Runnable;)V
public static executePendingTasks()V
public static getDevice()Lcom/mojang/renderpearl/api/device/GpuDevice;
public static tryGetDevice()Lcom/mojang/renderpearl/api/device/GpuDevice;
public static isWireframeAvailable()Z
public static getDynamicUniforms()Lnet/minecraft/client/renderer/DynamicGpuData;
public static bindDefaultUniforms(Lcom/mojang/renderpearl/api/commands/RenderPass;)V
public static resizeAllAutoStorageIndexBuffers()V
private static synthetic lambda$static$1(Lit/unimi/dsi/fastutil/ints/IntConsumer;I)V
private static synthetic lambda$static$0(Lit/unimi/dsi/fastutil/ints/IntConsumer;I)V
static <clinit>()V
```
