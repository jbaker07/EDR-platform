---
type: "interface"
fqcn: "net.minecraft.server.level.GenerationChunkHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.GenerationChunkHolder

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/ChunkPos;)V` | exact | invokespecial@2 in `ChunkHolderMixin.<init>` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getFullStatus` | `()Lnet/minecraft/server/level/FullChunkStatus;` | exact | invokevirtual@91 in `ChunkStatusTasksMixin.onChunkLoad` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (11 fields, 29 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final CHUNK_STATUSES : Ljava/util/List;
private static final NOT_DONE_YET : Lnet/minecraft/server/level/ChunkResult;
public static final UNLOADED_CHUNK : Lnet/minecraft/server/level/ChunkResult;
public static final UNLOADED_CHUNK_FUTURE : Ljava/util/concurrent/CompletableFuture;
protected final pos : Lnet/minecraft/world/level/ChunkPos;
private highestAllowedStatus : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private final startedWork : Ljava/util/concurrent/atomic/AtomicReference;
private final futures : Ljava/util/concurrent/atomic/AtomicReferenceArray;
private final task : Ljava/util/concurrent/atomic/AtomicReference;
private final generationRefCount : Ljava/util/concurrent/atomic/AtomicInteger;
private generationSaveSyncFuture : Ljava/util/concurrent/CompletableFuture;
public <init>(Lnet/minecraft/world/level/ChunkPos;)V
public scheduleChunkGenerationTask(Lnet/minecraft/world/level/chunk/status/ChunkStatus;Lnet/minecraft/server/level/ChunkMap;)Ljava/util/concurrent/CompletableFuture;
 applyStep(Lnet/minecraft/world/level/chunk/status/ChunkStep;Lnet/minecraft/server/level/GeneratingChunkMap;Lnet/minecraft/util/StaticCache2D;)Ljava/util/concurrent/CompletableFuture;
protected updateHighestAllowedStatus(Lnet/minecraft/server/level/ChunkMap;)V
public replaceProtoChunk(Lnet/minecraft/world/level/chunk/ImposterProtoChunk;)V
 removeTask(Lnet/minecraft/server/level/ChunkGenerationTask;)V
private rescheduleChunkTask(Lnet/minecraft/server/level/ChunkMap;Lnet/minecraft/world/level/chunk/status/ChunkStatus;)V
private getOrCreateFuture(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Ljava/util/concurrent/CompletableFuture;
private failAndClearPendingFuturesBetween(Lnet/minecraft/world/level/chunk/status/ChunkStatus;Lnet/minecraft/world/level/chunk/status/ChunkStatus;)V
private failAndClearPendingFuture(ILjava/util/concurrent/CompletableFuture;)V
private completeFuture(Lnet/minecraft/world/level/chunk/status/ChunkStatus;Lnet/minecraft/world/level/chunk/ChunkAccess;)V
private findHighestStatusWithPendingFuture(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private acquireStatusBump(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Z
private isStatusDisallowed(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Z
protected abstract addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V
public increaseGenerationRefCount()V
public decreaseGenerationRefCount()V
public getChunkIfPresentUnchecked(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Lnet/minecraft/world/level/chunk/ChunkAccess;
public getChunkIfPresent(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Lnet/minecraft/world/level/chunk/ChunkAccess;
public getLatestChunk()Lnet/minecraft/world/level/chunk/ChunkAccess;
public getPersistedStatus()Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public getPos()Lnet/minecraft/world/level/ChunkPos;
public getFullStatus()Lnet/minecraft/server/level/FullChunkStatus;
public abstract getTicketLevel()I
public abstract getQueueLevel()I
public getAllFutures()Ljava/util/List;
public getLatestStatus()Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private synthetic lambda$applyStep$0(Lnet/minecraft/world/level/chunk/status/ChunkStep;Lnet/minecraft/world/level/chunk/ChunkAccess;Ljava/lang/Throwable;)Lnet/minecraft/server/level/ChunkResult;
static <clinit>()V
```
