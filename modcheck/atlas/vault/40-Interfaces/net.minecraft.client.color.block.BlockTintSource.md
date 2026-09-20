---
type: "interface"
fqcn: "net.minecraft.client.color.block.BlockTintSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.color.block.BlockTintSource

System: [[20-Systems/net.minecraft.client.color|net.minecraft.client.color]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `color` | `(Lnet/minecraft/world/level/block/state/BlockState;)I` | exact | invokeinterface@75 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `colorInWorld` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokeinterface@62 in `AltModelBlockRendererImpl.computeTintColor` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `colorInWorld` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokeinterface@58 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract color(Lnet/minecraft/world/level/block/state/BlockState;)I
public colorInWorld(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)I
public colorAsTerrainParticle(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)I
public relevantProperties()Ljava/util/Set;
```
