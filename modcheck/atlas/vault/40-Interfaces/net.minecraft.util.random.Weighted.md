---
type: "interface"
fqcn: "net.minecraft.util.random.Weighted"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.random.Weighted

System: [[20-Systems/net.minecraft.util.random|net.minecraft.util.random]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/Object;I)V` | exact | invokespecial@36 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.addSpawn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/Object;I)V` | exact | invokespecial@126 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/Object;I)V` | exact | invokespecial@160 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;)Lnet/minecraft/util/random/Weighted;` | exact | invokevirtual@6 in `CustomUnbakedBlockStateModelRegistry.lambda$static$3` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@93 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@3 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.lambda$removeSpa | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@51 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@15 in `WeightedVariantsMixin.particleMaterial` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `weight` | `()I` | exact | invokevirtual@123 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `weight` | `()I` | exact | invokevirtual@157 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (3 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final value : Ljava/lang/Object;
private final weight : I
private static final LOGGER : Lorg/slf4j/Logger;
public <init>(Ljava/lang/Object;I)V
public static codec(Lcom/mojang/serialization/Codec;)Lcom/mojang/serialization/Codec;
public static codec(Lcom/mojang/serialization/MapCodec;)Lcom/mojang/serialization/Codec;
public static streamCodec(Lnet/minecraft/network/codec/StreamCodec;)Lnet/minecraft/network/codec/StreamCodec;
public map(Ljava/util/function/Function;)Lnet/minecraft/util/random/Weighted;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public value()Ljava/lang/Object;
public weight()I
private static synthetic lambda$codec$0(Lcom/mojang/serialization/MapCodec;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
