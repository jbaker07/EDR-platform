---
type: "interface"
fqcn: "net.minecraft.nbt.NbtOps"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.NbtOps

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

`class` public; extends `java/lang/Object`; implements `com/mojang/serialization/DynamicOps`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `convertTo` | `(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/nbt/Tag;)Ljava/la` | exact | invokevirtual@43 in `AttachmentSavedData$1.encode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lnet/minecraft/nbt/NbtOps;` | exact | getstatic@34 in `AttachmentSavedData$1.encode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lnet/minecraft/nbt/NbtOps;` | exact | getstatic@26 in `AttachmentSavedData$2.decode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lnet/minecraft/nbt/NbtOps;` | exact | getstatic@149 in `DefaultResourcePackStorage.read` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (1 fields, 100 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final INSTANCE : Lnet/minecraft/nbt/NbtOps;
private <init>()V
public empty()Lnet/minecraft/nbt/Tag;
public emptyList()Lnet/minecraft/nbt/Tag;
public emptyMap()Lnet/minecraft/nbt/Tag;
public convertTo(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/nbt/Tag;)Ljava/lang/Object;
public getNumberValue(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public createNumeric(Ljava/lang/Number;)Lnet/minecraft/nbt/Tag;
public createByte(B)Lnet/minecraft/nbt/Tag;
public createShort(S)Lnet/minecraft/nbt/Tag;
public createInt(I)Lnet/minecraft/nbt/Tag;
public createLong(J)Lnet/minecraft/nbt/Tag;
public createFloat(F)Lnet/minecraft/nbt/Tag;
public createDouble(D)Lnet/minecraft/nbt/Tag;
public getBooleanValue(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public createBoolean(Z)Lnet/minecraft/nbt/Tag;
public getStringValue(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public createString(Ljava/lang/String;)Lnet/minecraft/nbt/Tag;
public mergeToList(Lnet/minecraft/nbt/Tag;Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public mergeToList(Lnet/minecraft/nbt/Tag;Ljava/util/List;)Lcom/mojang/serialization/DataResult;
public mergeToMap(Lnet/minecraft/nbt/Tag;Lnet/minecraft/nbt/Tag;Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public mergeToMap(Lnet/minecraft/nbt/Tag;Lcom/mojang/serialization/MapLike;)Lcom/mojang/serialization/DataResult;
public mergeToMap(Lnet/minecraft/nbt/Tag;Ljava/util/Map;)Lcom/mojang/serialization/DataResult;
public getMapValues(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public getMapEntries(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public getMap(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public createMap(Ljava/util/stream/Stream;)Lnet/minecraft/nbt/Tag;
public getStream(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public getList(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public getByteBuffer(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public createByteList(Ljava/nio/ByteBuffer;)Lnet/minecraft/nbt/Tag;
public getIntStream(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public createIntList(Ljava/util/stream/IntStream;)Lnet/minecraft/nbt/Tag;
public getLongStream(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
public createLongList(Ljava/util/stream/LongStream;)Lnet/minecraft/nbt/Tag;
public createList(Ljava/util/stream/Stream;)Lnet/minecraft/nbt/Tag;
public remove(Lnet/minecraft/nbt/Tag;Ljava/lang/String;)Lnet/minecraft/nbt/Tag;
public toString()Ljava/lang/String;
public mapBuilder()Lcom/mojang/serialization/RecordBuilder;
private static createCollector(Lnet/minecraft/nbt/Tag;)Ljava/util/Optional;
public synthetic remove(Ljava/lang/Object;Ljava/lang/String;)Ljava/lang/Object;
public synthetic createLongList(Ljava/util/stream/LongStream;)Ljava/lang/Object;
public synthetic getLongStream(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic createIntList(Ljava/util/stream/IntStream;)Ljava/lang/Object;
public synthetic getIntStream(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic createByteList(Ljava/nio/ByteBuffer;)Ljava/lang/Object;
public synthetic getByteBuffer(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic createList(Ljava/util/stream/Stream;)Ljava/lang/Object;
public synthetic getList(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic getStream(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic getMap(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic createMap(Ljava/util/stream/Stream;)Ljava/lang/Object;
public synthetic getMapEntries(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic getMapValues(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic mergeToMap(Ljava/lang/Object;Lcom/mojang/serialization/MapLike;)Lcom/mojang/serialization/DataResult;
public synthetic mergeToMap(Ljava/lang/Object;Ljava/util/Map;)Lcom/mojang/serialization/DataResult;
public synthetic mergeToMap(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic mergeToList(Ljava/lang/Object;Ljava/util/List;)Lcom/mojang/serialization/DataResult;
public synthetic mergeToList(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic createString(Ljava/lang/String;)Ljava/lang/Object;
public synthetic getStringValue(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic createBoolean(Z)Ljava/lang/Object;
public synthetic getBooleanValue(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic createDouble(D)Ljava/lang/Object;
public synthetic createFloat(F)Ljava/lang/Object;
public synthetic createLong(J)Ljava/lang/Object;
public synthetic createInt(I)Ljava/lang/Object;
public synthetic createShort(S)Ljava/lang/Object;
public synthetic createByte(B)Ljava/lang/Object;
public synthetic createNumeric(Ljava/lang/Number;)Ljava/lang/Object;
public synthetic getNumberValue(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
public synthetic convertTo(Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)Ljava/lang/Object;
public synthetic emptyList()Ljava/lang/Object;
public synthetic emptyMap()Ljava/lang/Object;
public synthetic empty()Ljava/lang/Object;
private static synthetic lambda$getList$0(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private static synthetic lambda$getStream$0()Ljava/lang/String;
private static synthetic lambda$createMap$0(Lnet/minecraft/nbt/CompoundTag;Lcom/mojang/datafixers/util/Pair;)V
private static synthetic lambda$getMap$0(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private static synthetic lambda$getMapEntries$1(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private synthetic lambda$getMapEntries$0(Lnet/minecraft/nbt/CompoundTag;Ljava/util/function/BiConsumer;)V
private static synthetic lambda$getMapValues$1(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private synthetic lambda$getMapValues$0(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$mergeToMap$6(Ljava/util/List;)Ljava/lang/String;
private static synthetic lambda$mergeToMap$5(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private static synthetic lambda$mergeToMap$4(Ljava/util/List;)Ljava/lang/String;
private static synthetic lambda$mergeToMap$3(Ljava/util/List;Lnet/minecraft/nbt/CompoundTag;Lcom/mojang/datafixers/util/Pair;)V
private static synthetic lambda$mergeToMap$2(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private static synthetic lambda$mergeToMap$1(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private static synthetic lambda$mergeToMap$0(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private static synthetic lambda$mergeToList$4(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$mergeToList$5(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private static synthetic lambda$mergeToList$3(Ljava/util/List;Lnet/minecraft/nbt/NbtOps$ListCollector;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$mergeToList$1(Lnet/minecraft/nbt/Tag;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$mergeToList$2(Lnet/minecraft/nbt/Tag;)Ljava/lang/String;
private static synthetic lambda$mergeToList$0(Lnet/minecraft/nbt/Tag;Lnet/minecraft/nbt/NbtOps$ListCollector;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$getStringValue$0()Ljava/lang/String;
private static synthetic lambda$getBooleanValue$0(Ljava/lang/Number;)Ljava/lang/Boolean;
private static synthetic lambda$getNumberValue$0()Lcom/mojang/serialization/DataResult;
private static synthetic lambda$getNumberValue$1()Ljava/lang/String;
static <clinit>()V
```
