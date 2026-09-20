---
type: "interface"
fqcn: "net.minecraft.util.ExtraCodecs"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.ExtraCodecs

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `nonEmptyList` | `(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@9 in `CompositeBlockStateModelImpl$Unbaked.lambda$static$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `nonEmptyList` | `(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@104 in `CustomUnbakedBlockStateModelRegistry.<clinit>` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| reads | `POSITIVE_INT` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@12 in `CustomUnbakedBlockStateModelRegistry.lambda$static$1` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (49 fields, 169 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final JSON : Lcom/mojang/serialization/Codec;
public static final JAVA : Lcom/mojang/serialization/Codec;
public static final NBT : Lcom/mojang/serialization/Codec;
public static final VECTOR2F : Lcom/mojang/serialization/Codec;
public static final VECTOR3F : Lcom/mojang/serialization/Codec;
public static final VECTOR3I : Lcom/mojang/serialization/Codec;
public static final VECTOR4F : Lcom/mojang/serialization/Codec;
public static final QUATERNIONF_COMPONENTS : Lcom/mojang/serialization/Codec;
public static final AXISANGLE4F : Lcom/mojang/serialization/Codec;
public static final QUATERNIONF : Lcom/mojang/serialization/Codec;
public static final MATRIX4F : Lcom/mojang/serialization/Codec;
private static final HEX_COLOR_PREFIX : Ljava/lang/String;
public static final RGB_COLOR_CODEC : Lcom/mojang/serialization/Codec;
public static final ARGB_COLOR_CODEC : Lcom/mojang/serialization/Codec;
public static final RGB_COLOR_VEC3_CODEC : Lcom/mojang/serialization/Codec;
public static final ARGB_COLOR_VEC4_CODEC : Lcom/mojang/serialization/Codec;
public static final STRING_RGB_COLOR : Lcom/mojang/serialization/Codec;
public static final STRING_ARGB_COLOR : Lcom/mojang/serialization/Codec;
public static final STRING_RGB_VEC3_COLOR : Lcom/mojang/serialization/Codec;
public static final STRING_ARGB_VEC4_COLOR : Lcom/mojang/serialization/Codec;
public static final UNSIGNED_BYTE : Lcom/mojang/serialization/Codec;
public static final NON_NEGATIVE_INT : Lcom/mojang/serialization/Codec;
public static final POSITIVE_INT : Lcom/mojang/serialization/Codec;
public static final NON_NEGATIVE_LONG : Lcom/mojang/serialization/Codec;
public static final POSITIVE_LONG : Lcom/mojang/serialization/Codec;
public static final NON_NEGATIVE_FLOAT : Lcom/mojang/serialization/Codec;
public static final POSITIVE_FLOAT : Lcom/mojang/serialization/Codec;
public static final PATTERN : Lcom/mojang/serialization/Codec;
public static final INSTANT_ISO8601 : Lcom/mojang/serialization/Codec;
public static final BASE64_STRING : Lcom/mojang/serialization/Codec;
public static final ESCAPED_STRING : Lcom/mojang/serialization/Codec;
public static final TAG_OR_ELEMENT_ID : Lcom/mojang/serialization/Codec;
public static final toOptionalLong : Ljava/util/function/Function;
public static final fromOptionalLong : Ljava/util/function/Function;
public static final BIT_SET : Lcom/mojang/serialization/Codec;
public static final MAX_PROPERTY_NAME_LENGTH : I
public static final MAX_PROPERTY_VALUE_LENGTH : I
public static final MAX_PROPERTY_SIGNATURE_LENGTH : I
public static final MAX_PROPERTIES : I
private static final PROPERTY : Lcom/mojang/serialization/Codec;
public static final PROPERTY_MAP : Lcom/mojang/serialization/Codec;
public static final PLAYER_NAME : Lcom/mojang/serialization/Codec;
public static final AUTHLIB_GAME_PROFILE : Lcom/mojang/serialization/Codec;
public static final STORED_GAME_PROFILE : Lcom/mojang/serialization/MapCodec;
public static final NON_EMPTY_STRING : Lcom/mojang/serialization/Codec;
public static final CODEPOINT : Lcom/mojang/serialization/Codec;
public static final RESOURCE_PATH_CODEC : Lcom/mojang/serialization/Codec;
public static final UNTRUSTED_URI : Lcom/mojang/serialization/Codec;
public static final CHAT_STRING : Lcom/mojang/serialization/Codec;
public <init>()V
public static converter(Lcom/mojang/serialization/DynamicOps;)Lcom/mojang/serialization/Codec;
private static hexColor(I)Lcom/mojang/serialization/Codec;
public static intervalCodec(Lcom/mojang/serialization/Codec;Ljava/lang/String;Ljava/lang/String;Ljava/util/function/BiFunction;Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static orElsePartial(Ljava/lang/Object;)Lcom/mojang/serialization/Codec$ResultFunction;
public static idResolverCodec(Ljava/util/function/ToIntFunction;Ljava/util/function/IntFunction;I)Lcom/mojang/serialization/Codec;
public static idResolverCodec(Lcom/mojang/serialization/Codec;Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static orCompressed(Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static orCompressed(Lcom/mojang/serialization/MapCodec;Lcom/mojang/serialization/MapCodec;)Lcom/mojang/serialization/MapCodec;
public static overrideLifecycle(Lcom/mojang/serialization/Codec;Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static overrideLifecycle(Lcom/mojang/serialization/Codec;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static strictUnboundedMap(Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/Codec;)Lnet/minecraft/util/ExtraCodecs$StrictUnboundedMapCodec;
public static compactListCodec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static compactListCodec(Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
private static intRangeWithMessage(IILjava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static intRange(II)Lcom/mojang/serialization/Codec;
private static longRangeWithMessage(JJLjava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static longRange(II)Lcom/mojang/serialization/Codec;
private static floatRangeMinInclusiveWithMessage(FFLjava/util/function/Function;)Lcom/mojang/serialization/Codec;
private static floatRangeMinExclusiveWithMessage(FFLjava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static floatRange(FF)Lcom/mojang/serialization/Codec;
public static nonEmptyList(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static nonEmptyHolderSet(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static nonEmptyMap(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static retrieveContext(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;
public static ensureHomogenous(Ljava/util/function/Function;)Ljava/util/function/Function;
public static catchDecoderException(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static temporalCodec(Ljava/time/format/DateTimeFormatter;)Lcom/mojang/serialization/Codec;
public static asOptionalLong(Lcom/mojang/serialization/MapCodec;)Lcom/mojang/serialization/MapCodec;
private static gameProfileCodec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/MapCodec;
public static sizeLimitedMap(Lcom/mojang/serialization/Codec;I)Lcom/mojang/serialization/Codec;
public static object2BooleanMap(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static dispatchOptionalValue(Ljava/lang/String;Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;
public static optionalEmptyMap(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static legacyEnum(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static pathCodec(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static relativeNormalizedSubPathCodec(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;
public static guardedPathCodec(Ljava/nio/file/Path;)Lcom/mojang/serialization/Codec;
public static optionalAlwaysPresentFieldOf(Lcom/mojang/serialization/Codec;Ljava/lang/String;Ljava/lang/Object;Z)Lcom/mojang/serialization/MapCodec;
public static optionalAlwaysPresentFieldOf(Lcom/mojang/serialization/Codec;Ljava/lang/String;Ljava/lang/Object;)Lcom/mojang/serialization/MapCodec;
private static synthetic lambda$optionalAlwaysPresentFieldOf$0(Ljava/lang/Object;Ljava/util/Optional;)Ljava/lang/Object;
private static synthetic lambda$guardedPathCodec$1(Ljava/nio/file/Path;Ljava/nio/file/Path;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$guardedPathCodec$2(Ljava/nio/file/Path;Ljava/nio/file/Path;)Ljava/lang/String;
private static synthetic lambda$guardedPathCodec$0(Ljava/nio/file/FileSystem;Ljava/lang/String;)Ljava/nio/file/Path;
private static synthetic lambda$relativeNormalizedSubPathCodec$0(Ljava/nio/file/Path;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$relativeNormalizedSubPathCodec$3(Ljava/nio/file/Path;)Ljava/lang/String;
private static synthetic lambda$relativeNormalizedSubPathCodec$2(Ljava/nio/file/Path;)Ljava/lang/String;
private static synthetic lambda$relativeNormalizedSubPathCodec$1(Ljava/nio/file/Path;)Ljava/lang/String;
private static synthetic lambda$pathCodec$0(Ljava/nio/file/Path;)Ljava/lang/String;
private static synthetic lambda$legacyEnum$0(Ljava/util/function/Function;Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$legacyEnum$1(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$static$63(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$64(C)Ljava/lang/String;
private static synthetic lambda$static$62(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$60(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$61(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$sizeLimitedMap$0(ILjava/util/Map;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$sizeLimitedMap$1(Ljava/util/Map;I)Ljava/lang/String;
private static synthetic lambda$static$58(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$59(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$static$56(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$57()Ljava/lang/String;
private static synthetic lambda$gameProfileCodec$0(Lcom/mojang/serialization/Codec;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$54(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$55(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$static$53(Lcom/mojang/authlib/properties/PropertyMap;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$static$49(Lcom/mojang/datafixers/util/Either;)Lcom/mojang/authlib/properties/PropertyMap;
private static synthetic lambda$static$52(Lcom/google/common/collect/ImmutableMultimap$Builder;Ljava/util/List;)V
private static synthetic lambda$static$50(Lcom/google/common/collect/ImmutableMultimap$Builder;Ljava/util/Map;)V
private static synthetic lambda$static$51(Lcom/google/common/collect/ImmutableMultimap$Builder;Ljava/lang/String;Ljava/util/List;)V
private static synthetic lambda$static$47(Ljava/util/Map;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$48(Ljava/util/Map;)Ljava/lang/String;
private static synthetic lambda$static$44(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$46(Ljava/lang/String;Ljava/lang/String;Ljava/util/Optional;)Lcom/mojang/authlib/properties/Property;
private static synthetic lambda$static$45(Lcom/mojang/authlib/properties/Property;)Ljava/util/Optional;
private static synthetic lambda$static$43(Ljava/util/BitSet;)Ljava/util/stream/LongStream;
private static synthetic lambda$static$42(Ljava/util/stream/LongStream;)Ljava/util/BitSet;
private static synthetic lambda$static$41(Ljava/util/OptionalLong;)Ljava/util/Optional;
private static synthetic lambda$static$40(Ljava/util/Optional;)Ljava/util/OptionalLong;
private static synthetic lambda$static$37(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$39(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/util/ExtraCodecs$TagOrElementLocation;
private static synthetic lambda$static$38(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/util/ExtraCodecs$TagOrElementLocation;
private static synthetic lambda$static$36(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$35([B)Ljava/lang/String;
private static synthetic lambda$static$33(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$34()Ljava/lang/String;
private static synthetic lambda$temporalCodec$0(Ljava/time/format/DateTimeFormatter;Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$31(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$32(Ljava/lang/String;Ljava/util/regex/PatternSyntaxException;)Ljava/lang/String;
private static synthetic lambda$ensureHomogenous$0(Ljava/util/function/Function;Ljava/util/Collection;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$ensureHomogenous$1(Ljava/lang/Object;Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/String;
private static synthetic lambda$nonEmptyMap$0(Ljava/util/Map;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$nonEmptyMap$1()Ljava/lang/String;
private static synthetic lambda$nonEmptyHolderSet$0(Lnet/minecraft/core/HolderSet;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$nonEmptyHolderSet$1()Ljava/lang/String;
private static synthetic lambda$nonEmptyList$0(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$nonEmptyList$1()Ljava/lang/String;
private static synthetic lambda$floatRange$0(FFLjava/lang/Float;)Ljava/lang/String;
private static synthetic lambda$static$30(Ljava/lang/Float;)Ljava/lang/String;
private static synthetic lambda$static$29(Ljava/lang/Float;)Ljava/lang/String;
private static synthetic lambda$floatRangeMinExclusiveWithMessage$0(FFLjava/util/function/Function;Ljava/lang/Float;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$floatRangeMinExclusiveWithMessage$1(Ljava/util/function/Function;Ljava/lang/Float;)Ljava/lang/String;
private static synthetic lambda$floatRangeMinInclusiveWithMessage$0(FFLjava/util/function/Function;Ljava/lang/Float;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$floatRangeMinInclusiveWithMessage$1(Ljava/util/function/Function;Ljava/lang/Float;)Ljava/lang/String;
private static synthetic lambda$longRange$0(IILjava/lang/Long;)Ljava/lang/String;
private static synthetic lambda$static$28(Ljava/lang/Long;)Ljava/lang/String;
private static synthetic lambda$static$27(Ljava/lang/Long;)Ljava/lang/String;
private static synthetic lambda$longRangeWithMessage$0(JJLjava/util/function/Function;Ljava/lang/Long;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$longRangeWithMessage$1(Ljava/util/function/Function;Ljava/lang/Long;)Ljava/lang/String;
private static synthetic lambda$intRange$0(IILjava/lang/Integer;)Ljava/lang/String;
private static synthetic lambda$static$26(Ljava/lang/Integer;)Ljava/lang/String;
private static synthetic lambda$static$25(Ljava/lang/Integer;)Ljava/lang/String;
private static synthetic lambda$intRangeWithMessage$0(IILjava/util/function/Function;Ljava/lang/Integer;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$intRangeWithMessage$1(Ljava/util/function/Function;Ljava/lang/Integer;)Ljava/lang/String;
private static synthetic lambda$compactListCodec$2(Ljava/util/List;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$compactListCodec$0(Lcom/mojang/datafixers/util/Either;)Ljava/util/List;
private static synthetic lambda$compactListCodec$1(Ljava/util/List;)Ljava/util/List;
private static synthetic lambda$idResolverCodec$7(Ljava/util/function/Function;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$idResolverCodec$8(Ljava/lang/Object;)Ljava/lang/String;
private static synthetic lambda$idResolverCodec$5(Ljava/util/function/Function;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$idResolverCodec$6(Ljava/lang/Object;)Ljava/lang/String;
private static synthetic lambda$idResolverCodec$3(Ljava/util/function/ToIntFunction;ILjava/lang/Object;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$idResolverCodec$4(Ljava/lang/Object;)Ljava/lang/String;
private static synthetic lambda$idResolverCodec$0(Ljava/util/function/IntFunction;Ljava/lang/Integer;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$idResolverCodec$1(Ljava/lang/Integer;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$idResolverCodec$2(Ljava/lang/Integer;)Ljava/lang/String;
private static synthetic lambda$intervalCodec$8(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;
private static synthetic lambda$intervalCodec$6(Ljava/util/function/BiFunction;Lcom/mojang/datafixers/util/Either;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$intervalCodec$7(Ljava/util/function/BiFunction;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$intervalCodec$5(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$intervalCodec$4(Ljava/util/function/BiFunction;Lcom/mojang/datafixers/util/Pair;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$intervalCodec$3(Lcom/mojang/serialization/Codec;Ljava/lang/String;Ljava/lang/String;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$intervalCodec$2(Ljava/util/function/Function;Ljava/util/function/Function;Ljava/lang/Object;)Ljava/util/List;
private static synthetic lambda$intervalCodec$0(Ljava/util/function/BiFunction;Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$intervalCodec$1(Ljava/util/function/BiFunction;Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$23(Ljava/lang/Integer;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$24(Ljava/lang/Integer;)Ljava/lang/String;
private static synthetic lambda$static$22(Lorg/joml/Vector4fc;)Ljava/lang/Integer;
private static synthetic lambda$static$21(Lorg/joml/Vector3fc;)Ljava/lang/Integer;
private static synthetic lambda$hexColor$5(ILjava/lang/Integer;)Ljava/lang/String;
private static synthetic lambda$hexColor$0(IJLjava/lang/String;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$hexColor$4(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$hexColor$3(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$hexColor$2(II)Ljava/lang/String;
private static synthetic lambda$hexColor$1()Ljava/lang/String;
private static synthetic lambda$static$20(Lorg/joml/Matrix4fc;)Ljava/util/List;
private static synthetic lambda$static$18(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$19(Ljava/util/List;)Lorg/joml/Matrix4f;
private static synthetic lambda$static$15(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$17(Lorg/joml/AxisAngle4f;)Lorg/joml/Vector3fc;
private static synthetic lambda$static$16(Lorg/joml/AxisAngle4f;)Ljava/lang/Float;
private static synthetic lambda$static$14(Lorg/joml/Quaternionfc;)Ljava/util/List;
private static synthetic lambda$static$12(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$13(Ljava/util/List;)Lorg/joml/Quaternionf;
private static synthetic lambda$static$11(Lorg/joml/Vector4fc;)Ljava/util/List;
private static synthetic lambda$static$9(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$10(Ljava/util/List;)Lorg/joml/Vector4f;
private static synthetic lambda$static$8(Lorg/joml/Vector3ic;)Ljava/util/List;
private static synthetic lambda$static$6(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$7(Ljava/util/List;)Lorg/joml/Vector3i;
private static synthetic lambda$static$5(Lorg/joml/Vector3fc;)Ljava/util/List;
private static synthetic lambda$static$3(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$4(Ljava/util/List;)Lorg/joml/Vector3f;
private static synthetic lambda$static$2(Lorg/joml/Vector2fc;)Ljava/util/List;
private static synthetic lambda$static$0(Ljava/util/List;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1(Ljava/util/List;)Lorg/joml/Vector2f;
private static synthetic lambda$converter$1(Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)Lcom/mojang/serialization/Dynamic;
private static synthetic lambda$converter$0(Lcom/mojang/serialization/DynamicOps;Lcom/mojang/serialization/Dynamic;)Ljava/lang/Object;
static <clinit>()V
```
