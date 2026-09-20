---
type: "interface"
fqcn: "net.minecraft.util.ExtraCodecs"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.ExtraCodecs

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `nonEmptyList(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (218, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.ExtraCodecs {
    public static final com.mojang.serialization.Codec<com.google.gson.JsonElement> JSON;
    public static final com.mojang.serialization.Codec<java.lang.Object> JAVA;
    public static final com.mojang.serialization.Codec<net.minecraft.nbt.Tag> NBT;
    public static final com.mojang.serialization.Codec<org.joml.Vector2fc> VECTOR2F;
    public static final com.mojang.serialization.Codec<org.joml.Vector3fc> VECTOR3F;
    public static final com.mojang.serialization.Codec<org.joml.Vector3ic> VECTOR3I;
    public static final com.mojang.serialization.Codec<org.joml.Vector4fc> VECTOR4F;
    public static final com.mojang.serialization.Codec<org.joml.Quaternionfc> QUATERNIONF_COMPONENTS;
    public static final com.mojang.serialization.Codec<org.joml.AxisAngle4f> AXISANGLE4F;
    public static final com.mojang.serialization.Codec<org.joml.Quaternionfc> QUATERNIONF;
    public static final com.mojang.serialization.Codec<org.joml.Matrix4fc> MATRIX4F;
    private static final java.lang.String HEX_COLOR_PREFIX;
    public static final com.mojang.serialization.Codec<java.lang.Integer> RGB_COLOR_CODEC;
    public static final com.mojang.serialization.Codec<java.lang.Integer> ARGB_COLOR_CODEC;
    public static final com.mojang.serialization.Codec<org.joml.Vector3fc> RGB_COLOR_VEC3_CODEC;
    public static final com.mojang.serialization.Codec<org.joml.Vector4fc> ARGB_COLOR_VEC4_CODEC;
    public static final com.mojang.serialization.Codec<java.lang.Integer> STRING_RGB_COLOR;
    public static final com.mojang.serialization.Codec<java.lang.Integer> STRING_ARGB_COLOR;
    public static final com.mojang.serialization.Codec<org.joml.Vector3fc> STRING_RGB_VEC3_COLOR;
    public static final com.mojang.serialization.Codec<org.joml.Vector4fc> STRING_ARGB_VEC4_COLOR;
    public static final com.mojang.serialization.Codec<java.lang.Integer> UNSIGNED_BYTE;
    public static final com.mojang.serialization.Codec<java.lang.Integer> NON_NEGATIVE_INT;
    public static final com.mojang.serialization.Codec<java.lang.Integer> POSITIVE_INT;
    public static final com.mojang.serialization.Codec<java.lang.Long> NON_NEGATIVE_LONG;
    public static final com.mojang.serialization.Codec<java.lang.Long> POSITIVE_LONG;
    public static final com.mojang.serialization.Codec<java.lang.Float> NON_NEGATIVE_FLOAT;
    public static final com.mojang.serialization.Codec<java.lang.Float> POSITIVE_FLOAT;
    public static final com.mojang.serialization.Codec<java.util.regex.Pattern> PATTERN;
    public static final com.mojang.serialization.Codec<java.time.Instant> INSTANT_ISO8601;
    public static final com.mojang.serialization.Codec<byte[]> BASE64_STRING;
    public static final com.mojang.serialization.Codec<java.lang.String> ESCAPED_STRING;
    public static final com.mojang.serialization.Codec<net.minecraft.util.ExtraCodecs$TagOrElementLocation> TAG_OR_ELEMENT_ID;
    public static final java.util.function.Function<java.util.Optional<java.lang.Long>, java.util.OptionalLong> toOptionalLong;
    public static final java.util.function.Function<java.util.OptionalLong, java.util.Optional<java.lang.Long>> fromOptionalLong;
    public static final com.mojang.serialization.Codec<java.util.BitSet> BIT_SET;
    public static final int MAX_PROPERTY_NAME_LENGTH;
    public static final int MAX_PROPERTY_VALUE_LENGTH;
    public static final int MAX_PROPERTY_SIGNATURE_LENGTH;
    public static final int MAX_PROPERTIES;
    private static final com.mojang.serialization.Codec<com.mojang.authlib.properties.Property> PROPERTY;
    public static final com.mojang.serialization.Codec<com.mojang.authlib.properties.PropertyMap> PROPERTY_MAP;
    public static final com.mojang.serialization.Codec<java.lang.String> PLAYER_NAME;
    public static final com.mojang.serialization.Codec<com.mojang.authlib.GameProfile> AUTHLIB_GAME_PROFILE;
    public static final com.mojang.serialization.MapCodec<com.mojang.authlib.GameProfile> STORED_GAME_PROFILE;
    public static final com.mojang.serialization.Codec<java.lang.String> NON_EMPTY_STRING;
    public static final com.mojang.serialization.Codec<java.lang.Integer> CODEPOINT;
    public static final com.mojang.serialization.Codec<java.lang.String> RESOURCE_PATH_CODEC;
    public static final com.mojang.serialization.Codec<java.net.URI> UNTRUSTED_URI;
    public static final com.mojang.serialization.Codec<java.lang.String> CHAT_STRING;
    public net.minecraft.util.ExtraCodecs();
    public static <T> com.mojang.serialization.Codec<T> converter(com.mojang.serialization.DynamicOps<T>);
    private static com.mojang.serialization.Codec<java.lang.Integer> hexColor(int);
    public static <P, I> com.mojang.serialization.Codec<I> intervalCodec(com.mojang.serialization.Codec<P>, java.lang.String, java.lang.String, java.util.function.BiFunction<P, P, com.mojang.serialization.DataResult<I>>, java.util.function.Function<I, P>, java.util.function.Function<I, P>);
    public static <A> com.mojang.serialization.Codec$ResultFunction<A> orElsePartial(A);
    public static <E> com.mojang.serialization.Codec<E> idResolverCodec(java.util.function.ToIntFunction<E>, java.util.function.IntFunction<E>, int);
    public static <I, E> com.mojang.serialization.Codec<E> idResolverCodec(com.mojang.serialization.Codec<I>, java.util.function.Function<I, E>, java.util.function.Function<E, I>);
    public static <E> com.mojang.serialization.Codec<E> orCompressed(com.mojang.serialization.Codec<E>, com.mojang.serialization.Codec<E>);
    public static <E> com.mojang.serialization.MapCodec<E> orCompressed(com.mojang.serialization.MapCodec<E>, com.mojang.serialization.MapCodec<E>);
    public static <E> com.mojang.serialization.Codec<E> overrideLifecycle(com.mojang.serialization.Codec<E>, java.util.function.Function<E, com.mojang.serialization.Lifecycle>, java.util.function.Function<E, com.mojang.serialization.Lifecycle>);
    public static <E> com.mojang.serialization.Codec<E> overrideLifecycle(com.mojang.serialization.Codec<E>, java.util.function.Function<E, com.mojang.serialization.Lifecycle>);
    public static <K, V> net.minecraft.util.ExtraCodecs$StrictUnboundedMapCodec<K, V> strictUnboundedMap(com.mojang.serialization.Codec<K>, com.mojang.serialization.Codec<V>);
    public static <E> com.mojang.serialization.Codec<java.util.List<E>> compactListCodec(com.mojang.serialization.Codec<E>);
    public static <E> com.mojang.serialization.Codec<java.util.List<E>> compactListCodec(com.mojang.serialization.Codec<E>, com.mojang.serialization.Codec<java.util.List<E>>);
    private static com.mojang.serialization.Codec<java.lang.Integer> intRangeWithMessage(int, int, java.util.function.Function<java.lang.Integer, java.lang.String>);
    public static com.mojang.serialization.Codec<java.lang.Integer> intRange(int, int);
    private static com.mojang.serialization.Codec<java.lang.Long> longRangeWithMessage(long, long, java.util.function.Function<java.lang.Long, java.lang.String>);
    public static com.mojang.serialization.Codec<java.lang.Long> longRange(int, int);
    private static com.mojang.serialization.Codec<java.lang.Float> floatRangeMinInclusiveWithMessage(float, float, java.util.function.Function<java.lang.Float, java.lang.String>);
    private static com.mojang.serialization.Codec<java.lang.Float> floatRangeMinExclusiveWithMessage(float, float, java.util.function.Function<java.lang.Float, java.lang.String>);
    public static com.mojang.serialization.Codec<java.lang.Float> floatRange(float, float);
    public static <T> com.mojang.serialization.Codec<java.util.List<T>> nonEmptyList(com.mojang.serialization.Codec<java.util.List<T>>);
    public static <T> com.mojang.serialization.Codec<net.minecraft.core.HolderSet<T>> nonEmptyHolderSet(com.mojang.serialization.Codec<net.minecraft.core.HolderSet<T>>);
    public static <M extends java.util.Map<?, ?>> com.mojang.serialization.Codec<M> nonEmptyMap(com.mojang.serialization.Codec<M>);
    public static <E> com.mojang.serialization.MapCodec<E> retrieveContext(java.util.function.Function<com.mojang.serialization.DynamicOps<?>, com.mojang.serialization.DataResult<E>>);
    public static <E, L extends java.util.Collection<E>, T> java.util.function.Function<L, com.mojang.serialization.DataResult<L>> ensureHomogenous(java.util.function.Function<E, T>);
    public static <A> com.mojang.serialization.Codec<A> catchDecoderException(com.mojang.serialization.Codec<A>);
    public static com.mojang.serialization.Codec<java.time.temporal.TemporalAccessor> temporalCodec(java.time.format.DateTimeFormatter);
    public static com.mojang.serialization.MapCodec<java.util.OptionalLong> asOptionalLong(com.mojang.serialization.MapCodec<java.util.Optional<java.lang.Long>>);
    private static com.mojang.serialization.MapCodec<com.mojang.authlib.GameProfile> gameProfileCodec(com.mojang.serialization.Codec<java.util.UUID>);
    public static <K, V> com.mojang.serialization.Codec<java.util.Map<K, V>> sizeLimitedMap(com.mojang.serialization.Codec<java.util.Map<K, V>>, int);
    public static <T> com.mojang.serialization.Codec<it.unimi.dsi.fastutil.objects.Object2BooleanMap<T>> object2BooleanMap(com.mojang.serialization.Codec<T>);
    public static <K, V> com.mojang.serialization.MapCodec<V> dispatchOptionalValue(java.lang.String, java.lang.String, com.mojang.serialization.Codec<K>, java.util.function.Function<? super V, ? extends K>, java.util.function.Function<? super K, ? extends com.mojang.serialization.Codec<? extends V>>);
    public static <A> com.mojang.serialization.Codec<java.util.Optional<A>> optionalEmptyMap(com.mojang.serialization.Codec<A>);
    public static <E extends java.lang.Enum<E>> com.mojang.serialization.Codec<E> legacyEnum(java.util.function.Function<java.lang.String, E>);
    public static com.mojang.serialization.Codec<java.nio.file.Path> pathCodec(java.util.function.Function<java.lang.String, java.nio.file.Path>);
    public static com.mojang.serialization.Codec<java.nio.file.Path> relativeNormalizedSubPathCodec(java.util.function.Function<java.lang.String, java.nio.file.Path>);
    public static com.mojang.serialization.Codec<java.nio.file.Path> guardedPathCodec(java.nio.file.Path);
    public static <A> com.mojang.serialization.MapCodec<A> optionalAlwaysPresentFieldOf(com.mojang.serialization.Codec<A>, java.lang.String, A, boolean);
    public static <A> com.mojang.serialization.MapCodec<A> optionalAlwaysPresentFieldOf(com.mojang.serialization.Codec<A>, java.lang.String, A);
    private static java.lang.Object lambda$optionalAlwaysPresentFieldOf$0(java.lang.Object, java.util.Optional);
    private static com.mojang.serialization.DataResult lambda$guardedPathCodec$1(java.nio.file.Path, java.nio.file.Path);
    private static java.lang.String lambda$guardedPathCodec$2(java.nio.file.Path, java.nio.file.Path);
    private static java.nio.file.Path lambda$guardedPathCodec$0(java.nio.file.FileSystem, java.lang.String);
    private static com.mojang.serialization.DataResult lambda$relativeNormalizedSubPathCodec$0(java.nio.file.Path);
    private static java.lang.String lambda$relativeNormalizedSubPathCodec$3(java.nio.file.Path);
    private static java.lang.String lambda$relativeNormalizedSubPathCodec$2(java.nio.file.Path);
    private static java.lang.String lambda$relativeNormalizedSubPathCodec$1(java.nio.file.Path);
    private static java.lang.String lambda$pathCodec$0(java.nio.file.Path);
    private static com.mojang.serialization.DataResult lambda$legacyEnum$0(java.util.function.Function, java.lang.String);
    private static java.lang.String lambda$legacyEnum$1(java.lang.String);
    private static com.mojang.serialization.DataResult lambda$static$63(java.lang.String);
    private static java.lang.String lambda$static$64(char);
    private static com.mojang.serialization.DataResult lambda$static$62(java.lang.String);
    private static com.mojang.serialization.DataResult lambda$static$60(java.lang.String);
    private static java.lang.String lambda$static$61(java.lang.String);
    private static com.mojang.serialization.DataResult lambda$sizeLimitedMap$0(int, java.util.Map);
    private static java.lang.String lambda$sizeLimitedMap$1(java.util.Map, int);
    private static com.mojang.serialization.DataResult lambda$static$58(java.lang.String);
    private static java.lang.String lambda$static$59(java.lang.String);
    private static com.mojang.serialization.DataResult lambda$static$56(java.lang.String);
    private static java.lang.String lambda$static$57();
    private static com.mojang.datafixers.kinds.App lambda$gameProfileCodec$0(com.mojang.serialization.Codec, com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static com.mojang.serialization.DataResult lambda$static$54(java.lang.String);
    private static java.lang.String lambda$static$55(java.lang.String);
    private static com.mojang.datafixers.util.Either lambda$static$53(com.mojang.authlib.properties.PropertyMap);
    private static com.mojang.authlib.properties.PropertyMap lambda$static$49(com.mojang.datafixers.util.Either);
    private static void lambda$static$52(com.google.common.collect.ImmutableMultimap$Builder, java.util.List);
    private static void lambda$static$50(com.google.common.collect.ImmutableMultimap$Builder, java.util.Map);
    private static void lambda$static$51(com.google.common.collect.ImmutableMultimap$Builder, java.lang.String, java.util.List);
    private static com.mojang.serialization.DataResult lambda$static$47(java.util.Map);
    private static java.lang.String lambda$static$48(java.util.Map);
    private static com.mojang.datafixers.kinds.App lambda$static$44(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static com.mojang.authlib.properties.Property lambda$static$46(java.lang.String, java.lang.String, java.util.Optional);
    private static java.util.Optional lambda$static$45(com.mojang.authlib.properties.Property);
    private static java.util.stream.LongStream lambda$static$43(java.util.BitSet);
    private static java.util.BitSet lambda$static$42(java.util.stream.LongStream);
    private static java.util.Optional lambda$static$41(java.util.OptionalLong);
    private static java.util.OptionalLong lambda$static$40(java.util.Optional);
    private static com.mojang.serialization.DataResult lambda$static$37(java.lang.String);
    private static net.minecraft.util.ExtraCodecs$TagOrElementLocation lambda$static$39(net.minecraft.resources.Identifier);
    private static net.minecraft.util.ExtraCodecs$TagOrElementLocation lambda$static$38(net.minecraft.resources.Identifier);
    private static com.mojang.serialization.DataResult lambda$static$36(java.lang.String);
    private static java.lang.String lambda$static$35(byte[]);
    private static com.mojang.serialization.DataResult lambda$static$33(java.lang.String);
    private static java.lang.String lambda$static$34();
    private static com.mojang.serialization.DataResult lambda$temporalCodec$0(java.time.format.DateTimeFormatter, java.lang.String);
    private static com.mojang.serialization.DataResult lambda$static$31(java.lang.String);
    private static java.lang.String lambda$static$32(java.lang.String, java.util.regex.PatternSyntaxException);
    private static com.mojang.serialization.DataResult lambda$ensureHomogenous$0(java.util.function.Function, java.util.Collection);
    private static java.lang.String lambda$ensureHomogenous$1(java.lang.Object, java.lang.Object, java.lang.Object);
    private static com.mojang.serialization.DataResult lambda$nonEmptyMap$0(java.util.Map);
    private static java.lang.String lambda$nonEmptyMap$1();
    private static com.mojang.serialization.DataResult lambda$nonEmptyHolderSet$0(net.minecraft.core.HolderSet);
    private static java.lang.String lambda$nonEmptyHolderSet$1();
    private static com.mojang.serialization.DataResult lambda$nonEmptyList$0(java.util.List);
    private static java.lang.String lambda$nonEmptyList$1();
    private static java.lang.String lambda$floatRange$0(float, float, java.lang.Float);
    private static java.lang.String lambda$static$30(java.lang.Float);
    private static java.lang.String lambda$static$29(java.lang.Float);
    private static com.mojang.serialization.DataResult lambda$floatRangeMinExclusiveWithMessage$0(float, float, java.util.function.Function, java.lang.Float);
    private static java.lang.String lambda$floatRangeMinExclusiveWithMessage$1(java.util.function.Function, java.lang.Float);
    private static com.mojang.serialization.DataResult lambda$floatRangeMinInclusiveWithMessage$0(float, float, java.util.function.Function, java.lang.Float);
    private static java.lang.String lambda$floatRangeMinInclusiveWithMessage$1(java.util.function.Function, java.lang.Float);
    private static java.lang.String lambda$longRange$0(int, int, java.lang.Long);
    private static java.lang.String lambda$static$28(java.lang.Long);
    private static java.lang.String lambda$static$27(java.lang.Long);
    private static com.mojang.serialization.DataResult lambda$longRangeWithMessage$0(long, long, java.util.function.Function, java.lang.Long);
    private static java.lang.String lambda$longRangeWithMessage$1(java.util.function.Function, java.lang.Long);
    private static java.lang.String lambda$intRange$0(int, int, java.lang.Integer);
    private static java.lang.String lambda$static$26(java.lang.Integer);
    private static java.lang.String lambda$static$25(java.lang.Integer);
    private static com.mojang.serialization.DataResult lambda$intRangeWithMessage$0(int, int, java.util.function.Function, java.lang.Integer);
    private static java.lang.String lambda$intRangeWithMessage$1(java.util.function.Function, java.lang.Integer);
    private static com.mojang.datafixers.util.Either lambda$compactListCodec$2(java.util.List);
    private static java.util.List lambda$compactListCodec$0(com.mojang.datafixers.util.Either);
    private static java.util.List lambda$compactListCodec$1(java.util.List);
    private static com.mojang.serialization.DataResult lambda$idResolverCodec$7(java.util.function.Function, java.lang.Object);
    private static java.lang.String lambda$idResolverCodec$8(java.lang.Object);
    private static com.mojang.serialization.DataResult lambda$idResolverCodec$5(java.util.function.Function, java.lang.Object);
    private static java.lang.String lambda$idResolverCodec$6(java.lang.Object);
    private static com.mojang.serialization.DataResult lambda$idResolverCodec$3(java.util.function.ToIntFunction, int, java.lang.Object);
    private static java.lang.String lambda$idResolverCodec$4(java.lang.Object);
    private static com.mojang.serialization.DataResult lambda$idResolverCodec$0(java.util.function.IntFunction, java.lang.Integer);
    private static com.mojang.serialization.DataResult lambda$idResolverCodec$1(java.lang.Integer);
    private static java.lang.String lambda$idResolverCodec$2(java.lang.Integer);
    private static com.mojang.datafixers.util.Either lambda$intervalCodec$8(java.util.function.Function, java.util.function.Function, java.lang.Object);
    private static com.mojang.serialization.DataResult lambda$intervalCodec$6(java.util.function.BiFunction, com.mojang.datafixers.util.Either);
    private static com.mojang.serialization.DataResult lambda$intervalCodec$7(java.util.function.BiFunction, java.lang.Object);
    private static com.mojang.datafixers.util.Pair lambda$intervalCodec$5(java.util.function.Function, java.util.function.Function, java.lang.Object);
    private static com.mojang.serialization.DataResult lambda$intervalCodec$4(java.util.function.BiFunction, com.mojang.datafixers.util.Pair);
    private static com.mojang.datafixers.kinds.App lambda$intervalCodec$3(com.mojang.serialization.Codec, java.lang.String, java.lang.String, com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static java.util.List lambda$intervalCodec$2(java.util.function.Function, java.util.function.Function, java.lang.Object);
    private static com.mojang.serialization.DataResult lambda$intervalCodec$0(java.util.function.BiFunction, java.util.List);
    private static com.mojang.serialization.DataResult lambda$intervalCodec$1(java.util.function.BiFunction, java.util.List);
    private static com.mojang.serialization.DataResult lambda$static$23(java.lang.Integer);
    private static java.lang.String lambda$static$24(java.lang.Integer);
    private static java.lang.Integer lambda$static$22(org.joml.Vector4fc);
    private static java.lang.Integer lambda$static$21(org.joml.Vector3fc);
    private static java.lang.String lambda$hexColor$5(int, java.lang.Integer);
    private static com.mojang.serialization.DataResult lambda$hexColor$0(int, long, java.lang.String);
    private static java.lang.String lambda$hexColor$4(java.lang.String);
    private static java.lang.String lambda$hexColor$3(java.lang.String);
    private static java.lang.String lambda$hexColor$2(int, int);
    private static java.lang.String lambda$hexColor$1();
    private static java.util.List lambda$static$20(org.joml.Matrix4fc);
    private static com.mojang.serialization.DataResult lambda$static$18(java.util.List);
    private static org.joml.Matrix4f lambda$static$19(java.util.List);
    private static com.mojang.datafixers.kinds.App lambda$static$15(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static org.joml.Vector3fc lambda$static$17(org.joml.AxisAngle4f);
    private static java.lang.Float lambda$static$16(org.joml.AxisAngle4f);
    private static java.util.List lambda$static$14(org.joml.Quaternionfc);
    private static com.mojang.serialization.DataResult lambda$static$12(java.util.List);
    private static org.joml.Quaternionf lambda$static$13(java.util.List);
    private static java.util.List lambda$static$11(org.joml.Vector4fc);
    private static com.mojang.serialization.DataResult lambda$static$9(java.util.List);
    private static org.joml.Vector4f lambda$static$10(java.util.List);
    private static java.util.List lambda$static$8(org.joml.Vector3ic);
    private static com.mojang.serialization.DataResult lambda$static$6(java.util.List);
    private static org.joml.Vector3i lambda$static$7(java.util.List);
    private static java.util.List lambda$static$5(org.joml.Vector3fc);
    private static com.mojang.serialization.DataResult lambda$static$3(java.util.List);
    private static org.joml.Vector3f lambda$static$4(java.util.List);
    private static java.util.List lambda$static$2(org.joml.Vector2fc);
    private static com.mojang.serialization.DataResult lambda$static$0(java.util.List);
    private static org.joml.Vector2f lambda$static$1(java.util.List);
    private static com.mojang.serialization.Dynamic lambda$converter$1(com.mojang.serialization.DynamicOps, java.lang.Object);
    private static java.lang.Object lambda$converter$0(com.mojang.serialization.DynamicOps, com.mojang.serialization.Dynamic);
    static {};
}
```
