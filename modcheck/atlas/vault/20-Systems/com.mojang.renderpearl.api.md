---
type: "system"
package: "com.mojang.renderpearl.api"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.api

73 classes (53 top-level) across 7 packages in the processed jar; 3 changed by Loom processing; 7 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/com.mojang.renderpearl.api.commands.CommandEncoder|CommandEncoder]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/com.mojang.renderpearl.api.device.GpuDevice|GpuDevice]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/com.mojang.renderpearl.api.pipeline.PrimitiveTopology|PrimitiveTopology]] -- reads:1 -- by fabric-rendering-v1
- [[40-Interfaces/com.mojang.renderpearl.api.pipeline.RenderPipeline|RenderPipeline]] -- calls:3 -- by fabric-rendering-v1
- [[40-Interfaces/com.mojang.renderpearl.api.pipeline.RenderPipeline_Builder|RenderPipeline$Builder]] -- calls:2, injects_into:2, wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/com.mojang.renderpearl.api.pipeline.RenderPipeline_Snippet|RenderPipeline$Snippet]] -- injects_into:3 -- by fabric-rendering-v1
- [[40-Interfaces/com.mojang.renderpearl.api.textures.GpuTextureView|GpuTextureView]] -- calls:2 -- by fabric-client-gametest-api-v1

## Declared inventory

### `com.mojang.renderpearl.api` (2 top-level)

`GpuFormat`, `package-info`

### `com.mojang.renderpearl.api.buffers` (4 top-level)

`GpuBuffer`, `GpuBufferSlice`, `TransientMemory`, `package-info`

### `com.mojang.renderpearl.api.commands` (7 top-level)

[[40-Interfaces/com.mojang.renderpearl.api.commands.CommandEncoder|CommandEncoder]], `GpuFence`, `GpuQuery`, `GpuQueryPool`, `RenderPass`, `RenderPassDescriptor`, `package-info`

### `com.mojang.renderpearl.api.device` (14 top-level)

`BackendCreationException`, `DeviceFeatures`, `DeviceInfo`, `DeviceLimits`, `DeviceType`, `GpuBackend`, `GpuDebugOptions`, [[40-Interfaces/com.mojang.renderpearl.api.device.GpuDevice|GpuDevice]], `GpuDeviceLossException`, `GpuOutOfMemoryException`, `GpuSurface`, `HintsAndWorkarounds`, `SurfaceException`, `package-info`

### `com.mojang.renderpearl.api.pipeline` (17 top-level)

`BindGroupLayout`, `BlendEquation`, `BlendFactor`, `BlendFunction`, `BlendOp`, `ColorTargetState`, `CompareOp`, `CompiledRenderPipeline`, `DepthStencilState`, `IndexType`, `PolygonMode`, [[40-Interfaces/com.mojang.renderpearl.api.pipeline.PrimitiveTopology|PrimitiveTopology]], [[40-Interfaces/com.mojang.renderpearl.api.pipeline.RenderPipeline|RenderPipeline]], `ShaderSource`, `ShaderType`, `UniformType`, `package-info`

### `com.mojang.renderpearl.api.textures` (6 top-level)

`AddressMode`, `FilterMode`, `GpuSampler`, `GpuTexture`, [[40-Interfaces/com.mojang.renderpearl.api.textures.GpuTextureView|GpuTextureView]], `package-info`

### `com.mojang.renderpearl.api.vertex` (3 top-level)

`VertexFormat`, `VertexFormatElement`, `package-info`

