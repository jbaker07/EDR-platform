---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.FluidModel$Unbaked"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.FluidModel$Unbaked

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `bake` | `(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;Lnet/minec` | exact | invokevirtual@111 in `FluidStateModelSetMixin.bake` | unknown | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |

## Declared members (4 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final stillMaterial : Lnet/minecraft/client/resources/model/sprite/Material;
private final flowingMaterial : Lnet/minecraft/client/resources/model/sprite/Material;
private final overlayMaterial : Lnet/minecraft/client/resources/model/sprite/Material;
private final tintSource : Lnet/minecraft/client/color/block/BlockTintSource;
public <init>(Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/color/block/BlockTintSource;)V
public bake(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;Lnet/minecraft/client/resources/model/ModelDebugName;)Lnet/minecraft/client/renderer/block/FluidModel;
private getAndValidateMaterial(Lnet/minecraft/client/resources/model/sprite/Material;Lnet/minecraft/client/resources/model/sprite/MaterialBaker;Ljava/lang/String;Lnet/minecraft/client/resources/model/ModelDebugName;)Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private static getTransparency(Lnet/minecraft/client/resources/model/sprite/Material$Baked;)Lcom/mojang/blaze3d/platform/Transparency;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public stillMaterial()Lnet/minecraft/client/resources/model/sprite/Material;
public flowingMaterial()Lnet/minecraft/client/resources/model/sprite/Material;
public overlayMaterial()Lnet/minecraft/client/resources/model/sprite/Material;
public tintSource()Lnet/minecraft/client/color/block/BlockTintSource;
```
