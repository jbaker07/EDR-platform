---
type: "interface"
fqcn: "net.minecraft.world.level.biome.Biomes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.Biomes

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `END_BARRENS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@69 in `TheEndBiomeData$Overrides.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_BARRENS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@129 in `TheEndBiomeData$Overrides.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_BARRENS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@173 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_HIGHLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@0 in `TheEndBiomes.addHighlandsBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_HIGHLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@82 in `TheEndBiomeData$Overrides.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_HIGHLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@79 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_HIGHLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@95 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_HIGHLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@131 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_HIGHLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@157 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_MIDLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@56 in `TheEndBiomeData$Overrides.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_MIDLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@114 in `TheEndBiomeData$Overrides.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `END_MIDLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@147 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `SMALL_END_ISLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@0 in `TheEndBiomes.addSmallIslandsBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `SMALL_END_ISLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@105 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `SMALL_END_ISLANDS` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@121 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `THE_END` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@0 in `TheEndBiomes.addMainIslandBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `THE_END` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@99 in `TheEndBiomeData$Overrides.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `THE_END` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@53 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `THE_END` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@69 in `TheEndBiomeData.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (67 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final THE_VOID : Lnet/minecraft/resources/ResourceKey;
public static final PLAINS : Lnet/minecraft/resources/ResourceKey;
public static final SUNFLOWER_PLAINS : Lnet/minecraft/resources/ResourceKey;
public static final SNOWY_PLAINS : Lnet/minecraft/resources/ResourceKey;
public static final ICE_SPIKES : Lnet/minecraft/resources/ResourceKey;
public static final DESERT : Lnet/minecraft/resources/ResourceKey;
public static final SWAMP : Lnet/minecraft/resources/ResourceKey;
public static final MANGROVE_SWAMP : Lnet/minecraft/resources/ResourceKey;
public static final FOREST : Lnet/minecraft/resources/ResourceKey;
public static final FLOWER_FOREST : Lnet/minecraft/resources/ResourceKey;
public static final BIRCH_FOREST : Lnet/minecraft/resources/ResourceKey;
public static final DAPPLED_FOREST : Lnet/minecraft/resources/ResourceKey;
public static final DARK_FOREST : Lnet/minecraft/resources/ResourceKey;
public static final PALE_GARDEN : Lnet/minecraft/resources/ResourceKey;
public static final OLD_GROWTH_BIRCH_FOREST : Lnet/minecraft/resources/ResourceKey;
public static final OLD_GROWTH_PINE_TAIGA : Lnet/minecraft/resources/ResourceKey;
public static final OLD_GROWTH_SPRUCE_TAIGA : Lnet/minecraft/resources/ResourceKey;
public static final TAIGA : Lnet/minecraft/resources/ResourceKey;
public static final SNOWY_TAIGA : Lnet/minecraft/resources/ResourceKey;
public static final SAVANNA : Lnet/minecraft/resources/ResourceKey;
public static final SAVANNA_PLATEAU : Lnet/minecraft/resources/ResourceKey;
public static final WINDSWEPT_HILLS : Lnet/minecraft/resources/ResourceKey;
public static final WINDSWEPT_GRAVELLY_HILLS : Lnet/minecraft/resources/ResourceKey;
public static final WINDSWEPT_FOREST : Lnet/minecraft/resources/ResourceKey;
public static final WINDSWEPT_SAVANNA : Lnet/minecraft/resources/ResourceKey;
public static final JUNGLE : Lnet/minecraft/resources/ResourceKey;
public static final SPARSE_JUNGLE : Lnet/minecraft/resources/ResourceKey;
public static final BAMBOO_JUNGLE : Lnet/minecraft/resources/ResourceKey;
public static final BADLANDS : Lnet/minecraft/resources/ResourceKey;
public static final ERODED_BADLANDS : Lnet/minecraft/resources/ResourceKey;
public static final WOODED_BADLANDS : Lnet/minecraft/resources/ResourceKey;
public static final MEADOW : Lnet/minecraft/resources/ResourceKey;
public static final CHERRY_GROVE : Lnet/minecraft/resources/ResourceKey;
public static final GROVE : Lnet/minecraft/resources/ResourceKey;
public static final SNOWY_SLOPES : Lnet/minecraft/resources/ResourceKey;
public static final FROZEN_PEAKS : Lnet/minecraft/resources/ResourceKey;
public static final JAGGED_PEAKS : Lnet/minecraft/resources/ResourceKey;
public static final STONY_PEAKS : Lnet/minecraft/resources/ResourceKey;
public static final RIVER : Lnet/minecraft/resources/ResourceKey;
public static final FROZEN_RIVER : Lnet/minecraft/resources/ResourceKey;
public static final BEACH : Lnet/minecraft/resources/ResourceKey;
public static final SNOWY_BEACH : Lnet/minecraft/resources/ResourceKey;
public static final STONY_SHORE : Lnet/minecraft/resources/ResourceKey;
public static final WARM_OCEAN : Lnet/minecraft/resources/ResourceKey;
public static final LUKEWARM_OCEAN : Lnet/minecraft/resources/ResourceKey;
public static final DEEP_LUKEWARM_OCEAN : Lnet/minecraft/resources/ResourceKey;
public static final OCEAN : Lnet/minecraft/resources/ResourceKey;
public static final DEEP_OCEAN : Lnet/minecraft/resources/ResourceKey;
public static final COLD_OCEAN : Lnet/minecraft/resources/ResourceKey;
public static final DEEP_COLD_OCEAN : Lnet/minecraft/resources/ResourceKey;
public static final FROZEN_OCEAN : Lnet/minecraft/resources/ResourceKey;
public static final DEEP_FROZEN_OCEAN : Lnet/minecraft/resources/ResourceKey;
public static final MUSHROOM_FIELDS : Lnet/minecraft/resources/ResourceKey;
public static final DRIPSTONE_CAVES : Lnet/minecraft/resources/ResourceKey;
public static final LUSH_CAVES : Lnet/minecraft/resources/ResourceKey;
public static final DEEP_DARK : Lnet/minecraft/resources/ResourceKey;
public static final SULFUR_CAVES : Lnet/minecraft/resources/ResourceKey;
public static final NETHER_WASTES : Lnet/minecraft/resources/ResourceKey;
public static final WARPED_FOREST : Lnet/minecraft/resources/ResourceKey;
public static final CRIMSON_FOREST : Lnet/minecraft/resources/ResourceKey;
public static final SOUL_SAND_VALLEY : Lnet/minecraft/resources/ResourceKey;
public static final BASALT_DELTAS : Lnet/minecraft/resources/ResourceKey;
public static final THE_END : Lnet/minecraft/resources/ResourceKey;
public static final END_HIGHLANDS : Lnet/minecraft/resources/ResourceKey;
public static final END_MIDLANDS : Lnet/minecraft/resources/ResourceKey;
public static final SMALL_END_ISLANDS : Lnet/minecraft/resources/ResourceKey;
public static final END_BARRENS : Lnet/minecraft/resources/ResourceKey;
public <init>()V
private static register(Ljava/lang/String;)Lnet/minecraft/resources/ResourceKey;
static <clinit>()V
```
