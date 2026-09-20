---
type: "interface"
fqcn: "net.minecraft.world.level.storage.ValueInput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.ValueInput

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/serialization/v1/value/FabricValueInput`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getLongOr` | `(Ljava/lang/String;J)J` | exact | invokeinterface@25 in `SingleVariantStorage.readValue` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `read` | `(Lcom/mojang/serialization/MapCodec;)Ljava/util/Optional;` | exact | invokeinterface@7 in `FabricValueInput.keySet` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `read` | `(Lcom/mojang/serialization/MapCodec;)Ljava/util/Optional;` | exact | invokeinterface@8 in `FabricValueInput.contains` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `read` | `(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/util/Optiona` | exact | invokeinterface@14 in `AttachmentSerializingImpl.deserializeAttachmentData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `read` | `(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/util/Optiona` | exact | invokeinterface@8 in `FabricValueInput.getOptionalLongArray` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `read` | `(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/util/Optiona` | exact | invokeinterface@8 in `FabricValueInput.getOptionalByteArray` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `read` | `(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/util/Optiona` | exact | invokeinterface@5 in `SingleVariantStorage.readValue` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (0 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract read(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/util/Optional;
public abstract read(Lcom/mojang/serialization/MapCodec;)Ljava/util/Optional;
public abstract child(Ljava/lang/String;)Ljava/util/Optional;
public abstract childOrEmpty(Ljava/lang/String;)Lnet/minecraft/world/level/storage/ValueInput;
public abstract childrenList(Ljava/lang/String;)Ljava/util/Optional;
public abstract childrenListOrEmpty(Ljava/lang/String;)Lnet/minecraft/world/level/storage/ValueInput$ValueInputList;
public abstract list(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/util/Optional;
public abstract listOrEmpty(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Lnet/minecraft/world/level/storage/ValueInput$TypedInputList;
public abstract getBooleanOr(Ljava/lang/String;Z)Z
public abstract getByteOr(Ljava/lang/String;B)B
public abstract getShortOr(Ljava/lang/String;S)I
public abstract getInt(Ljava/lang/String;)Ljava/util/Optional;
public abstract getIntOr(Ljava/lang/String;I)I
public abstract getLongOr(Ljava/lang/String;J)J
public abstract getLong(Ljava/lang/String;)Ljava/util/Optional;
public abstract getFloatOr(Ljava/lang/String;F)F
public abstract getDoubleOr(Ljava/lang/String;D)D
public abstract getString(Ljava/lang/String;)Ljava/util/Optional;
public abstract getStringOr(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
public abstract getIntArray(Ljava/lang/String;)Ljava/util/Optional;
public abstract lookup()Lnet/minecraft/core/HolderLookup$Provider;
```
