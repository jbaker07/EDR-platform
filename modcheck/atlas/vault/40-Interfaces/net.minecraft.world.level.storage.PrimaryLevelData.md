---
type: "interface"
fqcn: "net.minecraft.world.level.storage.PrimaryLevelData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.PrimaryLevelData

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/storage/ServerLevelData`, `net/minecraft/world/level/storage/WorldData`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createTag` | `(Ljava/util/UUID;)Lnet/minecraft/nbt/CompoundTag;` | exact | invokevirtual@13 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (17 fields, 45 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final LEVEL_NAME : Ljava/lang/String;
protected static final OLD_PLAYER : Ljava/lang/String;
protected static final SINGLEPLAYER_UUID : Ljava/lang/String;
protected static final OLD_WORLD_GEN_SETTINGS : Ljava/lang/String;
private settings : Lnet/minecraft/world/level/LevelSettings;
private final specialWorldProperty : Lnet/minecraft/world/level/storage/PrimaryLevelData$SpecialWorldProperty;
private final worldGenSettingsLifecycle : Lcom/mojang/serialization/Lifecycle;
private respawnData : Lnet/minecraft/world/level/storage/LevelData$RespawnData;
private gameTime : J
private final singlePlayerUUID : Ljava/util/UUID;
private final version : I
private initialized : Z
private final knownServerBrands : Ljava/util/Set;
private wasModded : Z
private final removedFeatureFlags : Ljava/util/Set;
private versionHistory : Ljava/util/List;
private <init>(Ljava/util/UUID;ZLnet/minecraft/world/level/storage/LevelData$RespawnData;JIZLjava/util/Set;Ljava/util/Set;Lnet/minecraft/world/level/LevelSettings;Lnet/minecraft/world/level/storage/PrimaryLevelData$SpecialWorldProperty;Lcom/mojang/serialization/Lifecycle;Ljava/util/List;)V
public <init>(Lnet/minecraft/world/level/LevelSettings;Lnet/minecraft/world/level/storage/PrimaryLevelData$SpecialWorldProperty;Lcom/mojang/serialization/Lifecycle;)V
public static parse(Lcom/mojang/serialization/Dynamic;Lnet/minecraft/world/level/LevelSettings;Lnet/minecraft/world/level/storage/PrimaryLevelData$SpecialWorldProperty;Lcom/mojang/serialization/Lifecycle;)Lnet/minecraft/world/level/storage/PrimaryLevelData;
public createTag(Ljava/util/UUID;)Lnet/minecraft/nbt/CompoundTag;
private setTagData(Lnet/minecraft/nbt/CompoundTag;Ljava/util/UUID;)V
public static writeLastPlayed(Lnet/minecraft/nbt/CompoundTag;)V
public static writeLastPlayed(Lcom/mojang/serialization/Dynamic;)Lcom/mojang/serialization/Dynamic;
public static writeVersionTag(Lnet/minecraft/nbt/CompoundTag;)V
public static writeVersionTag(Lcom/mojang/serialization/Dynamic;)Lcom/mojang/serialization/Dynamic;
public writeVersionHistory(Lnet/minecraft/nbt/CompoundTag;)V
private static stringCollectionToTag(Ljava/util/Set;)Lnet/minecraft/nbt/ListTag;
public getRespawnData()Lnet/minecraft/world/level/storage/LevelData$RespawnData;
public getGameTime()J
public getSinglePlayerUUID()Ljava/util/UUID;
public setGameTime(J)V
public setSpawn(Lnet/minecraft/world/level/storage/LevelData$RespawnData;)V
public getLevelName()Ljava/lang/String;
public getVersion()I
public getGameType()Lnet/minecraft/world/level/GameType;
public setGameType(Lnet/minecraft/world/level/GameType;)V
public isHardcore()Z
public isAllowCommands()Z
public setAllowCommands(Z)V
public isInitialized()Z
public setInitialized(Z)V
public getDifficulty()Lnet/minecraft/world/Difficulty;
public setDifficulty(Lnet/minecraft/world/Difficulty;)V
public isDifficultyLocked()Z
public setDifficultyLocked(Z)V
public fillCrashReportCategory(Lnet/minecraft/CrashReportCategory;Lnet/minecraft/world/level/LevelHeightAccessor;)V
public isFlatWorld()Z
public isDebugWorld()Z
public worldGenSettingsLifecycle()Lcom/mojang/serialization/Lifecycle;
public getDataConfiguration()Lnet/minecraft/world/level/WorldDataConfiguration;
public setDataConfiguration(Lnet/minecraft/world/level/WorldDataConfiguration;)V
public setModdedInfo(Ljava/lang/String;Z)V
public wasModded()Z
public getKnownServerBrands()Ljava/util/Set;
public getRemovedFeatureFlags()Ljava/util/Set;
public overworldData()Lnet/minecraft/world/level/storage/ServerLevelData;
public getLevelSettings()Lnet/minecraft/world/level/LevelSettings;
private static synthetic lambda$parse$2(Lcom/mojang/serialization/Dynamic;)Ljava/lang/Integer;
private static synthetic lambda$parse$1(Lcom/mojang/serialization/Dynamic;)Ljava/util/stream/Stream;
private static synthetic lambda$parse$0(Lcom/mojang/serialization/Dynamic;)Ljava/util/stream/Stream;
static <clinit>()V
```
