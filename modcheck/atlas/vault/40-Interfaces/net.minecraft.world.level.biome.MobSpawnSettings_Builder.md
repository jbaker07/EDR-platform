---
type: "interface"
fqcn: "net.minecraft.world.level.biome.MobSpawnSettings$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.MobSpawnSettings$Builder

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@12 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.applyPendingCha | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `addAllCosts` | `(Ljava/util/Map;)Lnet/minecraft/world/level/biome/MobSpawnSettings$Bui` | exact | invokevirtual@71 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.applyPendingCha | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `addAllSpawns` | `(Lnet/minecraft/world/entity/MobCategory;Lnet/minecraft/util/random/We` | exact | invokevirtual@59 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.applyPendingCha | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/level/biome/MobSpawnSettings;` | exact | invokevirtual@74 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.applyPendingCha | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (2 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final spawnsByCategory : Ljava/util/Map;
private final mobSpawnCosts : Ljava/util/Map;
public <init>()V
public addSpawn(Lnet/minecraft/world/entity/EntityType;III)Lnet/minecraft/world/level/biome/MobSpawnSettings$Builder;
public addSpawn(Lnet/minecraft/world/entity/EntityType;ILnet/minecraft/util/valueproviders/IntProvider;)Lnet/minecraft/world/level/biome/MobSpawnSettings$Builder;
public addSpawn(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/MobCategory;ILnet/minecraft/util/valueproviders/IntProvider;)Lnet/minecraft/world/level/biome/MobSpawnSettings$Builder;
public addAllSpawns(Lnet/minecraft/world/entity/MobCategory;Lnet/minecraft/util/random/WeightedList;)Lnet/minecraft/world/level/biome/MobSpawnSettings$Builder;
public noSpawns(Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/world/level/biome/MobSpawnSettings$Builder;
public dontOverride(Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/world/level/biome/MobSpawnSettings$Builder;
public addMobSpawnCost(Lnet/minecraft/world/entity/EntityType;DD)Lnet/minecraft/world/level/biome/MobSpawnSettings$Builder;
public addAllCosts(Ljava/util/Map;)Lnet/minecraft/world/level/biome/MobSpawnSettings$Builder;
public build()Lnet/minecraft/world/level/biome/MobSpawnSettings;
private forCategory(Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/util/random/WeightedList$Builder;
private static synthetic lambda$forCategory$0(Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/util/random/WeightedList$Builder;
private static synthetic lambda$build$0(Ljava/util/Map$Entry;)Lnet/minecraft/util/random/WeightedList;
```
