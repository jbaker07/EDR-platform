---
type: "interface"
fqcn: "net.minecraft.world.level.saveddata.SavedDataType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.saveddata.SavedDataType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;Ljava/util/function/Supplier;Lcom` | exact | invokespecial@35 in `MinecraftServerMixin.initGlobalAttachments` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/resources/Identifier;Ljava/util/function/Supplier;Lcom` | exact | invokespecial@23 in `ServerLevelMixin.createAttachmentsPersistentState` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (4 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Lnet/minecraft/resources/Identifier;
private final constructor : Ljava/util/function/Supplier;
private final codec : Lcom/mojang/serialization/Codec;
private final dataFixType : Lnet/minecraft/util/datafix/DataFixTypes;
public <init>(Lnet/minecraft/resources/Identifier;Ljava/util/function/Supplier;Lcom/mojang/serialization/Codec;Lnet/minecraft/util/datafix/DataFixTypes;)V
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
public id()Lnet/minecraft/resources/Identifier;
public constructor()Ljava/util/function/Supplier;
public codec()Lcom/mojang/serialization/Codec;
public dataFixType()Lnet/minecraft/util/datafix/DataFixTypes;
```
