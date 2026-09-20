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
- mixin classes: 23 found by annotation, 23 declared in configs; extraction failures: 0

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

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`exitWorldAndClose` | `()V` | name_only | @Inject | INVOKE `Lorg/slf4j/Logger;info(Ljava/lang/String;)V` (exact) | client | 1000 (default) | `MinecraftMixin.onStopping` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`run` | `()V` | name_only | @Inject | FIELD `Lnet/minecraft/client/Minecraft;gameThread:Ljava/lang/Thread;` (exact) | client | 1000 (default) | `MinecraftMixin.onStart` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`tick` | `()V` | name_only | @Inject | HEAD | client | 1000 (default) | `MinecraftMixin.onStartTick` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`tick` | `()V` | name_only | @Inject | RETURN | client | 1000 (default) | `MinecraftMixin.onEndTick` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`updateLevelInEngines` | `(Lnet/minecraft/client/multiplayer/ClientLevel;Z)V` | exact | @Inject | TAIL | client | 1000 (default) | `MinecraftMixin.afterClientLevelChange` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]].`drop` | `(Lnet/minecraft/world/level/ChunkPos;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;drop(ILnet/minecraft/world/level/chunk/LevelChunk;)V` (exact) | client | 1000 (default) | `ClientChunkCacheMixin.onChunkUnload` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]].`replaceWithPacketData` | `(IILnet/minecraft/network/protocol/game/ClientboundLevelChunkPacketData;)Lnet/minecraft/world/level/chunk/LevelChunk;` | name_only | @Inject | TAIL | client | 1000 (default) | `ClientChunkCacheMixin.onChunkLoad` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]].`replaceWithPacketData` | `(IILnet/minecraft/network/protocol/game/ClientboundLevelChunkPacketData;)Lnet/minecraft/world/level/chunk/LevelChunk;` | name_only | @Inject | NEW `net/minecraft/world/level/chunk/LevelChunk` (exact) | client | 1000 (default) | `ClientChunkCacheMixin.onChunkUnload` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]].`updateViewRadius` | `(I)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;inRange(II)Z` (exact) | client | 1000 (default) | `ClientChunkCacheMixin.onUpdateLoadDistance` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]].`handleConfigurationFinished` | `(Lnet/minecraft/network/protocol/configuration/ClientboundFinishConfigurationPacket;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/network/Connection;setupInboundProtocol(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V` (exact) | client | 1000 (default) | `ClientConfigurationPacketListenerImplMixin.invokeTagsLoaded` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel|ClientLevel]].`tick` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `ClientLevelMixin.endLevelTick` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel|ClientLevel]].`tickEntities` | `()V` | name_only | @Inject | HEAD | client | 1000 (default) | `ClientLevelMixin.startLevelTick` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel_EntityCallbacks|ClientLevel$EntityCallbacks]].`onTrackingEnd` | `(Lnet/minecraft/world/entity/Entity;)V` | exact | @Inject | HEAD | client | 1000 (default) | `ClientLevelEntityCallbacksMixin.invokeUnloadEntity` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel_EntityCallbacks|ClientLevel$EntityCallbacks]].`onTrackingStart` | `(Lnet/minecraft/world/entity/Entity;)V` | exact | @Inject | TAIL | client | 1000 (default) | `ClientLevelEntityCallbacksMixin.invokeLoadEntity` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`clearLevel` | `()V` | name_only | @Inject | HEAD | client | 1000 (default) | `ClientPacketListenerMixin.onClearLevel` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleLogin` | `(Lnet/minecraft/network/protocol/game/ClientboundLoginPacket;)V` | name_only | @Inject | NEW `net/minecraft/client/multiplayer/ClientLevel` (exact) | client | 1000 (default) | `ClientPacketListenerMixin.onGameJoin` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleRespawn` | `(Lnet/minecraft/network/protocol/game/ClientboundRespawnPacket;)V` | name_only | @Inject | NEW `net/minecraft/client/multiplayer/ClientLevel` (exact) | client | 1000 (default) | `ClientPacketListenerMixin.onPlayerRespawn` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleUpdateTags` | `(Lnet/minecraft/network/protocol/common/ClientboundUpdateTagsPacket;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/item/CreativeModeTabs;searchTab()Lnet/minecraft/world/item/CreativeModeTab;` (exact) | client | 1000 (default) | `ClientPacketListenerMixin.invokeTagsLoaded` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`createLevels` | `()V` | name_only | @WrapOperation | INVOKE `Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` (exact) | both | 1000 (default) | `MinecraftServerMixin.onLoadWorld` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`reloadResources` | `(Ljava/util/Collection;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Inject | HEAD | both | 1000 (default) | `MinecraftServerMixin.startResourceReload` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`reloadResources` | `(Ljava/util/Collection;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Inject | TAIL | both | 1000 (default) | `MinecraftServerMixin.endResourceReload` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`runServer` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/MinecraftServer;initServer()Z` (exact) | both | 1000 (default) | `MinecraftServerMixin.beforeSetupServer` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`runServer` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/MinecraftServer;buildServerStatus()Lnet/minecraft/network/protocol/status/ServerStatus;` (exact) | both | 1000 (default) | `MinecraftServerMixin.afterSetupServer` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`saveAllChunks` | `(ZZZ)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `MinecraftServerMixin.startSave` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`saveAllChunks` | `(ZZZ)Z` | name_only | @Inject | TAIL | both | 1000 (default) | `MinecraftServerMixin.endSave` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`stopServer` | `()V` | name_only | @Inject | HEAD | both | 1000 (default) | `MinecraftServerMixin.beforeShutdownServer` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`stopServer` | `()V` | name_only | @Inject | TAIL | both | 1000 (default) | `MinecraftServerMixin.afterShutdownServer` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`stopServer` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/level/ServerLevel;close()V` (exact) | both | 1000 (default) | `MinecraftServerMixin.onUnloadWorldAtShutdown` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`tickServer` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/MinecraftServer;tickChildren(Ljava/util/function/BooleanSupplier;)V` (exact) | both | 1000 (default) | `MinecraftServerMixin.onStartTick` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`tickServer` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `MinecraftServerMixin.onEndTick` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]].`<init>` | `(Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/commands/Commands$CommandSelection;Ljava/util/List;Lnet/minecraft/server/permissions/PermissionSet;Ljava/util/List;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `ReloadableServerResourcesMixin.init` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]].`updateComponentsAndStaticRegistryTags` | `()V` | name_only | @Inject | TAIL | both | 1000 (default) | `ReloadableServerResourcesMixin.hookRefresh` |
| [[40-Interfaces/net.minecraft.server.dedicated.DedicatedServer|DedicatedServer]].`initServer` | `()Z` | name_only | @Redirect | INVOKE `Lnet/minecraft/server/notifications/NotificationManager;serverStarted()V` (exact) | server | 1000 (default) | `DedicatedServerMixin.deferServerStartedNotification` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]].`demoteFullChunk` | `(Lnet/minecraft/server/level/ChunkMap;Lnet/minecraft/server/level/FullChunkStatus;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ChunkHolderMixin.decreaseLevel` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]].`updateFutures` | `(Lnet/minecraft/server/level/ChunkMap;Ljava/util/concurrent/Executor;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V` (exact) | both | 1000 (default) | `ChunkHolderMixin.updateFutures$inaccessibleToFull` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]].`updateFutures` | `(Lnet/minecraft/server/level/ChunkMap;Ljava/util/concurrent/Executor;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V` (exact) | both | 1000 (default) | `ChunkHolderMixin.updateFutures$fullToBlockTicking` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]].`updateFutures` | `(Lnet/minecraft/server/level/ChunkMap;Ljava/util/concurrent/Executor;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/level/ChunkHolder;addSaveDependency(Ljava/util/concurrent/CompletableFuture;)V` (exact) | both | 1000 (default) | `ChunkHolderMixin.updateFutures$blockTickingToEntityTicking` |
| [[40-Interfaces/net.minecraft.server.level.ChunkMap|ChunkMap]].`lambda$scheduleUnload$0` | `(Lnet/minecraft/server/level/ChunkHolder;Ljava/util/concurrent/CompletableFuture;J)V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/level/ChunkMap;save(Lnet/minecraft/world/level/chunk/ChunkAccess;)Z` (exact) | both | 1000 (default) | `ChunkMapMixin.onChunkUnload` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]].`tick` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject | FIELD `Lnet/minecraft/server/level/ServerLevel;handlingTick:Z` (exact) | both | 1000 (default) | `ServerLevelMixin.startLevelTick` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]].`tick` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `ServerLevelMixin.endLevelTick` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel_EntityCallbacks|ServerLevel$EntityCallbacks]].`onTrackingEnd` | `(Lnet/minecraft/world/entity/Entity;)V` | exact | @Inject | HEAD | both | 1000 (default) | `ServerLevelEntityCallbacksMixin.invokeEntityUnloadEvent` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel_EntityCallbacks|ServerLevel$EntityCallbacks]].`onTrackingStart` | `(Lnet/minecraft/world/entity/Entity;)V` | exact | @Inject | TAIL | both | 1000 (default) | `ServerLevelEntityCallbacksMixin.invokeEntityLoadEvent` |
| [[40-Interfaces/net.minecraft.server.network.ServerHandshakePacketListenerImpl|ServerHandshakePacketListenerImpl]].`handleIntention` | `(Lnet/minecraft/network/protocol/handshake/ClientIntentionPacket;)V` | name_only | @Inject | HEAD | server | 1000 (default) | `ServerHandshakePacketListenerImplMixin.rejectConnectionsDuringStartup` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`placeNewPlayer` | `(Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/network/CommonListenerCookie;)V` | name_only | @Inject | NEW `net/minecraft/network/protocol/game/ClientboundUpdateRecipesPacket` (exact) | both | 1000 (default) | `PlayerListMixin.hookOnPlayerConnect` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`reloadResources` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/network/protocol/common/ClientboundUpdateTagsPacket;<init>(Ljava/util/Map;)V` (exact) | both | 1000 (default) | `PlayerListMixin.hookOnDataPacksReloaded` |
| [[40-Interfaces/net.minecraft.world.entity.EntityType|EntityType]].`create` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;)Lnet/minecraft/world/entity/Entity;` | exact | @Inject | RETURN | both | 1000 (default) | `EntityTypeMixin.setSpawnReason` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`collectEquipmentChanges` | `(Ljava/util/Map;)Ljava/util/Map;` | name_only | @Inject | INVOKE `Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` (exact) | both | 1000 (default) | `LivingEntityMixin.getEquipmentChanges` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;` | exact | @Redirect | INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;` (exact) | client | 1000 (default) | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`getBlockEntity` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/chunk/LevelChunk$EntityCreationType;)Lnet/minecraft/world/level/block/entity/BlockEntity;` | exact | @Redirect | INVOKE `Ljava/util/Map;remove(Ljava/lang/Object;)Ljava/lang/Object;` (exact) | server | 1000 (default) | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` (exact) | client | 1000 (default) | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`removeBlockEntity` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` (exact) | server | 1000 (default) | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` | `(Lnet/minecraft/world/level/block/entity/BlockEntity;)V` | name_only | @ModifyExpressionValue | INVOKE `Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` (exact) | client | 1000 (default) | `LevelChunkMixin.onLoadBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` | `(Lnet/minecraft/world/level/block/entity/BlockEntity;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` (exact) | client | 1000 (default) | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` | `(Lnet/minecraft/world/level/block/entity/BlockEntity;)V` | name_only | @ModifyExpressionValue | INVOKE `Ljava/util/Map;put(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;` (exact) | server | 1000 (default) | `LevelChunkMixin.onLoadBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`setBlockEntity` | `(Lnet/minecraft/world/level/block/entity/BlockEntity;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;setRemoved()V` (exact) | server | 1000 (default) | `LevelChunkMixin.onRemoveBlockEntity` |
| [[40-Interfaces/net.minecraft.world.level.chunk.status.ChunkStatusTasks|ChunkStatusTasks]].`lambda$full$0` | `(Lnet/minecraft/world/level/chunk/ChunkAccess;Lnet/minecraft/world/level/chunk/status/WorldGenContext;Lnet/minecraft/server/level/GenerationChunkHolder;)Lnet/minecraft/world/level/chunk/ChunkAccess;` | name_only | @Inject | TAIL | both | 1000 (default) | `ChunkStatusTasksMixin.onChunkLoad` |
| [[40-Interfaces/net.minecraft.world.level.entity.PersistentEntitySectionManager|PersistentEntitySectionManager]].`addEntity` | `(Lnet/minecraft/world/level/entity/EntityAccess;Z)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `PersistentEntitySectionManagerMixin.beforeAddEntity` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientBlockEntityEvents|ClientBlockEntityEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents|ClientChunkEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientEntityEvents|ClientEntityEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLevelEvents|ClientLevelEvents]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLifecycleEvents|ClientLifecycleEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents|ClientTickEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents|CommonLifecycleEvents]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.EntityLoadData|EntityLoadData]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerBlockEntityEvents|ServerBlockEntityEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerChunkEvents|ServerChunkEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents|ServerEntityEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLevelEvents|ServerLevelEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents|ServerLifecycleEvents]] (class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents|ServerTickEvents]] (class, 4 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
