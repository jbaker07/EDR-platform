---
type: "system"
package: "net.minecraft.network.protocol"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol

Analyst note: [[_authored/systems/net.minecraft.network.protocol|Packets and protocol phases]]

368 classes (298 top-level) across 11 packages in the processed jar; 2 changed by Loom processing; 34 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.network.protocol.BundlePacket|BundlePacket]] -- calls:1, injects_into:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.Packet|Packet]] -- calls:6 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.PacketFlow|PacketFlow]] -- reads:17 -- by fabric-events-interaction-v0, fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.PacketType|PacketType]] -- calls:4 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket|ClientboundCustomPayloadPacket]] -- calls:3, reads:1, wraps:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.ClientboundPingPacket|ClientboundPingPacket]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.CommonPacketTypes|CommonPacketTypes]] -- reads:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket|ServerboundCustomPayloadPacket]] -- calls:4, reads:1, wraps:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.ServerboundPongPacket|ServerboundPongPacket]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.custom.BrandPayload|BrandPayload]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload|CustomPacketPayload]] -- calls:27 -- by fabric-menu-api-v1, fabric-networking-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload_1|CustomPacketPayload$1]] -- wraps:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload_Type|CustomPacketPayload$Type]] -- calls:46 -- by fabric-client-gametest-api-v1, fabric-data-attachment-api-v1, fabric-menu-api-v1, fabric-networking-api-v1, fabric-particles-v1, fabric-recipe-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload_TypeAndCodec|CustomPacketPayload$TypeAndCodec]] -- calls:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks|ClientboundSelectKnownPacks]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.network.protocol.configuration.ConfigurationProtocols|ConfigurationProtocols]] -- reads:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks|ServerboundSelectKnownPacks]] -- injects_into:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket|ClientboundBlockUpdatePacket]] -- calls:2 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ClientboundBundlePacket|ClientboundBundlePacket]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.game.ClientboundRespawnPacket|ClientboundRespawnPacket]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.game.GameProtocols|GameProtocols]] -- reads:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundAttackPacket|ServerboundAttackPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundInteractPacket|ServerboundInteractPacket]] -- calls:3 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPickItemFromBlockPacket|ServerboundPickItemFromBlockPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPickItemFromEntityPacket|ServerboundPickItemFromEntityPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPlayerActionPacket|ServerboundPlayerActionPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPlayerActionPacket_Action|ServerboundPlayerActionPacket$Action]] -- reads:2 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundUseItemOnPacket|ServerboundUseItemOnPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundUseItemPacket|ServerboundUseItemPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundCustomQueryPacket|ClientboundCustomQueryPacket]] -- calls:7, injects_into:1, reads:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundLoginCompressionPacket|ClientboundLoginCompressionPacket]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket|ClientboundLoginFinishedPacket]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket|ServerboundCustomQueryAnswerPacket]] -- calls:4, injects_into:1, reads:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.login.custom.CustomQueryPayload|CustomQueryPayload]] -- calls:2 -- by fabric-networking-api-v1

## Declared inventory

### `net.minecraft.network.protocol` (13 top-level)

`BundleDelimiterPacket`, [[40-Interfaces/net.minecraft.network.protocol.BundlePacket|BundlePacket]], `BundlerInfo`, `CodecModifier`, [[40-Interfaces/net.minecraft.network.protocol.Packet|Packet]], [[40-Interfaces/net.minecraft.network.protocol.PacketFlow|PacketFlow]], [[40-Interfaces/net.minecraft.network.protocol.PacketType|PacketType]], `PacketUtils`, `ProtocolCodecBuilder`, `ProtocolInfoBuilder`, `SimpleUnboundProtocol`, `UnboundProtocol`, `package-info`

### `net.minecraft.network.protocol.common` (24 top-level)

`ClientCommonPacketListener`, `ClientboundClearDialogPacket`, [[40-Interfaces/net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket|ClientboundCustomPayloadPacket]], `ClientboundCustomReportDetailsPacket`, `ClientboundDisconnectPacket`, `ClientboundKeepAlivePacket`, [[40-Interfaces/net.minecraft.network.protocol.common.ClientboundPingPacket|ClientboundPingPacket]], `ClientboundPostEffectsPacket`, `ClientboundResourcePackPopPacket`, `ClientboundResourcePackPushPacket`, `ClientboundServerLinksPacket`, `ClientboundShowDialogPacket`, `ClientboundStoreCookiePacket`, `ClientboundTransferPacket`, `ClientboundUpdateTagsPacket`, [[40-Interfaces/net.minecraft.network.protocol.common.CommonPacketTypes|CommonPacketTypes]], `ServerCommonPacketListener`, `ServerboundClientInformationPacket`, `ServerboundCustomClickActionPacket`, [[40-Interfaces/net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket|ServerboundCustomPayloadPacket]], `ServerboundKeepAlivePacket`, [[40-Interfaces/net.minecraft.network.protocol.common.ServerboundPongPacket|ServerboundPongPacket]], `ServerboundResourcePackPacket`, `package-info`

### `net.minecraft.network.protocol.common.custom` (4 top-level)

[[40-Interfaces/net.minecraft.network.protocol.common.custom.BrandPayload|BrandPayload]], [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload|CustomPacketPayload]], `DiscardedPayload`, `package-info`

### `net.minecraft.network.protocol.configuration` (14 top-level)

`ClientConfigurationPacketListener`, `ClientboundCodeOfConductPacket`, `ClientboundFinishConfigurationPacket`, `ClientboundRegistryDataPacket`, `ClientboundResetChatPacket`, [[40-Interfaces/net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks|ClientboundSelectKnownPacks]], `ClientboundUpdateEnabledFeaturesPacket`, `ConfigurationPacketTypes`, [[40-Interfaces/net.minecraft.network.protocol.configuration.ConfigurationProtocols|ConfigurationProtocols]], `ServerConfigurationPacketListener`, `ServerboundAcceptCodeOfConductPacket`, `ServerboundFinishConfigurationPacket`, [[40-Interfaces/net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks|ServerboundSelectKnownPacks]], `package-info`

### `net.minecraft.network.protocol.cookie` (6 top-level)

`ClientCookiePacketListener`, `ClientboundCookieRequestPacket`, `CookiePacketTypes`, `ServerCookiePacketListener`, `ServerboundCookieResponsePacket`, `package-info`

### `net.minecraft.network.protocol.game` (198 top-level)

`ClientGamePacketListener`, `ClientboundAddEntityPacket`, `ClientboundAddTransientBlockPacket`, `ClientboundAnimatePacket`, `ClientboundAwardStatsPacket`, `ClientboundBlockChangedAckPacket`, `ClientboundBlockDestructionPacket`, `ClientboundBlockEntityDataPacket`, `ClientboundBlockEventPacket`, [[40-Interfaces/net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket|ClientboundBlockUpdatePacket]], `ClientboundBossEventPacket`, `ClientboundBundleDelimiterPacket`, [[40-Interfaces/net.minecraft.network.protocol.game.ClientboundBundlePacket|ClientboundBundlePacket]], `ClientboundChangeDifficultyPacket`, `ClientboundChunkBatchFinishedPacket`, `ClientboundChunkBatchStartPacket`, `ClientboundChunksBiomesPacket`, `ClientboundClearTitlesPacket`, `ClientboundCommandSuggestionsPacket`, `ClientboundCommandsPacket`, `ClientboundContainerClosePacket`, `ClientboundContainerSetContentPacket`, `ClientboundContainerSetDataPacket`, `ClientboundContainerSetSlotPacket`, `ClientboundCooldownPacket`, `ClientboundCustomChatCompletionsPacket`, `ClientboundDamageEventPacket`, `ClientboundDebugBlockValuePacket`, `ClientboundDebugChunkValuePacket`, `ClientboundDebugEntityValuePacket`, `ClientboundDebugEventPacket`, `ClientboundDebugSamplePacket`, `ClientboundDeleteChatPacket`, `ClientboundDisguisedChatPacket`, `ClientboundEntityEventPacket`, `ClientboundEntityPositionSyncPacket`, `ClientboundExplodePacket`, `ClientboundForgetLevelChunkPacket`, `ClientboundGameEventPacket`, `ClientboundGameRuleValuesPacket`, `ClientboundGameTestHighlightPosPacket`, `ClientboundHurtAnimationPacket`, `ClientboundInitializeBorderPacket`, `ClientboundLevelChunkPacketData`, `ClientboundLevelChunkWithLightPacket`, `ClientboundLevelEventPacket`, `ClientboundLevelParticlesPacket`, `ClientboundLightUpdatePacket`, `ClientboundLightUpdatePacketData`, `ClientboundLoginPacket`, `ClientboundLowDiskSpaceWarningPacket`, `ClientboundMapItemDataPacket`, `ClientboundMerchantOffersPacket`, `ClientboundMountScreenOpenPacket`, `ClientboundMoveEntityPacket`, `ClientboundMoveMinecartPacket`, `ClientboundMoveVehiclePacket`, `ClientboundOpenBookPacket`, `ClientboundOpenScreenPacket`, `ClientboundOpenSignEditorPacket`, `ClientboundPlaceGhostRecipePacket`, `ClientboundPlayerAbilitiesPacket`, `ClientboundPlayerChatPacket`, `ClientboundPlayerCombatEndPacket`, `ClientboundPlayerCombatEnterPacket`, `ClientboundPlayerCombatKillPacket`, `ClientboundPlayerInfoRemovePacket`, `ClientboundPlayerInfoUpdatePacket`, `ClientboundPlayerLookAtPacket`, `ClientboundPlayerPositionPacket`, `ClientboundPlayerRotationPacket`, `ClientboundProjectilePowerPacket`, `ClientboundRecipeBookAddPacket`, `ClientboundRecipeBookRemovePacket`, `ClientboundRecipeBookSettingsPacket`, `ClientboundRemoveEntitiesPacket`, `ClientboundRemoveMobEffectPacket`, `ClientboundResetScorePacket`, [[40-Interfaces/net.minecraft.network.protocol.game.ClientboundRespawnPacket|ClientboundRespawnPacket]], `ClientboundRotateHeadPacket`, `ClientboundSectionBlocksUpdatePacket`, `ClientboundSelectAdvancementsTabPacket`, `ClientboundServerDataPacket`, `ClientboundSetActionBarTextPacket`, `ClientboundSetBorderCenterPacket`, `ClientboundSetBorderLerpSizePacket`, `ClientboundSetBorderSizePacket`, `ClientboundSetBorderWarningDelayPacket`, `ClientboundSetBorderWarningDistancePacket`, `ClientboundSetCameraPacket`, `ClientboundSetChunkCacheCenterPacket`, `ClientboundSetChunkCacheRadiusPacket`, `ClientboundSetCursorItemPacket`, `ClientboundSetDefaultSpawnPositionPacket`, `ClientboundSetDisplayObjectivePacket`, `ClientboundSetEntityDataPacket`, `ClientboundSetEntityLinkPacket`, `ClientboundSetEntityMotionPacket`, `ClientboundSetEquipmentPacket`, `ClientboundSetExperiencePacket`, `ClientboundSetHealthPacket`, `ClientboundSetHeldSlotPacket`, `ClientboundSetObjectivePacket`, `ClientboundSetPassengersPacket`, `ClientboundSetPlayerInventoryPacket`, `ClientboundSetPlayerTeamPacket`, `ClientboundSetScorePacket`, `ClientboundSetSimulationDistancePacket`, `ClientboundSetSubtitleTextPacket`, `ClientboundSetTimePacket`, `ClientboundSetTitleTextPacket`, `ClientboundSetTitlesAnimationPacket`, `ClientboundSoundEntityPacket`, `ClientboundSoundPacket`, `ClientboundStartConfigurationPacket`, `ClientboundStopSoundPacket`, `ClientboundSwingAnimationPacket`, `ClientboundSystemChatPacket`, `ClientboundTabListPacket`, `ClientboundTagQueryPacket`, `ClientboundTakeItemEntityPacket`, `ClientboundTeleportEntityPacket`, `ClientboundTestInstanceBlockStatus`, `ClientboundTickingStatePacket`, `ClientboundTickingStepPacket`, `ClientboundTrackedWaypointPacket`, `ClientboundUpdateAdvancementsPacket`, `ClientboundUpdateAttributesPacket`, `ClientboundUpdateMobEffectPacket`, `ClientboundUpdateRecipesPacket`, `CommonPlayerSpawnInfo`, `DebugEntityNameGenerator`, `GamePacketTypes`, [[40-Interfaces/net.minecraft.network.protocol.game.GameProtocols|GameProtocols]], `MovementPacket`, `ServerGamePacketListener`, `ServerPacketListener`, `ServerboundAcceptTeleportationPacket`, [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundAttackPacket|ServerboundAttackPacket]], `ServerboundBlockEntityTagQueryPacket`, `ServerboundChangeDifficultyPacket`, `ServerboundChangeGameModePacket`, `ServerboundChatAckPacket`, `ServerboundChatCommandPacket`, `ServerboundChatCommandSignedPacket`, `ServerboundChatPacket`, `ServerboundChatSessionUpdatePacket`, `ServerboundChunkBatchReceivedPacket`, `ServerboundClientCommandPacket`, `ServerboundClientTickEndPacket`, `ServerboundCommandSuggestionPacket`, `ServerboundConfigurationAcknowledgedPacket`, `ServerboundContainerButtonClickPacket`, `ServerboundContainerClickPacket`, `ServerboundContainerClosePacket`, `ServerboundContainerSlotStateChangedPacket`, `ServerboundDebugSubscriptionRequestPacket`, `ServerboundEditBookPacket`, `ServerboundEntityTagQueryPacket`, [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundInteractPacket|ServerboundInteractPacket]], `ServerboundJigsawGeneratePacket`, `ServerboundLockDifficultyPacket`, `ServerboundMovePlayerPacket`, `ServerboundMoveVehiclePacket`, `ServerboundPaddleBoatPacket`, [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPickItemFromBlockPacket|ServerboundPickItemFromBlockPacket]], [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPickItemFromEntityPacket|ServerboundPickItemFromEntityPacket]], `ServerboundPlaceRecipePacket`, `ServerboundPlayerAbilitiesPacket`, [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPlayerActionPacket|ServerboundPlayerActionPacket]], `ServerboundPlayerCommandPacket`, `ServerboundPlayerInputPacket`, `ServerboundPlayerLoadedPacket`, `ServerboundPunchPacket`, `ServerboundRecipeBookChangeSettingsPacket`, `ServerboundRecipeBookSeenRecipePacket`, `ServerboundRenameItemPacket`, `ServerboundSeenAdvancementsPacket`, `ServerboundSelectBundleItemPacket`, `ServerboundSelectTradePacket`, `ServerboundSetBeaconPacket`, `ServerboundSetCarriedItemPacket`, `ServerboundSetCommandBlockPacket`, `ServerboundSetCommandMinecartPacket`, `ServerboundSetCreativeModeSlotPacket`, `ServerboundSetGameRulePacket`, `ServerboundSetJigsawBlockPacket`, `ServerboundSetStructureBlockPacket`, `ServerboundSetTestBlockPacket`, `ServerboundSignUpdatePacket`, `ServerboundSpectatorActionPacket`, `ServerboundTeleportToEntityPacket`, `ServerboundTestInstanceBlockActionPacket`, [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundUseItemOnPacket|ServerboundUseItemOnPacket]], [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundUseItemPacket|ServerboundUseItemPacket]], `VecDelta`, `VecDeltaCodec`, `package-info`

### `net.minecraft.network.protocol.handshake` (6 top-level)

`ClientIntent`, `ClientIntentionPacket`, `HandshakePacketTypes`, `HandshakeProtocols`, `ServerHandshakePacketListener`, `package-info`

### `net.minecraft.network.protocol.login` (14 top-level)

`ClientLoginPacketListener`, [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundCustomQueryPacket|ClientboundCustomQueryPacket]], `ClientboundHelloPacket`, [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundLoginCompressionPacket|ClientboundLoginCompressionPacket]], `ClientboundLoginDisconnectPacket`, [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket|ClientboundLoginFinishedPacket]], `LoginPacketTypes`, `LoginProtocols`, `ServerLoginPacketListener`, [[40-Interfaces/net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket|ServerboundCustomQueryAnswerPacket]], `ServerboundHelloPacket`, `ServerboundKeyPacket`, `ServerboundLoginAcknowledgedPacket`, `package-info`

### `net.minecraft.network.protocol.login.custom` (5 top-level)

`CustomQueryAnswerPayload`, [[40-Interfaces/net.minecraft.network.protocol.login.custom.CustomQueryPayload|CustomQueryPayload]], `DiscardedQueryAnswerPayload`, `DiscardedQueryPayload`, `package-info`

### `net.minecraft.network.protocol.ping` (6 top-level)

`ClientPongPacketListener`, `ClientboundPongResponsePacket`, `PingPacketTypes`, `ServerPingPacketListener`, `ServerboundPingRequestPacket`, `package-info`

### `net.minecraft.network.protocol.status` (8 top-level)

`ClientStatusPacketListener`, `ClientboundStatusResponsePacket`, `ServerStatus`, `ServerStatusPacketListener`, `ServerboundStatusRequestPacket`, `StatusPacketTypes`, `StatusProtocols`, `package-info`

