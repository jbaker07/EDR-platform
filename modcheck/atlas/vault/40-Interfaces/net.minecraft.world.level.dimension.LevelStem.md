---
type: "interface"
fqcn: "net.minecraft.world.level.dimension.LevelStem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.dimension.LevelStem

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `generator()Lnet/minecraft/world/level/chunk/ChunkGenerator;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `generator()Lnet/minecraft/world/level/chunk/ChunkGenerator;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `CODECLcom/mojang/serialization/Codec;` | `` | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.dimension.LevelStem extends java.lang.Record {
    private final net.minecraft.core.Holder<net.minecraft.world.level.dimension.DimensionType> type;
    private final net.minecraft.world.level.chunk.ChunkGenerator generator;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.dimension.LevelStem> CODEC;
    public static final net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem> OVERWORLD;
    public static final net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem> NETHER;
    public static final net.minecraft.resources.ResourceKey<net.minecraft.world.level.dimension.LevelStem> END;
    public net.minecraft.world.level.dimension.LevelStem(net.minecraft.core.Holder<net.minecraft.world.level.dimension.DimensionType>, net.minecraft.world.level.chunk.ChunkGenerator);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.Holder<net.minecraft.world.level.dimension.DimensionType> type();
    public net.minecraft.world.level.chunk.ChunkGenerator generator();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
