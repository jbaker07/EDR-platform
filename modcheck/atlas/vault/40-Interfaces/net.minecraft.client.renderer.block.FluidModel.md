---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.FluidModel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.FluidModel

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `tintSource` | `()Lnet/minecraft/client/color/block/BlockTintSource;` | exact | invokevirtual@29 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `tintSource` | `()Lnet/minecraft/client/color/block/BlockTintSource;` | exact | invokevirtual@47 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `tintSource` | `()Lnet/minecraft/client/color/block/BlockTintSource;` | exact | invokevirtual@66 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (5 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final layer : Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
private final stillMaterial : Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private final flowingMaterial : Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private final overlayMaterial : Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private final tintSource : Lnet/minecraft/client/color/block/BlockTintSource;
public <init>(Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;Lnet/minecraft/client/resources/model/sprite/Material$Baked;Lnet/minecraft/client/resources/model/sprite/Material$Baked;Lnet/minecraft/client/resources/model/sprite/Material$Baked;Lnet/minecraft/client/color/block/BlockTintSource;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public layer()Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
public stillMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public flowingMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public overlayMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public tintSource()Lnet/minecraft/client/color/block/BlockTintSource;
```
