---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.storage.SerializableChunkData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.storage.SerializableChunkData

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `copyOf` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/ch` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `parse` | `(Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/l` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `read` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/a` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `write` | `()Lnet/minecraft/nbt/CompoundTag;` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| wraps | `lambda$unpackStructureReferences$0` | `(Lnet/minecraft/core/Registry;Lnet/minecraft/world/level/ChunkPos;Ljav` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (30 fields, 47 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final containerFactory : Lnet/minecraft/world/level/chunk/PalettedContainerFactory;
private final chunkPos : Lnet/minecraft/world/level/ChunkPos;
private final minSectionY : I
private final lastUpdateTime : J
private final inhabitedTime : J
private final chunkStatus : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private final blendingData : Lnet/minecraft/world/level/levelgen/blending/BlendingData$Packed;
private final belowZeroRetrogen : Lnet/minecraft/world/level/levelgen/BelowZeroRetrogen;
private final upgradeData : Lnet/minecraft/world/level/chunk/UpgradeData;
private final heightmaps : Ljava/util/Map;
private final packedTicks : Lnet/minecraft/world/level/chunk/ChunkAccess$PackedTicks;
private final postProcessingSections : [Lit/unimi/dsi/fastutil/shorts/ShortList;
private final lightCorrect : Z
private final sectionData : Ljava/util/List;
private final entities : Ljava/util/List;
private final blockEntities : Ljava/util/List;
private final structureData : Lnet/minecraft/nbt/CompoundTag;
private static final BLOCK_TICKS_CODEC : Lcom/mojang/serialization/Codec;
private static final FLUID_TICKS_CODEC : Lcom/mojang/serialization/Codec;
private static final LOGGER : Lorg/slf4j/Logger;
private static final TAG_UPGRADE_DATA : Ljava/lang/String;
private static final BLOCK_TICKS_TAG : Ljava/lang/String;
private static final FLUID_TICKS_TAG : Ljava/lang/String;
public static final X_POS_TAG : Ljava/lang/String;
public static final Z_POS_TAG : Ljava/lang/String;
public static final HEIGHTMAPS_TAG : Ljava/lang/String;
public static final IS_LIGHT_ON_TAG : Ljava/lang/String;
public static final SECTIONS_TAG : Ljava/lang/String;
public static final BLOCK_LIGHT_TAG : Ljava/lang/String;
public static final SKY_LIGHT_TAG : Ljava/lang/String;
public <init>(Lnet/minecraft/world/level/chunk/PalettedContainerFactory;Lnet/minecraft/world/level/ChunkPos;IJJLnet/minecraft/world/level/chunk/status/ChunkStatus;Lnet/minecraft/world/level/levelgen/blending/BlendingData$Packed;Lnet/minecraft/world/level/levelgen/BelowZeroRetrogen;Lnet/minecraft/world/level/chunk/UpgradeData;Ljava/util/Map;Lnet/minecraft/world/level/chunk/ChunkAccess$PackedTicks;[Lit/unimi/dsi/fastutil/shorts/ShortList;ZLjava/util/List;Ljava/util/List;Ljava/util/List;Lnet/minecraft/nbt/CompoundTag;)V
public static parse(Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/chunk/PalettedContainerFactory;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/storage/SerializableChunkData;
public read(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/ai/village/poi/PoiManager;Lnet/minecraft/world/level/chunk/storage/RegionStorageInfo;Lnet/minecraft/world/level/ChunkPos;)Lnet/minecraft/world/level/chunk/ProtoChunk;
private static logErrors(Lnet/minecraft/world/level/ChunkPos;ILjava/lang/String;)V
public static copyOf(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/ChunkAccess;)Lnet/minecraft/world/level/chunk/storage/SerializableChunkData;
public write()Lnet/minecraft/nbt/CompoundTag;
private static saveTicks(Lnet/minecraft/nbt/CompoundTag;Lnet/minecraft/world/level/chunk/ChunkAccess$PackedTicks;)V
public static getChunkStatusFromTag(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private static postLoadChunk(Lnet/minecraft/server/level/ServerLevel;Ljava/util/List;Ljava/util/List;)Lnet/minecraft/world/level/chunk/LevelChunk$PostLoadProcessor;
private static packStructureData(Lnet/minecraft/world/level/levelgen/structure/pieces/StructurePieceSerializationContext;Lnet/minecraft/world/level/ChunkPos;Ljava/util/Map;Ljava/util/Map;)Lnet/minecraft/nbt/CompoundTag;
private static unpackStructureStart(Lnet/minecraft/world/level/levelgen/structure/pieces/StructurePieceSerializationContext;Lnet/minecraft/nbt/CompoundTag;J)Ljava/util/Map;
private static unpackStructureReferences(Lnet/minecraft/core/RegistryAccess;Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/nbt/CompoundTag;)Ljava/util/Map;
private static packOffsets([Lit/unimi/dsi/fastutil/shorts/ShortList;)Lnet/minecraft/nbt/ListTag;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public containerFactory()Lnet/minecraft/world/level/chunk/PalettedContainerFactory;
public chunkPos()Lnet/minecraft/world/level/ChunkPos;
public minSectionY()I
public lastUpdateTime()J
public inhabitedTime()J
public chunkStatus()Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public blendingData()Lnet/minecraft/world/level/levelgen/blending/BlendingData$Packed;
public belowZeroRetrogen()Lnet/minecraft/world/level/levelgen/BelowZeroRetrogen;
public upgradeData()Lnet/minecraft/world/level/chunk/UpgradeData;
public heightmaps()Ljava/util/Map;
public packedTicks()Lnet/minecraft/world/level/chunk/ChunkAccess$PackedTicks;
public postProcessingSections()[Lit/unimi/dsi/fastutil/shorts/ShortList;
public lightCorrect()Z
public sectionData()Ljava/util/List;
public entities()Ljava/util/List;
public blockEntities()Ljava/util/List;
public structureData()Lnet/minecraft/nbt/CompoundTag;
private static synthetic lambda$unpackStructureReferences$0(Lnet/minecraft/core/Registry;Lnet/minecraft/world/level/ChunkPos;Ljava/util/Map;Ljava/lang/String;Lnet/minecraft/nbt/Tag;)V
private static synthetic lambda$unpackStructureReferences$1(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/resources/Identifier;J)Z
private static synthetic lambda$postLoadChunk$0(Ljava/util/List;Lnet/minecraft/server/level/ServerLevel;Ljava/util/List;Lnet/minecraft/world/level/chunk/LevelChunk;)V
private static synthetic lambda$write$0(Lnet/minecraft/nbt/CompoundTag;Lnet/minecraft/world/level/levelgen/Heightmap$Types;[J)V
private static synthetic lambda$copyOf$1(I)[Lit/unimi/dsi/fastutil/shorts/ShortList;
private static synthetic lambda$copyOf$0(Lit/unimi/dsi/fastutil/shorts/ShortList;)Lit/unimi/dsi/fastutil/shorts/ShortArrayList;
private static synthetic lambda$parse$5(Lcom/mojang/serialization/Codec;Lnet/minecraft/world/level/ChunkPos;ILnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/PalettedContainerRO;
private static synthetic lambda$parse$6(Lnet/minecraft/world/level/ChunkPos;ILjava/lang/String;)V
private static synthetic lambda$parse$3(Lcom/mojang/serialization/Codec;Lnet/minecraft/world/level/ChunkPos;ILnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/PalettedContainer;
private static synthetic lambda$parse$4(Lnet/minecraft/world/level/ChunkPos;ILjava/lang/String;)V
private static synthetic lambda$parse$1(Ljava/util/Map;Lnet/minecraft/nbt/CompoundTag;)V
private static synthetic lambda$parse$2(Ljava/util/Map;Lnet/minecraft/world/level/levelgen/Heightmap$Types;[J)V
private static synthetic lambda$parse$0(Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/UpgradeData;
static <clinit>()V
```
