---
type: "interface"
fqcn: "net.minecraft.data.HashCache$ProviderCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.HashCache$ProviderCache

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

`record` final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `save` | `(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/lang/String;)V` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `save` | `(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/lang/String;)V` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (2 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final version : Ljava/lang/String;
private final data : Lcom/google/common/collect/ImmutableMap;
private <init>(Ljava/lang/String;Lcom/google/common/collect/ImmutableMap;)V
public get(Ljava/nio/file/Path;)Lcom/google/common/hash/HashCode;
public count()I
public static load(Ljava/nio/file/Path;Ljava/nio/file/Path;)Lnet/minecraft/data/HashCache$ProviderCache;
public save(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/lang/String;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public version()Ljava/lang/String;
public data()Lcom/google/common/collect/ImmutableMap;
private static synthetic lambda$load$0(Lcom/google/common/collect/ImmutableMap$Builder;Ljava/nio/file/Path;Ljava/lang/String;)V
```
