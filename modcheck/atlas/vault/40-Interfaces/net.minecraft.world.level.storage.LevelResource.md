---
type: "interface"
fqcn: "net.minecraft.world.level.storage.LevelResource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.LevelResource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `DATA` | `Lnet/minecraft/world/level/storage/LevelResource;` | exact | getstatic@1 in `RegistryCustomContentState.getPath` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `DATA` | `Lnet/minecraft/world/level/storage/LevelResource;` | exact | getstatic@1 in `RegistryCustomContentState.getPath` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (14 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Ljava/lang/String;
public static final PLAYER_ADVANCEMENTS_DIR : Lnet/minecraft/world/level/storage/LevelResource;
public static final PLAYER_STATS_DIR : Lnet/minecraft/world/level/storage/LevelResource;
public static final PLAYER_DATA_DIR : Lnet/minecraft/world/level/storage/LevelResource;
public static final PLAYER_OLD_DATA_DIR : Lnet/minecraft/world/level/storage/LevelResource;
public static final LEVEL_DATA_FILE : Lnet/minecraft/world/level/storage/LevelResource;
public static final OLD_LEVEL_DATA_FILE : Lnet/minecraft/world/level/storage/LevelResource;
public static final ICON_FILE : Lnet/minecraft/world/level/storage/LevelResource;
public static final LOCK_FILE : Lnet/minecraft/world/level/storage/LevelResource;
public static final GENERATED_DIR : Lnet/minecraft/world/level/storage/LevelResource;
public static final DATAPACK_DIR : Lnet/minecraft/world/level/storage/LevelResource;
public static final MAP_RESOURCE_FILE : Lnet/minecraft/world/level/storage/LevelResource;
public static final DATA : Lnet/minecraft/world/level/storage/LevelResource;
public static final ROOT : Lnet/minecraft/world/level/storage/LevelResource;
public <init>(Ljava/lang/String;)V
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public id()Ljava/lang/String;
static <clinit>()V
```
