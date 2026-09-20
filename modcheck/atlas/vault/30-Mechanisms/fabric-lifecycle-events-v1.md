---
type: "mechanism"
module: "fabric-lifecycle-events-v1"
version: "4.1.9+ffef5f675d"
sha256: "b9ba49109cf21f968d0b44c26b2c930af74d54e2f71d9fdf505e1359275842e2"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-lifecycle-events-v1

**Version** `4.1.9+ffef5f675d` -- **artifact sha256** `b9ba49109cf21f968d0b44c26b2c930af74d54e2f71d9fdf505e1359275842e2`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.event.lifecycle.LifecycleEventsImpl"], "client": ["net.fabricmc.fabric.impl.client.event.lifecycle.ClientLifecycleEventsImpl"]}`
- mixin configs: `["fabric-lifecycle-events-v1.mixins.json", {"config": "fabric-lifecycle-events-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-lifecycle-events-v1.classtweaker`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientBlockEntityEvents.BLOCK_ENTITY_LOAD|ClientBlockEntityEvents.BLOCK_ENTITY_LOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientBlockEntityEvents.BLOCK_ENTITY_UNLOAD|ClientBlockEntityEvents.BLOCK_ENTITY_UNLOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents.CHUNK_LOAD|ClientChunkEvents.CHUNK_LOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents.CHUNK_UNLOAD|ClientChunkEvents.CHUNK_UNLOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientEntityEvents.ENTITY_LOAD|ClientEntityEvents.ENTITY_LOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientEntityEvents.ENTITY_UNLOAD|ClientEntityEvents.ENTITY_UNLOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLevelEvents.AFTER_CLIENT_LEVEL_CHANGE|ClientLevelEvents.AFTER_CLIENT_LEVEL_CHANGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLifecycleEvents.CLIENT_STARTED|ClientLifecycleEvents.CLIENT_STARTED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLifecycleEvents.CLIENT_STOPPING|ClientLifecycleEvents.CLIENT_STOPPING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_CLIENT_TICK|ClientTickEvents.END_CLIENT_TICK]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_LEVEL_TICK|ClientTickEvents.END_LEVEL_TICK]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.START_CLIENT_TICK|ClientTickEvents.START_CLIENT_TICK]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.START_LEVEL_TICK|ClientTickEvents.START_LEVEL_TICK]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents.TAGS_LOADED|CommonLifecycleEvents.TAGS_LOADED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents.BLOCK_ENTITY_LOAD|ServerBlockEntityEvents.BLOCK_ENTITY_LOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents.BLOCK_ENTITY_UNLOAD|ServerBlockEntityEvents.BLOCK_ENTITY_UNLOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_GENERATE|ServerChunkEvents.CHUNK_GENERATE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_LOAD|ServerChunkEvents.CHUNK_LOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.CHUNK_UNLOAD|ServerChunkEvents.CHUNK_UNLOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents.FULL_CHUNK_STATUS_CHANGE|ServerChunkEvents.FULL_CHUNK_STATUS_CHANGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ALLOW_LOAD|ServerEntityEvents.ALLOW_LOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD|ServerEntityEvents.ENTITY_LOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_UNLOAD|ServerEntityEvents.ENTITY_UNLOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.EQUIPMENT_CHANGE|ServerEntityEvents.EQUIPMENT_CHANGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLevelEvents.LOAD|ServerLevelEvents.LOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLevelEvents.UNLOAD|ServerLevelEvents.UNLOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.AFTER_SAVE|ServerLifecycleEvents.AFTER_SAVE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.BEFORE_SAVE|ServerLifecycleEvents.BEFORE_SAVE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.END_DATA_PACK_RELOAD|ServerLifecycleEvents.END_DATA_PACK_RELOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTED|ServerLifecycleEvents.SERVER_STARTED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTING|ServerLifecycleEvents.SERVER_STARTING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STOPPED|ServerLifecycleEvents.SERVER_STOPPED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STOPPING|ServerLifecycleEvents.SERVER_STOPPING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.START_DATA_PACK_RELOAD|ServerLifecycleEvents.START_DATA_PACK_RELOAD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SYNC_DATA_PACK_CONTENTS|ServerLifecycleEvents.SYNC_DATA_PACK_CONTENTS]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK|ServerTickEvents.END_LEVEL_TICK]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_SERVER_TICK|ServerTickEvents.END_SERVER_TICK]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.START_LEVEL_TICK|ServerTickEvents.START_LEVEL_TICK]]
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.START_SERVER_TICK|ServerTickEvents.START_SERVER_TICK]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `exitWorldAndClose` | injects_into `@Inject at INVOKE Lorg/slf4j/Logger;info(Ljava/lang/String;)V` | client | `MinecraftMixin.onStopping` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `run` | injects_into `@Inject at FIELD Lnet/minecraft/client/Minecraft;gameThread:Ljava/lang/Thread;` | client | `MinecraftMixin.onStart` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `tick` | injects_into `@Inject at HEAD` | client | `MinecraftMixin.onStartTick` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `tick` | injects_into `@Inject at RETURN` | client | `MinecraftMixin.onEndTick` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `updateLevelInEngines(Lnet/minecraft/client/multiplayer/ClientLevel;Z)V` | injects_into `@Inject at TAIL` | client | `MinecraftMixin.afterClientLevelChange` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]] | `drop` | injects_into `@Inject at INVOKE Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;drop(ILnet/minecraft/world/level/chunk/LevelChunk;)V` | client | `ClientChunkCacheMixin.onChunkUnload` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]] | `replaceWithPacketData` | injects_into `@Inject at TAIL` | client | `ClientChunkCacheMixin.onChunkLoad` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]] | `replaceWithPacketData` | injects_into `@Inject at NEW net/minecraft/world/level/chunk/LevelChunk` | client | `ClientChunkCacheMixin.onChunkUnload` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]] | `updateViewRadius` | injects_into `@Inject at INVOKE Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;inRange(II)Z` | client | `ClientChunkCacheMixin.onUpdateLoadDistance` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]] | `handleConfigurationFinished` | injects_into `@Inject at INVOKE Lnet/minecraft/network/Connection;setupInboundProtocol(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V` | client | `ClientConfigurationPacketListenerImplMixin.invokeTagsLoaded` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel|ClientLevel]] | `tick` | injects_into `@Inject at RETURN` | client | `ClientLevelMixin.endLevelTick` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel|ClientLevel]] | `tickEntities` | injects_into `@Inject at HEAD` | client | `ClientLevelMixin.startLevelTick` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel_EntityCallbacks|ClientLevel$EntityCallbacks]] | `onTrackingEnd(Lnet/minecraft/world/entity/Entity;)V` | injects_into `@Inject at HEAD` | client | `ClientLevelEntityCallbacksMixin.invokeUnloadEntity` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel_EntityCallbacks|ClientLevel$EntityCallbacks]] | `onTrackingStart(Lnet/minecraft/world/entity/Entity;)V` | injects_into `@Inject at TAIL` | client | `ClientLevelEntityCallbacksMixin.invokeLoadEntity` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `clearLevel` | injects_into `@Inject at HEAD` | client | `ClientPacketListenerMixin.onClearLevel` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `handleLogin` | injects_into `@Inject at NEW net/minecraft/client/multiplayer/ClientLevel` | client | `ClientPacketListenerMixin.onGameJoin` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `handleRespawn` | injects_into `@Inject at NEW net/minecraft/client/multiplayer/ClientLevel` | client | `ClientPacketListenerMixin.onPlayerRespawn` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `handleUpdateTags` | injects_into `@Inject at INVOKE Lnet/minecraft/world/item/CreativeModeTabs;searchTab()Lnet/minecraft/world/item/CreativeModeTab;` | client | `ClientPacketListenerMixin.invokeTagsLoaded` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `reloadResources` | injects_into `@Inject at HEAD` | both | `MinecraftServerMixin.startResourceReload` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `reloadResources` | injects_into `@Inject at TAIL` | both | `MinecraftServerMixin.endResourceReload` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `runServer` | injects_into `@Inject at INVOKE Lnet/minecraft/server/MinecraftServer;initServer()Z` | both | `MinecraftServerMixin.beforeSetupServer` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `runServer` | injects_into `@Inject at INVOKE Lnet/minecraft/server/MinecraftServer;buildServerStatus()Lnet/minecraft/network/protocol/status/ServerStatus;` | both | `MinecraftServerMixin.afterSetupServer` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `saveAllChunks` | injects_into `@Inject at HEAD` | both | `MinecraftServerMixin.startSave` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `saveAllChunks` | injects_into `@Inject at TAIL` | both | `MinecraftServerMixin.endSave` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `stopServer` | injects_into `@Inject at HEAD` | both | `MinecraftServerMixin.beforeShutdownServer` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `stopServer` | injects_into `@Inject at TAIL` | both | `MinecraftServerMixin.afterShutdownServer` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `stopServer` | injects_into `@Inject at INVOKE Lnet/minecraft/server/level/ServerLevel;close()V` | both | `MinecraftServerMixin.onUnloadWorldAtShutdown` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `tickServer` | injects_into `@Inject at INVOKE Lnet/minecraft/server/MinecraftServer;tickChildren(Ljava/util/function/BooleanSupplier;)V` | both | `MinecraftServerMixin.onStartTick` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `tickServer` | injects_into `@Inject at TAIL` | both | `MinecraftServerMixin.onEndTick` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]] | `<init>` | injects_into `@Inject at TAIL` | both | `ReloadableServerResourcesMixin.init` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]] | `updateComponentsAndStaticRegistryTags` | injects_into `@Inject at TAIL` | both | `ReloadableServerResourcesMixin.hookRefresh` |
| [[40-Interfaces/net.minecraft.server.dedicated.DedicatedServer|DedicatedServer]] | `initServer` | wraps `@Redirect at INVOKE Lnet/minecraft/server/notifications/NotificationManager;serverStarted()V` | server | `DedicatedServerMixin.deferServerStartedNotification` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]] | `demoteFullChunk` | injects_into `@Inject at HEAD` | both | `ChunkHolderMixin.decreaseLevel` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]] | `updateFutures` | injects_into `@Inject at INVOKE Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V` | both | `ChunkHolderMixin.updateFutures$inaccessibleToFull` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]] | `updateFutures` | injects_into `@Inject at INVOKE Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V` | both | `ChunkHolderMixin.updateFutures$fullToBlockTicking` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]] | `updateFutures` | injects_into `@Inject at INVOKE Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V` | both | `ChunkHolderMixin.updateFutures$blockTickingToEntityTicking` |
| [[40-Interfaces/net.minecraft.server.level.ChunkMap|ChunkMap]] | `lambda$scheduleUnload$0` | injects_into `@Inject at INVOKE Lnet/minecraft/server/level/ChunkMap;save(Lnet/minecraft/world/level/chunk/ChunkAccess;)Z` | both | `ChunkMapMixin.onChunkUnload` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]] | `tick` | injects_into `@Inject at FIELD Lnet/minecraft/server/level/ServerLevel;handlingTick:Z` | both | `ServerLevelMixin.startLevelTick` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]] | `tick` | injects_into `@Inject at TAIL` | both | `ServerLevelMixin.endLevelTick` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel_EntityCallbacks|ServerLevel$EntityCallbacks]] | `onTrackingEnd(Lnet/minecraft/world/entity/Entity;)V` | injects_into `@Inject at HEAD` | both | `ServerLevelEntityCallbacksMixin.invokeEntityUnloadEvent` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel_EntityCallbacks|ServerLevel$EntityCallbacks]] | `onTrackingStart(Lnet/minecraft/world/entity/Entity;)V` | injects_into `@Inject at TAIL` | both | `ServerLevelEntityCallbacksMixin.invokeEntityLoadEvent` |
| [[40-Interfaces/net.minecraft.server.network.ServerHandshakePacketListenerImpl|ServerHandshakePacketListenerImpl]] | `handleIntention` | injects_into `@Inject at HEAD` | server | `ServerHandshakePacketListenerImplMixin.rejectConnectionsDuringStartup` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]] | `placeNewPlayer` | injects_into `@Inject at NEW net/minecraft/network/protocol/game/ClientboundUpdateRecipesPacket` | both | `PlayerListMixin.hookOnPlayerConnect` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]] | `reloadResources` | injects_into `@Inject at INVOKE Lnet/minecraft/network/protocol/common/ClientboundUpdateTagsPacket;<init>(Ljava/util/Map;)V` | both | `PlayerListMixin.hookOnDataPacksReloaded` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType|EntityType]] | `create(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;)Lnet/minecraft/world/entity/Entity;` | injects_into `@Inject at RETURN` | both | `EntityTypeMixin.setSpawnReason` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `collectEquipmentChanges` | injects_into `@Inject at INVOKE Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` | both | `LivingEntityMixin.getEquipmentChanges` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]] | `getBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;` | wraps `@Redirect at INVOKE Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;` | client | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]] | `getBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;` | wraps `@Redirect at INVOKE Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;` | server | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]] | `removeBlockEntity` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | client | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]] | `removeBlockEntity` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | server | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]] | `setBlockEntity` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | client | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]] | `setBlockEntity` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` | server | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatusTasks|ChunkStatusTasks]] | `lambda$full$0` | injects_into `@Inject at TAIL` | both | `ChunkStatusTasksMixin.onChunkLoad` |
| [[40-Interfaces/net.minecraft.world.level.entity.PersistentEntitySectionManager|PersistentEntitySectionManager]] | `addEntity` | injects_into `@Inject at HEAD` | both | `PersistentEntitySectionManagerMixin.beforeAddEntity` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientBlockEntityEvents|ClientBlockEntityEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents|ClientChunkEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientEntityEvents|ClientEntityEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLevelEvents|ClientLevelEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLifecycleEvents|ClientLifecycleEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents|ClientTickEvents]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents|CommonLifecycleEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.EntityLoadData|EntityLoadData]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents|ServerBlockEntityEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents|ServerChunkEvents]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents|ServerEntityEvents]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLevelEvents|ServerLevelEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents|ServerLifecycleEvents]] (class, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents|ServerTickEvents]] (class, 5 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
