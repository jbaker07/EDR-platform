---
type: "interface"
fqcn: "net.minecraft.world.attribute.EnvironmentAttributes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.attribute.EnvironmentAttributes

System: [[20-Systems/net.minecraft.world.attribute|net.minecraft.world.attribute]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `CREATURE_WORLD_GEN_SPAWN_PROBABILITY` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@7 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.setCreatureGeneratio | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `FAST_LAVA` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@8 in `FluidVariantAttributes$3.getViscosity` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `FOG_COLOR` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@7 in `BiomeModificationContextImpl$EffectsContextImpl.setFogColor` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `MUSIC_VOLUME` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@7 in `BiomeModificationContextImpl$EffectsContextImpl.setMusicVolume` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@9 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@12 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@1 in `BiomeModificationContextImpl$AttributesContextImpl.addAll` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@1 in `BiomeModificationContextImpl$AttributesContextImpl.set` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@1 in `BiomeModificationContextImpl$AttributesContextImpl.setModifier` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@10 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@44 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@57 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@92 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.applyPendingChanges | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NATURAL_MOB_SPAWNS` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@112 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.applyPendingChange | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `SKY_COLOR` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@7 in `BiomeModificationContextImpl$EffectsContextImpl.setSkyColor` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `WATER_FOG_COLOR` | `Lnet/minecraft/world/attribute/EnvironmentAttribute;` | exact | getstatic@7 in `BiomeModificationContextImpl$EffectsContextImpl.setWaterFogColor` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (52 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final FOG_COLOR : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final FOG_START_DISTANCE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final FOG_END_DISTANCE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final SKY_FOG_END_DISTANCE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final CLOUD_FOG_END_DISTANCE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final WATER_FOG_COLOR : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final WATER_FOG_START_DISTANCE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final WATER_FOG_END_DISTANCE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final SKY_COLOR : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final SUNRISE_SUNSET_COLOR : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final CLOUD_COLOR : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final CLOUD_HEIGHT : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final SUN_ANGLE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final MOON_ANGLE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final STAR_ANGLE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final MOON_PHASE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final STAR_BRIGHTNESS : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final BLOCK_LIGHT_TINT : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final SKY_LIGHT_COLOR : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final SKY_LIGHT_FACTOR : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final NIGHT_VISION_COLOR : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final AMBIENT_LIGHT_COLOR : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final DEFAULT_DRIPSTONE_PARTICLE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final AMBIENT_PARTICLES : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final BACKGROUND_MUSIC : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final MUSIC_VOLUME : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final AMBIENT_SOUNDS : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final FIREFLY_BUSH_SOUNDS : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final SKY_LIGHT_LEVEL : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final CAN_START_RAID : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final WATER_EVAPORATES : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final BED_RULE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final STRAW_BED_RULE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final RESPAWN_ANCHOR_WORKS : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final NETHER_PORTAL_SPAWNS_PIGLINS : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final FAST_LAVA : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final INCREASED_FIRE_BURNOUT : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final EYEBLOSSOM_OPEN : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final TURTLE_EGG_HATCH_CHANCE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final PIGLINS_ZOMBIFY : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final SNOW_GOLEM_MELTS : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final CREAKING_ACTIVE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final SURFACE_SLIME_SPAWN_CHANCE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final CAT_WAKING_UP_GIFT_CHANCE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final BEES_STAY_IN_HIVE : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final MONSTERS_BURN : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final CAN_PILLAGER_PATROL_SPAWN : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final NATURAL_MOB_SPAWNS : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final CREATURE_WORLD_GEN_SPAWN_PROBABILITY : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final VILLAGER_ACTIVITY : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final BABY_VILLAGER_ACTIVITY : Lnet/minecraft/world/attribute/EnvironmentAttribute;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static bootstrap(Lnet/minecraft/core/Registry;)Lnet/minecraft/world/attribute/EnvironmentAttribute;
private static register(Ljava/lang/String;Lnet/minecraft/world/attribute/EnvironmentAttribute$Builder;)Lnet/minecraft/world/attribute/EnvironmentAttribute;
static <clinit>()V
```
