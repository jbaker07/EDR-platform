---
type: "interface"
fqcn: "net.minecraft.server.level.ChunkHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ChunkHolder

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getFullChunkFuture()Ljava/util/concurrent/CompletableFuture;` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `broadcastBlockEntity` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `demoteFullChunk` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateFutures` | `@Inject at INVOKE Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljav` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateFutures` | `@Inject at INVOKE Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljav` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateFutures` | `@Inject at INVOKE Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljav` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (55, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.level.ChunkHolder extends net.minecraft.server.level.GenerationChunkHolder {
    public static final net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk> UNLOADED_LEVEL_CHUNK;
    private static final java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> UNLOADED_LEVEL_CHUNK_FUTURE;
    private final net.minecraft.world.level.LevelHeightAccessor levelHeightAccessor;
    private volatile java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> fullChunkFuture;
    private volatile java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> tickingChunkFuture;
    private volatile java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> entityTickingChunkFuture;
    private int oldTicketLevel;
    private int ticketLevel;
    private int queueLevel;
    private boolean hasChangedSections;
    private final it.unimi.dsi.fastutil.shorts.ShortSet[] changedBlocksPerSection;
    private final java.util.BitSet blockChangedLightSectionFilter;
    private final java.util.BitSet skyChangedLightSectionFilter;
    private final net.minecraft.world.level.lighting.LevelLightEngine lightEngine;
    private final net.minecraft.server.level.ChunkHolder$LevelChangeListener onLevelChange;
    private final net.minecraft.server.level.ChunkHolder$PlayerProvider playerProvider;
    private boolean wasAccessibleSinceLastSave;
    private java.util.concurrent.CompletableFuture<?> pendingFullStateConfirmation;
    private java.util.concurrent.CompletableFuture<?> sendSync;
    private java.util.concurrent.CompletableFuture<?> saveSync;
    public net.minecraft.server.level.ChunkHolder(net.minecraft.world.level.ChunkPos, int, net.minecraft.world.level.LevelHeightAccessor, net.minecraft.world.level.lighting.LevelLightEngine, net.minecraft.server.level.ChunkHolder$LevelChangeListener, net.minecraft.server.level.ChunkHolder$PlayerProvider);
    public java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> getTickingChunkFuture();
    public java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> getEntityTickingChunkFuture();
    public java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>> getFullChunkFuture();
    public net.minecraft.world.level.chunk.LevelChunk getTickingChunk();
    public net.minecraft.world.level.chunk.LevelChunk getChunkToSend();
    public java.util.concurrent.CompletableFuture<?> getSendSyncFuture();
    public void addSendDependency(java.util.concurrent.CompletableFuture<?>);
    public java.util.concurrent.CompletableFuture<?> getSaveSyncFuture();
    public boolean isReadyForSaving();
    protected void addSaveDependency(java.util.concurrent.CompletableFuture<?>);
    public boolean blockChanged(net.minecraft.core.BlockPos);
    public boolean sectionLightChanged(net.minecraft.world.level.LightLayer, int);
    public boolean hasChangesToBroadcast();
    public void broadcastChanges(net.minecraft.world.level.chunk.LevelChunk);
    private void broadcastBlockEntityIfNeeded(java.util.List<net.minecraft.server.level.ServerPlayer>, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    private void broadcastBlockEntity(java.util.List<net.minecraft.server.level.ServerPlayer>, net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    private void broadcast(java.util.List<net.minecraft.server.level.ServerPlayer>, net.minecraft.network.protocol.Packet<?>);
    public int getTicketLevel();
    public int getQueueLevel();
    private void setQueueLevel(int);
    public void setTicketLevel(int);
    private void scheduleFullChunkPromotion(net.minecraft.server.level.ChunkMap, java.util.concurrent.CompletableFuture<net.minecraft.server.level.ChunkResult<net.minecraft.world.level.chunk.LevelChunk>>, java.util.concurrent.Executor, net.minecraft.server.level.FullChunkStatus);
    private void demoteFullChunk(net.minecraft.server.level.ChunkMap, net.minecraft.server.level.FullChunkStatus);
    protected void updateFutures(net.minecraft.server.level.ChunkMap, java.util.concurrent.Executor);
    public boolean wasAccessibleSinceLastSave();
    public void refreshAccessibility();
    private static void lambda$scheduleFullChunkPromotion$1(java.util.concurrent.CompletableFuture, net.minecraft.server.level.ChunkResult);
    private static void lambda$scheduleFullChunkPromotion$2(java.util.concurrent.CompletableFuture, net.minecraft.world.level.chunk.LevelChunk);
    private void lambda$scheduleFullChunkPromotion$0(net.minecraft.server.level.ChunkMap, net.minecraft.server.level.FullChunkStatus);
    private static void lambda$broadcast$0(net.minecraft.network.protocol.Packet, net.minecraft.server.level.ServerPlayer);
    private void lambda$broadcastChanges$0(java.util.List, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    private static java.lang.Object lambda$addSaveDependency$0(java.lang.Object, java.lang.Object);
    private static java.lang.Object lambda$addSendDependency$0(java.lang.Object, java.lang.Object);
    static {};
}
```
