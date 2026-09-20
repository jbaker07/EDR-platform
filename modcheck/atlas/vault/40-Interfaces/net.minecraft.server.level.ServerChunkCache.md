---
type: "interface"
fqcn: "net.minecraft.server.level.ServerChunkCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerChunkCache

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`class` public; extends `net/minecraft/world/level/chunk/ChunkSource`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `blockChanged` | `(Lnet/minecraft/core/BlockPos;)V` | exact | invokevirtual@28 in `BlockEntityMixin.fabric_syncChange` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `chunkMap` | `Lnet/minecraft/server/level/ChunkMap;` | exact | getfield@21 in `BlockEntityMixin.fabric_markChanged` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `chunkMap` | `Lnet/minecraft/server/level/ChunkMap;` | exact | getfield@18 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `chunkMap` | `Lnet/minecraft/server/level/ChunkMap;` | exact | getfield@26 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (17 fields, 65 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final distanceManager : Lnet/minecraft/server/level/DistanceManager;
private final level : Lnet/minecraft/server/level/ServerLevel;
private final mainThread : Ljava/lang/Thread;
private final lightEngine : Lnet/minecraft/server/level/ThreadedLevelLightEngine;
private final mainThreadProcessor : Lnet/minecraft/server/level/ServerChunkCache$MainThreadExecutor;
public final chunkMap : Lnet/minecraft/server/level/ChunkMap;
private final savedDataStorage : Lnet/minecraft/world/level/storage/SavedDataStorage;
private final ticketStorage : Lnet/minecraft/world/level/TicketStorage;
private spawnEnemies : Z
private static final CACHE_SIZE : I
private final lastChunkPos : [J
private final lastChunkStatus : [Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private final lastChunk : [Lnet/minecraft/world/level/chunk/ChunkAccess;
private final spawningChunks : Ljava/util/List;
private final chunkHoldersToBroadcast : Ljava/util/Set;
private lastSpawnState : Lnet/minecraft/world/level/NaturalSpawner$SpawnState;
public <init>(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;Ljava/util/concurrent/Executor;Lnet/minecraft/world/level/chunk/ChunkGenerator;IIZLnet/minecraft/world/level/entity/ChunkStatusUpdateListener;)V
public getLightEngine()Lnet/minecraft/server/level/ThreadedLevelLightEngine;
private getVisibleChunkIfPresent(J)Lnet/minecraft/server/level/ChunkHolder;
private storeInCache(JLnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/chunk/status/ChunkStatus;)V
public getChunk(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Lnet/minecraft/world/level/chunk/ChunkAccess;
public getChunkNow(II)Lnet/minecraft/world/level/chunk/LevelChunk;
private clearCache()V
public getChunkFuture(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Ljava/util/concurrent/CompletableFuture;
private getChunkFutureMainThread(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Ljava/util/concurrent/CompletableFuture;
private chunkAbsent(Lnet/minecraft/server/level/ChunkHolder;I)Z
public hasChunk(II)Z
public getChunkForLighting(II)Lnet/minecraft/world/level/chunk/LightChunk;
public getLevel()Lnet/minecraft/world/level/Level;
public pollTask()Z
 runDistanceManagerUpdates()Z
public isPositionTicking(J)Z
public save(Z)V
public close()V
public tick(Ljava/util/function/BooleanSupplier;Z)V
private tickChunks()V
private broadcastChangedChunks(Lnet/minecraft/util/profiling/ProfilerFiller;)V
private tickChunks(Lnet/minecraft/util/profiling/ProfilerFiller;)V
private tickSpawningChunk(Lnet/minecraft/world/level/chunk/LevelChunk;Ljava/util/List;Lnet/minecraft/world/level/NaturalSpawner$SpawnState;)V
private getFullChunk(JLjava/util/function/Consumer;)V
public gatherStats()Ljava/lang/String;
public getPendingTasksCount()I
public getGenerator()Lnet/minecraft/world/level/chunk/ChunkGenerator;
public getGeneratorState()Lnet/minecraft/world/level/chunk/ChunkGeneratorStructureState;
public randomState()Lnet/minecraft/world/level/levelgen/RandomState;
public getLoadedChunksCount()I
public blockChanged(Lnet/minecraft/core/BlockPos;)V
public onLightUpdate(Lnet/minecraft/world/level/LightLayer;Lnet/minecraft/core/SectionPos;)V
public hasActiveTickets()Z
public addTicket(Lnet/minecraft/server/level/Ticket;Lnet/minecraft/world/level/ChunkPos;)V
public addTicketAndLoadWithRadius(Lnet/minecraft/server/level/TicketType;Lnet/minecraft/world/level/ChunkPos;I)Ljava/util/concurrent/CompletableFuture;
public addTicketWithRadius(Lnet/minecraft/server/level/TicketType;Lnet/minecraft/world/level/ChunkPos;I)V
public removeTicketWithRadius(Lnet/minecraft/server/level/TicketType;Lnet/minecraft/world/level/ChunkPos;I)V
public updateChunkForced(Lnet/minecraft/world/level/ChunkPos;Z)Z
public getForceLoadedChunks()Lit/unimi/dsi/fastutil/longs/LongSet;
public move(Lnet/minecraft/server/level/ServerPlayer;)V
public hasEntityWithId(I)Z
public removeEntity(Lnet/minecraft/world/entity/Entity;)V
public addEntity(Lnet/minecraft/world/entity/Entity;)V
public sendToTrackingPlayersAndSelf(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/network/protocol/Packet;)V
public sendToTrackingPlayers(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/network/protocol/Packet;)V
public sendToTrackingPlayersFiltered(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/network/protocol/Packet;Ljava/util/function/Predicate;)V
public setViewDistance(I)V
public setSimulationDistance(I)V
public setSpawnSettings(Z)V
public getChunkDebugData(Lnet/minecraft/world/level/ChunkPos;)Ljava/lang/String;
public getDataStorage()Lnet/minecraft/world/level/storage/SavedDataStorage;
public getPoiManager()Lnet/minecraft/world/entity/ai/village/poi/PoiManager;
public chunkScanner()Lnet/minecraft/world/level/chunk/storage/ChunkScanAccess;
public getLastSpawnState()Lnet/minecraft/world/level/NaturalSpawner$SpawnState;
public deactivateTicketsOnClosing()V
public onChunkReadyToSend(Lnet/minecraft/server/level/ChunkHolder;)V
public synthetic getLightEngine()Lnet/minecraft/world/level/lighting/LevelLightEngine;
public synthetic getLevel()Lnet/minecraft/world/level/BlockGetter;
private static synthetic lambda$addTicketAndLoadWithRadius$0(I)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private synthetic lambda$onLightUpdate$0(Lnet/minecraft/core/SectionPos;Lnet/minecraft/world/level/LightLayer;)V
private synthetic lambda$tickChunks$0(ILnet/minecraft/world/level/chunk/LevelChunk;)V
private static synthetic lambda$getChunkFuture$1(Ljava/util/concurrent/CompletableFuture;)Ljava/util/concurrent/CompletionStage;
private synthetic lambda$getChunkFuture$0(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$getChunk$0(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Lnet/minecraft/world/level/chunk/ChunkAccess;
static <clinit>()V
```
