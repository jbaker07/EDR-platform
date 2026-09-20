---
type: "interface"
fqcn: "net.minecraft.world.entity.SpawnPlacements"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.SpawnPlacements

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `register` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/Sp` | exact | invokestatic@25 in `FabricEntityTypeImpl$Builder$Mob.onBuild` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (1 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DATA_BY_TYPE : Ljava/util/Map;
public <init>()V
public static register(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/SpawnPlacementType;Lnet/minecraft/world/level/levelgen/Heightmap$Types;Lnet/minecraft/world/entity/SpawnPlacements$SpawnPredicate;)V
public static getPlacementType(Lnet/minecraft/world/entity/EntityType;)Lnet/minecraft/world/entity/SpawnPlacementType;
public static isSpawnPositionOk(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)Z
public static getHeightmapType(Lnet/minecraft/world/entity/EntityType;)Lnet/minecraft/world/level/levelgen/Heightmap$Types;
public static checkSpawnRules(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)Z
static <clinit>()V
```
