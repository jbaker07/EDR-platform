---
type: "interface"
fqcn: "com.mojang.renderpearl.api.device.GpuDevice"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.api.device.GpuDevice

System: [[20-Systems/com.mojang.renderpearl.api|com.mojang.renderpearl.api]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createCommandEncoder` | `()Lcom/mojang/renderpearl/api/commands/CommandEncoder;` | exact | invokeinterface@35 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (0 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract createSurface(JLjava/util/function/BooleanSupplier;)Lcom/mojang/renderpearl/api/device/GpuSurface;
public abstract createCommandEncoder()Lcom/mojang/renderpearl/api/commands/CommandEncoder;
public abstract createSampler(Lcom/mojang/renderpearl/api/textures/AddressMode;Lcom/mojang/renderpearl/api/textures/AddressMode;Lcom/mojang/renderpearl/api/textures/FilterMode;Lcom/mojang/renderpearl/api/textures/FilterMode;ILjava/util/OptionalDouble;)Lcom/mojang/renderpearl/api/textures/GpuSampler;
public abstract createTexture(Ljava/util/function/Supplier;ILcom/mojang/renderpearl/api/GpuFormat;IIII)Lcom/mojang/renderpearl/api/textures/GpuTexture;
public abstract createTexture(Ljava/lang/String;ILcom/mojang/renderpearl/api/GpuFormat;IIII)Lcom/mojang/renderpearl/api/textures/GpuTexture;
public abstract createTextureView(Lcom/mojang/renderpearl/api/textures/GpuTexture;)Lcom/mojang/renderpearl/api/textures/GpuTextureView;
public abstract createTextureView(Lcom/mojang/renderpearl/api/textures/GpuTexture;II)Lcom/mojang/renderpearl/api/textures/GpuTextureView;
public abstract createBuffer(Ljava/util/function/Supplier;IJ)Lcom/mojang/renderpearl/api/buffers/GpuBuffer;
public abstract createBuffer(Ljava/util/function/Supplier;ILjava/nio/ByteBuffer;)Lcom/mojang/renderpearl/api/buffers/GpuBuffer;
public abstract getLastDebugMessages()Ljava/util/List;
public abstract isDebuggingEnabled()Z
public abstract compilePipeline(Lcom/mojang/renderpearl/api/pipeline/RenderPipeline;Lcom/mojang/renderpearl/api/pipeline/ShaderSource;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
public abstract close()V
public abstract createTimestampQueryPool(I)Lcom/mojang/renderpearl/api/commands/GpuQueryPool;
public abstract getDeviceInfo()Lcom/mojang/renderpearl/api/device/DeviceInfo;
```
