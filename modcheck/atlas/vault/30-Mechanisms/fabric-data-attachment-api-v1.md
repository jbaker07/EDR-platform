---
type: "mechanism"
module: "fabric-data-attachment-api-v1"
version: "2.2.30+3434d6d95d"
sha256: "916e1b1046113d5289920fb4058d26ac184e2581f2403b3a482848a2e1dcf7cc"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-data-attachment-api-v1

**Version** `2.2.30+3434d6d95d` -- **artifact sha256** `916e1b1046113d5289920fb4058d26ac184e2581f2403b3a482848a2e1dcf7cc`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-entity-events-v1": "*", "fabric-object-builder-api-v1": "*", "fabric-networking-api-v1": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.attachment.AttachmentEntrypoint", "net.fabricmc.fabric.impl.attachment.sync.AttachmentSync"], "client": ["net.fabricmc.fabric.impl.attachment.client.AttachmentSyncClient"]}`
- mixin configs: `["fabric-data-attachment-api-v1.mixins.json", {"config": "fabric-data-attachment-api-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-data-attachment-api-v1.classtweaker`
- mixin classes: 20 found by annotation, 20 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lnet/minecraft/client/multiplayer/CommonListenerCookie;)V` | name_only | @Inject | TAIL | client | 1000 (default) | `ClientPacketListenerMixin.initGlobalAttachments` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleRespawn` | `(Lnet/minecraft/network/protocol/game/ClientboundRespawnPacket;)V` | name_only | @WrapOperation | FIELD `Lnet/minecraft/client/Minecraft;player:Lnet/minecraft/client/player/LocalPlayer;` (exact) | client | 1000 (default) | `ClientPacketListenerMixin.copyAttachmentsOnClientRespawn` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`<init>` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Ljava/net/Proxy;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/server/Services;Lnet/minecraft/server/level/progress/LevelLoadListener;ZLnet/minecraft/server/notifications/NotificationManager;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `MinecraftServerMixin.initGlobalAttachments` |
| [[40-Interfaces/net.minecraft.server.commands.data.BlockDataAccessor|BlockDataAccessor]].`setData` | `(Lnet/minecraft/nbt/CompoundTag;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/level/block/entity/BlockEntity;loadWithComponents(Lnet/minecraft/world/level/storage/ValueInput;)V` (exact) | both | 1000 (default) | `BlockDataAccessorMixin.setData` |
| [[40-Interfaces/net.minecraft.server.commands.data.EntityDataAccessor|EntityDataAccessor]].`setData` | `(Lnet/minecraft/nbt/CompoundTag;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/entity/Entity;load(Lnet/minecraft/world/level/storage/ValueInput;)V` (exact) | both | 1000 (default) | `EntityDataAccessorMixin.setData` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]].`broadcastBlockEntity` | `(Ljava/util/List;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `ChunkHolderMixin.broadcastBlockEntity` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]].`<init>` | `(Lnet/minecraft/server/MinecraftServer;Ljava/util/concurrent/Executor;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/world/level/storage/ServerLevelData;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/dimension/LevelStem;ZJLjava/util/List;Z)V` | name_only | @Inject | TAIL | both | 1000 (default) | `ServerLevelMixin.createAttachmentsPersistentState` |
| [[40-Interfaces/net.minecraft.server.network.PlayerChunkSender|PlayerChunkSender]].`sendNextChunks` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/server/network/PlayerChunkSender;sendChunk(Lnet/minecraft/server/network/ServerGamePacketListenerImpl;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/LevelChunk;)V` (exact) | both | 1000 (default) | `PlayerChunkSenderMixin.sendInitialAttachmentData` |
| [[40-Interfaces/net.minecraft.util.filefix.fixes.DimensionStorageFileFix|DimensionStorageFileFix]].`makeFixer` | `()V` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/util/filefix/operations/FileFixOperations;applyInFolders(Lnet/minecraft/util/filefix/access/FileRelation;Ljava/util/List;)Lnet/minecraft/util/filefix/operations/ApplyInFolders;` (exact) | both | 1000 (default) | `DimensionStorageFileFixMixin.addFabricAttachmentsMigration` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`load` | `(Lnet/minecraft/world/level/storage/ValueInput;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/entity/Entity;readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V` (exact) | both | 1000 (default) | `EntityMixin.readEntityAttachments` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`saveWithoutId` | `(Lnet/minecraft/world/level/storage/ValueOutput;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/entity/Entity;addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V` (exact) | both | 1000 (default) | `EntityMixin.writeEntityAttachments` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`setId` | `(I)V` | name_only | @Inject | HEAD | both | 1000 (default) | `EntityMixin.setId` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BannerBlockEntity|BannerBlockEntity]].`getUpdateTag` | `(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/level/block/entity/BannerBlockEntity;saveWithoutMetadata(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/nbt/CompoundTag;` (inherited_exact) | both | 1000 (default) | `BannerBlockEntityMixin.removeAttachments` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntity|BlockEntity]].`loadWithComponents` | `(Lnet/minecraft/world/level/storage/ValueInput;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `BlockEntityMixin.readBlockEntityAttachments` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntity|BlockEntity]].`saveWithoutMetadata` | `(Lnet/minecraft/world/level/storage/ValueOutput;)V` | exact | @Inject | TAIL | both | 1000 (default) | `BlockEntityMixin.writeBlockEntityAttachments` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]].`<init>` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/ProtoChunk;Lnet/minecraft/world/level/chunk/LevelChunk$PostLoadProcessor;)V` | exact | @Inject | TAIL | both | 1000 (default) | `LevelChunkMixin.transferProtoChunkAttachment` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]].`copyOf` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/ChunkAccess;)Lnet/minecraft/world/level/chunk/storage/SerializableChunkData;` | name_only | @Inject | RETURN | both | 1000 (default) | `SerializableChunkDataMixin.storeAttachmentNbtData` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]].`parse` | `(Lnet/minecraft/world/level/LevelHeightAccessor;Lnet/minecraft/world/level/chunk/PalettedContainerFactory;Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/world/level/chunk/storage/SerializableChunkData;` | name_only | @Inject | RETURN | both | 1000 (default) | `SerializableChunkDataMixin.storeAttachmentNbtData` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]].`read` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/ai/village/poi/PoiManager;Lnet/minecraft/world/level/chunk/storage/RegionStorageInfo;Lnet/minecraft/world/level/ChunkPos;)Lnet/minecraft/world/level/chunk/ProtoChunk;` | name_only | @Inject | RETURN | both | 1000 (default) | `SerializableChunkDataMixin.setAttachmentDataInChunk` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]].`write` | `()Lnet/minecraft/nbt/CompoundTag;` | name_only | @Inject | RETURN | both | 1000 (default) | `SerializableChunkDataMixin.writeChunkAttachments` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry|AttachmentRegistry]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.AttachmentSyncPredicate|AttachmentSyncPredicate]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.AttachmentTarget|AttachmentTarget]] (interface, 13 members)
- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.AttachmentType|AttachmentType]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.GlobalAttachments|GlobalAttachments]] (interface, 0 members)
- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.GlobalAttachmentsProvider|GlobalAttachmentsProvider]] (interface, 1 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
