---
type: "interface"
fqcn: "net.minecraft.server.level.ChunkHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ChunkHolder

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`class` public; extends `net/minecraft/server/level/GenerationChunkHolder`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getFullChunkFuture` | `()Ljava/util/concurrent/CompletableFuture;` | exact | invokevirtual@41 in `BlockEntityMixin.fabric_markChanged` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `broadcastBlockEntity` | `(Ljava/util/List;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `demoteFullChunk` | `(Lnet/minecraft/server/level/ChunkMap;Lnet/minecraft/server/level/Full` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateFutures` | `(Lnet/minecraft/server/level/ChunkMap;Ljava/util/concurrent/Executor;)` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateFutures` | `(Lnet/minecraft/server/level/ChunkMap;Ljava/util/concurrent/Executor;)` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateFutures` | `(Lnet/minecraft/server/level/ChunkMap;Ljava/util/concurrent/Executor;)` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `levelHeightAccessor` | `Lnet/minecraft/world/level/LevelHeightAccessor;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |
| reads | `oldTicketLevel` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |

## Declared members (20 fields, 35 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final UNLOADED_LEVEL_CHUNK : Lnet/minecraft/server/level/ChunkResult;
private static final UNLOADED_LEVEL_CHUNK_FUTURE : Ljava/util/concurrent/CompletableFuture;
private final levelHeightAccessor : Lnet/minecraft/world/level/LevelHeightAccessor;
private fullChunkFuture : Ljava/util/concurrent/CompletableFuture;
private tickingChunkFuture : Ljava/util/concurrent/CompletableFuture;
private entityTickingChunkFuture : Ljava/util/concurrent/CompletableFuture;
private oldTicketLevel : I
private ticketLevel : I
private queueLevel : I
private hasChangedSections : Z
private final changedBlocksPerSection : [Lit/unimi/dsi/fastutil/shorts/ShortSet;
private final blockChangedLightSectionFilter : Ljava/util/BitSet;
private final skyChangedLightSectionFilter : Ljava/util/BitSet;
private final lightEngine : Lnet/minecraft/world/level/lighting/LevelLightEngine;
private final onLevelChange : Lnet/minecraft/server/level/ChunkHolder$LevelChangeListener;
private final playerProvider : Lnet/minecraft/server/level/ChunkHolder$PlayerProvider;
private wasAccessibleSinceLastSave : Z
private pendingFullStateConfirmation : Ljava/util/concurrent/CompletableFuture;
private sendSync : Ljava/util/concurrent/CompletableFuture;
private saveSync : Ljava/util/concurrent/CompletableFuture;
public <init>(Lnet/minecraft/world/level/ChunkPos;ILnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/lighting/LevelLightEngine;Lnet/minecraft/server/level/ChunkHolder$LevelChangeListener;Lnet/minecraft/server/level/ChunkHolder$PlayerProvider;)V
public getTickingChunkFuture()Ljava/util/concurrent/CompletableFuture;
public getEntityTickingChunkFuture()Ljava/util/concurrent/CompletableFuture;
public getFullChunkFuture()Ljava/util/concurrent/CompletableFuture;
public getTickingChunk()Lnet/minecraft/world/level/chunk/LevelChunk;
public getChunkToSend()Lnet/minecraft/world/level/chunk/LevelChunk;
public getSendSyncFuture()Ljava/util/concurrent/CompletableFuture;
public addSendDependency(Ljava/util/concurrent/CompletableFuture;)V
public getSaveSyncFuture()Ljava/util/concurrent/CompletableFuture;
public isReadyForSaving()Z
protected addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V
public blockChanged(Lnet/minecraft/core/BlockPos;)Z
public sectionLightChanged(Lnet/minecraft/world/level/LightLayer;I)Z
public hasChangesToBroadcast()Z
public broadcastChanges(Lnet/minecraft/world/level/chunk/LevelChunk;)V
private broadcastBlockEntityIfNeeded(Ljava/util/List;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
private broadcastBlockEntity(Ljava/util/List;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
private broadcast(Ljava/util/List;Lnet/minecraft/network/protocol/Packet;)V
public getTicketLevel()I
public getQueueLevel()I
private setQueueLevel(I)V
public setTicketLevel(I)V
private scheduleFullChunkPromotion(Lnet/minecraft/server/level/ChunkMap;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/Executor;Lnet/minecraft/server/level/FullChunkStatus;)V
private demoteFullChunk(Lnet/minecraft/server/level/ChunkMap;Lnet/minecraft/server/level/FullChunkStatus;)V
protected updateFutures(Lnet/minecraft/server/level/ChunkMap;Ljava/util/concurrent/Executor;)V
public wasAccessibleSinceLastSave()Z
public refreshAccessibility()V
private static synthetic lambda$scheduleFullChunkPromotion$1(Ljava/util/concurrent/CompletableFuture;Lnet/minecraft/server/level/ChunkResult;)V
private static synthetic lambda$scheduleFullChunkPromotion$2(Ljava/util/concurrent/CompletableFuture;Lnet/minecraft/world/level/chunk/LevelChunk;)V
private synthetic lambda$scheduleFullChunkPromotion$0(Lnet/minecraft/server/level/ChunkMap;Lnet/minecraft/server/level/FullChunkStatus;)V
private static synthetic lambda$broadcast$0(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/server/level/ServerPlayer;)V
private synthetic lambda$broadcastChanges$0(Ljava/util/List;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
private static synthetic lambda$addSaveDependency$0(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
private static synthetic lambda$addSendDependency$0(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;
static <clinit>()V
```
