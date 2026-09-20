---
type: "interface"
fqcn: "net.minecraft.network.codec.ByteBufCodecs"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.codec.ByteBufCodecs

System: [[20-Systems/net.minecraft.network.codec|net.minecraft.network.codec]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `collection(Ljava/util/function/IntFunction;Lnet/minecraft/network/code` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `collection(Ljava/util/function/IntFunction;)Lnet/minecraft/network/cod` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `collection(Ljava/util/function/IntFunction;Lnet/minecraft/network/code` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `collection(Ljava/util/function/IntFunction;Lnet/minecraft/network/code` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `holderRegistry(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/networ` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `list()Lnet/minecraft/network/codec/StreamCodec$CodecOperation;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (90, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.network.codec.ByteBufCodecs {
    public static final int MAX_INITIAL_COLLECTION_SIZE;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Boolean> BOOL;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Byte> BYTE;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Float> ROTATION_BYTE;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Short> SHORT;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Integer> UNSIGNED_SHORT;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Integer> INT;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Integer> VAR_INT;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.util.OptionalInt> OPTIONAL_VAR_INT;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Long> LONG;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Long> VAR_LONG;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Float> FLOAT;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Double> DOUBLE;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, byte[]> BYTE_ARRAY;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, long[]> LONG_ARRAY;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.util.BitSet> BIT_SET;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.String> STRING_UTF8;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.nbt.Tag> TAG;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.nbt.Tag> TRUSTED_TAG;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.nbt.CompoundTag> COMPOUND_TAG;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.nbt.CompoundTag> TRUSTED_COMPOUND_TAG;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.util.Optional<net.minecraft.nbt.CompoundTag>> OPTIONAL_COMPOUND_TAG;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, org.joml.Vector3fc> VECTOR3F;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, org.joml.Quaternionfc> QUATERNIONF;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Integer> CONTAINER_ID;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, com.mojang.authlib.properties.PropertyMap> GAME_PROFILE_PROPERTIES;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.String> PLAYER_NAME;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, com.mojang.authlib.GameProfile> GAME_PROFILE;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.Integer> RGB_COLOR;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.time.Instant> INSTANT;
    public static final int PUBLIC_KEY_SIZE;
    public static final int MAX_PUBLIC_KEY_HEADER_SIZE;
    public static final int MAX_PUBLIC_KEY_LENGTH;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.security.PublicKey> PUBLIC_KEY;
    public static net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, byte[]> byteArray(int);
    public static net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.lang.String> stringUtf8(int);
    public static net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.util.BitSet> fixedBitSet(int);
    public static net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, java.util.Optional<net.minecraft.nbt.Tag>> optionalTagCodec(java.util.function.Supplier<net.minecraft.nbt.NbtAccounter>);
    public static net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.nbt.Tag> tagCodec(java.util.function.Supplier<net.minecraft.nbt.NbtAccounter>);
    public static net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.nbt.CompoundTag> compoundTagCodec(java.util.function.Supplier<net.minecraft.nbt.NbtAccounter>);
    public static <T> net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, T> fromCodecTrusted(com.mojang.serialization.Codec<T>);
    public static <T> net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, T> fromCodec(com.mojang.serialization.Codec<T>);
    public static <T, B extends io.netty.buffer.ByteBuf, V> net.minecraft.network.codec.StreamCodec$CodecOperation<B, T, V> fromCodec(com.mojang.serialization.DynamicOps<T>, com.mojang.serialization.Codec<V>);
    public static <T> net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, T> fromCodec(com.mojang.serialization.Codec<T>, java.util.function.Supplier<net.minecraft.nbt.NbtAccounter>);
    public static <T> net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, T> fromCodecWithRegistriesTrusted(com.mojang.serialization.Codec<T>);
    public static <T> net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, T> fromCodecWithRegistries(com.mojang.serialization.Codec<T>);
    public static <T> net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, T> fromCodecWithRegistries(com.mojang.serialization.Codec<T>, java.util.function.Supplier<net.minecraft.nbt.NbtAccounter>);
    public static <B extends io.netty.buffer.ByteBuf, V> net.minecraft.network.codec.StreamCodec<B, java.util.Optional<V>> optional(net.minecraft.network.codec.StreamCodec<? super B, V>);
    public static int readCount(io.netty.buffer.ByteBuf, int);
    public static void writeCount(io.netty.buffer.ByteBuf, int, int);
    public static <B extends io.netty.buffer.ByteBuf, V, C extends java.util.Collection<V>> net.minecraft.network.codec.StreamCodec<B, C> collection(java.util.function.IntFunction<C>, net.minecraft.network.codec.StreamCodec<? super B, V>);
    public static <B extends io.netty.buffer.ByteBuf, V, C extends java.util.Collection<V>> net.minecraft.network.codec.StreamCodec<B, C> collection(java.util.function.IntFunction<C>, net.minecraft.network.codec.StreamCodec<? super B, V>, int);
    public static <B extends io.netty.buffer.ByteBuf, V, C extends java.util.Collection<V>> net.minecraft.network.codec.StreamCodec$CodecOperation<B, V, C> collection(java.util.function.IntFunction<C>);
    public static <B extends io.netty.buffer.ByteBuf, V, C extends java.util.Collection<V>> net.minecraft.network.codec.StreamCodec$CodecOperation<B, V, C> collection(java.util.function.IntFunction<C>, int);
    public static <B extends io.netty.buffer.ByteBuf, V> net.minecraft.network.codec.StreamCodec$CodecOperation<B, V, java.util.List<V>> list();
    public static <B extends io.netty.buffer.ByteBuf, V> net.minecraft.network.codec.StreamCodec$CodecOperation<B, V, java.util.List<V>> list(int);
    public static <B extends io.netty.buffer.ByteBuf, V, C extends java.util.Collection<V>> net.minecraft.network.codec.StreamCodec<B, C> fixedSizeCollection(java.util.function.IntFunction<C>, net.minecraft.network.codec.StreamCodec<? super B, V>, int);
    public static <B extends io.netty.buffer.ByteBuf, V, C extends java.util.Collection<V>> net.minecraft.network.codec.StreamCodec$CodecOperation<B, V, C> fixedSizeCollection(java.util.function.IntFunction<C>, int);
    public static <B extends io.netty.buffer.ByteBuf, V> net.minecraft.network.codec.StreamCodec$CodecOperation<B, V, java.util.List<V>> fixedSizeList(int);
    public static <B extends io.netty.buffer.ByteBuf, K, V, M extends java.util.Map<K, V>> net.minecraft.network.codec.StreamCodec<B, M> map(java.util.function.IntFunction<? extends M>, net.minecraft.network.codec.StreamCodec<? super B, K>, net.minecraft.network.codec.StreamCodec<? super B, V>);
    public static <B extends io.netty.buffer.ByteBuf, K, V, M extends java.util.Map<K, V>> net.minecraft.network.codec.StreamCodec<B, M> map(java.util.function.IntFunction<? extends M>, net.minecraft.network.codec.StreamCodec<? super B, K>, net.minecraft.network.codec.StreamCodec<? super B, V>, int);
    public static <B extends io.netty.buffer.ByteBuf, L, R> net.minecraft.network.codec.StreamCodec<B, com.mojang.datafixers.util.Either<L, R>> either(net.minecraft.network.codec.StreamCodec<? super B, L>, net.minecraft.network.codec.StreamCodec<? super B, R>);
    public static <B extends io.netty.buffer.ByteBuf, V> net.minecraft.network.codec.StreamCodec$CodecOperation<B, V, V> lengthPrefixed(int, java.util.function.BiFunction<B, io.netty.buffer.ByteBuf, B>);
    public static <V> net.minecraft.network.codec.StreamCodec$CodecOperation<io.netty.buffer.ByteBuf, V, V> lengthPrefixed(int);
    public static <V> net.minecraft.network.codec.StreamCodec$CodecOperation<net.minecraft.network.RegistryFriendlyByteBuf, V, V> registryFriendlyLengthPrefixed(int);
    public static <T> net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, T> idMapper(java.util.function.IntFunction<T>, java.util.function.ToIntFunction<T>);
    public static <T> net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, T> idMapper(net.minecraft.core.IdMap<T>);
    private static <T, R> net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, R> registry(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, java.util.function.Function<net.minecraft.core.Registry<T>, net.minecraft.core.IdMap<R>>);
    public static <T> net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, T> registry(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>);
    public static <T> net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.Holder<T>> holderRegistry(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>);
    public static <T> net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.Holder<T>> holder(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, net.minecraft.network.codec.StreamCodec<? super net.minecraft.network.RegistryFriendlyByteBuf, T>);
    public static <T> net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.HolderSet<T>> holderSet(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>);
    public static net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, com.google.gson.JsonElement> lenientJson(int);
    private static java.security.PublicKey lambda$static$2(byte[]);
    private static net.minecraft.core.IdMap lambda$registry$0(net.minecraft.core.Registry);
    private static net.minecraft.network.RegistryFriendlyByteBuf lambda$registryFriendlyLengthPrefixed$0(net.minecraft.network.RegistryFriendlyByteBuf, io.netty.buffer.ByteBuf);
    private static io.netty.buffer.ByteBuf lambda$lengthPrefixed$1(io.netty.buffer.ByteBuf, io.netty.buffer.ByteBuf);
    private static net.minecraft.network.codec.StreamCodec lambda$lengthPrefixed$0(int, java.util.function.BiFunction, net.minecraft.network.codec.StreamCodec);
    private static net.minecraft.network.codec.StreamCodec lambda$fixedSizeList$0(int, net.minecraft.network.codec.StreamCodec);
    private static net.minecraft.network.codec.StreamCodec lambda$fixedSizeCollection$0(java.util.function.IntFunction, int, net.minecraft.network.codec.StreamCodec);
    private static net.minecraft.network.codec.StreamCodec lambda$list$1(int, net.minecraft.network.codec.StreamCodec);
    private static net.minecraft.network.codec.StreamCodec lambda$list$0(net.minecraft.network.codec.StreamCodec);
    private static net.minecraft.network.codec.StreamCodec lambda$collection$1(java.util.function.IntFunction, int, net.minecraft.network.codec.StreamCodec);
    private static net.minecraft.network.codec.StreamCodec lambda$collection$0(java.util.function.IntFunction, net.minecraft.network.codec.StreamCodec);
    private static net.minecraft.network.codec.StreamCodec lambda$fromCodec$0(com.mojang.serialization.Codec, com.mojang.serialization.DynamicOps, net.minecraft.network.codec.StreamCodec);
    private static net.minecraft.nbt.Tag lambda$compoundTagCodec$1(net.minecraft.nbt.CompoundTag);
    private static net.minecraft.nbt.CompoundTag lambda$compoundTagCodec$0(net.minecraft.nbt.Tag);
    private static java.lang.Integer lambda$static$1(java.util.OptionalInt);
    private static java.util.OptionalInt lambda$static$0(java.lang.Integer);
    static {};
}
```
