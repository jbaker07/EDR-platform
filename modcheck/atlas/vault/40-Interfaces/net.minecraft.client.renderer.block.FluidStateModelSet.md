---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.FluidStateModelSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.FluidStateModelSet

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get` | `(Lnet/minecraft/world/level/material/FluidState;)Lnet/minecraft/client` | exact | invokevirtual@22 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| wraps | `bake` | `(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;)Ljava/uti` | name_only | @WrapMethod | client | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |

## Declared members (4 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final WATER_MODEL : Lnet/minecraft/client/renderer/block/FluidModel$Unbaked;
private static final LAVA_MODEL : Lnet/minecraft/client/renderer/block/FluidModel$Unbaked;
private final modelByFluid : Ljava/util/Map;
private final missingModel : Lnet/minecraft/client/renderer/block/FluidModel;
public <init>(Ljava/util/Map;Lnet/minecraft/client/renderer/block/FluidModel;)V
public static bake(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;)Ljava/util/Map;
public get(Lnet/minecraft/world/level/material/FluidState;)Lnet/minecraft/client/renderer/block/FluidModel;
private static synthetic lambda$bake$1()Ljava/lang/String;
private static synthetic lambda$bake$0()Ljava/lang/String;
static <clinit>()V
```
