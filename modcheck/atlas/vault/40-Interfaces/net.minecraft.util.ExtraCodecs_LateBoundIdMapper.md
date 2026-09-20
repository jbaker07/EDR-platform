---
type: "interface"
fqcn: "net.minecraft.util.ExtraCodecs$LateBoundIdMapper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.ExtraCodecs$LateBoundIdMapper

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@4 in `CustomUnbakedBlockStateModelRegistry.<clinit>` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `codec` | `(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;` | exact | invokevirtual@16 in `CustomUnbakedBlockStateModelRegistry.<clinit>` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `put` | `(Ljava/lang/Object;Ljava/lang/Object;)Lnet/minecraft/util/ExtraCodecs$` | exact | invokevirtual@5 in `CustomUnbakedBlockStateModelRegistry.register` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `put` | `(Ljava/lang/Object;Ljava/lang/Object;)Lnet/minecraft/util/ExtraCodecs$` | exact | invokevirtual@19 in `SpriteSourceRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final idToValue : Lcom/google/common/collect/BiMap;
public <init>()V
public codec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public put(Ljava/lang/Object;Ljava/lang/Object;)Lnet/minecraft/util/ExtraCodecs$LateBoundIdMapper;
public values()Ljava/util/Set;
private static synthetic lambda$put$0(Ljava/lang/Object;)Ljava/lang/String;
```
