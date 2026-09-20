---
type: "interface"
fqcn: "net.minecraft.world.item.component.BlockTransformers"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.component.BlockTransformers

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `AXE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@14 in `BlockTransformerHelperImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `HOE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@38 in `BlockTransformerHelperImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `SHOVEL` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@62 in `BlockTransformerHelperImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (3 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SHOVEL : Lnet/minecraft/resources/ResourceKey;
public static final AXE : Lnet/minecraft/resources/ResourceKey;
public static final HOE : Lnet/minecraft/resources/ResourceKey;
public <init>()V
private static createKey(Ljava/lang/String;)Lnet/minecraft/resources/ResourceKey;
public static bootstrap(Lnet/minecraft/data/worldgen/BootstrapContext;)V
private static axeStrippables()Lnet/minecraft/core/component/BlockTransformer$BlockTransformData;
private static axe(Ljava/util/Set;Lnet/minecraft/core/Holder;Lnet/minecraft/core/component/BlockTransformer$TransformParticle;)Ljava/util/List;
static <clinit>()V
```
