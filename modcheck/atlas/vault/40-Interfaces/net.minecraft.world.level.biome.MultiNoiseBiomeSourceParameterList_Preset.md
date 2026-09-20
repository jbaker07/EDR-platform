---
type: "interface"
fqcn: "net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `usedBiomes()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset extends java.lang.Record {
    private final net.minecraft.resources.Identifier id;
    private final net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset$SourceProvider provider;
    public static final net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset NETHER;
    public static final net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset OVERWORLD;
    private static final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset> BY_NAME;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset> CODEC;
    public net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset(net.minecraft.resources.Identifier, net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset$SourceProvider);
    private static <T> net.minecraft.world.level.biome.Climate$ParameterList<T> generateOverworldBiomes(java.util.function.Function<net.minecraft.resources.ResourceKey<net.minecraft.world.level.biome.Biome>, T>);
    public java.util.stream.Stream<net.minecraft.resources.ResourceKey<net.minecraft.world.level.biome.Biome>> usedBiomes();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.resources.Identifier id();
    public net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset$SourceProvider provider();
    private static net.minecraft.resources.ResourceKey lambda$usedBiomes$0(net.minecraft.resources.ResourceKey);
    private static void lambda$generateOverworldBiomes$0(com.google.common.collect.ImmutableList$Builder, java.util.function.Function, com.mojang.datafixers.util.Pair);
    private static com.mojang.serialization.DataResult lambda$static$4(net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset);
    private static com.mojang.serialization.DataResult lambda$static$1(net.minecraft.resources.Identifier);
    private static com.mojang.serialization.DataResult lambda$static$2(net.minecraft.resources.Identifier);
    private static java.lang.String lambda$static$3(net.minecraft.resources.Identifier);
    private static net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset lambda$static$0(net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset);
    static {};
}
```
