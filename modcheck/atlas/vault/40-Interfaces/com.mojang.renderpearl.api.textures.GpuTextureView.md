---
type: "interface"
fqcn: "com.mojang.renderpearl.api.textures.GpuTextureView"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.renderpearl.api.textures.GpuTextureView

System: [[20-Systems/com.mojang.renderpearl.api|com.mojang.renderpearl.api]]

`interface` public abstract; extends `java/lang/Object`; implements `com/mojang/renderpearl/util/UncheckedAutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `texture` | `()Lcom/mojang/renderpearl/api/textures/GpuTexture;` | exact | invokeinterface@2 in `GlCommandEncoderMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `texture` | `()Lcom/mojang/renderpearl/api/textures/GpuTexture;` | exact | invokeinterface@2 in `VulkanGpuSurfaceMixin.blitFrameBuffer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (0 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract isClosed()Z
public abstract texture()Lcom/mojang/renderpearl/api/textures/GpuTexture;
public abstract baseMipLevel()I
public abstract mipLevels()I
public abstract getWidth(I)I
public abstract getHeight(I)I
```
