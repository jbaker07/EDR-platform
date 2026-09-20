---
type: "interface"
fqcn: "net.minecraft.world.level.storage.TagValueOutput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.TagValueOutput

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/storage/ValueOutput`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `buildResult` | `()Lnet/minecraft/nbt/CompoundTag;` | exact | invokevirtual@40 in `AttachmentSavedData$1.encode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `buildResult` | `()Lnet/minecraft/nbt/CompoundTag;` | exact | invokevirtual@34 in `SerializableChunkDataMixin.storeAttachmentNbtData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `createWithContext` | `(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/HolderLookup$` | exact | invokestatic@16 in `SerializableChunkDataMixin.storeAttachmentNbtData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `createWithoutContext` | `(Lnet/minecraft/util/ProblemReporter;)Lnet/minecraft/world/level/stora` | exact | invokestatic@18 in `AttachmentSavedData$1.encode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `output` | `Lnet/minecraft/nbt/CompoundTag;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | declared |

## Declared members (3 fields, 24 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final problemReporter : Lnet/minecraft/util/ProblemReporter;
private final ops : Lcom/mojang/serialization/DynamicOps;
private final output : Lnet/minecraft/nbt/CompoundTag;
private <init>(Lnet/minecraft/util/ProblemReporter;Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/nbt/CompoundTag;)V
public static createWithContext(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/level/storage/TagValueOutput;
public static createWithoutContext(Lnet/minecraft/util/ProblemReporter;)Lnet/minecraft/world/level/storage/TagValueOutput;
public store(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)V
public storeNullable(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)V
public store(Lcom/mojang/serialization/MapCodec;Ljava/lang/Object;)V
public putBoolean(Ljava/lang/String;Z)V
public putByte(Ljava/lang/String;B)V
public putShort(Ljava/lang/String;S)V
public putInt(Ljava/lang/String;I)V
public putLong(Ljava/lang/String;J)V
public putFloat(Ljava/lang/String;F)V
public putDouble(Ljava/lang/String;D)V
public putString(Ljava/lang/String;Ljava/lang/String;)V
public putIntArray(Ljava/lang/String;[I)V
private reporterForChild(Ljava/lang/String;)Lnet/minecraft/util/ProblemReporter;
public child(Ljava/lang/String;)Lnet/minecraft/world/level/storage/ValueOutput;
public childrenList(Ljava/lang/String;)Lnet/minecraft/world/level/storage/ValueOutput$ValueOutputList;
public list(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Lnet/minecraft/world/level/storage/ValueOutput$TypedOutputList;
public discard(Ljava/lang/String;)V
public isEmpty()Z
public buildResult()Lnet/minecraft/nbt/CompoundTag;
private synthetic lambda$store$1(Lnet/minecraft/nbt/Tag;)V
private synthetic lambda$store$0(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)V
```
