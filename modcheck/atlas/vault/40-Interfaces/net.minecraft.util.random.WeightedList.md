---
type: "interface"
fqcn: "net.minecraft.util.random.WeightedList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.random.WeightedList

System: [[20-Systems/net.minecraft.util.random|net.minecraft.util.random]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getRandomOrThrow` | `(Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;` | exact | invokevirtual@6 in `WeightedVariantsMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getRandomOrThrow` | `(Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;` | exact | invokevirtual@6 in `WeightedVariantsMixin.createGeometryKey` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getRandomOrThrow` | `(Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;` | exact | invokevirtual@6 in `WeightedVariantsMixin.materialFlags` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `of` | `(Ljava/util/List;)Lnet/minecraft/util/random/WeightedList;` | exact | invokestatic@56 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.applyPendingChan | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `of` | `(Ljava/util/List;)Lnet/minecraft/util/random/WeightedList;` | exact | invokestatic@13 in `CustomUnbakedBlockStateModelRegistry.lambda$static$2` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `unwrap` | `()Ljava/util/List;` | exact | invokevirtual@58 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `unwrap` | `()Ljava/util/List;` | exact | invokevirtual@157 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `unwrap` | `()Ljava/util/List;` | exact | invokevirtual@4 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `unwrap` | `()Ljava/util/List;` | exact | invokevirtual@4 in `WeightedVariantsMixin.particleMaterial` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final FLAT_THRESHOLD : I
private final totalWeight : I
private final items : Ljava/util/List;
private final selector : Lnet/minecraft/util/random/WeightedList$Selector;
private <init>(Ljava/util/List;)V
public static of()Lnet/minecraft/util/random/WeightedList;
public static of(Ljava/lang/Object;)Lnet/minecraft/util/random/WeightedList;
public static of([Ljava/lang/Object;)Lnet/minecraft/util/random/WeightedList;
public static of([Lnet/minecraft/util/random/Weighted;)Lnet/minecraft/util/random/WeightedList;
public static of(Ljava/util/List;)Lnet/minecraft/util/random/WeightedList;
public static builder()Lnet/minecraft/util/random/WeightedList$Builder;
public isEmpty()Z
public map(Ljava/util/function/Function;)Lnet/minecraft/util/random/WeightedList;
public getRandom(Lnet/minecraft/util/RandomSource;)Ljava/util/Optional;
public getRandomOrThrow(Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;
public unwrap()Ljava/util/List;
private static entryToListCodec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static codec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static codec(Lcom/mojang/serialization/MapCodec;)Lcom/mojang/serialization/Codec;
private static entryToNonEmptyListCodec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static nonEmptyCodec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static nonEmptyCodec(Lcom/mojang/serialization/MapCodec;)Lcom/mojang/serialization/Codec;
public static streamCodec(Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
public contains(Ljava/lang/Object;)Z
public equals(Ljava/lang/Object;)Z
public hashCode()I
private static synthetic lambda$entryToNonEmptyListCodec$0(Lnet/minecraft/util/random/WeightedList;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$entryToNonEmptyListCodec$1()Ljava/lang/String;
private static synthetic lambda$map$0(Ljava/util/function/Function;Lnet/minecraft/util/random/Weighted;)Lnet/minecraft/util/random/Weighted;
```
