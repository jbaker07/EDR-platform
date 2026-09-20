---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.WeightedVariants$Unbaked

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/client/renderer/block/dispatch/BlockStateModel$Unbaked`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/util/random/WeightedList;)V` | exact | invokespecial@16 in `CustomUnbakedBlockStateModelRegistry.lambda$static$2` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `entries` | `()Lnet/minecraft/util/random/WeightedList;` | exact | invokevirtual@1 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (1 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entries : Lnet/minecraft/util/random/WeightedList;
public <init>(Lnet/minecraft/util/random/WeightedList;)V
public bake(Lnet/minecraft/client/resources/model/ModelBaker;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;
public resolveDependencies(Lnet/minecraft/client/resources/model/ResolvableModel$Resolver;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public entries()Lnet/minecraft/util/random/WeightedList;
private static synthetic lambda$resolveDependencies$0(Lnet/minecraft/client/resources/model/ResolvableModel$Resolver;Lnet/minecraft/util/random/Weighted;)V
private static synthetic lambda$bake$0(Lnet/minecraft/client/resources/model/ModelBaker;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel$Unbaked;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;
```
