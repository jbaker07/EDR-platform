---
type: "interface"
fqcn: "net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getLevelPath` | `(Lnet/minecraft/world/level/storage/LevelResource;)Ljava/nio/file/Path` | exact | invokevirtual@4 in `RegistryCustomContentState.getPath` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (5 fields, 35 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private lock : Lnet/minecraft/util/DirectoryLock;
private final levelDirectory : Lnet/minecraft/world/level/storage/LevelStorageSource$LevelDirectory;
private final levelId : Ljava/lang/String;
private final resources : Ljava/util/Map;
final synthetic this$0 : Lnet/minecraft/world/level/storage/LevelStorageSource;
private <init>(Lnet/minecraft/world/level/storage/LevelStorageSource;Ljava/lang/String;Ljava/nio/file/Path;)V
private createLock()V
public releaseTemporarilyAndRun(Lorg/apache/commons/io/function/IORunnable;)V
public estimateDiskSpace()J
public checkForLowDiskSpace()Z
public safeClose()V
public parent()Lnet/minecraft/world/level/storage/LevelStorageSource;
public getLevelDirectory()Lnet/minecraft/world/level/storage/LevelStorageSource$LevelDirectory;
public getLevelId()Ljava/lang/String;
public getLevelPath(Lnet/minecraft/world/level/storage/LevelResource;)Ljava/nio/file/Path;
public getDimensionPath(Lnet/minecraft/resources/ResourceKey;)Ljava/nio/file/Path;
private checkLock()V
public createPlayerStorage()Lnet/minecraft/world/level/storage/PlayerDataStorage;
public collectIssues(Z)V
public fixAndGetSummary()Lnet/minecraft/world/level/storage/LevelSummary;
public fixAndGetSummaryFromTag(Lcom/mojang/serialization/Dynamic;)Lnet/minecraft/world/level/storage/LevelSummary;
public getUnfixedDataTagWithFallback()Lcom/mojang/serialization/Dynamic;
public getUnfixedDataTag(Z)Lcom/mojang/serialization/Dynamic;
private getDataFile(Z)Ljava/nio/file/Path;
public saveDataTag(Lnet/minecraft/world/level/storage/WorldData;)V
public saveDataTag(Lnet/minecraft/world/level/storage/WorldData;Ljava/util/UUID;)V
public saveLevelData(Lcom/mojang/serialization/Dynamic;)V
private saveLevelData(Lnet/minecraft/nbt/CompoundTag;)V
public getIconFile()Ljava/util/Optional;
public deleteLevel()V
public renameLevel(Ljava/lang/String;)V
public renameAndDropPlayer(Ljava/lang/String;)V
private modifyLevelDataWithoutDatafix(Ljava/util/function/Consumer;)V
public makeWorldBackup()J
public hasWorldData()Z
public close()V
public restoreLevelDataFromOld()Z
public getFileModificationTime(Z)Ljava/time/Instant;
private static synthetic lambda$renameAndDropPlayer$0(Ljava/lang/String;Lnet/minecraft/nbt/CompoundTag;)V
private static synthetic lambda$renameLevel$0(Ljava/lang/String;Lnet/minecraft/nbt/CompoundTag;)V
```
