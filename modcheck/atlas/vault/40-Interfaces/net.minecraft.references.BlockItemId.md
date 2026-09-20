---
type: "interface"
fqcn: "net.minecraft.references.BlockItemId"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.references.BlockItemId

System: [[20-Systems/net.minecraft.references|net.minecraft.references]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `block` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `FabricTagsProvider$BlockTagsProvider$1.convertElement` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `item` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `FabricTagsProvider$ItemTagsProvider$1.convertElement` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (2 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final block : Lnet/minecraft/resources/ResourceKey;
private final item : Lnet/minecraft/resources/ResourceKey;
public <init>(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;)V
public static create(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/references/BlockItemId;
public static create(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/references/BlockItemId;
public static create(Ljava/lang/String;)Lnet/minecraft/references/BlockItemId;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public block()Lnet/minecraft/resources/ResourceKey;
public item()Lnet/minecraft/resources/ResourceKey;
```
