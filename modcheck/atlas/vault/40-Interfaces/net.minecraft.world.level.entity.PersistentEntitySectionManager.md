---
type: "interface"
fqcn: "net.minecraft.world.level.entity.PersistentEntitySectionManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.entity.PersistentEntitySectionManager

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `addEntity` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (65, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.entity.PersistentEntitySectionManager<T extends net.minecraft.world.level.entity.EntityAccess> implements java.lang.AutoCloseable {
    private static final org.slf4j.Logger LOGGER;
    private final java.util.Set<java.util.UUID> knownUuids;
    private final net.minecraft.world.level.entity.LevelCallback<T> callbacks;
    private final net.minecraft.world.level.entity.EntityPersistentStorage<T> permanentStorage;
    private final net.minecraft.world.level.entity.EntityLookup<T> visibleEntityStorage;
    private final net.minecraft.world.level.entity.EntitySectionStorage<T> sectionStorage;
    private final net.minecraft.world.level.entity.LevelEntityGetter<T> entityGetter;
    private final it.unimi.dsi.fastutil.longs.Long2ObjectMap<net.minecraft.world.level.entity.Visibility> chunkVisibility;
    private final it.unimi.dsi.fastutil.longs.Long2ObjectMap<net.minecraft.world.level.entity.PersistentEntitySectionManager$ChunkLoadStatus> chunkLoadStatuses;
    private final it.unimi.dsi.fastutil.longs.LongSet chunksToUnload;
    private final java.util.Queue<net.minecraft.world.level.entity.ChunkEntities<T>> loadingInbox;
    public net.minecraft.world.level.entity.PersistentEntitySectionManager(java.lang.Class<T>, net.minecraft.world.level.entity.LevelCallback<T>, net.minecraft.world.level.entity.EntityPersistentStorage<T>);
    private void removeSectionIfEmpty(long, net.minecraft.world.level.entity.EntitySection<T>);
    private boolean addEntityUuid(T);
    public boolean addNewEntity(T);
    private boolean addEntity(T, boolean);
    private static <T extends net.minecraft.world.level.entity.EntityAccess> net.minecraft.world.level.entity.Visibility getEffectiveStatus(T, net.minecraft.world.level.entity.Visibility);
    public boolean isTicking(net.minecraft.world.level.ChunkPos);
    public void addLegacyChunkEntities(java.util.stream.Stream<T>);
    public void addWorldGenChunkEntities(java.util.stream.Stream<T>);
    private void startTicking(T);
    private void stopTicking(T);
    private void startTracking(T);
    private void stopTracking(T);
    public void updateChunkStatus(net.minecraft.world.level.ChunkPos, net.minecraft.server.level.FullChunkStatus);
    public void updateChunkStatus(net.minecraft.world.level.ChunkPos, net.minecraft.world.level.entity.Visibility);
    private void ensureChunkQueuedForLoad(long);
    private boolean storeChunkSections(long, java.util.function.Consumer<T>);
    private void requestChunkLoad(long);
    private boolean processChunkUnload(long);
    private void unloadEntity(net.minecraft.world.level.entity.EntityAccess);
    private void processUnloads();
    public void processPendingLoads();
    public void tick();
    private it.unimi.dsi.fastutil.longs.LongSet getAllChunksToSave();
    public void autoSave();
    public void saveAll();
    public void close() throws java.io.IOException;
    public boolean isLoaded(java.util.UUID);
    public net.minecraft.world.level.entity.LevelEntityGetter<T> getEntityGetter();
    public boolean canPositionTick(net.minecraft.core.BlockPos);
    public boolean canPositionTick(net.minecraft.world.level.ChunkPos);
    public boolean areEntitiesLoaded(long);
    public void dumpSections(java.io.Writer) throws java.io.IOException;
    public java.lang.String gatherStats();
    public int count();
    private void lambda$dumpSections$0(net.minecraft.util.CsvOutput, long);
    private void lambda$dumpSections$1(net.minecraft.util.CsvOutput, net.minecraft.world.level.entity.PersistentEntitySectionManager$ChunkLoadStatus, long);
    private boolean lambda$saveAll$0(long);
    private static void lambda$saveAll$1(net.minecraft.world.level.entity.EntityAccess);
    private void lambda$autoSave$0(long);
    private static void lambda$autoSave$1(net.minecraft.world.level.entity.EntityAccess);
    private void lambda$processPendingLoads$0(net.minecraft.world.level.entity.EntityAccess);
    private boolean lambda$processUnloads$0(long);
    private void lambda$processChunkUnload$0(net.minecraft.world.level.entity.EntityAccess);
    private static java.lang.Void lambda$requestChunkLoad$0(net.minecraft.world.level.ChunkPos, java.lang.Throwable);
    private static java.util.stream.Stream lambda$storeChunkSections$0(net.minecraft.world.level.entity.EntitySection);
    private void lambda$updateChunkStatus$0(net.minecraft.world.level.entity.Visibility, net.minecraft.world.level.entity.EntitySection);
    private static boolean lambda$updateChunkStatus$4(net.minecraft.world.level.entity.EntityAccess);
    private static boolean lambda$updateChunkStatus$3(net.minecraft.world.level.entity.EntityAccess);
    private static boolean lambda$updateChunkStatus$2(net.minecraft.world.level.entity.EntityAccess);
    private static boolean lambda$updateChunkStatus$1(net.minecraft.world.level.entity.EntityAccess);
    private void lambda$addWorldGenChunkEntities$0(net.minecraft.world.level.entity.EntityAccess);
    private void lambda$addLegacyChunkEntities$0(net.minecraft.world.level.entity.EntityAccess);
    static {};
}
```
