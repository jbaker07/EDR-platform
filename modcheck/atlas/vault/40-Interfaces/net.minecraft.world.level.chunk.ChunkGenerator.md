---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.ChunkGenerator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.ChunkGenerator

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBiomeGenerationSettings` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/biome/BiomeGen` | exact | invokevirtual@5 in `BiomeModificationImpl.lambda$finalizeWorldGen$3` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getBiomeSource` | `()Lnet/minecraft/world/level/biome/BiomeSource;` | exact | invokevirtual@4 in `BiomeModificationImpl.lambda$finalizeWorldGen$2` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getBiomeSource` | `()Lnet/minecraft/world/level/biome/BiomeSource;` | exact | invokevirtual@32 in `BiomeSelectionContextImpl.canGenerateIn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `featuresPerStep` | `Ljava/util/function/Supplier;` | exact | putfield@13 in `BiomeModificationImpl.lambda$finalizeWorldGen$1` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (4 fields, 55 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
protected final biomeSource : Lnet/minecraft/world/level/biome/BiomeSource;
private final featuresPerStep : Ljava/util/function/Supplier;
private final generationSettingsGetter : Ljava/util/function/Function;
public <init>(Lnet/minecraft/world/level/biome/BiomeSource;)V
public <init>(Lnet/minecraft/world/level/biome/BiomeSource;Ljava/util/function/Function;)V
public validate()V
public getOrigin(Lnet/minecraft/world/level/levelgen/RandomState;)Lnet/minecraft/world/level/ChunkPos;
protected abstract codec()Lcom/mojang/serialization/MapCodec;
public createState(Lnet/minecraft/core/HolderLookup;Lnet/minecraft/world/level/levelgen/RandomState;J)Lnet/minecraft/world/level/chunk/ChunkGeneratorStructureState;
public getTypeNameForDataFixer()Ljava/util/Optional;
public createBiomes(Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/levelgen/blending/Blender;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/chunk/ChunkAccess;)Ljava/util/concurrent/CompletableFuture;
private doCreateBiomes(Lnet/minecraft/world/level/levelgen/blending/Blender;Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/chunk/ChunkAccess;)V
protected decorateBiomeResolver(Lnet/minecraft/world/level/levelgen/blending/Blender;Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/biome/BiomeResolver;)Lnet/minecraft/world/level/biome/BiomeResolver;
public findNearestMapStructure(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/HolderSet;Lnet/minecraft/core/BlockPos;IZ)Lcom/mojang/datafixers/util/Pair;
private getNearestGeneratedStructure(Ljava/util/Set;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/core/BlockPos;ZLnet/minecraft/world/level/levelgen/structure/placement/ConcentricRingsStructurePlacement;)Lcom/mojang/datafixers/util/Pair;
private static getNearestGeneratedStructure(Ljava/util/Set;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/world/level/StructureManager;IIIZJLnet/minecraft/world/level/levelgen/structure/placement/RandomSpreadStructurePlacement;)Lcom/mojang/datafixers/util/Pair;
private static getStructureGeneratingAt(Ljava/util/Set;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/world/level/StructureManager;ZLnet/minecraft/world/level/levelgen/structure/placement/StructurePlacement;Lnet/minecraft/world/level/ChunkPos;)Lcom/mojang/datafixers/util/Pair;
private static tryAddReference(Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/levelgen/structure/StructureStart;)Z
public applyBiomeDecoration(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/StructureManager;)V
private static getWritableArea(Lnet/minecraft/world/level/chunk/ChunkAccess;)Lnet/minecraft/world/level/levelgen/structure/BoundingBox;
public abstract spawnOriginalMobs(Lnet/minecraft/server/level/WorldGenRegion;)V
public getSpawnHeight(Lnet/minecraft/world/level/LevelHeightAccessor;)I
public getBiomeSource()Lnet/minecraft/world/level/biome/BiomeSource;
public abstract getGenDepth()I
public getMobsAt(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/entity/MobCategory;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/util/random/WeightedList;
public createStructures(Lnet/minecraft/core/RegistryAccess;Lnet/minecraft/world/level/chunk/ChunkGeneratorStructureState;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;Lnet/minecraft/resources/ResourceKey;)V
private tryGenerateStructure(Lnet/minecraft/world/level/levelgen/structure/StructureSet$StructureSelectionEntry;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/core/RegistryAccess;Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;JLnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/biome/Climate$Sampler;)Z
private static fetchReferences(Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/levelgen/structure/Structure;)I
public createReferences(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/chunk/ChunkAccess;)V
public abstract buildTerrain(Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/levelgen/blending/Blender;Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/biome/BiomeManager;Lnet/minecraft/server/level/WorldGenRegion;Ljava/util/Set;)Ljava/util/concurrent/CompletableFuture;
public abstract getSeaLevel()I
public abstract getMinY()I
public abstract getBaseHeight(IILnet/minecraft/world/level/levelgen/Heightmap$Types;Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/levelgen/RandomState;)I
public abstract getBaseColumn(IILnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/levelgen/RandomState;)Lnet/minecraft/world/level/NoiseColumn;
public getFirstFreeHeight(IILnet/minecraft/world/level/levelgen/Heightmap$Types;Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/levelgen/RandomState;)I
public getFirstOccupiedHeight(IILnet/minecraft/world/level/levelgen/Heightmap$Types;Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/levelgen/RandomState;)I
public abstract addDebugScreenInfo(Ljava/util/List;Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/densityfunction/SamplerContext;)V
public getBiomeGenerationSettings(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/biome/BiomeGenerationSettings;
private static synthetic lambda$createReferences$3(Lnet/minecraft/world/level/levelgen/structure/StructureStart;)Ljava/lang/String;
private static synthetic lambda$createReferences$2(Lnet/minecraft/world/level/levelgen/structure/StructureStart;)Ljava/lang/String;
private static synthetic lambda$createReferences$0(Ljava/util/Optional;Lnet/minecraft/world/level/levelgen/structure/StructureStart;)Ljava/lang/String;
private static synthetic lambda$createReferences$1(Lnet/minecraft/world/level/levelgen/structure/StructureStart;Lnet/minecraft/core/Registry;)Ljava/lang/String;
private synthetic lambda$createStructures$0(Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/chunk/ChunkGeneratorStructureState;Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/core/RegistryAccess;Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/biome/Climate$Sampler;Lnet/minecraft/core/Holder;)V
private static synthetic lambda$getMobsAt$2(Lorg/apache/commons/lang3/mutable/MutableBoolean;Ljava/util/function/Predicate;Lnet/minecraft/world/level/levelgen/structure/StructureStart;)V
private static synthetic lambda$getMobsAt$1(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/structure/StructureStart;)Z
private static synthetic lambda$getMobsAt$0(Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/levelgen/structure/StructureStart;)Z
private static synthetic lambda$applyBiomeDecoration$5(Lnet/minecraft/core/Registry;Lnet/minecraft/world/level/levelgen/placement/PlacedFeature;)Ljava/lang/String;
private static synthetic lambda$applyBiomeDecoration$4(Lit/unimi/dsi/fastutil/ints/IntSet;Lnet/minecraft/world/level/biome/FeatureSorter$StepFeatureData;Lnet/minecraft/world/level/levelgen/placement/PlacedFeature;)V
private synthetic lambda$applyBiomeDecoration$3(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/levelgen/WorldgenRandom;Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/levelgen/structure/StructureStart;)V
private static synthetic lambda$applyBiomeDecoration$2(Lnet/minecraft/core/Registry;Lnet/minecraft/world/level/levelgen/structure/Structure;)Ljava/lang/String;
private static synthetic lambda$applyBiomeDecoration$1(Lnet/minecraft/world/level/WorldGenLevel;Ljava/util/Set;Lnet/minecraft/world/level/ChunkPos;)V
private static synthetic lambda$applyBiomeDecoration$0(Lnet/minecraft/world/level/levelgen/structure/Structure;)Ljava/lang/Integer;
private static synthetic lambda$findNearestMapStructure$0(Lnet/minecraft/world/level/levelgen/structure/placement/StructurePlacement;)Ljava/util/Set;
private synthetic lambda$createBiomes$0(Lnet/minecraft/world/level/levelgen/blending/Blender;Lnet/minecraft/world/level/levelgen/RandomState;Lnet/minecraft/world/level/chunk/ChunkAccess;)Lnet/minecraft/world/level/chunk/ChunkAccess;
private static synthetic lambda$new$1(Lnet/minecraft/world/level/biome/BiomeSource;Ljava/util/function/Function;)Ljava/util/List;
private static synthetic lambda$new$2(Ljava/util/function/Function;Lnet/minecraft/core/Holder;)Ljava/util/List;
private static synthetic lambda$new$0(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/level/biome/BiomeGenerationSettings;
static <clinit>()V
```
