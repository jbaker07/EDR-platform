---
type: "interface"
fqcn: "net.minecraft.world.level.biome.MobSpawnSettings"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.MobSpawnSettings

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `allSpawnCosts` | `()Ljava/util/Map;` | exact | invokevirtual@186 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getMobsInCategory` | `(Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/util/random/W` | exact | invokevirtual@107 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getMobsToSpawn` | `(Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/util/random/W` | exact | invokevirtual@55 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (8 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final DEFAULT_CREATURE_WORLD_GEN_SPAWN_PROBABILITY : F
public static final EMPTY_MOB_LIST : Lnet/minecraft/util/random/WeightedList;
public static final EMPTY : Lnet/minecraft/world/level/biome/MobSpawnSettings;
public static final NO_SPAWNS : Lnet/minecraft/world/level/biome/MobSpawnSettings;
public static final CODEC : Lcom/mojang/serialization/Codec;
private final spawnsByCategory : Ljava/util/Map;
private final mobSpawnCosts : Ljava/util/Map;
private <init>(Ljava/util/Map;Ljava/util/Map;)V
public getMobsToSpawn(Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/util/random/WeightedList;
public getMobsInCategory(Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/util/random/WeightedList;
public definedCategories()Ljava/util/Set;
public getMobSpawnCost(Lnet/minecraft/world/entity/EntityType;)Lnet/minecraft/world/level/biome/MobSpawnSettings$MobSpawnCost;
public allSpawnCosts()Ljava/util/Map;
private static synthetic lambda$static$1(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$3(Lnet/minecraft/world/level/biome/MobSpawnSettings;)Ljava/util/Map;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/biome/MobSpawnSettings;)Ljava/util/Map;
private static synthetic lambda$static$0()Lnet/minecraft/world/level/biome/MobSpawnSettings;
static <clinit>()V
```
