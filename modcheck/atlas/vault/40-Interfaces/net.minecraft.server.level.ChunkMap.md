---
type: "interface"
fqcn: "net.minecraft.server.level.ChunkMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ChunkMap

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getUpdatingChunkIfPresent(J)Lnet/minecraft/server/level/ChunkHolder;` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `lambda$scheduleUnload$0` | `@Inject at INVOKE Lnet/minecraft/server/level/ChunkMap;save(Lnet/minecraft/world` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (169, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.level.ChunkMap extends net.minecraft.world.level.chunk.storage.SimpleRegionStorage implements net.minecraft.server.level.ChunkHolder$PlayerProvider,net.minecraft.server.level.GeneratingChunkMap {
    private static final net.minecraft.server.level.ChunkResult<java.util.List<net.minecraft.world.level.chunk.ChunkAccess>> UNLOADED_CHUNK_LIST_RESULT;
    private static final java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<java.util.List<net.minecraft.world.level.chunk.ChunkAccess>>> UNLOADED_CHUNK_LIST_FUTURE;
    private static final byte CHUNK_TYPE_REPLACEABLE;
    private static final byte CHUNK_TYPE_UNKNOWN;
    private static final byte CHUNK_TYPE_FULL;
    private static final org.slf4j.Logger LOGGER;
    private static final int CHUNK_SAVED_PER_TICK;
    private static final int CHUNK_SAVED_EAGERLY_PER_TICK;
    private static final int EAGER_CHUNK_SAVE_COOLDOWN_IN_MILLIS;
    private static final int MAX_ACTIVE_CHUNK_WRITES;
    public static final int MIN_VIEW_DISTANCE;
    public static final int MAX_VIEW_DISTANCE;
    public static final int FORCED_TICKET_LEVEL;
    private final it.unimi.dsi.fastutil.longs.Long2ObjectLinkedOpenHashMap<net.minecraft.server.level.ChunkHolder> updatingChunkMap;
    private volatile it.unimi.dsi.fastutil.longs.Long2ObjectLinkedOpenHashMap<net.minecraft.server.level.ChunkHolder> visibleChunkMap;
    private final it.unimi.dsi.fastutil.longs.Long2ObjectLinkedOpenHashMap<net.minecraft.server.level.ChunkHolder> pendingUnloads;
    private final java.util.List<net.minecraft.server.level.ChunkGenerationTask> pendingGenerationTasks;
    private final net.minecraft.server.level.ServerLevel level;
    private final net.minecraft.server.level.ThreadedLevelLightEngine lightEngine;
    private final net.minecraft.util.thread.BlockableEventLoop<java.lang.Runnable> mainThreadExecutor;
    private final net.minecraft.world.level.levelgen.RandomState randomState;
    private final net.minecraft.world.level.chunk.ChunkGeneratorStructureState chunkGeneratorState;
    private final net.minecraft.world.level.TicketStorage ticketStorage;
    private final net.minecraft.world.entity.ai.village.poi.PoiManager poiManager;
    private final it.unimi.dsi.fastutil.longs.LongSet toDrop;
    private boolean modified;
    private final net.minecraft.server.level.ChunkTaskDispatcher worldgenTaskDispatcher;
    private final net.minecraft.server.level.ChunkTaskDispatcher lightTaskDispatcher;
    private final net.minecraft.world.level.entity.ChunkStatusUpdateListener chunkStatusListener;
    private final net.minecraft.server.level.ChunkMap$DistanceManager distanceManager;
    private final net.minecraft.server.level.PlayerMap playerMap;
    private final it.unimi.dsi.fastutil.ints.Int2ObjectMap<net.minecraft.server.level.ChunkMap$TrackedEntity> entityMap;
    private final it.unimi.dsi.fastutil.longs.Long2ByteMap chunkTypeCache;
    private final it.unimi.dsi.fastutil.longs.Long2LongMap nextChunkSaveTime;
    private final it.unimi.dsi.fastutil.longs.LongSet chunksToEagerlySave;
    private final java.util.Queue<java.lang.Runnable> unloadQueue;
    private final java.util.concurrent.atomic.AtomicInteger activeChunkWrites;
    private int serverViewDistance;
    private final net.minecraft.world.level.chunk.status.WorldGenContext worldGenContext;
    public net.minecraft.server.level.ChunkMap(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.datafixers.DataFixer, net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager, java.util.concurrent.Executor, net.minecraft.util.thread.BlockableEventLoop<java.lang.Runnable>, net.minecraft.world.level.chunk.LightChunkGetter, net.minecraft.world.level.chunk.ChunkGenerator, net.minecraft.world.level.entity.ChunkStatusUpdateListener, net.minecraft.world.level.TicketStorage, int, boolean);
    private void setChunkUnsaved(net.minecraft.world.level.ChunkPos);
    protected net.minecraft.world.level.chunk.ChunkGenerator generator();
    protected net.minecraft.world.level.chunk.ChunkGeneratorStructureState generatorState();
    protected net.minecraft.world.level.levelgen.RandomState randomState();
    public boolean isChunkTracked(net.minecraft.server.level.ServerPlayer, int, int);
    private boolean isChunkOnTrackedBorder(net.minecraft.server.level.ServerPlayer, int, int);
    protected net.minecraft.server.level.ThreadedLevelLightEngine getLightEngine();
    public net.minecraft.server.level.ChunkHolder getUpdatingChunkIfPresent(long);
    protected net.minecraft.server.level.ChunkHolder getVisibleChunkIfPresent(long);
    public net.minecraft.world.level.chunk.status.ChunkStatus getLatestStatus(long);
    protected java.util.function.IntSupplier getChunkQueueLevel(long);
    public java.lang.String getChunkDebugData(net.minecraft.world.level.ChunkPos);
    java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<java.util.List<net.minecraft.world.level.chunk.ChunkAccess>>> getChunkRangeFuture(net.minecraft.server.level.ChunkHolder, int, java.util.function.IntFunction<net.minecraft.world.level.chunk.status.ChunkStatus>);
    public net.minecraft.ReportedException debugFuturesAndCreateReportedException(java.lang.IllegalStateException, java.lang.String);
    public java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> prepareEntityTickingChunk(net.minecraft.server.level.ChunkHolder);
    private net.minecraft.server.level.ChunkHolder updateChunkScheduling(long, int, net.minecraft.server.level.ChunkHolder, int);
    private void onLevelChange(net.minecraft.world.level.ChunkPos, java.util.function.IntSupplier, int, java.util.function.IntConsumer);
    public void close() throws java.io.IOException;
    protected void saveAllChunks(boolean);
    protected void tick(java.util.function.BooleanSupplier);
    public boolean hasWork();
    private void processUnloads(java.util.function.BooleanSupplier);
    private void saveChunksEagerly(java.util.function.BooleanSupplier);
    private void scheduleUnload(long, net.minecraft.server.level.ChunkHolder);
    protected boolean promoteChunkMap();
    private java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> scheduleChunkLoad(net.minecraft.world.level.ChunkPos);
    private net.minecraft.world.level.chunk.ChunkAccess handleChunkLoadFailure(java.lang.Throwable, net.minecraft.world.level.ChunkPos);
    private net.minecraft.world.level.chunk.ChunkAccess createEmptyChunk(net.minecraft.world.level.ChunkPos);
    private void markPositionReplaceable(net.minecraft.world.level.ChunkPos);
    private byte markPosition(net.minecraft.world.level.ChunkPos, net.minecraft.world.level.chunk.status.ChunkType);
    public net.minecraft.server.level.GenerationChunkHolder acquireGeneration(long);
    public void releaseGeneration(net.minecraft.server.level.GenerationChunkHolder);
    public java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> applyStep(net.minecraft.server.level.GenerationChunkHolder, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>);
    public net.minecraft.server.level.ChunkGenerationTask scheduleGenerationTask(net.minecraft.world.level.chunk.status.ChunkStatus, net.minecraft.world.level.ChunkPos);
    private void runGenerationTask(net.minecraft.server.level.ChunkGenerationTask);
    public void runGenerationTasks();
    public java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> prepareTickingChunk(net.minecraft.server.level.ChunkHolder);
    private void onChunkReadyToSend(net.minecraft.server.level.ChunkHolder, net.minecraft.world.level.chunk.LevelChunk);
    public java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> prepareAccessibleChunk(net.minecraft.server.level.ChunkHolder);
    java.util.stream.Stream<net.minecraft.server.level.ChunkHolder> allChunksWithAtLeastStatus(net.minecraft.world.level.chunk.status.ChunkStatus);
    private boolean saveChunkIfNeeded(net.minecraft.server.level.ChunkHolder, long);
    private boolean save(net.minecraft.world.level.chunk.ChunkAccess);
    private boolean isExistingChunkFull(net.minecraft.world.level.ChunkPos);
    protected void setServerViewDistance(int);
    private int getPlayerViewDistance(net.minecraft.server.level.ServerPlayer);
    private void markChunkPendingToSend(net.minecraft.server.level.ServerPlayer, net.minecraft.world.level.ChunkPos);
    private static void markChunkPendingToSend(net.minecraft.server.level.ServerPlayer, net.minecraft.world.level.chunk.LevelChunk);
    private static void dropChunk(net.minecraft.server.level.ServerPlayer, net.minecraft.world.level.ChunkPos);
    public net.minecraft.world.level.chunk.LevelChunk getChunkToSend(long);
    public int size();
    public net.minecraft.server.level.DistanceManager getDistanceManager();
    void dumpChunks(java.io.Writer) throws java.io.IOException;
    private static java.lang.String printFuture(java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>>);
    private java.util.concurrent.CompletableFuture<java.util.Optional<net.minecraft.nbt.CompoundTag>> readChunk(net.minecraft.world.level.ChunkPos);
    private net.minecraft.nbt.CompoundTag upgradeChunkTag(net.minecraft.nbt.CompoundTag);
    public static net.minecraft.nbt.CompoundTag getChunkDataFixContextTag(net.minecraft.resources.ResourceKey<net.minecraft.world.level.Level>, java.util.Optional<net.minecraft.resources.Identifier>);
    public void collectSpawningChunks(java.util.List<net.minecraft.world.level.chunk.LevelChunk>);
    public void forEachBlockTickingChunk(java.util.function.Consumer<net.minecraft.world.level.chunk.LevelChunk>);
    public boolean anyPlayerCloseEnoughForSpawning(net.minecraft.world.level.ChunkPos);
    public boolean anyPlayerCloseEnoughTo(net.minecraft.core.BlockPos, int);
    private boolean anyPlayerCloseEnoughForSpawningInternal(net.minecraft.world.level.ChunkPos);
    public java.util.List<net.minecraft.server.level.ServerPlayer> getPlayersCloseForSpawning(net.minecraft.world.level.ChunkPos);
    private boolean playerIsCloseEnoughForSpawning(net.minecraft.server.level.ServerPlayer, net.minecraft.world.level.ChunkPos);
    private boolean playerIsCloseEnoughTo(net.minecraft.server.level.ServerPlayer, net.minecraft.world.phys.Vec3, int);
    private static double euclideanDistanceSquared(net.minecraft.world.level.ChunkPos, net.minecraft.world.phys.Vec3);
    private boolean skipPlayer(net.minecraft.server.level.ServerPlayer);
    private void updatePlayerStatus(net.minecraft.server.level.ServerPlayer, boolean);
    private void updatePlayerPos(net.minecraft.server.level.ServerPlayer);
    public void move(net.minecraft.server.level.ServerPlayer);
    private void updateChunkTracking(net.minecraft.server.level.ServerPlayer);
    private void applyChunkTrackingView(net.minecraft.server.level.ServerPlayer, net.minecraft.server.level.ChunkTrackingView);
    public java.util.List<net.minecraft.server.level.ServerPlayer> getPlayers(net.minecraft.world.level.ChunkPos, boolean);
    public boolean hasEntityWithId(int);
    protected void addEntity(net.minecraft.world.entity.Entity);
    protected void removeEntity(net.minecraft.world.entity.Entity);
    protected void tick();
    public void sendToTrackingPlayers(net.minecraft.world.entity.Entity, net.minecraft.network.protocol.Packet<? super net.minecraft.network.protocol.game.ClientGamePacketListener>);
    public void sendToTrackingPlayersFiltered(net.minecraft.world.entity.Entity, net.minecraft.network.protocol.Packet<? super net.minecraft.network.protocol.game.ClientGamePacketListener>, java.util.function.Predicate<net.minecraft.server.level.ServerPlayer>);
    protected void sendToTrackingPlayersAndSelf(net.minecraft.world.entity.Entity, net.minecraft.network.protocol.Packet<? super net.minecraft.network.protocol.game.ClientGamePacketListener>);
    public boolean isTrackedByAnyPlayer(net.minecraft.world.entity.Entity);
    public void forEachEntityTrackedBy(net.minecraft.server.level.ServerPlayer, java.util.function.Consumer<net.minecraft.world.entity.Entity>);
    public void resendBiomesForChunks(java.util.List<net.minecraft.world.level.chunk.ChunkAccess>);
    protected net.minecraft.world.entity.ai.village.poi.PoiManager getPoiManager();
    void onFullChunkStatusChange(net.minecraft.world.level.ChunkPos, net.minecraft.server.level.FullChunkStatus);
    public void waitForLightBeforeSending(net.minecraft.world.level.ChunkPos, int);
    public void forEachReadyToSendChunk(java.util.function.Consumer<net.minecraft.world.level.chunk.LevelChunk>);
    private void lambda$waitForLightBeforeSending$0(net.minecraft.world.level.ChunkPos);
    private static void lambda$resendBiomesForChunks$1(net.minecraft.server.level.ServerPlayer, java.util.List);
    private static java.util.List lambda$resendBiomesForChunks$0(net.minecraft.server.level.ServerPlayer);
    private static void lambda$applyChunkTrackingView$1(net.minecraft.server.level.ServerPlayer, net.minecraft.world.level.ChunkPos);
    private void lambda$applyChunkTrackingView$0(net.minecraft.server.level.ServerPlayer, net.minecraft.world.level.ChunkPos);
    private void lambda$forEachBlockTickingChunk$0(java.util.function.Consumer, long);
    private static void lambda$getChunkDataFixContextTag$0(net.minecraft.nbt.CompoundTag, net.minecraft.resources.Identifier);
    private java.util.Optional lambda$readChunk$0(java.util.Optional);
    private static java.lang.Integer lambda$dumpChunks$3(net.minecraft.world.level.chunk.LevelChunk);
    private static java.lang.Integer lambda$dumpChunks$2(net.minecraft.world.level.chunk.LevelChunk);
    private static java.lang.Integer lambda$dumpChunks$1(net.minecraft.world.level.chunk.LevelChunk);
    private static java.util.Optional lambda$dumpChunks$0(net.minecraft.world.level.chunk.ChunkAccess);
    private java.lang.Object lambda$save$0(net.minecraft.world.level.ChunkPos, java.lang.Void, java.lang.Throwable);
    private static boolean lambda$allChunksWithAtLeastStatus$0(int, net.minecraft.server.level.ChunkHolder);
    private static net.minecraft.server.level.ChunkResult lambda$prepareAccessibleChunk$0(net.minecraft.server.level.ChunkResult);
    private static net.minecraft.world.level.chunk.LevelChunk lambda$prepareAccessibleChunk$1(java.util.List);
    private net.minecraft.server.level.ChunkResult lambda$prepareTickingChunk$1(net.minecraft.server.level.ChunkHolder, net.minecraft.server.level.ChunkResult);
    private net.minecraft.world.level.chunk.LevelChunk lambda$prepareTickingChunk$2(net.minecraft.server.level.ChunkHolder, java.util.List);
    private void lambda$prepareTickingChunk$3(net.minecraft.server.level.ChunkHolder, net.minecraft.world.level.chunk.LevelChunk, java.lang.Object);
    private static net.minecraft.world.level.chunk.status.ChunkStatus lambda$prepareTickingChunk$0(int);
    private void lambda$runGenerationTask$0(net.minecraft.server.level.ChunkGenerationTask);
    private void lambda$runGenerationTask$1(net.minecraft.server.level.ChunkGenerationTask);
    private static java.lang.String lambda$applyStep$0(net.minecraft.world.level.chunk.status.ChunkStep) throws java.lang.Exception;
    private net.minecraft.world.level.chunk.ChunkAccess lambda$scheduleChunkLoad$4(net.minecraft.world.level.ChunkPos, java.lang.Throwable);
    private net.minecraft.world.level.chunk.ChunkAccess lambda$scheduleChunkLoad$3(net.minecraft.world.level.ChunkPos, java.util.Optional);
    private static java.util.Optional lambda$scheduleChunkLoad$2(java.util.Optional, java.lang.Object);
    private java.util.Optional lambda$scheduleChunkLoad$0(net.minecraft.world.level.ChunkPos, java.util.Optional);
    private net.minecraft.world.level.chunk.storage.SerializableChunkData lambda$scheduleChunkLoad$1(net.minecraft.world.level.ChunkPos, net.minecraft.nbt.CompoundTag);
    private static void lambda$scheduleUnload$1(net.minecraft.server.level.ChunkHolder, java.lang.Void, java.lang.Throwable);
    private void lambda$scheduleUnload$0(net.minecraft.server.level.ChunkHolder, java.util.concurrent.CompletableFuture, long);
    private static boolean lambda$saveAllChunks$3();
    private static void lambda$saveAllChunks$2(org.apache.commons.lang3.mutable.MutableBoolean, net.minecraft.world.level.chunk.ChunkAccess);
    private static boolean lambda$saveAllChunks$1(net.minecraft.world.level.chunk.ChunkAccess);
    private net.minecraft.world.level.chunk.ChunkAccess lambda$saveAllChunks$0(net.minecraft.server.level.ChunkHolder);
    private static net.minecraft.server.level.ChunkResult lambda$prepareEntityTickingChunk$1(net.minecraft.server.level.ChunkResult);
    private static net.minecraft.world.level.chunk.LevelChunk lambda$prepareEntityTickingChunk$2(java.util.List);
    private static net.minecraft.world.level.chunk.status.ChunkStatus lambda$prepareEntityTickingChunk$0(int);
    private static void lambda$debugFuturesAndCreateReportedException$0(java.lang.StringBuilder, net.minecraft.server.level.ChunkHolder);
    private static void lambda$debugFuturesAndCreateReportedException$1(java.lang.StringBuilder, net.minecraft.server.level.ChunkHolder, com.mojang.datafixers.util.Pair);
    private net.minecraft.server.level.ChunkResult lambda$getChunkRangeFuture$1(java.util.List);
    private static net.minecraft.server.level.ChunkResult lambda$getChunkRangeFuture$0(net.minecraft.server.level.ChunkResult);
    private int lambda$getChunkQueueLevel$0(long);
    static {};
}
```
