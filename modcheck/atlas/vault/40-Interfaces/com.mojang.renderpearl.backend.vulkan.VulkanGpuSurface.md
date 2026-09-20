---
type: "interface"
fqcn: "com.mojang.renderpearl.backend.vulkan.VulkanGpuSurface"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.backend.vulkan.VulkanGpuSurface

System: [[20-Systems/com.mojang.renderpearl.backend|com.mojang.renderpearl.backend]]

`class` public; extends `java/lang/Object`; implements `com/mojang/renderpearl/backend/api/GpuSurfaceBackend`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `blitFromTexture` | `(Lcom/mojang/renderpearl/backend/api/CommandEncoderBackend;Lcom/mojang` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (18 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final NO_CURRENT_IMAGE : I
private final device : Lcom/mojang/renderpearl/backend/vulkan/VulkanDevice;
private final presentQueue : Lorg/lwjgl/vulkan/VkQueue;
private final surface : J
private final swapchainImageFormat : I
private swapchain : J
private swapchainWidth : I
private swapchainHeight : I
private final swapchainImages : Lit/unimi/dsi/fastutil/longs/LongList;
private final acquireSemaphores : [J
private currentAcquireSemaphore : I
private presentSemaphores : [J
private currentImageIndex : I
private eatenException : Lcom/mojang/renderpearl/api/device/SurfaceException;
private swapchainSuboptimal : Z
private swapchainOutOfDate : Z
private final supportedPresentModes : Ljava/util/Set;
static final synthetic $assertionsDisabled : Z
public <init>(Lcom/mojang/renderpearl/backend/vulkan/VulkanDevice;J)V
private convertPresentModes(Ljava/nio/IntBuffer;)Ljava/util/Set;
public supportedPresentModes()Ljava/util/Collection;
public pickSwapchainSurfaceFormat(Lorg/lwjgl/vulkan/VkSurfaceFormatKHR$Buffer;)Lorg/lwjgl/vulkan/VkSurfaceFormatKHR;
public static throwIfFailure(ILjava/lang/String;)V
public close()V
private destroySwapchain()V
public configure(Lcom/mojang/renderpearl/api/device/GpuSurface$Configuration;)V
public isSuboptimal()Z
public acquireNextTexture()V
public blitFromTexture(Lcom/mojang/renderpearl/backend/api/CommandEncoderBackend;Lcom/mojang/renderpearl/api/textures/GpuTextureView;)V
public present()V
static <clinit>()V
```
