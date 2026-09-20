---
type: "interface"
fqcn: "net.minecraft.network.FriendlyByteBuf"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.FriendlyByteBuf

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`class` public; extends `io/netty/buffer/ByteBuf`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@7 in `FriendlyByteBufs.create` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@16 in `FriendlyByteBufs.readBytes` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@16 in `FriendlyByteBufs.readSlice` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@16 in `FriendlyByteBufs.readRetainedSlice` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@15 in `FriendlyByteBufs.copy` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@17 in `FriendlyByteBufs.copy` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@15 in `FriendlyByteBufs.slice` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@15 in `FriendlyByteBufs.retainedSlice` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@17 in `FriendlyByteBufs.slice` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@17 in `FriendlyByteBufs.retainedSlice` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@15 in `FriendlyByteBufs.duplicate` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@15 in `FriendlyByteBufs.retainedDuplicate` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@7 in `FriendlyByteBufs.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lio/netty/buffer/ByteBuf;)V` | exact | invokespecial@34 in `IdDispatchCodecMixin.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `copy` | `()Lio/netty/buffer/ByteBuf;` | exact | invokevirtual@2 in `PayloadHelper.write` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `copy` | `()Lio/netty/buffer/ByteBuf;` | exact | invokevirtual@11 in `PayloadHelper.read` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `isReadable` | `()Z` | exact | invokevirtual@17 in `RegistrationPayload.read` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readBoolean` | `()Z` | exact | invokevirtual@1 in `ServerboundCustomQueryAnswerPacketMixin.readResponse` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readByte` | `()B` | exact | invokevirtual@24 in `RegistrationPayload.read` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readByte` | `()B` | exact | invokevirtual@62 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readIdentifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@48 in `IdDispatchCodecMixin.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readUtf` | `()Ljava/lang/String;` | exact | invokevirtual@6 in `CommonRegisterPayload.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readUtf` | `()Ljava/lang/String;` | exact | invokevirtual@31 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readUtf` | `()Ljava/lang/String;` | exact | invokevirtual@56 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readUtf` | `()Ljava/lang/String;` | exact | invokevirtual@99 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readUtf` | `()Ljava/lang/String;` | exact | invokevirtual@158 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readVarInt` | `()I` | exact | invokevirtual@2 in `CommonRegisterPayload.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readVarInt` | `()I` | exact | invokevirtual@17 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readVarInt` | `()I` | exact | invokevirtual@40 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readVarInt` | `()I` | exact | invokevirtual@80 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readVarInt` | `()I` | exact | invokevirtual@108 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readVarInt` | `()I` | exact | invokevirtual@124 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readVarInt` | `()I` | exact | invokevirtual@130 in `RegistrySyncPayload.read` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readVarIntArray` | `()[I` | exact | invokevirtual@2 in `CommonVersionPayload.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readableBytes` | `()I` | exact | invokevirtual@20 in `PayloadHelper.read` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readableBytes` | `()I` | exact | invokevirtual@1 in `PayloadHelper.assertSize` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readableBytes` | `()I` | exact | invokevirtual@46 in `ServerLoginPacketListenerImplMixin.handleCustomPayloadReceivedAsync` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `readableBytes` | `()I` | exact | invokevirtual@68 in `ClientHandshakePacketListenerImplMixin.handleQueryRequest` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `skipBytes` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@23 in `PayloadHelper.read` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `skipBytes` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@49 in `ServerLoginPacketListenerImplMixin.handleCustomPayloadReceivedAsync` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `skipBytes` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@71 in `ClientHandshakePacketListenerImplMixin.handleQueryRequest` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `writeByte` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@43 in `RegistrationPayload.write` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `writeByte` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@83 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeBytes` | `(Lio/netty/buffer/ByteBuf;)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@5 in `PayloadHelper.write` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `writeBytes` | `(Lio/netty/buffer/ByteBuf;)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@14 in `PayloadHelper.read` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `writeBytes` | `([B)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@59 in `RegistrationPayload.write` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `writeUtf` | `(Ljava/lang/String;)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@14 in `CommonRegisterPayload.write` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `writeUtf` | `(Ljava/lang/String;)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@5 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeUtf` | `(Ljava/lang/String;)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@56 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeUtf` | `(Ljava/lang/String;)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@387 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeUtf` | `(Ljava/lang/String;)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@523 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@5 in `CommonRegisterPayload.write` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@38 in `RegistrySyncPayload.write` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@16 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@154 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@399 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@462 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeVarInt` | `(I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@474 in `RegistrySyncPayload.lambda$write$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `writeVarIntArray` | `([I)Lnet/minecraft/network/FriendlyByteBuf;` | exact | invokevirtual@5 in `CommonVersionPayload.write` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (4 fields, 358 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final source : Lio/netty/buffer/ByteBuf;
public static final MAX_STRING_LENGTH : S
public static final MAX_COMPONENT_STRING_LENGTH : I
private static final GSON : Lcom/google/gson/Gson;
public <init>(Lio/netty/buffer/ByteBuf;)V
public readWithCodecTrusted(Lcom/mojang/serialization/DynamicOps;Lcom/mojang/serialization/Codec;)Ljava/lang/Object;
public readWithCodec(Lcom/mojang/serialization/DynamicOps;Lcom/mojang/serialization/Codec;Lnet/minecraft/nbt/NbtAccounter;)Ljava/lang/Object;
public writeWithCodec(Lcom/mojang/serialization/DynamicOps;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)Lnet/minecraft/network/FriendlyByteBuf;
public readLenientJsonWithCodec(Lcom/mojang/serialization/Codec;)Ljava/lang/Object;
public writeJsonWithCodec(Lcom/mojang/serialization/Codec;Ljava/lang/Object;)V
public readWithCount(Ljava/util/function/Consumer;)V
public writeEnumSet(Ljava/util/EnumSet;Ljava/lang/Class;)V
public readEnumSet(Ljava/lang/Class;)Ljava/util/EnumSet;
public writeOptional(Ljava/util/Optional;Lnet/minecraft/network/codec/StreamEncoder;)V
public readOptional(Lnet/minecraft/network/codec/StreamDecoder;)Ljava/util/Optional;
public writeEither(Lcom/mojang/datafixers/util/Either;Lnet/minecraft/network/codec/StreamEncoder;Lnet/minecraft/network/codec/StreamEncoder;)V
public readEither(Lnet/minecraft/network/codec/StreamDecoder;Lnet/minecraft/network/codec/StreamDecoder;)Lcom/mojang/datafixers/util/Either;
public readNullable(Lnet/minecraft/network/codec/StreamDecoder;)Ljava/lang/Object;
public static readNullable(Lio/netty/buffer/ByteBuf;Lnet/minecraft/network/codec/StreamDecoder;)Ljava/lang/Object;
public writeNullable(Ljava/lang/Object;Lnet/minecraft/network/codec/StreamEncoder;)V
public static writeNullable(Lio/netty/buffer/ByteBuf;Ljava/lang/Object;Lnet/minecraft/network/codec/StreamEncoder;)V
public readByteArray()[B
public static readByteArray(Lio/netty/buffer/ByteBuf;)[B
public writeByteArray([B)Lnet/minecraft/network/FriendlyByteBuf;
public static writeByteArray(Lio/netty/buffer/ByteBuf;[B)V
public readByteArray(I)[B
public static readByteArray(Lio/netty/buffer/ByteBuf;I)[B
public writeVarIntArray([I)Lnet/minecraft/network/FriendlyByteBuf;
public readVarIntArray()[I
public readVarIntArray(I)[I
public writeLongArray([J)Lnet/minecraft/network/FriendlyByteBuf;
public static writeLongArray(Lio/netty/buffer/ByteBuf;[J)V
public writeFixedSizeLongArray([J)Lnet/minecraft/network/FriendlyByteBuf;
public static writeFixedSizeLongArray(Lio/netty/buffer/ByteBuf;[J)V
public readLongArray()[J
public readFixedSizeLongArray([J)[J
public static readLongArray(Lio/netty/buffer/ByteBuf;)[J
public static readFixedSizeLongArray(Lio/netty/buffer/ByteBuf;[J)[J
public readBlockPos()Lnet/minecraft/core/BlockPos;
public static readBlockPos(Lio/netty/buffer/ByteBuf;)Lnet/minecraft/core/BlockPos;
public writeBlockPos(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/network/FriendlyByteBuf;
public static writeBlockPos(Lio/netty/buffer/ByteBuf;Lnet/minecraft/core/BlockPos;)V
public readChunkPos()Lnet/minecraft/world/level/ChunkPos;
public writeChunkPos(Lnet/minecraft/world/level/ChunkPos;)Lnet/minecraft/network/FriendlyByteBuf;
public static readChunkPos(Lio/netty/buffer/ByteBuf;)Lnet/minecraft/world/level/ChunkPos;
public static writeChunkPos(Lio/netty/buffer/ByteBuf;Lnet/minecraft/world/level/ChunkPos;)V
public readGlobalPos()Lnet/minecraft/core/GlobalPos;
public writeGlobalPos(Lnet/minecraft/core/GlobalPos;)V
public readVector3f()Lorg/joml/Vector3f;
public static readVector3f(Lio/netty/buffer/ByteBuf;)Lorg/joml/Vector3f;
public writeVector3f(Lorg/joml/Vector3f;)V
public static writeVector3f(Lio/netty/buffer/ByteBuf;Lorg/joml/Vector3fc;)V
public readQuaternion()Lorg/joml/Quaternionf;
public static readQuaternion(Lio/netty/buffer/ByteBuf;)Lorg/joml/Quaternionf;
public writeQuaternion(Lorg/joml/Quaternionf;)V
public static writeQuaternion(Lio/netty/buffer/ByteBuf;Lorg/joml/Quaternionfc;)V
public readEnum(Ljava/lang/Class;)Ljava/lang/Enum;
public writeEnum(Ljava/lang/Enum;)Lnet/minecraft/network/FriendlyByteBuf;
public readById(Ljava/util/function/IntFunction;)Ljava/lang/Object;
public writeById(Ljava/util/function/ToIntFunction;Ljava/lang/Object;)Lnet/minecraft/network/FriendlyByteBuf;
public readVarInt()I
public readVarLong()J
public writeUUID(Ljava/util/UUID;)Lnet/minecraft/network/FriendlyByteBuf;
public static writeUUID(Lio/netty/buffer/ByteBuf;Ljava/util/UUID;)V
public readUUID()Ljava/util/UUID;
public static readUUID(Lio/netty/buffer/ByteBuf;)Ljava/util/UUID;
public writeVarInt(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeVarLong(J)Lnet/minecraft/network/FriendlyByteBuf;
public writeNbt(Lnet/minecraft/nbt/Tag;)Lnet/minecraft/network/FriendlyByteBuf;
public static writeNbt(Lio/netty/buffer/ByteBuf;Lnet/minecraft/nbt/Tag;)V
public readNbt()Lnet/minecraft/nbt/CompoundTag;
public static readNbt(Lio/netty/buffer/ByteBuf;)Lnet/minecraft/nbt/CompoundTag;
public static readNbt(Lio/netty/buffer/ByteBuf;Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/nbt/Tag;
public readNbt(Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/nbt/Tag;
public readUtf()Ljava/lang/String;
public readUtf(I)Ljava/lang/String;
public writeUtf(Ljava/lang/String;)Lnet/minecraft/network/FriendlyByteBuf;
public writeUtf(Ljava/lang/String;I)Lnet/minecraft/network/FriendlyByteBuf;
public readIdentifier()Lnet/minecraft/resources/Identifier;
public writeIdentifier(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/FriendlyByteBuf;
public readResourceKey(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/resources/ResourceKey;
public writeResourceKey(Lnet/minecraft/resources/ResourceKey;)V
public readRegistryKey()Lnet/minecraft/resources/ResourceKey;
public readBitSet()Ljava/util/BitSet;
public writeBitSet(Ljava/util/BitSet;)V
public readFixedBitSet(I)Ljava/util/BitSet;
public writeFixedBitSet(Ljava/util/BitSet;I)V
public static readFixedBitSet(Lio/netty/buffer/ByteBuf;I)Ljava/util/BitSet;
public static writeFixedBitSet(Lio/netty/buffer/ByteBuf;Ljava/util/BitSet;I)V
public static readContainerId(Lio/netty/buffer/ByteBuf;)I
public readContainerId()I
public static writeContainerId(Lio/netty/buffer/ByteBuf;I)V
public writeContainerId(I)V
public isContiguous()Z
public maxFastWritableBytes()I
public capacity()I
public capacity(I)Lnet/minecraft/network/FriendlyByteBuf;
public maxCapacity()I
public alloc()Lio/netty/buffer/ByteBufAllocator;
public order()Ljava/nio/ByteOrder;
public order(Ljava/nio/ByteOrder;)Lio/netty/buffer/ByteBuf;
public unwrap()Lio/netty/buffer/ByteBuf;
public isDirect()Z
public isReadOnly()Z
public asReadOnly()Lio/netty/buffer/ByteBuf;
public readerIndex()I
public readerIndex(I)Lnet/minecraft/network/FriendlyByteBuf;
public writerIndex()I
public writerIndex(I)Lnet/minecraft/network/FriendlyByteBuf;
public setIndex(II)Lnet/minecraft/network/FriendlyByteBuf;
public readableBytes()I
public writableBytes()I
public maxWritableBytes()I
public isReadable()Z
public isReadable(I)Z
public isWritable()Z
public isWritable(I)Z
public clear()Lnet/minecraft/network/FriendlyByteBuf;
public markReaderIndex()Lnet/minecraft/network/FriendlyByteBuf;
public resetReaderIndex()Lnet/minecraft/network/FriendlyByteBuf;
public markWriterIndex()Lnet/minecraft/network/FriendlyByteBuf;
public resetWriterIndex()Lnet/minecraft/network/FriendlyByteBuf;
public discardReadBytes()Lnet/minecraft/network/FriendlyByteBuf;
public discardSomeReadBytes()Lnet/minecraft/network/FriendlyByteBuf;
public ensureWritable(I)Lnet/minecraft/network/FriendlyByteBuf;
public ensureWritable(IZ)I
public getBoolean(I)Z
public getByte(I)B
public getUnsignedByte(I)S
public getShort(I)S
public getShortLE(I)S
public getUnsignedShort(I)I
public getUnsignedShortLE(I)I
public getMedium(I)I
public getMediumLE(I)I
public getUnsignedMedium(I)I
public getUnsignedMediumLE(I)I
public getInt(I)I
public getIntLE(I)I
public getUnsignedInt(I)J
public getUnsignedIntLE(I)J
public getLong(I)J
public getLongLE(I)J
public getChar(I)C
public getFloat(I)F
public getDouble(I)D
public getBytes(ILio/netty/buffer/ByteBuf;)Lnet/minecraft/network/FriendlyByteBuf;
public getBytes(ILio/netty/buffer/ByteBuf;I)Lnet/minecraft/network/FriendlyByteBuf;
public getBytes(ILio/netty/buffer/ByteBuf;II)Lnet/minecraft/network/FriendlyByteBuf;
public getBytes(I[B)Lnet/minecraft/network/FriendlyByteBuf;
public getBytes(I[BII)Lnet/minecraft/network/FriendlyByteBuf;
public getBytes(ILjava/nio/ByteBuffer;)Lnet/minecraft/network/FriendlyByteBuf;
public getBytes(ILjava/io/OutputStream;I)Lnet/minecraft/network/FriendlyByteBuf;
public getBytes(ILjava/nio/channels/GatheringByteChannel;I)I
public getBytes(ILjava/nio/channels/FileChannel;JI)I
public getCharSequence(IILjava/nio/charset/Charset;)Ljava/lang/CharSequence;
public setBoolean(IZ)Lnet/minecraft/network/FriendlyByteBuf;
public setByte(II)Lnet/minecraft/network/FriendlyByteBuf;
public setShort(II)Lnet/minecraft/network/FriendlyByteBuf;
public setShortLE(II)Lnet/minecraft/network/FriendlyByteBuf;
public setMedium(II)Lnet/minecraft/network/FriendlyByteBuf;
public setMediumLE(II)Lnet/minecraft/network/FriendlyByteBuf;
public setInt(II)Lnet/minecraft/network/FriendlyByteBuf;
public setIntLE(II)Lnet/minecraft/network/FriendlyByteBuf;
public setLong(IJ)Lnet/minecraft/network/FriendlyByteBuf;
public setLongLE(IJ)Lnet/minecraft/network/FriendlyByteBuf;
public setChar(II)Lnet/minecraft/network/FriendlyByteBuf;
public setFloat(IF)Lnet/minecraft/network/FriendlyByteBuf;
public setDouble(ID)Lnet/minecraft/network/FriendlyByteBuf;
public setBytes(ILio/netty/buffer/ByteBuf;)Lnet/minecraft/network/FriendlyByteBuf;
public setBytes(ILio/netty/buffer/ByteBuf;I)Lnet/minecraft/network/FriendlyByteBuf;
public setBytes(ILio/netty/buffer/ByteBuf;II)Lnet/minecraft/network/FriendlyByteBuf;
public setBytes(I[B)Lnet/minecraft/network/FriendlyByteBuf;
public setBytes(I[BII)Lnet/minecraft/network/FriendlyByteBuf;
public setBytes(ILjava/nio/ByteBuffer;)Lnet/minecraft/network/FriendlyByteBuf;
public setBytes(ILjava/io/InputStream;I)I
public setBytes(ILjava/nio/channels/ScatteringByteChannel;I)I
public setBytes(ILjava/nio/channels/FileChannel;JI)I
public setZero(II)Lnet/minecraft/network/FriendlyByteBuf;
public setCharSequence(ILjava/lang/CharSequence;Ljava/nio/charset/Charset;)I
public readBoolean()Z
public readByte()B
public readUnsignedByte()S
public readShort()S
public readShortLE()S
public readUnsignedShort()I
public readUnsignedShortLE()I
public readMedium()I
public readMediumLE()I
public readUnsignedMedium()I
public readUnsignedMediumLE()I
public readInt()I
public readIntLE()I
public readUnsignedInt()J
public readUnsignedIntLE()J
public readLong()J
public readLongLE()J
public readChar()C
public readFloat()F
public readDouble()D
public readBytes(I)Lio/netty/buffer/ByteBuf;
public readSlice(I)Lio/netty/buffer/ByteBuf;
public readRetainedSlice(I)Lio/netty/buffer/ByteBuf;
public readBytes(Lio/netty/buffer/ByteBuf;)Lnet/minecraft/network/FriendlyByteBuf;
public readBytes(Lio/netty/buffer/ByteBuf;I)Lnet/minecraft/network/FriendlyByteBuf;
public readBytes(Lio/netty/buffer/ByteBuf;II)Lnet/minecraft/network/FriendlyByteBuf;
public readBytes([B)Lnet/minecraft/network/FriendlyByteBuf;
public readBytes([BII)Lnet/minecraft/network/FriendlyByteBuf;
public readBytes(Ljava/nio/ByteBuffer;)Lnet/minecraft/network/FriendlyByteBuf;
public readBytes(Ljava/io/OutputStream;I)Lnet/minecraft/network/FriendlyByteBuf;
public readBytes(Ljava/nio/channels/GatheringByteChannel;I)I
public readCharSequence(ILjava/nio/charset/Charset;)Ljava/lang/CharSequence;
public readString(ILjava/nio/charset/Charset;)Ljava/lang/String;
public readBytes(Ljava/nio/channels/FileChannel;JI)I
public skipBytes(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeBoolean(Z)Lnet/minecraft/network/FriendlyByteBuf;
public writeByte(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeShort(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeShortLE(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeMedium(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeMediumLE(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeInt(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeIntLE(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeLong(J)Lnet/minecraft/network/FriendlyByteBuf;
public writeLongLE(J)Lnet/minecraft/network/FriendlyByteBuf;
public writeChar(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeFloat(F)Lnet/minecraft/network/FriendlyByteBuf;
public writeDouble(D)Lnet/minecraft/network/FriendlyByteBuf;
public writeBytes(Lio/netty/buffer/ByteBuf;)Lnet/minecraft/network/FriendlyByteBuf;
public writeBytes(Lio/netty/buffer/ByteBuf;I)Lnet/minecraft/network/FriendlyByteBuf;
public writeBytes(Lio/netty/buffer/ByteBuf;II)Lnet/minecraft/network/FriendlyByteBuf;
public writeBytes([B)Lnet/minecraft/network/FriendlyByteBuf;
public writeBytes([BII)Lnet/minecraft/network/FriendlyByteBuf;
public writeBytes(Ljava/nio/ByteBuffer;)Lnet/minecraft/network/FriendlyByteBuf;
public writeBytes(Ljava/io/InputStream;I)I
public writeBytes(Ljava/nio/channels/ScatteringByteChannel;I)I
public writeBytes(Ljava/nio/channels/FileChannel;JI)I
public writeZero(I)Lnet/minecraft/network/FriendlyByteBuf;
public writeCharSequence(Ljava/lang/CharSequence;Ljava/nio/charset/Charset;)I
public indexOf(IIB)I
public bytesBefore(B)I
public bytesBefore(IB)I
public bytesBefore(IIB)I
public forEachByte(Lio/netty/util/ByteProcessor;)I
public forEachByte(IILio/netty/util/ByteProcessor;)I
public forEachByteDesc(Lio/netty/util/ByteProcessor;)I
public forEachByteDesc(IILio/netty/util/ByteProcessor;)I
public copy()Lio/netty/buffer/ByteBuf;
public copy(II)Lio/netty/buffer/ByteBuf;
public slice()Lio/netty/buffer/ByteBuf;
public retainedSlice()Lio/netty/buffer/ByteBuf;
public slice(II)Lio/netty/buffer/ByteBuf;
public retainedSlice(II)Lio/netty/buffer/ByteBuf;
public duplicate()Lio/netty/buffer/ByteBuf;
public retainedDuplicate()Lio/netty/buffer/ByteBuf;
public nioBufferCount()I
public nioBuffer()Ljava/nio/ByteBuffer;
public nioBuffer(II)Ljava/nio/ByteBuffer;
public internalNioBuffer(II)Ljava/nio/ByteBuffer;
public nioBuffers()[Ljava/nio/ByteBuffer;
public nioBuffers(II)[Ljava/nio/ByteBuffer;
public hasArray()Z
public array()[B
public arrayOffset()I
public hasMemoryAddress()Z
public memoryAddress()J
public toString(Ljava/nio/charset/Charset;)Ljava/lang/String;
public toString(IILjava/nio/charset/Charset;)Ljava/lang/String;
public hashCode()I
public equals(Ljava/lang/Object;)Z
public compareTo(Lio/netty/buffer/ByteBuf;)I
public toString()Ljava/lang/String;
public retain(I)Lnet/minecraft/network/FriendlyByteBuf;
public retain()Lnet/minecraft/network/FriendlyByteBuf;
public touch()Lnet/minecraft/network/FriendlyByteBuf;
public touch(Ljava/lang/Object;)Lnet/minecraft/network/FriendlyByteBuf;
public refCnt()I
public release()Z
public release(I)Z
public synthetic touch(Ljava/lang/Object;)Lio/netty/buffer/ByteBuf;
public synthetic touch()Lio/netty/buffer/ByteBuf;
public synthetic retain()Lio/netty/buffer/ByteBuf;
public synthetic retain(I)Lio/netty/buffer/ByteBuf;
public synthetic writeZero(I)Lio/netty/buffer/ByteBuf;
public synthetic writeBytes(Ljava/nio/ByteBuffer;)Lio/netty/buffer/ByteBuf;
public synthetic writeBytes([BII)Lio/netty/buffer/ByteBuf;
public synthetic writeBytes([B)Lio/netty/buffer/ByteBuf;
public synthetic writeBytes(Lio/netty/buffer/ByteBuf;II)Lio/netty/buffer/ByteBuf;
public synthetic writeBytes(Lio/netty/buffer/ByteBuf;I)Lio/netty/buffer/ByteBuf;
public synthetic writeBytes(Lio/netty/buffer/ByteBuf;)Lio/netty/buffer/ByteBuf;
public synthetic writeDouble(D)Lio/netty/buffer/ByteBuf;
public synthetic writeFloat(F)Lio/netty/buffer/ByteBuf;
public synthetic writeChar(I)Lio/netty/buffer/ByteBuf;
public synthetic writeLongLE(J)Lio/netty/buffer/ByteBuf;
public synthetic writeLong(J)Lio/netty/buffer/ByteBuf;
public synthetic writeIntLE(I)Lio/netty/buffer/ByteBuf;
public synthetic writeInt(I)Lio/netty/buffer/ByteBuf;
public synthetic writeMediumLE(I)Lio/netty/buffer/ByteBuf;
public synthetic writeMedium(I)Lio/netty/buffer/ByteBuf;
public synthetic writeShortLE(I)Lio/netty/buffer/ByteBuf;
public synthetic writeShort(I)Lio/netty/buffer/ByteBuf;
public synthetic writeByte(I)Lio/netty/buffer/ByteBuf;
public synthetic writeBoolean(Z)Lio/netty/buffer/ByteBuf;
public synthetic skipBytes(I)Lio/netty/buffer/ByteBuf;
public synthetic readBytes(Ljava/io/OutputStream;I)Lio/netty/buffer/ByteBuf;
public synthetic readBytes(Ljava/nio/ByteBuffer;)Lio/netty/buffer/ByteBuf;
public synthetic readBytes([BII)Lio/netty/buffer/ByteBuf;
public synthetic readBytes([B)Lio/netty/buffer/ByteBuf;
public synthetic readBytes(Lio/netty/buffer/ByteBuf;II)Lio/netty/buffer/ByteBuf;
public synthetic readBytes(Lio/netty/buffer/ByteBuf;I)Lio/netty/buffer/ByteBuf;
public synthetic readBytes(Lio/netty/buffer/ByteBuf;)Lio/netty/buffer/ByteBuf;
public synthetic setZero(II)Lio/netty/buffer/ByteBuf;
public synthetic setBytes(ILjava/nio/ByteBuffer;)Lio/netty/buffer/ByteBuf;
public synthetic setBytes(I[BII)Lio/netty/buffer/ByteBuf;
public synthetic setBytes(I[B)Lio/netty/buffer/ByteBuf;
public synthetic setBytes(ILio/netty/buffer/ByteBuf;II)Lio/netty/buffer/ByteBuf;
public synthetic setBytes(ILio/netty/buffer/ByteBuf;I)Lio/netty/buffer/ByteBuf;
public synthetic setBytes(ILio/netty/buffer/ByteBuf;)Lio/netty/buffer/ByteBuf;
public synthetic setDouble(ID)Lio/netty/buffer/ByteBuf;
public synthetic setFloat(IF)Lio/netty/buffer/ByteBuf;
public synthetic setChar(II)Lio/netty/buffer/ByteBuf;
public synthetic setLongLE(IJ)Lio/netty/buffer/ByteBuf;
public synthetic setLong(IJ)Lio/netty/buffer/ByteBuf;
public synthetic setIntLE(II)Lio/netty/buffer/ByteBuf;
public synthetic setInt(II)Lio/netty/buffer/ByteBuf;
public synthetic setMediumLE(II)Lio/netty/buffer/ByteBuf;
public synthetic setMedium(II)Lio/netty/buffer/ByteBuf;
public synthetic setShortLE(II)Lio/netty/buffer/ByteBuf;
public synthetic setShort(II)Lio/netty/buffer/ByteBuf;
public synthetic setByte(II)Lio/netty/buffer/ByteBuf;
public synthetic setBoolean(IZ)Lio/netty/buffer/ByteBuf;
public synthetic getBytes(ILjava/io/OutputStream;I)Lio/netty/buffer/ByteBuf;
public synthetic getBytes(ILjava/nio/ByteBuffer;)Lio/netty/buffer/ByteBuf;
public synthetic getBytes(I[BII)Lio/netty/buffer/ByteBuf;
public synthetic getBytes(I[B)Lio/netty/buffer/ByteBuf;
public synthetic getBytes(ILio/netty/buffer/ByteBuf;II)Lio/netty/buffer/ByteBuf;
public synthetic getBytes(ILio/netty/buffer/ByteBuf;I)Lio/netty/buffer/ByteBuf;
public synthetic getBytes(ILio/netty/buffer/ByteBuf;)Lio/netty/buffer/ByteBuf;
public synthetic ensureWritable(I)Lio/netty/buffer/ByteBuf;
public synthetic discardSomeReadBytes()Lio/netty/buffer/ByteBuf;
public synthetic discardReadBytes()Lio/netty/buffer/ByteBuf;
public synthetic resetWriterIndex()Lio/netty/buffer/ByteBuf;
public synthetic markWriterIndex()Lio/netty/buffer/ByteBuf;
public synthetic resetReaderIndex()Lio/netty/buffer/ByteBuf;
public synthetic markReaderIndex()Lio/netty/buffer/ByteBuf;
public synthetic clear()Lio/netty/buffer/ByteBuf;
public synthetic setIndex(II)Lio/netty/buffer/ByteBuf;
public synthetic writerIndex(I)Lio/netty/buffer/ByteBuf;
public synthetic readerIndex(I)Lio/netty/buffer/ByteBuf;
public synthetic capacity(I)Lio/netty/buffer/ByteBuf;
public synthetic touch(Ljava/lang/Object;)Lio/netty/util/ReferenceCounted;
public synthetic touch()Lio/netty/util/ReferenceCounted;
public synthetic retain(I)Lio/netty/util/ReferenceCounted;
public synthetic retain()Lio/netty/util/ReferenceCounted;
public synthetic compareTo(Ljava/lang/Object;)I
private synthetic lambda$writeEither$1(Lnet/minecraft/network/codec/StreamEncoder;Ljava/lang/Object;)V
private synthetic lambda$writeEither$0(Lnet/minecraft/network/codec/StreamEncoder;Ljava/lang/Object;)V
private static synthetic lambda$writeJsonWithCodec$0(Ljava/lang/Object;Ljava/lang/String;)Lio/netty/handler/codec/EncoderException;
private static synthetic lambda$readLenientJsonWithCodec$0(Ljava/lang/String;)Lio/netty/handler/codec/DecoderException;
private static synthetic lambda$writeWithCodec$0(Ljava/lang/Object;Ljava/lang/String;)Lio/netty/handler/codec/EncoderException;
private static synthetic lambda$readWithCodec$0(Lnet/minecraft/nbt/Tag;Ljava/lang/String;)Lio/netty/handler/codec/DecoderException;
static <clinit>()V
```
