---
type: "interface"
fqcn: "net.minecraft.server.level.GenerationChunkHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.GenerationChunkHolder

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/ChunkPos;)V` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getFullStatus()Lnet/minecraft/server/level/FullChunkStatus;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (40, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.server.level.GenerationChunkHolder {
    private static final java.util.List<net.minecraft.world.level.chunk.status.ChunkStatus> CHUNK_STATUSES;
    private static final net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.ChunkAccess> NOT_DONE_YET;
    public static final net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.ChunkAccess> UNLOADED_CHUNK;
    public static final java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.ChunkAccess>> UNLOADED_CHUNK_FUTURE;
    protected final net.minecraft.world.level.ChunkPos pos;
    private volatile net.minecraft.world.level.chunk.status.ChunkStatus highestAllowedStatus;
    private final java.util.concurrent.atomic.AtomicReference<net.minecraft.world.level.chunk.status.ChunkStatus> startedWork;
    private final java.util.concurrent.atomic.AtomicReferenceArray<java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.ChunkAccess>>> futures;
    private final java.util.concurrent.atomic.AtomicReference<net.minecraft.server.level.ChunkGenerationTask> task;
    private final java.util.concurrent.atomic.AtomicInteger generationRefCount;
    private volatile java.util.concurrent.CompletableFuture<java.lang.Void> generationSaveSyncFuture;
    public net.minecraft.server.level.GenerationChunkHolder(net.minecraft.world.level.ChunkPos);
    public java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.ChunkAccess>> scheduleChunkGenerationTask(net.minecraft.world.level.chunk.status.ChunkStatus, net.minecraft.server.level.ChunkMap);
    java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.ChunkAccess>> applyStep(net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.server.level.GeneratingChunkMap, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>);
    protected void updateHighestAllowedStatus(net.minecraft.server.level.ChunkMap);
    public void replaceProtoChunk(net.minecraft.world.level.chunk.ImposterProtoChunk);
    void removeTask(net.minecraft.server.level.ChunkGenerationTask);
    private void rescheduleChunkTask(net.minecraft.server.level.ChunkMap, net.minecraft.world.level.chunk.status.ChunkStatus);
    private java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.ChunkAccess>> getOrCreateFuture(net.minecraft.world.level.chunk.status.ChunkStatus);
    private void failAndClearPendingFuturesBetween(net.minecraft.world.level.chunk.status.ChunkStatus, net.minecraft.world.level.chunk.status.ChunkStatus);
    private void failAndClearPendingFuture(int, java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.ChunkAccess>>);
    private void completeFuture(net.minecraft.world.level.chunk.status.ChunkStatus, net.minecraft.world.level.chunk.ChunkAccess);
    private net.minecraft.world.level.chunk.status.ChunkStatus findHighestStatusWithPendingFuture(net.minecraft.world.level.chunk.status.ChunkStatus);
    private boolean acquireStatusBump(net.minecraft.world.level.chunk.status.ChunkStatus);
    private boolean isStatusDisallowed(net.minecraft.world.level.chunk.status.ChunkStatus);
    protected abstract void addSaveDependency(java.util.concurrent.CompletableFuture<?>);
    public void increaseGenerationRefCount();
    public void decreaseGenerationRefCount();
    public net.minecraft.world.level.chunk.ChunkAccess getChunkIfPresentUnchecked(net.minecraft.world.level.chunk.status.ChunkStatus);
    public net.minecraft.world.level.chunk.ChunkAccess getChunkIfPresent(net.minecraft.world.level.chunk.status.ChunkStatus);
    public net.minecraft.world.level.chunk.ChunkAccess getLatestChunk();
    public net.minecraft.world.level.chunk.status.ChunkStatus getPersistedStatus();
    public net.minecraft.world.level.ChunkPos getPos();
    public net.minecraft.server.level.FullChunkStatus getFullStatus();
    public abstract int getTicketLevel();
    public abstract int getQueueLevel();
    public java.util.List<com.mojang.datafixers.util.Pair<net.minecraft.world.level.chunk.status.ChunkStatus, java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.ChunkAccess>>>> getAllFutures();
    public net.minecraft.world.level.chunk.status.ChunkStatus getLatestStatus();
    private net.minecraft.server.level.ChunkResult lambda$applyStep$0(net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.world.level.chunk.ChunkAccess, java.lang.Throwable);
    static {};
}
```
