---
type: "interface"
fqcn: "net.minecraft.server.level.ChunkMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ChunkMap

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`class` public; extends `net/minecraft/world/level/chunk/storage/SimpleRegionStorage`; implements `net/minecraft/server/level/ChunkHolder$PlayerProvider`, `net/minecraft/server/level/GeneratingChunkMap`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getPlayers` | `(Lnet/minecraft/world/level/ChunkPos;Z)Ljava/util/List;` | exact | invokevirtual@23 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getUpdatingChunkIfPresent` | `(J)Lnet/minecraft/server/level/ChunkHolder;` | exact | invokevirtual@31 in `BlockEntityMixin.fabric_markChanged` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `lambda$scheduleUnload$0` | `(Lnet/minecraft/server/level/ChunkHolder;Ljava/util/concurrent/Complet` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/server/level/ServerLevel;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |

## Declared members (39 fields, 130 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final UNLOADED_CHUNK_LIST_RESULT : Lnet/minecraft/server/level/ChunkResult;
private static final UNLOADED_CHUNK_LIST_FUTURE : Ljava/util/concurrent/CompletableFuture;
private static final CHUNK_TYPE_REPLACEABLE : B
private static final CHUNK_TYPE_UNKNOWN : B
private static final CHUNK_TYPE_FULL : B
private static final LOGGER : Lorg/slf4j/Logger;
private static final CHUNK_SAVED_PER_TICK : I
private static final CHUNK_SAVED_EAGERLY_PER_TICK : I
private static final EAGER_CHUNK_SAVE_COOLDOWN_IN_MILLIS : I
private static final MAX_ACTIVE_CHUNK_WRITES : I
public static final MIN_VIEW_DISTANCE : I
public static final MAX_VIEW_DISTANCE : I
public static final FORCED_TICKET_LEVEL : I
private final updatingChunkMap : Lit/unimi/dsi/fastutil/longs/Long2ObjectLinkedOpenHashMap;
private visibleChunkMap : Lit/unimi/dsi/fastutil/longs/Long2ObjectLinkedOpenHashMap;
private final pendingUnloads : Lit/unimi/dsi/fastutil/longs/Long2ObjectLinkedOpenHashMap;
private final pendingGenerationTasks : Ljava/util/List;
private final level : Lnet/minecraft/server/level/ServerLevel;
private final lightEngine : Lnet/minecraft/server/level/ThreadedLevelLightEngine;
private final mainThreadExecutor : Lnet/minecraft/util/thread/BlockableEventLoop;
private final randomState : Lnet/minecraft/world/level/levelgen/RandomState;
private final chunkGeneratorState : Lnet/minecraft/world/level/chunk/ChunkGeneratorStructureState;
private final ticketStorage : Lnet/minecraft/world/level/TicketStorage;
private final poiManager : Lnet/minecraft/world/entity/ai/village/poi/PoiManager;
private final toDrop : Lit/unimi/dsi/fastutil/longs/LongSet;
private modified : Z
private final worldgenTaskDispatcher : Lnet/minecraft/server/level/ChunkTaskDispatcher;
private final lightTaskDispatcher : Lnet/minecraft/server/level/ChunkTaskDispatcher;
private final chunkStatusListener : Lnet/minecraft/world/level/entity/ChunkStatusUpdateListener;
private final distanceManager : Lnet/minecraft/server/level/ChunkMap$DistanceManager;
private final playerMap : Lnet/minecraft/server/level/PlayerMap;
private final entityMap : Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;
private final chunkTypeCache : Lit/unimi/dsi/fastutil/longs/Long2ByteMap;
private final nextChunkSaveTime : Lit/unimi/dsi/fastutil/longs/Long2LongMap;
private final chunksToEagerlySave : Lit/unimi/dsi/fastutil/longs/LongSet;
private final unloadQueue : Ljava/util/Queue;
private final activeChunkWrites : Ljava/util/concurrent/atomic/AtomicInteger;
private serverViewDistance : I
private final worldGenContext : Lnet/minecraft/world/level/chunk/status/WorldGenContext;
public <init>(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;Ljava/util/concurrent/Executor;Lnet/minecraft/util/thread/BlockableEventLoop;Lnet/minecraft/world/level/chunk/LightChunkGetter;Lnet/minecraft/world/level/chunk/ChunkGenerator;Lnet/minecraft/world/level/entity/ChunkStatusUpdateListener;Lnet/minecraft/world/level/TicketStorage;IZ)V
private setChunkUnsaved(Lnet/minecraft/world/level/ChunkPos;)V
protected generator()Lnet/minecraft/world/level/chunk/ChunkGenerator;
protected generatorState()Lnet/minecraft/world/level/chunk/ChunkGeneratorStructureState;
protected randomState()Lnet/minecraft/world/level/levelgen/RandomState;
public isChunkTracked(Lnet/minecraft/server/level/ServerPlayer;II)Z
private isChunkOnTrackedBorder(Lnet/minecraft/server/level/ServerPlayer;II)Z
protected getLightEngine()Lnet/minecraft/server/level/ThreadedLevelLightEngine;
public getUpdatingChunkIfPresent(J)Lnet/minecraft/server/level/ChunkHolder;
protected getVisibleChunkIfPresent(J)Lnet/minecraft/server/level/ChunkHolder;
public getLatestStatus(J)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
protected getChunkQueueLevel(J)Ljava/util/function/IntSupplier;
public getChunkDebugData(Lnet/minecraft/world/level/ChunkPos;)Ljava/lang/String;
 getChunkRangeFuture(Lnet/minecraft/server/level/ChunkHolder;ILjava/util/function/IntFunction;)Ljava/util/concurrent/CompletableFuture;
public debugFuturesAndCreateReportedException(Ljava/lang/IllegalStateException;Ljava/lang/String;)Lnet/minecraft/ReportedException;
public prepareEntityTickingChunk(Lnet/minecraft/server/level/ChunkHolder;)Ljava/util/concurrent/CompletableFuture;
private updateChunkScheduling(JILnet/minecraft/server/level/ChunkHolder;I)Lnet/minecraft/server/level/ChunkHolder;
private onLevelChange(Lnet/minecraft/world/level/ChunkPos;Ljava/util/function/IntSupplier;ILjava/util/function/IntConsumer;)V
public close()V
protected saveAllChunks(Z)V
protected tick(Ljava/util/function/BooleanSupplier;)V
public hasWork()Z
private processUnloads(Ljava/util/function/BooleanSupplier;)V
private saveChunksEagerly(Ljava/util/function/BooleanSupplier;)V
private scheduleUnload(JLnet/minecraft/server/level/ChunkHolder;)V
protected promoteChunkMap()Z
private scheduleChunkLoad(Lnet/minecraft/world/level/ChunkPos;)Ljava/util/concurrent/CompletableFuture;
private handleChunkLoadFailure(Ljava/lang/Throwable;Lnet/minecraft/world/level/ChunkPos;)Lnet/minecraft/world/level/chunk/ChunkAccess;
private createEmptyChunk(Lnet/minecraft/world/level/ChunkPos;)Lnet/minecraft/world/level/chunk/ChunkAccess;
private markPositionReplaceable(Lnet/minecraft/world/level/ChunkPos;)V
private markPosition(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/chunk/status/ChunkType;)B
public acquireGeneration(J)Lnet/minecraft/server/level/GenerationChunkHolder;
public releaseGeneration(Lnet/minecraft/server/level/GenerationChunkHolder;)V
public applyStep(Lnet/minecraft/server/level/GenerationChunkHolder;Lnet/minecraft/world/level/chunk/status/ChunkStep;Lnet/minecraft/util/StaticCache2D;)Ljava/util/concurrent/CompletableFuture;
public scheduleGenerationTask(Lnet/minecraft/world/level/chunk/status/ChunkStatus;Lnet/minecraft/world/level/ChunkPos;)Lnet/minecraft/server/level/ChunkGenerationTask;
private runGenerationTask(Lnet/minecraft/server/level/ChunkGenerationTask;)V
public runGenerationTasks()V
public prepareTickingChunk(Lnet/minecraft/server/level/ChunkHolder;)Ljava/util/concurrent/CompletableFuture;
private onChunkReadyToSend(Lnet/minecraft/server/level/ChunkHolder;Lnet/minecraft/world/level/chunk/LevelChunk;)V
public prepareAccessibleChunk(Lnet/minecraft/server/level/ChunkHolder;)Ljava/util/concurrent/CompletableFuture;
 allChunksWithAtLeastStatus(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Ljava/util/stream/Stream;
private saveChunkIfNeeded(Lnet/minecraft/server/level/ChunkHolder;J)Z
private save(Lnet/minecraft/world/level/chunk/ChunkAccess;)Z
private isExistingChunkFull(Lnet/minecraft/world/level/ChunkPos;)Z
protected setServerViewDistance(I)V
private getPlayerViewDistance(Lnet/minecraft/server/level/ServerPlayer;)I
private markChunkPendingToSend(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/ChunkPos;)V
private static markChunkPendingToSend(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/chunk/LevelChunk;)V
private static dropChunk(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/ChunkPos;)V
public getChunkToSend(J)Lnet/minecraft/world/level/chunk/LevelChunk;
public size()I
public getDistanceManager()Lnet/minecraft/server/level/DistanceManager;
 dumpChunks(Ljava/io/Writer;)V
private static printFuture(Ljava/util/concurrent/CompletableFuture;)Ljava/lang/String;
private readChunk(Lnet/minecraft/world/level/ChunkPos;)Ljava/util/concurrent/CompletableFuture;
private upgradeChunkTag(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/CompoundTag;
public static getChunkDataFixContextTag(Lnet/minecraft/resources/ResourceKey;Ljava/util/Optional;)Lnet/minecraft/nbt/CompoundTag;
public collectSpawningChunks(Ljava/util/List;)V
public forEachBlockTickingChunk(Ljava/util/function/Consumer;)V
public anyPlayerCloseEnoughForSpawning(Lnet/minecraft/world/level/ChunkPos;)Z
public anyPlayerCloseEnoughTo(Lnet/minecraft/core/BlockPos;I)Z
private anyPlayerCloseEnoughForSpawningInternal(Lnet/minecraft/world/level/ChunkPos;)Z
public getPlayersCloseForSpawning(Lnet/minecraft/world/level/ChunkPos;)Ljava/util/List;
private playerIsCloseEnoughForSpawning(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/ChunkPos;)Z
private playerIsCloseEnoughTo(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/phys/Vec3;I)Z
private static euclideanDistanceSquared(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/phys/Vec3;)D
private skipPlayer(Lnet/minecraft/server/level/ServerPlayer;)Z
private updatePlayerStatus(Lnet/minecraft/server/level/ServerPlayer;Z)V
private updatePlayerPos(Lnet/minecraft/server/level/ServerPlayer;)V
public move(Lnet/minecraft/server/level/ServerPlayer;)V
private updateChunkTracking(Lnet/minecraft/server/level/ServerPlayer;)V
private applyChunkTrackingView(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/level/ChunkTrackingView;)V
public getPlayers(Lnet/minecraft/world/level/ChunkPos;Z)Ljava/util/List;
public hasEntityWithId(I)Z
protected addEntity(Lnet/minecraft/world/entity/Entity;)V
protected removeEntity(Lnet/minecraft/world/entity/Entity;)V
protected tick()V
public sendToTrackingPlayers(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/network/protocol/Packet;)V
public sendToTrackingPlayersFiltered(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/network/protocol/Packet;Ljava/util/function/Predicate;)V
protected sendToTrackingPlayersAndSelf(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/network/protocol/Packet;)V
public isTrackedByAnyPlayer(Lnet/minecraft/world/entity/Entity;)Z
public forEachEntityTrackedBy(Lnet/minecraft/server/level/ServerPlayer;Ljava/util/function/Consumer;)V
public resendBiomesForChunks(Ljava/util/List;)V
protected getPoiManager()Lnet/minecraft/world/entity/ai/village/poi/PoiManager;
 onFullChunkStatusChange(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/server/level/FullChunkStatus;)V
public waitForLightBeforeSending(Lnet/minecraft/world/level/ChunkPos;I)V
public forEachReadyToSendChunk(Ljava/util/function/Consumer;)V
private synthetic lambda$waitForLightBeforeSending$0(Lnet/minecraft/world/level/ChunkPos;)V
private static synthetic lambda$resendBiomesForChunks$1(Lnet/minecraft/server/level/ServerPlayer;Ljava/util/List;)V
private static synthetic lambda$resendBiomesForChunks$0(Lnet/minecraft/server/level/ServerPlayer;)Ljava/util/List;
private static synthetic lambda$applyChunkTrackingView$1(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/ChunkPos;)V
private synthetic lambda$applyChunkTrackingView$0(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/ChunkPos;)V
private synthetic lambda$forEachBlockTickingChunk$0(Ljava/util/function/Consumer;J)V
private static synthetic lambda$getChunkDataFixContextTag$0(Lnet/minecraft/nbt/CompoundTag;Lnet/minecraft/resources/Identifier;)V
private synthetic lambda$readChunk$0(Ljava/util/Optional;)Ljava/util/Optional;
private static synthetic lambda$dumpChunks$3(Lnet/minecraft/world/level/chunk/LevelChunk;)Ljava/lang/Integer;
private static synthetic lambda$dumpChunks$2(Lnet/minecraft/world/level/chunk/LevelChunk;)Ljava/lang/Integer;
private static synthetic lambda$dumpChunks$1(Lnet/minecraft/world/level/chunk/LevelChunk;)Ljava/lang/Integer;
private static synthetic lambda$dumpChunks$0(Lnet/minecraft/world/level/chunk/ChunkAccess;)Ljava/util/Optional;
private synthetic lambda$save$0(Lnet/minecraft/world/level/ChunkPos;Ljava/lang/Void;Ljava/lang/Throwable;)Ljava/lang/Object;
private static synthetic lambda$allChunksWithAtLeastStatus$0(ILnet/minecraft/server/level/ChunkHolder;)Z
private static synthetic lambda$prepareAccessibleChunk$0(Lnet/minecraft/server/level/ChunkResult;)Lnet/minecraft/server/level/ChunkResult;
private static synthetic lambda$prepareAccessibleChunk$1(Ljava/util/List;)Lnet/minecraft/world/level/chunk/LevelChunk;
private synthetic lambda$prepareTickingChunk$1(Lnet/minecraft/server/level/ChunkHolder;Lnet/minecraft/server/level/ChunkResult;)Lnet/minecraft/server/level/ChunkResult;
private synthetic lambda$prepareTickingChunk$2(Lnet/minecraft/server/level/ChunkHolder;Ljava/util/List;)Lnet/minecraft/world/level/chunk/LevelChunk;
private synthetic lambda$prepareTickingChunk$3(Lnet/minecraft/server/level/ChunkHolder;Lnet/minecraft/world/level/chunk/LevelChunk;Ljava/lang/Object;)V
private static synthetic lambda$prepareTickingChunk$0(I)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private synthetic lambda$runGenerationTask$0(Lnet/minecraft/server/level/ChunkGenerationTask;)V
private synthetic lambda$runGenerationTask$1(Lnet/minecraft/server/level/ChunkGenerationTask;)V
private static synthetic lambda$applyStep$0(Lnet/minecraft/world/level/chunk/status/ChunkStep;)Ljava/lang/String;
private synthetic lambda$scheduleChunkLoad$4(Lnet/minecraft/world/level/ChunkPos;Ljava/lang/Throwable;)Lnet/minecraft/world/level/chunk/ChunkAccess;
private synthetic lambda$scheduleChunkLoad$3(Lnet/minecraft/world/level/ChunkPos;Ljava/util/Optional;)Lnet/minecraft/world/level/chunk/ChunkAccess;
private static synthetic lambda$scheduleChunkLoad$2(Ljava/util/Optional;Ljava/lang/Object;)Ljava/util/Optional;
private synthetic lambda$scheduleChunkLoad$0(Lnet/minecraft/world/level/ChunkPos;Ljava/util/Optional;)Ljava/util/Optional;
private synthetic lambda$scheduleChunkLoad$1(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/storage/SerializableChunkData;
private static synthetic lambda$scheduleUnload$1(Lnet/minecraft/server/level/ChunkHolder;Ljava/lang/Void;Ljava/lang/Throwable;)V
private synthetic lambda$scheduleUnload$0(Lnet/minecraft/server/level/ChunkHolder;Ljava/util/concurrent/CompletableFuture;J)V
private static synthetic lambda$saveAllChunks$3()Z
private static synthetic lambda$saveAllChunks$2(Lorg/apache/commons/lang3/mutable/MutableBoolean;Lnet/minecraft/world/level/chunk/ChunkAccess;)V
private static synthetic lambda$saveAllChunks$1(Lnet/minecraft/world/level/chunk/ChunkAccess;)Z
private synthetic lambda$saveAllChunks$0(Lnet/minecraft/server/level/ChunkHolder;)Lnet/minecraft/world/level/chunk/ChunkAccess;
private static synthetic lambda$prepareEntityTickingChunk$1(Lnet/minecraft/server/level/ChunkResult;)Lnet/minecraft/server/level/ChunkResult;
private static synthetic lambda$prepareEntityTickingChunk$2(Ljava/util/List;)Lnet/minecraft/world/level/chunk/LevelChunk;
private static synthetic lambda$prepareEntityTickingChunk$0(I)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private static synthetic lambda$debugFuturesAndCreateReportedException$0(Ljava/lang/StringBuilder;Lnet/minecraft/server/level/ChunkHolder;)V
private static synthetic lambda$debugFuturesAndCreateReportedException$1(Ljava/lang/StringBuilder;Lnet/minecraft/server/level/ChunkHolder;Lcom/mojang/datafixers/util/Pair;)V
private synthetic lambda$getChunkRangeFuture$1(Ljava/util/List;)Lnet/minecraft/server/level/ChunkResult;
private static synthetic lambda$getChunkRangeFuture$0(Lnet/minecraft/server/level/ChunkResult;)Lnet/minecraft/server/level/ChunkResult;
private synthetic lambda$getChunkQueueLevel$0(J)I
static <clinit>()V
```
