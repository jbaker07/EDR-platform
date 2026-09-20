---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.LevelChunk"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.LevelChunk

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/chunk/ChunkAccess`; implements `net/minecraft/util/debug/DebugValueSource`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getAttached` | `(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;)Ljava/lang/Obj` | inherited_exact | invokevirtual@5 in `ImposterProtoChunkMixin.getAttached` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getBlockEntities` | `()Ljava/util/Map;` | exact | invokevirtual@1 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getBlockEntities` | `()Ljava/util/Map;` | exact | invokevirtual@80 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getBlockEntities` | `()Ljava/util/Map;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| calls | `getBlockEntities` | `()Ljava/util/Map;` | exact | invokevirtual@1 in `ClientLifecycleEventsImpl.lambda$onInitializeClient$2` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getBlockEntities` | `()Ljava/util/Map;` | exact | invokevirtual@35 in `LifecycleEventsImpl.lambda$onInitialize$3` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getBlockEntities` | `()Ljava/util/Map;` | exact | invokevirtual@1 in `LifecycleEventsImpl.lambda$onInitialize$2` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getBlockEntities` | `()Ljava/util/Map;` | exact | invokevirtual@103 in `ClientPacketListenerMixin.onPlayerRespawn` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getBlockEntities` | `()Ljava/util/Map;` | exact | invokevirtual@103 in `ClientPacketListenerMixin.onGameJoin` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getBlockEntities` | `()Ljava/util/Map;` | exact | invokevirtual@99 in `ClientPacketListenerMixin.onClearLevel` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getLevel` | `()Lnet/minecraft/world/level/Level;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |
| calls | `getLevel` | `()Lnet/minecraft/world/level/Level;` | exact | @Shadow declaration | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |
| calls | `hasAttached` | `(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;)Z` | inherited_exact | invokevirtual@5 in `ImposterProtoChunkMixin.hasAttached` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `setAttached` | `(Lnet/fabricmc/fabric/api/attachment/v1/AttachmentType;Ljava/lang/Obje` | inherited_exact | invokevirtual@6 in `ImposterProtoChunkMixin.setAttached` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/ch` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `removeBlockEntity` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `removeBlockEntity` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @Inject at ['INVOKE'] | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `setBlockEntity` | `(Lnet/minecraft/world/level/block/entity/BlockEntity;)V` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `setBlockEntity` | `(Lnet/minecraft/world/level/block/entity/BlockEntity;)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `setBlockEntity` | `(Lnet/minecraft/world/level/block/entity/BlockEntity;)V` | name_only | @ModifyExpressionValue at ['INVOKE'] | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `setBlockEntity` | `(Lnet/minecraft/world/level/block/entity/BlockEntity;)V` | name_only | @Inject at ['INVOKE'] | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/world/level/Level;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| wraps | `getBlockEntity` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelCh` | exact | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| wraps | `getBlockEntity` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelCh` | exact | @Redirect at ['INVOKE'] | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (11 fields, 58 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final NULL_TICKER : Lnet/minecraft/world/level/block/entity/TickingBlockEntity;
private final tickersInLevel : Ljava/util/Map;
private loaded : Z
private final level : Lnet/minecraft/world/level/Level;
private fullStatus : Ljava/util/function/Supplier;
private postLoad : Lnet/minecraft/world/level/chunk/LevelChunk$PostLoadProcessor;
private final gameEventListenerRegistrySections : Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;
private final blockTicks : Lnet/minecraft/world/ticks/LevelChunkTicks;
private final fluidTicks : Lnet/minecraft/world/ticks/LevelChunkTicks;
private unsavedListener : Lnet/minecraft/world/level/chunk/LevelChunk$UnsavedListener;
public <init>(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/ChunkPos;)V
public <init>(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/chunk/UpgradeData;Lnet/minecraft/world/ticks/LevelChunkTicks;Lnet/minecraft/world/ticks/LevelChunkTicks;J[Lnet/minecraft/world/level/chunk/LevelChunkSection;Lnet/minecraft/world/level/chunk/LevelChunk$PostLoadProcessor;Lnet/minecraft/world/level/levelgen/blending/BlendingData;)V
public <init>(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/ProtoChunk;Lnet/minecraft/world/level/chunk/LevelChunk$PostLoadProcessor;)V
public setUnsavedListener(Lnet/minecraft/world/level/chunk/LevelChunk$UnsavedListener;)V
public markUnsaved()V
public getBlockTicks()Lnet/minecraft/world/ticks/TickContainerAccess;
public getFluidTicks()Lnet/minecraft/world/ticks/TickContainerAccess;
public getTicksForSerialization(J)Lnet/minecraft/world/level/chunk/ChunkAccess$PackedTicks;
public getListenerRegistry(I)Lnet/minecraft/world/level/gameevent/GameEventListenerRegistry;
public getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public getFluidState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/FluidState;
public getFluidState(III)Lnet/minecraft/world/level/material/FluidState;
public setBlockState(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Lnet/minecraft/world/level/block/state/BlockState;
public addEntity(Lnet/minecraft/world/entity/Entity;)V
private createBlockEntity(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public getBlockEntity(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public getBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public addAndRegisterBlockEntity(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
private isInLevel()Z
private isTicking(Lnet/minecraft/core/BlockPos;)Z
public setBlockEntity(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
public getBlockEntityNbtForSaving(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;
public removeBlockEntity(Lnet/minecraft/core/BlockPos;)V
private removeGameEventListener(Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/server/level/ServerLevel;)V
private removeGameEventListenerRegistry(I)V
private removeBlockEntityTicker(Lnet/minecraft/core/BlockPos;)V
public runPostLoad()V
public isEmpty()Z
public replaceWithPacketData(IILnet/minecraft/network/protocol/game/ClientboundLevelChunkPacketData;)V
public replaceBiomes(Lnet/minecraft/network/FriendlyByteBuf;)V
public setLoaded(Z)V
public getLevel()Lnet/minecraft/world/level/Level;
public getBlockEntities()Ljava/util/Map;
public postProcessGeneration(Lnet/minecraft/server/level/ServerLevel;)V
private promotePendingBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public unpackTicks(J)V
public registerTickContainerInLevel(Lnet/minecraft/server/level/ServerLevel;)V
public unregisterTickContainerFromLevel(Lnet/minecraft/server/level/ServerLevel;)V
public registerDebugValues(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/util/debug/DebugValueSource$Registration;)V
public getPersistedStatus()Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public getFullStatus()Lnet/minecraft/server/level/FullChunkStatus;
public setFullStatus(Ljava/util/function/Supplier;)V
public clearAllBlockEntities()V
public registerAllBlockEntitiesAfterLevelLoad()V
private addGameEventListener(Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/server/level/ServerLevel;)V
private updateBlockEntityTicker(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
private createTicker(Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/level/block/entity/BlockEntityTicker;)Lnet/minecraft/world/level/block/entity/TickingBlockEntity;
private synthetic lambda$updateBlockEntityTicker$0(Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/level/block/entity/BlockEntityTicker;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$RebindableTickingBlockEntityWrapper;)Lnet/minecraft/world/level/chunk/LevelChunk$RebindableTickingBlockEntityWrapper;
private synthetic lambda$registerAllBlockEntitiesAfterLevelLoad$0(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
private static synthetic lambda$clearAllBlockEntities$0(Lnet/minecraft/world/level/chunk/LevelChunk$RebindableTickingBlockEntityWrapper;)V
private synthetic lambda$registerDebugValues$1(Lnet/minecraft/server/level/ServerLevel;)Ljava/util/List;
private synthetic lambda$registerDebugValues$0()Ljava/util/List;
private synthetic lambda$replaceWithPacketData$0(Lnet/minecraft/util/ProblemReporter$ScopedCollector;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/nbt/CompoundTag;)V
private synthetic lambda$getFluidState$0(III)Ljava/lang/String;
private synthetic lambda$getBlockState$0(III)Ljava/lang/String;
private synthetic lambda$getListenerRegistry$0(Lnet/minecraft/server/level/ServerLevel;II)Lnet/minecraft/world/level/gameevent/GameEventListenerRegistry;
private static synthetic lambda$new$0(Lnet/minecraft/world/level/ChunkPos;)V
static <clinit>()V
```
