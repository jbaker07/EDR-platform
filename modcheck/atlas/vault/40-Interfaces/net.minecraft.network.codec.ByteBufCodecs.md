---
type: "interface"
fqcn: "net.minecraft.network.codec.ByteBufCodecs"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.codec.ByteBufCodecs

System: [[20-Systems/net.minecraft.network.codec|net.minecraft.network.codec]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `collection` | `(Ljava/util/function/IntFunction;)Lnet/minecraft/network/codec/StreamC` | exact | invokestatic@39 in `CommonRegisterPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `collection` | `(Ljava/util/function/IntFunction;Lnet/minecraft/network/codec/StreamCo` | exact | invokestatic@8 in `ServerboundAcceptedAttachmentsPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `collection` | `(Ljava/util/function/IntFunction;Lnet/minecraft/network/codec/StreamCo` | exact | invokestatic@16 in `ServerboundCustomIngredientPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `collection` | `(Ljava/util/function/IntFunction;Lnet/minecraft/network/codec/StreamCo` | exact | invokestatic@8 in `ServerboundSupportedRecipeSerializersPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `holderRegistry` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/network/codec/St` | exact | invokestatic@30 in `VariantCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `holderRegistry` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/network/codec/St` | exact | invokestatic@71 in `VariantCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `list` | `()Lnet/minecraft/network/codec/StreamCodec$CodecOperation;` | exact | invokestatic@18 in `CombinedIngredient$Serializer.<init>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `list` | `()Lnet/minecraft/network/codec/StreamCodec$CodecOperation;` | exact | invokestatic@3 in `ClientboundRecipeSyncPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `BYTE` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@0 in `AttachmentTargetInfo.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `COMPOUND_TAG` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@29 in `CustomDataIngredient$Serializer.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `VAR_INT` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@0 in `AttachmentTargetInfo$EntityTarget.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `VAR_INT` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@0 in `ClientboundCustomIngredientPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `VAR_INT` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@0 in `ServerboundCustomIngredientPayload.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `VAR_LONG` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@0 in `AttachmentTargetInfo$ChunkTarget.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (34 fields, 56 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAX_INITIAL_COLLECTION_SIZE : I
public static final BOOL : Lnet/minecraft/network/codec/StreamCodec;
public static final BYTE : Lnet/minecraft/network/codec/StreamCodec;
public static final ROTATION_BYTE : Lnet/minecraft/network/codec/StreamCodec;
public static final SHORT : Lnet/minecraft/network/codec/StreamCodec;
public static final UNSIGNED_SHORT : Lnet/minecraft/network/codec/StreamCodec;
public static final INT : Lnet/minecraft/network/codec/StreamCodec;
public static final VAR_INT : Lnet/minecraft/network/codec/StreamCodec;
public static final OPTIONAL_VAR_INT : Lnet/minecraft/network/codec/StreamCodec;
public static final LONG : Lnet/minecraft/network/codec/StreamCodec;
public static final VAR_LONG : Lnet/minecraft/network/codec/StreamCodec;
public static final FLOAT : Lnet/minecraft/network/codec/StreamCodec;
public static final DOUBLE : Lnet/minecraft/network/codec/StreamCodec;
public static final BYTE_ARRAY : Lnet/minecraft/network/codec/StreamCodec;
public static final LONG_ARRAY : Lnet/minecraft/network/codec/StreamCodec;
public static final BIT_SET : Lnet/minecraft/network/codec/StreamCodec;
public static final STRING_UTF8 : Lnet/minecraft/network/codec/StreamCodec;
public static final TAG : Lnet/minecraft/network/codec/StreamCodec;
public static final TRUSTED_TAG : Lnet/minecraft/network/codec/StreamCodec;
public static final COMPOUND_TAG : Lnet/minecraft/network/codec/StreamCodec;
public static final TRUSTED_COMPOUND_TAG : Lnet/minecraft/network/codec/StreamCodec;
public static final OPTIONAL_COMPOUND_TAG : Lnet/minecraft/network/codec/StreamCodec;
public static final VECTOR3F : Lnet/minecraft/network/codec/StreamCodec;
public static final QUATERNIONF : Lnet/minecraft/network/codec/StreamCodec;
public static final CONTAINER_ID : Lnet/minecraft/network/codec/StreamCodec;
public static final GAME_PROFILE_PROPERTIES : Lnet/minecraft/network/codec/StreamCodec;
public static final PLAYER_NAME : Lnet/minecraft/network/codec/StreamCodec;
public static final GAME_PROFILE : Lnet/minecraft/network/codec/StreamCodec;
public static final RGB_COLOR : Lnet/minecraft/network/codec/StreamCodec;
public static final INSTANT : Lnet/minecraft/network/codec/StreamCodec;
public static final PUBLIC_KEY_SIZE : I
public static final MAX_PUBLIC_KEY_HEADER_SIZE : I
public static final MAX_PUBLIC_KEY_LENGTH : I
public static final PUBLIC_KEY : Lnet/minecraft/network/codec/StreamCodec;
public static byteArray(I)Lnet/minecraft/network/codec/StreamCodec;
public static stringUtf8(I)Lnet/minecraft/network/codec/StreamCodec;
public static fixedBitSet(I)Lnet/minecraft/network/codec/StreamCodec;
public static optionalTagCodec(Ljava/util/function/Supplier;)Lnet/minecraft/network/codec/StreamCodec;
public static tagCodec(Ljava/util/function/Supplier;)Lnet/minecraft/network/codec/StreamCodec;
public static compoundTagCodec(Ljava/util/function/Supplier;)Lnet/minecraft/network/codec/StreamCodec;
public static fromCodecTrusted(Lcom/mojang/serialization/Codec;)Lnet/minecraft/network/codec/StreamCodec;
public static fromCodec(Lcom/mojang/serialization/Codec;)Lnet/minecraft/network/codec/StreamCodec;
public static fromCodec(Lcom/mojang/serialization/DynamicOps;Lcom/mojang/serialization/Codec;)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static fromCodec(Lcom/mojang/serialization/Codec;Ljava/util/function/Supplier;)Lnet/minecraft/network/codec/StreamCodec;
public static fromCodecWithRegistriesTrusted(Lcom/mojang/serialization/Codec;)Lnet/minecraft/network/codec/StreamCodec;
public static fromCodecWithRegistries(Lcom/mojang/serialization/Codec;)Lnet/minecraft/network/codec/StreamCodec;
public static fromCodecWithRegistries(Lcom/mojang/serialization/Codec;Ljava/util/function/Supplier;)Lnet/minecraft/network/codec/StreamCodec;
public static optional(Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
public static readCount(Lio/netty/buffer/ByteBuf;I)I
public static writeCount(Lio/netty/buffer/ByteBuf;II)V
public static collection(Ljava/util/function/IntFunction;Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
public static collection(Ljava/util/function/IntFunction;Lnet/minecraft/network/codec/StreamCodec;I)Lnet/minecraft/network/codec/StreamCodec;
public static collection(Ljava/util/function/IntFunction;)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static collection(Ljava/util/function/IntFunction;I)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static list()Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static list(I)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static fixedSizeCollection(Ljava/util/function/IntFunction;Lnet/minecraft/network/codec/StreamCodec;I)Lnet/minecraft/network/codec/StreamCodec;
public static fixedSizeCollection(Ljava/util/function/IntFunction;I)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static fixedSizeList(I)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static map(Ljava/util/function/IntFunction;Lnet/minecraft/network/codec/StreamCodec;Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
public static map(Ljava/util/function/IntFunction;Lnet/minecraft/network/codec/StreamCodec;Lnet/minecraft/network/codec/StreamCodec;I)Lnet/minecraft/network/codec/StreamCodec;
public static either(Lnet/minecraft/network/codec/StreamCodec;Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
public static lengthPrefixed(ILjava/util/function/BiFunction;)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static lengthPrefixed(I)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static registryFriendlyLengthPrefixed(I)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;
public static idMapper(Ljava/util/function/IntFunction;Ljava/util/function/ToIntFunction;)Lnet/minecraft/network/codec/StreamCodec;
public static idMapper(Lnet/minecraft/core/IdMap;)Lnet/minecraft/network/codec/StreamCodec;
private static registry(Lnet/minecraft/resources/ResourceKey;Ljava/util/function/Function;)Lnet/minecraft/network/codec/StreamCodec;
public static registry(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/network/codec/StreamCodec;
public static holderRegistry(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/network/codec/StreamCodec;
public static holder(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
public static holderSet(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/network/codec/StreamCodec;
public static lenientJson(I)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$static$2([B)Ljava/security/PublicKey;
private static synthetic lambda$registry$0(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/IdMap;
private static synthetic lambda$registryFriendlyLengthPrefixed$0(Lnet/minecraft/network/RegistryFriendlyByteBuf;Lio/netty/buffer/ByteBuf;)Lnet/minecraft/network/RegistryFriendlyByteBuf;
private static synthetic lambda$lengthPrefixed$1(Lio/netty/buffer/ByteBuf;Lio/netty/buffer/ByteBuf;)Lio/netty/buffer/ByteBuf;
private static synthetic lambda$lengthPrefixed$0(ILjava/util/function/BiFunction;Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$fixedSizeList$0(ILnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$fixedSizeCollection$0(Ljava/util/function/IntFunction;ILnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$list$1(ILnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$list$0(Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$collection$1(Ljava/util/function/IntFunction;ILnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$collection$0(Ljava/util/function/IntFunction;Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$fromCodec$0(Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
private static synthetic lambda$compoundTagCodec$1(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/Tag;
private static synthetic lambda$compoundTagCodec$0(Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/CompoundTag;
private static synthetic lambda$static$1(Ljava/util/OptionalInt;)Ljava/lang/Integer;
private static synthetic lambda$static$0(Ljava/lang/Integer;)Ljava/util/OptionalInt;
static <clinit>()V
```
