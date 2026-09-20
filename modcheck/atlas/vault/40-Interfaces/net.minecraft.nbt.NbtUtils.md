---
type: "interface"
fqcn: "net.minecraft.nbt.NbtUtils"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.NbtUtils

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getDataVersion` | `(Lnet/minecraft/nbt/CompoundTag;I)I` | exact | invokestatic@62 in `StructureTemplateManagerMixin$1.load` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `snbtToStructure` | `(Ljava/lang/String;)Lnet/minecraft/nbt/CompoundTag;` | exact | invokestatic@43 in `StructureTemplateManagerMixin$1.load` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (15 fields, 44 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final YXZ_LISTTAG_INT_COMPARATOR : Ljava/util/Comparator;
private static final YXZ_LISTTAG_DOUBLE_COMPARATOR : Ljava/util/Comparator;
private static final BLOCK_NAME_CODEC : Lcom/mojang/serialization/Codec;
public static final SNBT_DATA_TAG : Ljava/lang/String;
private static final PROPERTIES_START : C
private static final PROPERTIES_END : C
private static final ELEMENT_SEPARATOR : Ljava/lang/String;
private static final KEY_VALUE_SEPARATOR : C
private static final COMMA_SPLITTER : Lcom/google/common/base/Splitter;
private static final COLON_SPLITTER : Lcom/google/common/base/Splitter;
private static final LOGGER : Lorg/slf4j/Logger;
private static final NOT_FOUND : I
private static final BLOCK_STATE_ID_PROPERTIES_RENAME_VERSION : I
public static final LEGACY_BLOCK_STATE_ID_TAG : Ljava/lang/String;
public static final LEGACY_BLOCKSTATE_PROPERTY_TAG : Ljava/lang/String;
private <init>()V
public static compareNbt(Lnet/minecraft/nbt/Tag;Lnet/minecraft/nbt/Tag;Z)Z
public static readBlockState(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/block/state/BlockState;
private static setValueHelper(Lnet/minecraft/world/level/block/state/StateHolder;Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/String;Lnet/minecraft/nbt/CompoundTag;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/block/state/StateHolder;
private static writeStateProperties(Lnet/minecraft/world/level/block/state/StateHolder;Lnet/minecraft/nbt/CompoundTag;)V
public static writeBlockState(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/nbt/CompoundTag;
public static toPrettyComponent(Lnet/minecraft/nbt/Tag;)Lnet/minecraft/network/chat/Component;
public static structureToSnbt(Lnet/minecraft/nbt/CompoundTag;)Ljava/lang/String;
public static snbtToStructure(Ljava/lang/String;)Lnet/minecraft/nbt/CompoundTag;
static packStructureTemplate(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/CompoundTag;
static unpackStructureTemplate(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/CompoundTag;
static packBlockState(Lnet/minecraft/nbt/CompoundTag;I)Ljava/lang/String;
static unpackBlockState(Ljava/lang/String;I)Lnet/minecraft/nbt/CompoundTag;
public static addCurrentDataVersion(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/CompoundTag;
public static addDataVersion(Lnet/minecraft/nbt/CompoundTag;I)Lnet/minecraft/nbt/CompoundTag;
public static addDataVersion(Lcom/mojang/serialization/Dynamic;I)Lcom/mojang/serialization/Dynamic;
public static addCurrentDataVersion(Lnet/minecraft/world/level/storage/ValueOutput;)V
public static addDataVersion(Lnet/minecraft/world/level/storage/ValueOutput;I)V
public static getDataVersion(Lnet/minecraft/nbt/CompoundTag;)I
public static getDataVersion(Lnet/minecraft/nbt/CompoundTag;I)I
public static getDataVersion(Lcom/mojang/serialization/Dynamic;)I
public static getDataVersion(Lcom/mojang/serialization/Dynamic;I)I
private static synthetic lambda$unpackBlockState$0(Lnet/minecraft/nbt/CompoundTag;Ljava/lang/String;Ljava/lang/String;)V
private static synthetic lambda$packBlockState$0(Ljava/lang/StringBuilder;Lnet/minecraft/nbt/CompoundTag;)V
private static synthetic lambda$packBlockState$1(Ljava/util/Map$Entry;)Ljava/lang/String;
private static synthetic lambda$unpackStructureTemplate$2(Ljava/util/Map;ILnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/ListTag;
private static synthetic lambda$unpackStructureTemplate$4(ILjava/lang/String;)Lnet/minecraft/nbt/CompoundTag;
private static synthetic lambda$unpackStructureTemplate$3(Lnet/minecraft/nbt/CompoundTag;Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$unpackStructureTemplate$1(ILjava/lang/String;)Lnet/minecraft/nbt/Tag;
private static synthetic lambda$unpackStructureTemplate$0(Lnet/minecraft/nbt/Tag;)Ljava/util/stream/Stream;
private static synthetic lambda$packStructureTemplate$5(Lnet/minecraft/nbt/ListTag;Lnet/minecraft/nbt/CompoundTag;)V
private static synthetic lambda$packStructureTemplate$4(Lnet/minecraft/nbt/CompoundTag;)Ljava/util/Optional;
private static synthetic lambda$packStructureTemplate$3(Lnet/minecraft/nbt/CompoundTag;)Ljava/util/Optional;
private static synthetic lambda$packStructureTemplate$2(Lnet/minecraft/nbt/ListTag;ILnet/minecraft/nbt/ListTag;Lnet/minecraft/nbt/ListTag;)V
private static synthetic lambda$packStructureTemplate$1(Lnet/minecraft/nbt/Tag;)Ljava/util/stream/Stream;
private static synthetic lambda$packStructureTemplate$0(ILnet/minecraft/nbt/CompoundTag;)Ljava/lang/String;
private static synthetic lambda$writeStateProperties$0(Lnet/minecraft/nbt/CompoundTag;Lnet/minecraft/world/level/block/state/properties/Property$Value;)V
private static synthetic lambda$static$5(Lnet/minecraft/nbt/ListTag;)D
private static synthetic lambda$static$4(Lnet/minecraft/nbt/ListTag;)D
private static synthetic lambda$static$3(Lnet/minecraft/nbt/ListTag;)D
private static synthetic lambda$static$2(Lnet/minecraft/nbt/ListTag;)I
private static synthetic lambda$static$1(Lnet/minecraft/nbt/ListTag;)I
private static synthetic lambda$static$0(Lnet/minecraft/nbt/ListTag;)I
static <clinit>()V
```
