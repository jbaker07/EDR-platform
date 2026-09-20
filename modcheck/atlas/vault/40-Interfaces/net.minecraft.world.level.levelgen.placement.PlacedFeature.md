---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.placement.PlacedFeature"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.placement.PlacedFeature

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getFeatures` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@81 in `BiomeSelectionContext.hasFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getFeatures` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@9 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.lambda$rebu | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (6 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final feature : Lnet/minecraft/core/Holder;
private final placement : Ljava/util/List;
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final LIST_CODEC : Lcom/mojang/serialization/Codec;
public static final LIST_OF_LISTS_CODEC : Lcom/mojang/serialization/Codec;
public <init>(Lnet/minecraft/core/Holder;Ljava/util/List;)V
public place(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/world/level/chunk/ChunkGenerator;Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/BlockPos;)Z
public getFeatures()Ljava/util/stream/Stream;
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public feature()Lnet/minecraft/core/Holder;
public placement()Ljava/util/List;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/levelgen/placement/PlacedFeature;)Ljava/util/List;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/levelgen/placement/PlacedFeature;)Lnet/minecraft/core/Holder;
static <clinit>()V
```
