---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.WeightedVariants"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.WeightedVariants

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/block/dispatch/BlockStateModel`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `list` | `Lnet/minecraft/util/random/WeightedList;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |

## Declared members (3 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final list : Lnet/minecraft/util/random/WeightedList;
private final particleMaterial : Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private final materialFlags : I
public <init>(Lnet/minecraft/util/random/WeightedList;)V
private static computeMaterialFlags(Lnet/minecraft/util/random/WeightedList;)I
public particleMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public materialFlags()I
public collectParts(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V
```
