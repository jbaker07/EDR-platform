---
type: "interface"
fqcn: "net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getLevelPath(Lnet/minecraft/world/level/storage/LevelResource;)Ljava/nio` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (40, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess implements java.lang.AutoCloseable {
    private net.minecraft.util.DirectoryLock lock;
    private final net.minecraft.world.level.storage.LevelStorageSource$LevelDirectory levelDirectory;
    private final java.lang.String levelId;
    private final java.util.Map<net.minecraft.world.level.storage.LevelResource, java.nio.file.Path> resources;
    final net.minecraft.world.level.storage.LevelStorageSource this$0;
    private net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess(net.minecraft.world.level.storage.LevelStorageSource, java.lang.String, java.nio.file.Path) throws java.io.IOException;
    private void createLock() throws java.io.IOException;
    public void releaseTemporarilyAndRun(org.apache.commons.io.function.IORunnable) throws java.io.IOException;
    public long estimateDiskSpace();
    public boolean checkForLowDiskSpace();
    public void safeClose();
    public net.minecraft.world.level.storage.LevelStorageSource parent();
    public net.minecraft.world.level.storage.LevelStorageSource$LevelDirectory getLevelDirectory();
    public java.lang.String getLevelId();
    public java.nio.file.Path getLevelPath(net.minecraft.world.level.storage.LevelResource);
    public java.nio.file.Path getDimensionPath(net.minecraft.resources.ResourceKey<net.minecraft.world.level.Level>);
    private void checkLock();
    public net.minecraft.world.level.storage.PlayerDataStorage createPlayerStorage();
    public void collectIssues(boolean) throws java.io.IOException;
    public net.minecraft.world.level.storage.LevelSummary fixAndGetSummary() throws java.io.IOException;
    public net.minecraft.world.level.storage.LevelSummary fixAndGetSummaryFromTag(com.mojang.serialization.Dynamic<?>);
    public com.mojang.serialization.Dynamic<?> getUnfixedDataTagWithFallback() throws java.io.IOException;
    public com.mojang.serialization.Dynamic<?> getUnfixedDataTag(boolean) throws java.io.IOException;
    private java.nio.file.Path getDataFile(boolean);
    public void saveDataTag(net.minecraft.world.level.storage.WorldData);
    public void saveDataTag(net.minecraft.world.level.storage.WorldData, java.util.UUID);
    public void saveLevelData(com.mojang.serialization.Dynamic<?>);
    private void saveLevelData(net.minecraft.nbt.CompoundTag);
    public java.util.Optional<java.nio.file.Path> getIconFile();
    public void deleteLevel() throws java.io.IOException;
    public void renameLevel(java.lang.String) throws java.io.IOException;
    public void renameAndDropPlayer(java.lang.String) throws java.io.IOException;
    private void modifyLevelDataWithoutDatafix(java.util.function.Consumer<net.minecraft.nbt.CompoundTag>) throws java.io.IOException;
    public long makeWorldBackup() throws java.io.IOException;
    public boolean hasWorldData();
    public void close() throws java.io.IOException;
    public boolean restoreLevelDataFromOld();
    public java.time.Instant getFileModificationTime(boolean);
    private static void lambda$renameAndDropPlayer$0(java.lang.String, net.minecraft.nbt.CompoundTag);
    private static void lambda$renameLevel$0(java.lang.String, net.minecraft.nbt.CompoundTag);
}
```
