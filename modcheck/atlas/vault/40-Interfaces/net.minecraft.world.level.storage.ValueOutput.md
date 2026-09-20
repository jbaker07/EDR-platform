---
type: "interface"
fqcn: "net.minecraft.world.level.storage.ValueOutput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.ValueOutput

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/serialization/v1/value/FabricValueOutput`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `putLong` | `(Ljava/lang/String;J)V` | exact | invokeinterface@20 in `SingleVariantStorage.writeValue` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `store` | `(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)` | exact | invokeinterface@78 in `AttachmentSerializingImpl.serializeAttachmentData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `store` | `(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)` | exact | invokeinterface@9 in `FabricValueOutput.putLongArray` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `store` | `(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)` | exact | invokeinterface@9 in `FabricValueOutput.putByteArray` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `store` | `(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)` | exact | invokeinterface@8 in `SingleVariantStorage.writeValue` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (0 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract store(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)V
public abstract storeNullable(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)V
public abstract store(Lcom/mojang/serialization/MapCodec;Ljava/lang/Object;)V
public abstract putBoolean(Ljava/lang/String;Z)V
public abstract putByte(Ljava/lang/String;B)V
public abstract putShort(Ljava/lang/String;S)V
public abstract putInt(Ljava/lang/String;I)V
public abstract putLong(Ljava/lang/String;J)V
public abstract putFloat(Ljava/lang/String;F)V
public abstract putDouble(Ljava/lang/String;D)V
public abstract putString(Ljava/lang/String;Ljava/lang/String;)V
public abstract putIntArray(Ljava/lang/String;[I)V
public abstract child(Ljava/lang/String;)Lnet/minecraft/world/level/storage/ValueOutput;
public abstract childrenList(Ljava/lang/String;)Lnet/minecraft/world/level/storage/ValueOutput$ValueOutputList;
public abstract list(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Lnet/minecraft/world/level/storage/ValueOutput$TypedOutputList;
public abstract discard(Ljava/lang/String;)V
public abstract isEmpty()Z
```
