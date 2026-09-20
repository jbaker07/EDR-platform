---
type: "interface"
fqcn: "net.minecraft.world.level.biome.BiomeGenerationSettings"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.BiomeGenerationSettings

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `features()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.biome.BiomeGenerationSettings {
    private static final org.slf4j.Logger LOGGER;
    public static final net.minecraft.world.level.biome.BiomeGenerationSettings EMPTY;
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.level.biome.BiomeGenerationSettings> CODEC;
    private final net.minecraft.core.HolderSet<net.minecraft.world.level.levelgen.carver.WorldCarver> carvers;
    private final java.util.List<net.minecraft.core.HolderSet<net.minecraft.world.level.levelgen.placement.PlacedFeature>> features;
    private final java.util.function.Supplier<java.util.List<net.minecraft.world.level.levelgen.feature.Feature>> boneMealFeatures;
    private final java.util.function.Supplier<java.util.Set<net.minecraft.world.level.levelgen.placement.PlacedFeature>> featureSet;
    private net.minecraft.world.level.biome.BiomeGenerationSettings(net.minecraft.core.HolderSet<net.minecraft.world.level.levelgen.carver.WorldCarver>, java.util.List<net.minecraft.core.HolderSet<net.minecraft.world.level.levelgen.placement.PlacedFeature>>);
    public java.lang.Iterable<net.minecraft.core.Holder<net.minecraft.world.level.levelgen.carver.WorldCarver>> getCarvers();
    public java.util.List<net.minecraft.world.level.levelgen.feature.Feature> getBoneMealFeatures();
    public java.util.List<net.minecraft.core.HolderSet<net.minecraft.world.level.levelgen.placement.PlacedFeature>> features();
    public boolean hasFeature(net.minecraft.world.level.levelgen.placement.PlacedFeature);
    private static java.util.Set lambda$new$3(java.util.List);
    private static java.util.List lambda$new$0(java.util.List);
    private static boolean lambda$new$2(net.minecraft.core.Holder);
    private static java.util.stream.Stream lambda$new$1(net.minecraft.core.Holder);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static java.util.List lambda$static$2(net.minecraft.world.level.biome.BiomeGenerationSettings);
    private static net.minecraft.core.HolderSet lambda$static$1(net.minecraft.world.level.biome.BiomeGenerationSettings);
    static {};
}
```
