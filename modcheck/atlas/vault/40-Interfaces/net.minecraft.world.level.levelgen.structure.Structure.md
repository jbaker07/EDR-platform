---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.structure.Structure"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.structure.Structure

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `biomes()Lnet/minecraft/core/HolderSet;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.level.levelgen.structure.Structure {
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.levelgen.structure.Structure> DIRECT_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.Holder<net.minecraft.world.level.levelgen.structure.Structure>> CODEC;
    protected final net.minecraft.world.level.levelgen.structure.Structure$StructureSettings settings;
    public static <S extends net.minecraft.world.level.levelgen.structure.Structure> com.mojang.serialization.codecs.RecordCodecBuilder<S, net.minecraft.world.level.levelgen.structure.Structure$StructureSettings> settingsCodec(com.mojang.serialization.codecs.RecordCodecBuilder$Instance<S>);
    public static <S extends net.minecraft.world.level.levelgen.structure.Structure> com.mojang.serialization.MapCodec<S> simpleCodec(java.util.function.Function<net.minecraft.world.level.levelgen.structure.Structure$StructureSettings, S>);
    protected net.minecraft.world.level.levelgen.structure.Structure(net.minecraft.world.level.levelgen.structure.Structure$StructureSettings);
    public net.minecraft.core.HolderSet<net.minecraft.world.level.biome.Biome> biomes();
    public java.util.Map<net.minecraft.world.entity.MobCategory, net.minecraft.world.level.levelgen.structure.StructureSpawnOverride> spawnOverrides();
    public net.minecraft.world.level.levelgen.GenerationStep$Decoration step();
    public net.minecraft.world.level.levelgen.structure.TerrainAdjustment terrainAdaptation();
    public net.minecraft.world.level.levelgen.structure.BoundingBox adjustBoundingBox(net.minecraft.world.level.levelgen.structure.BoundingBox);
    public net.minecraft.world.level.levelgen.structure.StructureStart generate(net.minecraft.core.Holder<net.minecraft.world.level.levelgen.structure.Structure>, net.minecraft.resources.ResourceKey<net.minecraft.world.level.Level>, net.minecraft.core.RegistryAccess, net.minecraft.world.level.chunk.ChunkGenerator, net.minecraft.world.level.biome.BiomeSource, net.minecraft.world.level.biome.Climate$Sampler, net.minecraft.world.level.levelgen.RandomState, net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager, long, net.minecraft.world.level.ChunkPos, int, net.minecraft.world.level.LevelHeightAccessor, java.util.function.Predicate<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>>);
    protected static java.util.Optional<net.minecraft.world.level.levelgen.structure.Structure$GenerationStub> onTopOfChunkCenter(net.minecraft.world.level.levelgen.structure.Structure$GenerationContext, net.minecraft.world.level.levelgen.Heightmap$Types, java.util.function.Consumer<net.minecraft.world.level.levelgen.structure.pieces.StructurePiecesBuilder>);
    protected static java.util.Optional<net.minecraft.world.level.levelgen.structure.Structure$GenerationStub> onTopOfChunkCenterWithoutBiomeCheck(net.minecraft.world.level.levelgen.structure.Structure$GenerationContext, net.minecraft.world.level.levelgen.Heightmap$Types, java.util.function.Consumer<net.minecraft.world.level.levelgen.structure.pieces.StructurePiecesBuilder>);
    public void afterPlace(net.minecraft.world.level.WorldGenLevel, net.minecraft.world.level.StructureManager, net.minecraft.world.level.chunk.ChunkGenerator, net.minecraft.util.RandomSource, net.minecraft.world.level.levelgen.structure.BoundingBox, net.minecraft.world.level.ChunkPos, net.minecraft.world.level.levelgen.structure.pieces.PiecesContainer);
    private static int[] getCornerHeights(net.minecraft.world.level.levelgen.structure.Structure$GenerationContext, int, int, int, int);
    public static int getMeanFirstOccupiedHeight(net.minecraft.world.level.levelgen.structure.Structure$GenerationContext, int, int, int, int);
    protected static int getLowestY(net.minecraft.world.level.levelgen.structure.Structure$GenerationContext, int, int);
    protected static int getLowestY(net.minecraft.world.level.levelgen.structure.Structure$GenerationContext, int, int, int, int);
    protected net.minecraft.core.BlockPos getLowestYIn5by5Box(net.minecraft.world.level.levelgen.structure.Structure$GenerationContext, int, int, net.minecraft.world.level.block.Rotation);
    protected abstract java.util.Optional<net.minecraft.world.level.levelgen.structure.Structure$GenerationStub> findGenerationPoint(net.minecraft.world.level.levelgen.structure.Structure$GenerationContext);
    public java.util.Optional<net.minecraft.world.level.levelgen.structure.Structure$GenerationStub> findValidGenerationPoint(net.minecraft.world.level.levelgen.structure.Structure$GenerationContext);
    public abstract net.minecraft.world.level.levelgen.structure.StructureType<?> type();
    private static com.mojang.datafixers.kinds.App lambda$simpleCodec$0(java.util.function.Function, com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    private static net.minecraft.world.level.levelgen.structure.Structure$StructureSettings lambda$settingsCodec$0(net.minecraft.world.level.levelgen.structure.Structure);
    static {};
}
```
