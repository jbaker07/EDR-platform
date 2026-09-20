---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.WorldDimensions"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.WorldDimensions

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `checkStability` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| wraps | `lambda$static$0` | `@Redirect at INVOKE Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance` | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (32, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.levelgen.WorldDimensions extends java.lang.Record {
    private final java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.dimension.LevelStem> dimensions;
    public static final com.mojang.serialization.MapCodec<net.minecraft.world.level.levelgen.WorldDimensions> CODEC;
    private static final java.util.Set<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>> BUILTIN_ORDER;
    public net.minecraft.world.level.levelgen.WorldDimensions(java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.dimension.LevelStem>);
    public net.minecraft.world.level.levelgen.WorldDimensions(net.minecraft.core.Registry<net.minecraft.world.level.dimension.LevelStem>);
    public static java.util.stream.Stream<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>> keysInOrder(java.util.Set<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>>);
    public net.minecraft.world.level.levelgen.WorldDimensions replaceOverworldGenerator(net.minecraft.core.HolderLookup$Provider, net.minecraft.world.level.chunk.ChunkGenerator);
    public static java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.dimension.LevelStem> withOverworld(net.minecraft.core.HolderLookup<net.minecraft.world.level.dimension.DimensionType>, java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.chunk.ChunkGenerator);
    public static java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.dimension.LevelStem> withOverworld(java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.dimension.LevelStem>, net.minecraft.core.Holder<net.minecraft.world.level.dimension.DimensionType>, net.minecraft.world.level.chunk.ChunkGenerator);
    public net.minecraft.world.level.chunk.ChunkGenerator overworld();
    public java.util.Optional<net.minecraft.world.level.dimension.LevelStem> get(net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>);
    public com.google.common.collect.ImmutableSet<net.minecraft.resources.ResourceKey<net.minecraft.world.level.Level>> levels();
    public boolean isDebug();
    private static net.minecraft.world.level.storage.PrimaryLevelData$SpecialWorldProperty specialWorldProperty(net.minecraft.core.Registry<net.minecraft.world.level.dimension.LevelStem>);
    private static com.mojang.serialization.Lifecycle checkStability(net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.dimension.LevelStem);
    private static boolean isVanillaLike(net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.dimension.LevelStem);
    private static boolean isStableOverworld(net.minecraft.world.level.dimension.LevelStem);
    private static boolean isStableNether(net.minecraft.world.level.dimension.LevelStem);
    private static boolean isStableEnd(net.minecraft.world.level.dimension.LevelStem);
    public net.minecraft.world.level.levelgen.WorldDimensions$Complete bake(net.minecraft.core.Registry<net.minecraft.world.level.dimension.LevelStem>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.world.level.dimension.LevelStem> dimensions();
    private static void lambda$bake$3(net.minecraft.core.WritableRegistry, net.minecraft.world.level.levelgen.WorldDimensions$1Entry);
    private void lambda$bake$0(net.minecraft.core.Registry, java.util.List, net.minecraft.resources.ResourceKey);
    private static void lambda$bake$2(java.util.List, net.minecraft.resources.ResourceKey, net.minecraft.world.level.dimension.LevelStem);
    private java.util.Optional lambda$bake$1(net.minecraft.resources.ResourceKey);
    private static net.minecraft.world.level.storage.PrimaryLevelData$SpecialWorldProperty lambda$specialWorldProperty$0(net.minecraft.world.level.dimension.LevelStem);
    private static boolean lambda$keysInOrder$0(net.minecraft.resources.ResourceKey);
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
