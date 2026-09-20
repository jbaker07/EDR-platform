---
type: "interface"
fqcn: "net.minecraft.world.entity.MobCategory"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.MobCategory

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`enum` public final; extends `java/lang/Enum`; implements `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `values` | `()[Lnet/minecraft/world/entity/MobCategory;` | exact | invokestatic@28 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/world/entity/MobCategory;` | exact | invokestatic@80 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/world/entity/MobCategory;` | exact | invokestatic@2 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.removeSpawns` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `MISC` | `Lnet/minecraft/world/entity/MobCategory;` | exact | getstatic@4 in `BiomeModifications.addSpawn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (17 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MONSTER : Lnet/minecraft/world/entity/MobCategory;
public static final CREATURE : Lnet/minecraft/world/entity/MobCategory;
public static final AMBIENT : Lnet/minecraft/world/entity/MobCategory;
public static final AXOLOTLS : Lnet/minecraft/world/entity/MobCategory;
public static final UNDERGROUND_WATER_CREATURE : Lnet/minecraft/world/entity/MobCategory;
public static final WATER_CREATURE : Lnet/minecraft/world/entity/MobCategory;
public static final WATER_AMBIENT : Lnet/minecraft/world/entity/MobCategory;
public static final MISC : Lnet/minecraft/world/entity/MobCategory;
public static final CODEC : Lcom/mojang/serialization/Codec;
private final max : I
private final isFriendly : Z
private final isPersistent : Z
private final name : Ljava/lang/String;
private final debugAbbreviation : Ljava/lang/String;
private final noDespawnDistance : I
private final despawnDistance : I
private static final synthetic $VALUES : [Lnet/minecraft/world/entity/MobCategory;
public static values()[Lnet/minecraft/world/entity/MobCategory;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/entity/MobCategory;
private <init>(Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;IZZI)V
public getName()Ljava/lang/String;
public getDebugAbbreviation()Ljava/lang/String;
public getSerializedName()Ljava/lang/String;
public getMaxInstancesPerChunk()I
public isFriendly()Z
public isPersistent()Z
public getDespawnDistance()I
public getNoDespawnDistance()I
private static synthetic $values()[Lnet/minecraft/world/entity/MobCategory;
static <clinit>()V
```
