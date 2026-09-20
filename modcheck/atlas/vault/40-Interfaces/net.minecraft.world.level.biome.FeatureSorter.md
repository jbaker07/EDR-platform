---
type: "interface"
fqcn: "net.minecraft.world.level.biome.FeatureSorter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.FeatureSorter

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `buildFeaturesPerStep(Ljava/util/List;Ljava/util/function/Function;Z)Ljava/util/L` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.biome.FeatureSorter {
    public net.minecraft.world.level.biome.FeatureSorter();
    public static <T> java.util.List<net.minecraft.world.level.biome.FeatureSorter$StepFeatureData> buildFeaturesPerStep(java.util.List<T>, java.util.function.Function<T, java.util.List<net.minecraft.core.HolderSet<net.minecraft.world.level.levelgen.placement.PlacedFeature>>>, boolean);
    private static boolean lambda$buildFeaturesPerStep$2(int, net.minecraft.world.level.biome.FeatureSorter$1FeatureData);
    private static java.util.Set lambda$buildFeaturesPerStep$1(java.util.Comparator, net.minecraft.world.level.biome.FeatureSorter$1FeatureData);
    private static int lambda$buildFeaturesPerStep$0(org.apache.commons.lang3.mutable.MutableInt, java.lang.Object);
}
```
