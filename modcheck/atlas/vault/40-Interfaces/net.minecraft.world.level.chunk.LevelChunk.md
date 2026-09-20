---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.LevelChunk"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.LevelChunk

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getAttached(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;)Ljav` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getBlockEntities()Ljava/util/Map;` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getBlockEntities()Ljava/util/Map;` | `` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getBlockEntities()Ljava/util/Map;` | `` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getBlockEntities()Ljava/util/Map;` | `` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `hasAttached(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;)Z` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `setAttached(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;Ljava` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `<init>(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/ProtoChunk;Lnet/minecraft/world/level/chunk/LevelChunk$PostLoadProcessor;)V` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `removeBlockEntity` | `@Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `removeBlockEntity` | `@Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved` | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `setBlockEntity` | `@Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `setBlockEntity` | `@Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved` | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| wraps | `getBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;` | `@Redirect at INVOKE Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| wraps | `getBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;` | `@Redirect at INVOKE Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;` | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (69, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.chunk.LevelChunk extends net.minecraft.world.level.chunk.ChunkAccess implements net.minecraft.util.debug.DebugValueSource {
    private static final org.slf4j.Logger LOGGER;
    private static final net.minecraft.world.level.block.entity.TickingBlockEntity NULL_TICKER;
    private final java.util.Map<net.minecraft.core.BlockPos, net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper> tickersInLevel;
    private boolean loaded;
    private final net.minecraft.world.level.Level level;
    private java.util.function.Supplier<net.minecraft.server.level.FullChunkStatus> fullStatus;
    private net.minecraft.world.level.chunk.LevelChunk$PostLoadProcessor postLoad;
    private final it.unimi.dsi.fastutil.ints.Int2ObjectMap<net.minecraft.world.level.gameevent.GameEventListenerRegistry> gameEventListenerRegistrySections;
    private final net.minecraft.world.ticks.LevelChunkTicks<net.minecraft.world.level.block.Block> blockTicks;
    private final net.minecraft.world.ticks.LevelChunkTicks<net.minecraft.world.level.material.Fluid> fluidTicks;
    private net.minecraft.world.level.chunk.LevelChunk$UnsavedListener unsavedListener;
    public net.minecraft.world.level.chunk.LevelChunk(net.minecraft.world.level.Level, net.minecraft.world.level.ChunkPos);
    public net.minecraft.world.level.chunk.LevelChunk(net.minecraft.world.level.Level, net.minecraft.world.level.ChunkPos, net.minecraft.world.level.chunk.UpgradeData, net.minecraft.world.ticks.LevelChunkTicks<net.minecraft.world.level.block.Block>, net.minecraft.world.ticks.LevelChunkTicks<net.minecraft.world.level.material.Fluid>, long, net.minecraft.world.level.chunk.LevelChunkSection[], net.minecraft.world.level.chunk.LevelChunk$PostLoadProcessor, net.minecraft.world.level.levelgen.blending.BlendingData);
    public net.minecraft.world.level.chunk.LevelChunk(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.chunk.ProtoChunk, net.minecraft.world.level.chunk.LevelChunk$PostLoadProcessor);
    public void setUnsavedListener(net.minecraft.world.level.chunk.LevelChunk$UnsavedListener);
    public void markUnsaved();
    public net.minecraft.world.ticks.TickContainerAccess<net.minecraft.world.level.block.Block> getBlockTicks();
    public net.minecraft.world.ticks.TickContainerAccess<net.minecraft.world.level.material.Fluid> getFluidTicks();
    public net.minecraft.world.level.chunk.ChunkAccess$PackedTicks getTicksForSerialization(long);
    public net.minecraft.world.level.gameevent.GameEventListenerRegistry getListenerRegistry(int);
    public net.minecraft.world.level.block.state.BlockState getBlockState(net.minecraft.core.BlockPos);
    public net.minecraft.world.level.material.FluidState getFluidState(net.minecraft.core.BlockPos);
    public net.minecraft.world.level.material.FluidState getFluidState(int, int, int);
    public net.minecraft.world.level.block.state.BlockState setBlockState(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, int);
    public void addEntity(net.minecraft.world.entity.Entity);
    private net.minecraft.world.level.block.entity.BlockEntity createBlockEntity(net.minecraft.core.BlockPos);
    public net.minecraft.world.level.block.entity.BlockEntity getBlockEntity(net.minecraft.core.BlockPos);
    public net.minecraft.world.level.block.entity.BlockEntity getBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.chunk.LevelChunk$EntityCreationType);
    public void addAndRegisterBlockEntity(net.minecraft.world.level.block.entity.BlockEntity);
    private boolean isInLevel();
    private boolean isTicking(net.minecraft.core.BlockPos);
    public void setBlockEntity(net.minecraft.world.level.block.entity.BlockEntity);
    public net.minecraft.nbt.CompoundTag getBlockEntityNbtForSaving(net.minecraft.core.BlockPos, net.minecraft.core.HolderLookup$Provider);
    public void removeBlockEntity(net.minecraft.core.BlockPos);
    private <T extends net.minecraft.world.level.block.entity.BlockEntity> void removeGameEventListener(T, net.minecraft.server.level.ServerLevel);
    private void removeGameEventListenerRegistry(int);
    private void removeBlockEntityTicker(net.minecraft.core.BlockPos);
    public void runPostLoad();
    public boolean isEmpty();
    public void replaceWithPacketData(int, int, net.minecraft.network.protocol.game.ClientboundLevelChunkPacketData);
    public void replaceBiomes(net.minecraft.network.FriendlyByteBuf);
    public void setLoaded(boolean);
    public net.minecraft.world.level.Level getLevel();
    public java.util.Map<net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.BlockEntity> getBlockEntities();
    public void postProcessGeneration(net.minecraft.server.level.ServerLevel);
    private net.minecraft.world.level.block.entity.BlockEntity promotePendingBlockEntity(net.minecraft.core.BlockPos, net.minecraft.nbt.CompoundTag);
    public void unpackTicks(long);
    public void registerTickContainerInLevel(net.minecraft.server.level.ServerLevel);
    public void unregisterTickContainerFromLevel(net.minecraft.server.level.ServerLevel);
    public void registerDebugValues(net.minecraft.server.level.ServerLevel, net.minecraft.util.debug.DebugValueSource$Registration);
    public net.minecraft.world.level.chunk.status.ChunkStatus getPersistedStatus();
    public net.minecraft.server.level.FullChunkStatus getFullStatus();
    public void setFullStatus(java.util.function.Supplier<net.minecraft.server.level.FullChunkStatus>);
    public void clearAllBlockEntities();
    public void registerAllBlockEntitiesAfterLevelLoad();
    private <T extends net.minecraft.world.level.block.entity.BlockEntity> void addGameEventListener(T, net.minecraft.server.level.ServerLevel);
    private <T extends net.minecraft.world.level.block.entity.BlockEntity> void updateBlockEntityTicker(T);
    private <T extends net.minecraft.world.level.block.entity.BlockEntity> net.minecraft.world.level.block.entity.TickingBlockEntity createTicker(T, net.minecraft.world.level.block.entity.BlockEntityTicker<T>);
    private net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper lambda$updateBlockEntityTicker$0(net.minecraft.world.level.block.entity.BlockEntity, net.minecraft.world.level.block.entity.BlockEntityTicker, net.minecraft.core.BlockPos, net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper);
    private void lambda$registerAllBlockEntitiesAfterLevelLoad$0(net.minecraft.world.level.block.entity.BlockEntity);
    private static void lambda$clearAllBlockEntities$0(net.minecraft.world.level.chunk.LevelChunk$RebindableTickingBlockEntityWrapper);
    private java.util.List lambda$registerDebugValues$1(net.minecraft.server.level.ServerLevel);
    private java.util.List lambda$registerDebugValues$0();
    private void lambda$replaceWithPacketData$0(net.minecraft.util.ProblemReporter$ScopedCollector, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.BlockEntityType, net.minecraft.nbt.CompoundTag);
    private java.lang.String lambda$getFluidState$0(int, int, int) throws java.lang.Exception;
    private java.lang.String lambda$getBlockState$0(int, int, int) throws java.lang.Exception;
    private net.minecraft.world.level.gameevent.GameEventListenerRegistry lambda$getListenerRegistry$0(net.minecraft.server.level.ServerLevel, int, int);
    private static void lambda$new$0(net.minecraft.world.level.ChunkPos);
    static {};
}
```
