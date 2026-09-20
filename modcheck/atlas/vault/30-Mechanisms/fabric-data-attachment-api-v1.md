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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `<init>` | injects_into `@Inject at TAIL` | client | `ClientPacketListenerMixin.initGlobalAttachments` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `<init>` | injects_into `@Inject at TAIL` | both | `MinecraftServerMixin.initGlobalAttachments` |
| [[40-Interfaces/net.minecraft.server.level.ChunkHolder|ChunkHolder]] | `broadcastBlockEntity` | injects_into `@Inject at TAIL` | both | `ChunkHolderMixin.broadcastBlockEntity` |
| [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]] | `<init>` | injects_into `@Inject at TAIL` | both | `ServerLevelMixin.createAttachmentsPersistentState` |
| [[40-Interfaces/net.minecraft.util.filefix.fixes.DimensionStorageFileFix|DimensionStorageFileFix]] | `makeFixer` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/util/filefix/operations/FileFixOperations;applyInFolders(Lnet/minecraft/util/filefix/access/FileRelation;Ljava/util/List;)Lnet/minecraft/util/filefix/operations/ApplyInFolders;` | both | `DimensionStorageFileFixMixin.addFabricAttachmentsMigration` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]] | `load` | injects_into `@Inject at INVOKE Lnet/minecraft/world/entity/Entity;readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V` | both | `EntityMixin.readEntityAttachments` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]] | `saveWithoutId` | injects_into `@Inject at INVOKE Lnet/minecraft/world/entity/Entity;addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V` | both | `EntityMixin.writeEntityAttachments` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]] | `setId` | injects_into `@Inject at HEAD` | both | `EntityMixin.setId` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntity|BlockEntity]] | `loadWithComponents` | injects_into `@Inject at RETURN` | both | `BlockEntityMixin.readBlockEntityAttachments` |
| [[40-Interfaces/net.minecraft.world.level.block.entity.BlockEntity|BlockEntity]] | `saveWithoutMetadata(Lnet/minecraft/world/level/storage/ValueOutput;)V` | injects_into `@Inject at TAIL` | both | `BlockEntityMixin.writeBlockEntityAttachments` |
| [[40-Interfaces/net.minecraft.world.level.chunk.LevelChunk|LevelChunk]] | `<init>(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/ProtoChunk;Lnet/minecraft/world/level/chunk/LevelChunk$PostLoadProcessor;)V` | injects_into `@Inject at TAIL` | both | `LevelChunkMixin.transferProtoChunkAttachment` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]] | `copyOf` | injects_into `@Inject at RETURN` | both | `SerializableChunkDataMixin.storeAttachmentNbtData` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]] | `parse` | injects_into `@Inject at RETURN` | both | `SerializableChunkDataMixin.storeAttachmentNbtData` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]] | `read` | injects_into `@Inject at RETURN` | both | `SerializableChunkDataMixin.setAttachmentDataInChunk` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]] | `write` | injects_into `@Inject at RETURN` | both | `SerializableChunkDataMixin.writeChunkAttachments` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.AttachmentRegistry|AttachmentRegistry]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.AttachmentSyncPredicate|AttachmentSyncPredicate]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.AttachmentTarget|AttachmentTarget]] (interface, 13 members)
- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.AttachmentType|AttachmentType]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.attachment.v1.GlobalAttachmentsProvider|GlobalAttachmentsProvider]] (interface, 1 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
