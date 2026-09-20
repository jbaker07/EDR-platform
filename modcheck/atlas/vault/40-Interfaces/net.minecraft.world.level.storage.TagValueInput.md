---
type: "interface"
fqcn: "net.minecraft.world.level.storage.TagValueInput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.TagValueInput

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/storage/ValueInput`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `create` | `(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/HolderLookup$` | exact | invokestatic@38 in `AttachmentSavedData$2.decode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/HolderLookup$` | exact | invokestatic@63 in `SerializableChunkDataMixin.setAttachmentDataInChunk` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `input` | `Lnet/minecraft/nbt/CompoundTag;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | declared |

## Declared members (3 fields, 31 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final problemReporter : Lnet/minecraft/util/ProblemReporter;
private final context : Lnet/minecraft/world/level/storage/ValueInputContextHelper;
private final input : Lnet/minecraft/nbt/CompoundTag;
private <init>(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/world/level/storage/ValueInputContextHelper;Lnet/minecraft/nbt/CompoundTag;)V
public static create(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/storage/ValueInput;
public static create(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/HolderLookup$Provider;Ljava/util/List;)Lnet/minecraft/world/level/storage/ValueInput$ValueInputList;
public read(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/util/Optional;
public read(Lcom/mojang/serialization/MapCodec;)Ljava/util/Optional;
private getOptionalTypedTag(Ljava/lang/String;Lnet/minecraft/nbt/TagType;)Lnet/minecraft/nbt/Tag;
private getNumericTag(Ljava/lang/String;)Lnet/minecraft/nbt/NumericTag;
public child(Ljava/lang/String;)Ljava/util/Optional;
public childOrEmpty(Ljava/lang/String;)Lnet/minecraft/world/level/storage/ValueInput;
public childrenList(Ljava/lang/String;)Ljava/util/Optional;
public childrenListOrEmpty(Ljava/lang/String;)Lnet/minecraft/world/level/storage/ValueInput$ValueInputList;
public list(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/util/Optional;
public listOrEmpty(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Lnet/minecraft/world/level/storage/ValueInput$TypedInputList;
public getBooleanOr(Ljava/lang/String;Z)Z
public getByteOr(Ljava/lang/String;B)B
public getShortOr(Ljava/lang/String;S)I
public getInt(Ljava/lang/String;)Ljava/util/Optional;
public getIntOr(Ljava/lang/String;I)I
public getLongOr(Ljava/lang/String;J)J
public getLong(Ljava/lang/String;)Ljava/util/Optional;
public getFloatOr(Ljava/lang/String;F)F
public getDoubleOr(Ljava/lang/String;D)D
public getString(Ljava/lang/String;)Ljava/util/Optional;
public getStringOr(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
public getIntArray(Ljava/lang/String;)Ljava/util/Optional;
public lookup()Lnet/minecraft/core/HolderLookup$Provider;
private wrapChild(Ljava/lang/String;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/storage/ValueInput;
private static wrapChild(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/world/level/storage/ValueInputContextHelper;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/storage/ValueInput;
private wrapList(Ljava/lang/String;Lnet/minecraft/world/level/storage/ValueInputContextHelper;Lnet/minecraft/nbt/ListTag;)Lnet/minecraft/world/level/storage/ValueInput$ValueInputList;
private wrapTypedList(Ljava/lang/String;Lnet/minecraft/nbt/ListTag;Lcom/mojang/serialization/Codec;)Lnet/minecraft/world/level/storage/ValueInput$TypedInputList;
private static synthetic lambda$read$0(Lcom/mojang/serialization/MapCodec;Lcom/mojang/serialization/DynamicOps;Lcom/mojang/serialization/MapLike;)Lcom/mojang/serialization/DataResult;
```
