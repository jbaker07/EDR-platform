---
type: "interface"
fqcn: "net.minecraft.world.level.biome.FeatureSorter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.FeatureSorter

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `buildFeaturesPerStep` | `(Ljava/util/List;Ljava/util/function/Function;Z)Ljava/util/List;` | exact | invokestatic@20 in `BiomeModificationImpl.lambda$finalizeWorldGen$2` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (0 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public static buildFeaturesPerStep(Ljava/util/List;Ljava/util/function/Function;Z)Ljava/util/List;
private static synthetic lambda$buildFeaturesPerStep$2(ILnet/minecraft/world/level/biome/FeatureSorter$1FeatureData;)Z
private static synthetic lambda$buildFeaturesPerStep$1(Ljava/util/Comparator;Lnet/minecraft/world/level/biome/FeatureSorter$1FeatureData;)Ljava/util/Set;
private static synthetic lambda$buildFeaturesPerStep$0(Lorg/apache/commons/lang3/mutable/MutableInt;Ljava/lang/Object;)I
```
