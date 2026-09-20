---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.structure.Structure"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.structure.Structure

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `biomes` | `()Lnet/minecraft/core/HolderSet;` | exact | invokevirtual@29 in `BiomeSelectionContextImpl.validForStructure` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (3 fields, 23 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
protected final settings : Lnet/minecraft/world/level/levelgen/structure/Structure$StructureSettings;
public static settingsCodec(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/serialization/codecs/RecordCodecBuilder;
public static simpleCodec(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;
protected <init>(Lnet/minecraft/world/level/levelgen/structure/Structure$StructureSettings;)V
public biomes()Lnet/minecraft/core/HolderSet;
public spawnOverrides()Ljava/util/Map;
public step()Lnet/minecraft/world/level/levelgen/GenerationStep$Decoration;
public terrainAdaptation()Lnet/minecraft/world/level/levelgen/structure/TerrainAdjustment;
public adjustBoundingBox(Lnet/minecraft/world/level/levelgen/structure/BoundingBox;)Lnet/minecraft/world/level/levelgen/structure/BoundingBox;
public generate(Lnet/minecraft/core/Holder;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/RegistryAccess;Lnet/minecraft/world/level/chunk/ChunkGenerator;Lnet/minecraft/world/level/biome/BiomeSource;Lnet/minecraft/world/level/biome/Climate$Sampler;Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;JLnet/minecraft/world/level/ChunkPos;ILnet/minecraft/world/level/LevelHeightAccessor;Ljava/util/function/Predicate;)Lnet/minecraft/world/level/levelgen/structure/StructureStart;
protected static onTopOfChunkCenter(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;Lnet/minecraft/world/level/levelgen/Heightmap$Types;Ljava/util/function/Consumer;)Ljava/util/Optional;
protected static onTopOfChunkCenterWithoutBiomeCheck(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;Lnet/minecraft/world/level/levelgen/Heightmap$Types;Ljava/util/function/Consumer;)Ljava/util/Optional;
public afterPlace(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/chunk/ChunkGenerator;Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/level/levelgen/structure/BoundingBox;Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/levelgen/structure/pieces/PiecesContainer;)V
private static getCornerHeights(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;IIII)[I
public static getMeanFirstOccupiedHeight(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;IIII)I
protected static getLowestY(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;II)I
protected static getLowestY(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;IIII)I
protected getLowestYIn5by5Box(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;IILnet/minecraft/world/level/block/Rotation;)Lnet/minecraft/core/BlockPos;
protected abstract findGenerationPoint(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;)Ljava/util/Optional;
public findValidGenerationPoint(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;)Ljava/util/Optional;
public abstract type()Lnet/minecraft/world/level/levelgen/structure/StructureType;
private static synthetic lambda$simpleCodec$0(Ljava/util/function/Function;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$settingsCodec$0(Lnet/minecraft/world/level/levelgen/structure/Structure;)Lnet/minecraft/world/level/levelgen/structure/Structure$StructureSettings;
static <clinit>()V
```
