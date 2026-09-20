---
type: "interface"
fqcn: "net.minecraft.world.level.storage.SavedDataStorage"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.SavedDataStorage

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/nio/file/Path;Lcom/mojang/datafixers/DataFixer;Lnet/m` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close()V` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `computeIfAbsent(Lnet/minecraft/world/level/saveddata/SavedDataType;)Lnet/mi` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `computeIfAbsent(Lnet/minecraft/world/level/saveddata/SavedDataType;)Lnet/mi` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `saveAndJoin()V` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `set(Lnet/minecraft/world/level/saveddata/SavedDataType;Lnet/min` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (32, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.SavedDataStorage implements java.lang.AutoCloseable {
    private static final org.slf4j.Logger LOGGER;
    private final java.util.Map<net.minecraft.world.level.saveddata.SavedDataType<?>, java.util.Optional<net.minecraft.world.level.saveddata.SavedData>> cache;
    private final com.mojang.datafixers.DataFixer fixerUpper;
    private final net.minecraft.core.HolderLookup$Provider registries;
    private final java.nio.file.Path dataFolder;
    private java.util.concurrent.CompletableFuture<?> pendingWriteFuture;
    private boolean closed;
    public net.minecraft.world.level.storage.SavedDataStorage(java.nio.file.Path, com.mojang.datafixers.DataFixer, net.minecraft.core.HolderLookup$Provider);
    private java.nio.file.Path getDataFile(net.minecraft.resources.Identifier);
    public <T extends net.minecraft.world.level.saveddata.SavedData> T computeIfAbsent(net.minecraft.world.level.saveddata.SavedDataType<T>);
    public <T extends net.minecraft.world.level.saveddata.SavedData> T get(net.minecraft.world.level.saveddata.SavedDataType<T>);
    private <T extends net.minecraft.world.level.saveddata.SavedData> T readSavedData(net.minecraft.world.level.saveddata.SavedDataType<T>);
    public <T extends net.minecraft.world.level.saveddata.SavedData> void set(net.minecraft.world.level.saveddata.SavedDataType<T>, T);
    public net.minecraft.nbt.CompoundTag readTagFromDisk(java.nio.file.Path, net.minecraft.util.datafix.DataFixTypes, int) throws java.io.IOException;
    private boolean isGzip(java.io.PushbackInputStream) throws java.io.IOException;
    public java.util.concurrent.CompletableFuture<?> scheduleSave();
    private java.util.Map<net.minecraft.world.level.saveddata.SavedDataType<?>, net.minecraft.nbt.CompoundTag> collectDirtyTagsToSave();
    private <T extends net.minecraft.world.level.saveddata.SavedData> net.minecraft.nbt.CompoundTag encodeUnchecked(net.minecraft.world.level.saveddata.SavedDataType<T>, net.minecraft.world.level.saveddata.SavedData, net.minecraft.resources.RegistryOps<net.minecraft.nbt.Tag>);
    private void tryWrite(net.minecraft.world.level.saveddata.SavedDataType<?>, net.minecraft.nbt.CompoundTag);
    public void saveAndJoin();
    public void close();
    private void lambda$collectDirtyTagsToSave$0(java.util.Map, net.minecraft.resources.RegistryOps, net.minecraft.world.level.saveddata.SavedDataType, java.util.Optional);
    private void lambda$collectDirtyTagsToSave$1(java.util.Map, net.minecraft.world.level.saveddata.SavedDataType, net.minecraft.resources.RegistryOps, net.minecraft.world.level.saveddata.SavedData);
    private java.util.concurrent.CompletionStage lambda$scheduleSave$3(java.util.Map, java.lang.Object);
    private static java.util.concurrent.CompletableFuture[] lambda$scheduleSave$6(int);
    private java.util.concurrent.CompletableFuture lambda$scheduleSave$4(java.util.Map$Entry);
    private void lambda$scheduleSave$5(java.util.Map$Entry);
    private java.util.concurrent.CompletionStage lambda$scheduleSave$0(int, int, java.util.Map, java.lang.Object);
    private static java.util.concurrent.CompletableFuture[] lambda$scheduleSave$2(int);
    private void lambda$scheduleSave$1(java.util.List);
    private static void lambda$readSavedData$0(net.minecraft.world.level.saveddata.SavedDataType, java.lang.String);
    static {};
}
```
