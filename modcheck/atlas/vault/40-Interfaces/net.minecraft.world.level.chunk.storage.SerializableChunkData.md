---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.storage.SerializableChunkData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.storage.SerializableChunkData

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `copyOf` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `parse` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `read` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `write` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| wraps | `lambda$unpackStructureReferences$0` | `@Redirect at INVOKE Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;L` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (77, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.chunk.storage.SerializableChunkData extends java.lang.Record {
    private final net.minecraft.world.level.chunk.PalettedContainerFactory containerFactory;
    private final net.minecraft.world.level.ChunkPos chunkPos;
    private final int minSectionY;
    private final long lastUpdateTime;
    private final long inhabitedTime;
    private final net.minecraft.world.level.chunk.status.ChunkStatus chunkStatus;
    private final net.minecraft.world.level.levelgen.blending.BlendingData$Packed blendingData;
    private final net.minecraft.world.level.levelgen.BelowZeroRetrogen belowZeroRetrogen;
    private final net.minecraft.world.level.chunk.UpgradeData upgradeData;
    private final java.util.Map<net.minecraft.world.level.levelgen.Heightmap$Types, long[]> heightmaps;
    private final net.minecraft.world.level.chunk.ChunkAccess$PackedTicks packedTicks;
    private final it.unimi.dsi.fastutil.shorts.ShortList[] postProcessingSections;
    private final boolean lightCorrect;
    private final java.util.List<net.minecraft.world.level.chunk.storage.SerializableChunkData$SectionData> sectionData;
    private final java.util.List<net.minecraft.nbt.CompoundTag> entities;
    private final java.util.List<net.minecraft.nbt.CompoundTag> blockEntities;
    private final net.minecraft.nbt.CompoundTag structureData;
    private static final com.mojang.serialization.Codec<java.util.List<net.minecraft.world.ticks.SavedTick<net.minecraft.world.level.block.Block>>> BLOCK_TICKS_CODEC;
    private static final com.mojang.serialization.Codec<java.util.List<net.minecraft.world.ticks.SavedTick<net.minecraft.world.level.material.Fluid>>> FLUID_TICKS_CODEC;
    private static final org.slf4j.Logger LOGGER;
    private static final java.lang.String TAG_UPGRADE_DATA;
    private static final java.lang.String BLOCK_TICKS_TAG;
    private static final java.lang.String FLUID_TICKS_TAG;
    public static final java.lang.String X_POS_TAG;
    public static final java.lang.String Z_POS_TAG;
    public static final java.lang.String HEIGHTMAPS_TAG;
    public static final java.lang.String IS_LIGHT_ON_TAG;
    public static final java.lang.String SECTIONS_TAG;
    public static final java.lang.String BLOCK_LIGHT_TAG;
    public static final java.lang.String SKY_LIGHT_TAG;
    public net.minecraft.world.level.chunk.storage.SerializableChunkData(net.minecraft.world.level.chunk.PalettedContainerFactory, net.minecraft.world.level.ChunkPos, int, long, long, net.minecraft.world.level.chunk.status.ChunkStatus, net.minecraft.world.level.levelgen.blending.BlendingData$Packed, net.minecraft.world.level.levelgen.BelowZeroRetrogen, net.minecraft.world.level.chunk.UpgradeData, java.util.Map<net.minecraft.world.level.levelgen.Heightmap$Types, long[]>, net.minecraft.world.level.chunk.ChunkAccess$PackedTicks, it.unimi.dsi.fastutil.shorts.ShortList[], boolean, java.util.List<net.minecraft.world.level.chunk.storage.SerializableChunkData$SectionData>, java.util.List<net.minecraft.nbt.CompoundTag>, java.util.List<net.minecraft.nbt.CompoundTag>, net.minecraft.nbt.CompoundTag);
    public static net.minecraft.world.level.chunk.storage.SerializableChunkData parse(net.minecraft.world.level.LevelHeightAccessor, net.minecraft.world.level.chunk.PalettedContainerFactory, net.minecraft.nbt.CompoundTag);
    public net.minecraft.world.level.chunk.ProtoChunk read(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.ai.village.poi.PoiManager, net.minecraft.world.level.chunk.storage.RegionStorageInfo, net.minecraft.world.level.ChunkPos);
    private static void logErrors(net.minecraft.world.level.ChunkPos, int, java.lang.String);
    public static net.minecraft.world.level.chunk.storage.SerializableChunkData copyOf(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.chunk.ChunkAccess);
    public net.minecraft.nbt.CompoundTag write();
    private static void saveTicks(net.minecraft.nbt.CompoundTag, net.minecraft.world.level.chunk.ChunkAccess$PackedTicks);
    public static net.minecraft.world.level.chunk.status.ChunkStatus getChunkStatusFromTag(net.minecraft.nbt.CompoundTag);
    private static net.minecraft.world.level.chunk.LevelChunk$PostLoadProcessor postLoadChunk(net.minecraft.server.level.ServerLevel, java.util.List<net.minecraft.nbt.CompoundTag>, java.util.List<net.minecraft.nbt.CompoundTag>);
    private static net.minecraft.nbt.CompoundTag packStructureData(net.minecraft.world.level.levelgen.structure.pieces.StructurePieceSerializationContext, net.minecraft.world.level.ChunkPos, java.util.Map<net.minecraft.world.level.levelgen.structure.Structure, net.minecraft.world.level.levelgen.structure.StructureStart>, java.util.Map<net.minecraft.world.level.levelgen.structure.Structure, it.unimi.dsi.fastutil.longs.LongSet>);
    private static java.util.Map<net.minecraft.world.level.levelgen.structure.Structure, net.minecraft.world.level.levelgen.structure.StructureStart> unpackStructureStart(net.minecraft.world.level.levelgen.structure.pieces.StructurePieceSerializationContext, net.minecraft.nbt.CompoundTag, long);
    private static java.util.Map<net.minecraft.world.level.levelgen.structure.Structure, it.unimi.dsi.fastutil.longs.LongSet> unpackStructureReferences(net.minecraft.core.RegistryAccess, net.minecraft.world.level.ChunkPos, net.minecraft.nbt.CompoundTag);
    private static net.minecraft.nbt.ListTag packOffsets(it.unimi.dsi.fastutil.shorts.ShortList[]);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.level.chunk.PalettedContainerFactory containerFactory();
    public net.minecraft.world.level.ChunkPos chunkPos();
    public int minSectionY();
    public long lastUpdateTime();
    public long inhabitedTime();
    public net.minecraft.world.level.chunk.status.ChunkStatus chunkStatus();
    public net.minecraft.world.level.levelgen.blending.BlendingData$Packed blendingData();
    public net.minecraft.world.level.levelgen.BelowZeroRetrogen belowZeroRetrogen();
    public net.minecraft.world.level.chunk.UpgradeData upgradeData();
    public java.util.Map<net.minecraft.world.level.levelgen.Heightmap$Types, long[]> heightmaps();
    public net.minecraft.world.level.chunk.ChunkAccess$PackedTicks packedTicks();
    public it.unimi.dsi.fastutil.shorts.ShortList[] postProcessingSections();
    public boolean lightCorrect();
    public java.util.List<net.minecraft.world.level.chunk.storage.SerializableChunkData$SectionData> sectionData();
    public java.util.List<net.minecraft.nbt.CompoundTag> entities();
    public java.util.List<net.minecraft.nbt.CompoundTag> blockEntities();
    public net.minecraft.nbt.CompoundTag structureData();
    private static void lambda$unpackStructureReferences$0(net.minecraft.core.Registry, net.minecraft.world.level.ChunkPos, java.util.Map, java.lang.String, net.minecraft.nbt.Tag);
    private static boolean lambda$unpackStructureReferences$1(net.minecraft.world.level.ChunkPos, net.minecraft.resources.Identifier, long);
    private static void lambda$postLoadChunk$0(java.util.List, net.minecraft.server.level.ServerLevel, java.util.List, net.minecraft.world.level.chunk.LevelChunk);
    private static void lambda$write$0(net.minecraft.nbt.CompoundTag, net.minecraft.world.level.levelgen.Heightmap$Types, long[]);
    private static it.unimi.dsi.fastutil.shorts.ShortList[] lambda$copyOf$1(int);
    private static it.unimi.dsi.fastutil.shorts.ShortArrayList lambda$copyOf$0(it.unimi.dsi.fastutil.shorts.ShortList);
    private static net.minecraft.world.level.chunk.PalettedContainerRO lambda$parse$5(com.mojang.serialization.Codec, net.minecraft.world.level.ChunkPos, int, net.minecraft.nbt.CompoundTag);
    private static void lambda$parse$6(net.minecraft.world.level.ChunkPos, int, java.lang.String);
    private static net.minecraft.world.level.chunk.PalettedContainer lambda$parse$3(com.mojang.serialization.Codec, net.minecraft.world.level.ChunkPos, int, net.minecraft.nbt.CompoundTag);
    private static void lambda$parse$4(net.minecraft.world.level.ChunkPos, int, java.lang.String);
    private static void lambda$parse$1(java.util.Map, net.minecraft.nbt.CompoundTag);
    private static void lambda$parse$2(java.util.Map, net.minecraft.world.level.levelgen.Heightmap$Types, long[]);
    private static net.minecraft.world.level.chunk.UpgradeData lambda$parse$0(net.minecraft.world.level.LevelHeightAccessor, net.minecraft.nbt.CompoundTag);
    static {};
}
```
