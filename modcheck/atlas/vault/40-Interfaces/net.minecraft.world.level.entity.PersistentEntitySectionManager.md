---
type: "interface"
fqcn: "net.minecraft.world.level.entity.PersistentEntitySectionManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.entity.PersistentEntitySectionManager

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `addEntity` | `(Lnet/minecraft/world/level/entity/EntityAccess;Z)Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (11 fields, 54 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final knownUuids : Ljava/util/Set;
private final callbacks : Lnet/minecraft/world/level/entity/LevelCallback;
private final permanentStorage : Lnet/minecraft/world/level/entity/EntityPersistentStorage;
private final visibleEntityStorage : Lnet/minecraft/world/level/entity/EntityLookup;
private final sectionStorage : Lnet/minecraft/world/level/entity/EntitySectionStorage;
private final entityGetter : Lnet/minecraft/world/level/entity/LevelEntityGetter;
private final chunkVisibility : Lit/unimi/dsi/fastutil/longs/Long2ObjectMap;
private final chunkLoadStatuses : Lit/unimi/dsi/fastutil/longs/Long2ObjectMap;
private final chunksToUnload : Lit/unimi/dsi/fastutil/longs/LongSet;
private final loadingInbox : Ljava/util/Queue;
public <init>(Ljava/lang/Class;Lnet/minecraft/world/level/entity/LevelCallback;Lnet/minecraft/world/level/entity/EntityPersistentStorage;)V
private removeSectionIfEmpty(JLnet/minecraft/world/level/entity/EntitySection;)V
private addEntityUuid(Lnet/minecraft/world/level/entity/EntityAccess;)Z
public addNewEntity(Lnet/minecraft/world/level/entity/EntityAccess;)Z
private addEntity(Lnet/minecraft/world/level/entity/EntityAccess;Z)Z
private static getEffectiveStatus(Lnet/minecraft/world/level/entity/EntityAccess;Lnet/minecraft/world/level/entity/Visibility;)Lnet/minecraft/world/level/entity/Visibility;
public isTicking(Lnet/minecraft/world/level/ChunkPos;)Z
public addLegacyChunkEntities(Ljava/util/stream/Stream;)V
public addWorldGenChunkEntities(Ljava/util/stream/Stream;)V
private startTicking(Lnet/minecraft/world/level/entity/EntityAccess;)V
private stopTicking(Lnet/minecraft/world/level/entity/EntityAccess;)V
private startTracking(Lnet/minecraft/world/level/entity/EntityAccess;)V
private stopTracking(Lnet/minecraft/world/level/entity/EntityAccess;)V
public updateChunkStatus(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/server/level/FullChunkStatus;)V
public updateChunkStatus(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/entity/Visibility;)V
private ensureChunkQueuedForLoad(J)V
private storeChunkSections(JLjava/util/function/Consumer;)Z
private requestChunkLoad(J)V
private processChunkUnload(J)Z
private unloadEntity(Lnet/minecraft/world/level/entity/EntityAccess;)V
private processUnloads()V
public processPendingLoads()V
public tick()V
private getAllChunksToSave()Lit/unimi/dsi/fastutil/longs/LongSet;
public autoSave()V
public saveAll()V
public close()V
public isLoaded(Ljava/util/UUID;)Z
public getEntityGetter()Lnet/minecraft/world/level/entity/LevelEntityGetter;
public canPositionTick(Lnet/minecraft/core/BlockPos;)Z
public canPositionTick(Lnet/minecraft/world/level/ChunkPos;)Z
public areEntitiesLoaded(J)Z
public dumpSections(Ljava/io/Writer;)V
public gatherStats()Ljava/lang/String;
public count()I
private synthetic lambda$dumpSections$0(Lnet/minecraft/util/CsvOutput;J)V
private synthetic lambda$dumpSections$1(Lnet/minecraft/util/CsvOutput;Lnet/minecraft/world/level/entity/PersistentEntitySectionManager$ChunkLoadStatus;J)V
private synthetic lambda$saveAll$0(J)Z
private static synthetic lambda$saveAll$1(Lnet/minecraft/world/level/entity/EntityAccess;)V
private synthetic lambda$autoSave$0(J)V
private static synthetic lambda$autoSave$1(Lnet/minecraft/world/level/entity/EntityAccess;)V
private synthetic lambda$processPendingLoads$0(Lnet/minecraft/world/level/entity/EntityAccess;)V
private synthetic lambda$processUnloads$0(J)Z
private synthetic lambda$processChunkUnload$0(Lnet/minecraft/world/level/entity/EntityAccess;)V
private static synthetic lambda$requestChunkLoad$0(Lnet/minecraft/world/level/ChunkPos;Ljava/lang/Throwable;)Ljava/lang/Void;
private static synthetic lambda$storeChunkSections$0(Lnet/minecraft/world/level/entity/EntitySection;)Ljava/util/stream/Stream;
private synthetic lambda$updateChunkStatus$0(Lnet/minecraft/world/level/entity/Visibility;Lnet/minecraft/world/level/entity/EntitySection;)V
private static synthetic lambda$updateChunkStatus$4(Lnet/minecraft/world/level/entity/EntityAccess;)Z
private static synthetic lambda$updateChunkStatus$3(Lnet/minecraft/world/level/entity/EntityAccess;)Z
private static synthetic lambda$updateChunkStatus$2(Lnet/minecraft/world/level/entity/EntityAccess;)Z
private static synthetic lambda$updateChunkStatus$1(Lnet/minecraft/world/level/entity/EntityAccess;)Z
private synthetic lambda$addWorldGenChunkEntities$0(Lnet/minecraft/world/level/entity/EntityAccess;)V
private synthetic lambda$addLegacyChunkEntities$0(Lnet/minecraft/world/level/entity/EntityAccess;)V
static <clinit>()V
```
