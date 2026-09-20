---
type: "system"
package: "com.mojang.renderpearl.backend"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.backend

151 classes (76 top-level) across 8 packages in the processed jar; 0 changed by Loom processing; 2 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/com.mojang.renderpearl.backend.opengl.GlCommandEncoder|GlCommandEncoder]] -- wraps:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/com.mojang.renderpearl.backend.vulkan.VulkanGpuSurface|VulkanGpuSurface]] -- wraps:1 -- by fabric-client-gametest-api-v1

## Declared inventory

### `com.mojang.renderpearl.backend` (1 top-level)

`package-info`

### `com.mojang.renderpearl.backend.api` (7 top-level)

`BackendRenderPipeline`, `CommandEncoderBackend`, `GpuDeviceBackend`, `GpuSurfaceBackend`, `RenderPassBackend`, `SpvModule`, `package-info`

### `com.mojang.renderpearl.backend.common` (4 top-level)

`BaseGpuBuffer`, `BaseGpuTexture`, `BaseGpuTextureView`, `package-info`

### `com.mojang.renderpearl.backend.opengl` (29 top-level)

`BufferStorage`, `DirectStateAccess`, `FrameBufferAttachment`, `FrameBufferCache`, `GlBackend`, `GlBuffer`, [[40-Interfaces/com.mojang.renderpearl.backend.opengl.GlCommandEncoder|GlCommandEncoder]], `GlConst`, `GlDebug`, `GlDebugLabel`, `GlDevice`, `GlFence`, `GlHeuristics`, `GlPipelineRecompiler`, `GlProgram`, `GlQueryPool`, `GlRenderPass`, `GlRenderPipeline`, `GlSampler`, `GlShaderModule`, `GlStateManager`, `GlSurface`, `GlTexture`, `GlTextureView`, `GlTransientMemory`, `GlUtil`, `Uniform`, `VertexArray`, `package-info`

### `com.mojang.renderpearl.backend.util` (2 top-level)

`TransientBlockAllocator`, `package-info`

### `com.mojang.renderpearl.backend.vulkan` (23 top-level)

`Destroyable`, `DestructionQueue`, `VulkanBackend`, `VulkanCommandEncoder`, `VulkanCommandPool`, `VulkanConst`, `VulkanDebug`, `VulkanDevice`, `VulkanFeatureSets`, `VulkanGpuBuffer`, `VulkanGpuSampler`, [[40-Interfaces/com.mojang.renderpearl.backend.vulkan.VulkanGpuSurface|VulkanGpuSurface]], `VulkanGpuTexture`, `VulkanGpuTextureView`, `VulkanInstance`, `VulkanPhysicalDevice`, `VulkanQueryPool`, `VulkanQueue`, `VulkanRenderPass`, `VulkanRenderPipeline`, `VulkanTransientMemory`, `VulkanUtils`, `package-info`

### `com.mojang.renderpearl.backend.vulkan.checkpoints` (6 top-level)

`AbstractCheckpointStorage`, `AmdCheckpointExtension`, `CheckpointExtension`, `NoopCheckpointExtension`, `NvidiaCheckpointExtension`, `package-info`

### `com.mojang.renderpearl.backend.vulkan.init` (4 top-level)

`FeatureSet`, `VulkanFeature`, `VulkanPNextStruct`, `package-info`

