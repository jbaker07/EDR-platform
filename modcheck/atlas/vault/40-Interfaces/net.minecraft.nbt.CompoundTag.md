---
type: "interface"
fqcn: "net.minecraft.nbt.CompoundTag"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.CompoundTag

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

`class` public final; extends `java/lang/Object`; implements `net/minecraft/nbt/Tag`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@22 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@26 in `SerializableChunkDataMixin.setAttachmentDataInChunk` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@4 in `RegistryMapSerializer.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@24 in `RegistryMapSerializer.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@4 in `RegistryMapSerializer.lambda$toNbt$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@4 in `RegistryCustomContentState.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@19 in `RegistryCustomContentState.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `contains` | `(Ljava/lang/String;)Z` | exact | invokevirtual@5 in `TagValueInputMixin.contains` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | exact | invokevirtual@51 in `CustomDataIngredient.equals` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getByteArray` | `(Ljava/lang/String;)Ljava/util/Optional;` | exact | invokevirtual@5 in `TagValueInputMixin.getOptionalByteArray` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `getCompound` | `(Ljava/lang/String;)Ljava/util/Optional;` | exact | invokevirtual@18 in `SerializableChunkDataMixin.storeAttachmentNbtData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getCompound` | `(Ljava/lang/String;)Ljava/util/Optional;` | exact | invokevirtual@39 in `SerializableChunkDataMixin.storeAttachmentNbtData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getCompound` | `(Ljava/lang/String;)Ljava/util/Optional;` | exact | invokevirtual@3 in `RegistryMapSerializer.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getCompound` | `(Ljava/lang/String;)Ljava/util/Optional;` | exact | invokevirtual@63 in `RegistryMapSerializer.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getCompound` | `(Ljava/lang/String;)Ljava/util/Optional;` | exact | invokevirtual@29 in `RegistryCustomContentState.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getIntOr` | `(Ljava/lang/String;I)I` | exact | invokevirtual@120 in `RegistryMapSerializer.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getIntOr` | `(Ljava/lang/String;I)I` | exact | invokevirtual@8 in `RegistryCustomContentState.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getList` | `(Ljava/lang/String;)Ljava/util/Optional;` | exact | invokevirtual@88 in `RegistryCustomContentState.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getLongArray` | `(Ljava/lang/String;)Ljava/util/Optional;` | exact | invokevirtual@5 in `TagValueInputMixin.getOptionalLongArray` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `isEmpty` | `()Z` | exact | invokevirtual@9 in `CustomDataIngredient.<init>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `keySet` | `()Ljava/util/Set;` | exact | invokevirtual@22 in `RegistryMapSerializer.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `keySet` | `()Ljava/util/Set;` | exact | invokevirtual@76 in `RegistryMapSerializer.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `keySet` | `()Ljava/util/Set;` | exact | invokevirtual@47 in `RegistryCustomContentState.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `keySet` | `()Ljava/util/Set;` | exact | invokevirtual@4 in `TagValueInputMixin.keySet` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `put` | `(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;` | exact | invokevirtual@33 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `put` | `(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;` | exact | invokevirtual@39 in `SerializableChunkDataMixin.setAttachmentDataInChunk` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `put` | `(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;` | exact | invokevirtual@20 in `SerializableChunkDataMixin.writeChunkAttachments` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `put` | `(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;` | exact | invokevirtual@39 in `RegistryMapSerializer.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `put` | `(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;` | exact | invokevirtual@76 in `RegistryMapSerializer.lambda$toNbt$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `put` | `(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;` | exact | invokevirtual@139 in `RegistryCustomContentState.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `put` | `(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;` | exact | invokevirtual@150 in `RegistryCustomContentState.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `putByteArray` | `(Ljava/lang/String;[B)V` | exact | invokevirtual@6 in `TagValueOutputMixin.putByteArray` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `putInt` | `(Ljava/lang/String;I)V` | exact | invokevirtual@32 in `RegistryMapSerializer.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `putInt` | `(Ljava/lang/String;I)V` | exact | invokevirtual@64 in `RegistryMapSerializer.lambda$toNbt$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `putInt` | `(Ljava/lang/String;I)V` | exact | invokevirtual@12 in `RegistryCustomContentState.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `putLongArray` | `(Ljava/lang/String;[J)V` | exact | invokevirtual@6 in `TagValueOutputMixin.putLongArray` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `remove` | `(Ljava/lang/String;)Lnet/minecraft/nbt/Tag;` | exact | invokevirtual@3 in `BannerBlockEntityMixin.removeAttachments` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (6 fields, 80 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final CODEC : Lcom/mojang/serialization/Codec;
private static final SELF_SIZE_IN_BYTES : I
private static final MAP_ENTRY_SIZE_IN_BYTES : I
public static final TYPE : Lnet/minecraft/nbt/TagType;
private final tags : Ljava/util/Map;
 <init>(Ljava/util/Map;)V
public <init>()V
public write(Ljava/io/DataOutput;)V
public sizeInBytes()I
public keySet()Ljava/util/Set;
public entrySet()Ljava/util/Set;
public values()Ljava/util/Collection;
public forEach(Ljava/util/function/BiConsumer;)V
public getId()B
public getType()Lnet/minecraft/nbt/TagType;
public size()I
public put(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;
public putByte(Ljava/lang/String;B)V
public putShort(Ljava/lang/String;S)V
public putInt(Ljava/lang/String;I)V
public putLong(Ljava/lang/String;J)V
public putFloat(Ljava/lang/String;F)V
public putDouble(Ljava/lang/String;D)V
public putString(Ljava/lang/String;Ljava/lang/String;)V
public putByteArray(Ljava/lang/String;[B)V
public putIntArray(Ljava/lang/String;[I)V
public putLongArray(Ljava/lang/String;[J)V
public putBoolean(Ljava/lang/String;Z)V
public get(Ljava/lang/String;)Lnet/minecraft/nbt/Tag;
public contains(Ljava/lang/String;)Z
private getOptional(Ljava/lang/String;)Ljava/util/Optional;
public getByte(Ljava/lang/String;)Ljava/util/Optional;
public getByteOr(Ljava/lang/String;B)B
public getShort(Ljava/lang/String;)Ljava/util/Optional;
public getShortOr(Ljava/lang/String;S)S
public getInt(Ljava/lang/String;)Ljava/util/Optional;
public getIntOr(Ljava/lang/String;I)I
public getLong(Ljava/lang/String;)Ljava/util/Optional;
public getLongOr(Ljava/lang/String;J)J
public getFloat(Ljava/lang/String;)Ljava/util/Optional;
public getFloatOr(Ljava/lang/String;F)F
public getDouble(Ljava/lang/String;)Ljava/util/Optional;
public getDoubleOr(Ljava/lang/String;D)D
public getString(Ljava/lang/String;)Ljava/util/Optional;
public getStringOr(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
public getByteArray(Ljava/lang/String;)Ljava/util/Optional;
public getIntArray(Ljava/lang/String;)Ljava/util/Optional;
public getLongArray(Ljava/lang/String;)Ljava/util/Optional;
public getCompound(Ljava/lang/String;)Ljava/util/Optional;
public getCompoundOrEmpty(Ljava/lang/String;)Lnet/minecraft/nbt/CompoundTag;
public getList(Ljava/lang/String;)Ljava/util/Optional;
public getListOrEmpty(Ljava/lang/String;)Lnet/minecraft/nbt/ListTag;
public getBoolean(Ljava/lang/String;)Ljava/util/Optional;
public getBooleanOr(Ljava/lang/String;Z)Z
public remove(Ljava/lang/String;)Lnet/minecraft/nbt/Tag;
public toString()Ljava/lang/String;
public isEmpty()Z
 shallowCopy()Lnet/minecraft/nbt/CompoundTag;
public copy()Lnet/minecraft/nbt/CompoundTag;
public asCompound()Ljava/util/Optional;
public equals(Ljava/lang/Object;)Z
public hashCode()I
private static writeNamedTag(Ljava/lang/String;Lnet/minecraft/nbt/Tag;Ljava/io/DataOutput;)V
private static readNamedTagData(Lnet/minecraft/nbt/TagType;Ljava/lang/String;Ljava/io/DataInput;Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/nbt/Tag;
public merge(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/CompoundTag;
public accept(Lnet/minecraft/nbt/TagVisitor;)V
public accept(Lnet/minecraft/nbt/StreamTagVisitor;)Lnet/minecraft/nbt/StreamTagVisitor$ValueResult;
public store(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)V
public storeNullable(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)V
public store(Ljava/lang/String;Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)V
public storeNullable(Ljava/lang/String;Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)V
public store(Lcom/mojang/serialization/MapCodec;Ljava/lang/Object;)V
public store(Lcom/mojang/serialization/MapCodec;Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)V
public read(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/util/Optional;
public read(Ljava/lang/String;Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/DynamicOps;)Ljava/util/Optional;
public read(Lcom/mojang/serialization/MapCodec;)Ljava/util/Optional;
public read(Lcom/mojang/serialization/MapCodec;Lcom/mojang/serialization/DynamicOps;)Ljava/util/Optional;
public synthetic copy()Lnet/minecraft/nbt/Tag;
private synthetic lambda$read$1(Ljava/lang/String;)V
private static synthetic lambda$read$0(Ljava/lang/String;Lnet/minecraft/nbt/Tag;Ljava/lang/String;)V
private static synthetic lambda$copy$0(Ljava/util/HashMap;Ljava/lang/String;Lnet/minecraft/nbt/Tag;)V
private static synthetic lambda$static$2(Lnet/minecraft/nbt/CompoundTag;)Lcom/mojang/serialization/Dynamic;
private static synthetic lambda$static$0(Lcom/mojang/serialization/Dynamic;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
static <clinit>()V
```
