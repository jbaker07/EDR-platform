---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientPacketListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientPacketListener

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `net/minecraft/client/multiplayer/ClientCommonPacketListenerImpl`; implements `net/minecraft/network/protocol/game/ClientGamePacketListener`, `net/minecraft/network/TickablePacketListener`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getAdvancements` | `()Lnet/minecraft/client/multiplayer/ClientAdvancements;` | exact | invokevirtual@73 in `AdvancementToastMixin.extractAdvancementIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/network/Connection;` | exact | invokevirtual@5 in `ClientPlayNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getLocalGameProfile` | `()Lcom/mojang/authlib/GameProfile;` | exact | invokevirtual@9 in `ClientPlayNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@17 in `ClientPlayNetworking$Context.packetContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `LocalPlayerMixin.getPacketContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `handleCommands` | `(Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket;)V` | exact | invokevirtual@47 in `ClientCommands.refreshCommandCompletions` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `recipes` | `()Lnet/minecraft/world/item/crafting/RecipeAccess;` | exact | invokevirtual@99 in `RecipeSyncImplClient.onRecipeSyncPacket` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;)V` | inherited_exact | invokevirtual@101 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;)V` | inherited_exact | invokevirtual@57 in `MultiPlayerGameModeMixin.attackEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;)V` | inherited_exact | invokevirtual@50 in `ClientPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lne` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lne` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lne` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `clearLevel` | `()V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `handleCommands` | `(Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket;)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| injects_into | `handleCommands` | `(Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket;)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| injects_into | `handleLogin` | `(Lnet/minecraft/network/protocol/game/ClientboundLoginPacket;)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| injects_into | `handleLogin` | `(Lnet/minecraft/network/protocol/game/ClientboundLoginPacket;)V` | name_only | @Inject at ['NEW'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `handleLogin` | `(Lnet/minecraft/network/protocol/game/ClientboundLoginPacket;)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleRespawn` | `(Lnet/minecraft/network/protocol/game/ClientboundRespawnPacket;)V` | name_only | @Inject at ['NEW'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `handleUpdateTags` | `(Lnet/minecraft/network/protocol/common/ClientboundUpdateTagsPacket;)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `sendChat` | `(Ljava/lang/String;)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `sendCommand` | `(Ljava/lang/String;)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| injects_into | `sendCommand` | `(Ljava/lang/String;)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `sendUnattendedCommand` | `(Ljava/lang/String;Lnet/minecraft/client/gui/screens/Screen;)V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `commands` | `Lcom/mojang/brigadier/CommandDispatcher;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | declared |
| reads | `enabledFeatures` | `Lnet/minecraft/world/flag/FeatureFlagSet;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | declared |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |
| reads | `recipes` | `Lnet/minecraft/client/multiplayer/ClientRecipeContainer;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | declared |
| reads | `registryAccess` | `Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | declared |
| reads | `registryAccess` | `Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |
| reads | `restrictedSuggestionsProvider` | `Lnet/minecraft/client/multiplayer/ClientSuggestionProvider;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | declared |
| reads | `suggestionsProvider` | `Lnet/minecraft/client/multiplayer/ClientSuggestionProvider;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | declared |
| wraps | `handleRespawn` | `(Lnet/minecraft/network/protocol/game/ClientboundRespawnPacket;)V` | name_only | @WrapOperation at ['FIELD'] | client | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| wraps | `handleUpdateRecipes` | `(Lnet/minecraft/network/protocol/game/ClientboundUpdateRecipesPacket;)` | name_only | @WrapOperation at ['FIELD'] | client | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (55 fields, 222 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final UNSECURE_SERVER_TOAST_TITLE : Lnet/minecraft/network/chat/Component;
private static final UNSERURE_SERVER_TOAST : Lnet/minecraft/network/chat/Component;
private static final INVALID_PACKET : Lnet/minecraft/network/chat/Component;
private static final RECONFIGURE_SCREEN_MESSAGE : Lnet/minecraft/network/chat/Component;
private static final BAD_CHAT_INDEX : Lnet/minecraft/network/chat/Component;
private static final COMMAND_SEND_CONFIRM_TITLE : Lnet/minecraft/network/chat/Component;
private static final BUTTON_RUN_COMMAND : Lnet/minecraft/network/chat/Component;
private static final BUTTON_SUGGEST_COMMAND : Lnet/minecraft/network/chat/Component;
private static final PENDING_OFFSET_THRESHOLD : I
public static final TELEPORT_INTERPOLATION_THRESHOLD : I
private static final RESTRICTED_COMMAND : Lnet/minecraft/server/permissions/Permission;
private static final RESTRICTED_COMMAND_CHECK : Lnet/minecraft/server/permissions/PermissionCheck;
private static final ALLOW_RESTRICTED_COMMANDS : Lnet/minecraft/server/permissions/PermissionSet;
private static final COMMAND_NODE_BUILDER : Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket$NodeBuilder;
private static final ENTITY_SPAWN_REQUEST : Lnet/minecraft/world/entity/EntitySpawnRequest;
private final localGameProfile : Lcom/mojang/authlib/GameProfile;
private level : Lnet/minecraft/client/multiplayer/ClientLevel;
private levelData : Lnet/minecraft/client/multiplayer/ClientLevel$ClientLevelData;
private final playerInfoMap : Ljava/util/Map;
private final listedPlayers : Ljava/util/Set;
private final advancements : Lnet/minecraft/client/multiplayer/ClientAdvancements;
private final suggestionsProvider : Lnet/minecraft/client/multiplayer/ClientSuggestionProvider;
private final restrictedSuggestionsProvider : Lnet/minecraft/client/multiplayer/ClientSuggestionProvider;
private final debugQueryHandler : Lnet/minecraft/client/DebugQueryHandler;
private serverChunkRadius : I
private serverSimulationDistance : I
private final random : Lnet/minecraft/util/RandomSource;
private commands : Lcom/mojang/brigadier/CommandDispatcher;
private recipes : Lnet/minecraft/client/multiplayer/ClientRecipeContainer;
private levels : Ljava/util/Set;
private final registryAccess : Lnet/minecraft/core/RegistryAccess$Frozen;
private final enabledFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private final decoratedHashOpsGenerator : Lnet/minecraft/network/HashedPatchMap$HashGenerator;
private removedPlayerVehicleId : Ljava/util/OptionalInt;
private chatSession : Lnet/minecraft/network/chat/LocalChatSession;
private signedMessageEncoder : Lnet/minecraft/network/chat/SignedMessageChain$Encoder;
private nextChatIndex : I
private lastSeenMessages : Lnet/minecraft/network/chat/LastSeenMessagesTracker;
private messageSignatureCache : Lnet/minecraft/network/chat/MessageSignatureCache;
private keyPairFuture : Ljava/util/concurrent/CompletableFuture;
private remoteClientInformation : Lnet/minecraft/server/level/ClientInformation;
private final chunkBatchSizeCalculator : Lnet/minecraft/client/multiplayer/ChunkBatchSizeCalculator;
private final pingDebugMonitor : Lnet/minecraft/client/multiplayer/PingDebugMonitor;
private final debugSubscriber : Lnet/minecraft/client/multiplayer/ClientDebugSubscriber;
private levelLoadTracker : Lnet/minecraft/client/multiplayer/LevelLoadTracker;
private serverEnforcesSecureChat : Z
private onlineMode : Z
private closed : Z
private final scoreboard : Lnet/minecraft/world/scores/Scoreboard;
private final waypointManager : Lnet/minecraft/client/waypoints/ClientWaypointManager;
private final clockManager : Lnet/minecraft/client/ClientClockManager;
private final searchTrees : Lnet/minecraft/client/multiplayer/SessionSearchTrees;
private final cacheSlots : Ljava/util/List;
private clientLoaded : Z
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lnet/minecraft/client/multiplayer/CommonListenerCookie;)V
public getSuggestionsProvider()Lnet/minecraft/client/multiplayer/ClientSuggestionProvider;
public close()V
public clearLevel()V
private clearCacheSlots()V
public recipes()Lnet/minecraft/world/item/crafting/RecipeAccess;
public handleLogin(Lnet/minecraft/network/protocol/game/ClientboundLoginPacket;)V
public handleAddEntity(Lnet/minecraft/network/protocol/game/ClientboundAddEntityPacket;)V
private createEntityFromPacket(Lnet/minecraft/network/protocol/game/ClientboundAddEntityPacket;)Lnet/minecraft/world/entity/Entity;
private postAddEntitySoundInstance(Lnet/minecraft/world/entity/Entity;)V
public handleSetEntityMotion(Lnet/minecraft/network/protocol/game/ClientboundSetEntityMotionPacket;)V
public handleSetEntityData(Lnet/minecraft/network/protocol/game/ClientboundSetEntityDataPacket;)V
public handleEntityPositionSync(Lnet/minecraft/network/protocol/game/ClientboundEntityPositionSyncPacket;)V
public handleTeleportEntity(Lnet/minecraft/network/protocol/game/ClientboundTeleportEntityPacket;)V
public handleTickingState(Lnet/minecraft/network/protocol/game/ClientboundTickingStatePacket;)V
public handleTickingStep(Lnet/minecraft/network/protocol/game/ClientboundTickingStepPacket;)V
public handleSetHeldSlot(Lnet/minecraft/network/protocol/game/ClientboundSetHeldSlotPacket;)V
public handleMoveEntity(Lnet/minecraft/network/protocol/game/ClientboundMoveEntityPacket;)V
public handleMinecartAlongTrack(Lnet/minecraft/network/protocol/game/ClientboundMoveMinecartPacket;)V
public handleRotateMob(Lnet/minecraft/network/protocol/game/ClientboundRotateHeadPacket;)V
public handleRemoveEntities(Lnet/minecraft/network/protocol/game/ClientboundRemoveEntitiesPacket;)V
public handleMovePlayer(Lnet/minecraft/network/protocol/game/ClientboundPlayerPositionPacket;)V
private static setValuesFromPositionPacket(Lnet/minecraft/world/entity/PositionMoveRotation;Ljava/util/Set;Lnet/minecraft/world/entity/Entity;Z)Z
public handleRotatePlayer(Lnet/minecraft/network/protocol/game/ClientboundPlayerRotationPacket;)V
public handleChunkBlocksUpdate(Lnet/minecraft/network/protocol/game/ClientboundSectionBlocksUpdatePacket;)V
public handleLevelChunkWithLight(Lnet/minecraft/network/protocol/game/ClientboundLevelChunkWithLightPacket;)V
public handleChunksBiomes(Lnet/minecraft/network/protocol/game/ClientboundChunksBiomesPacket;)V
private enableChunkLight(Lnet/minecraft/world/level/chunk/LevelChunk;II)V
public handleForgetLevelChunk(Lnet/minecraft/network/protocol/game/ClientboundForgetLevelChunkPacket;)V
private queueLightRemoval(Lnet/minecraft/network/protocol/game/ClientboundForgetLevelChunkPacket;)V
public handleBlockUpdate(Lnet/minecraft/network/protocol/game/ClientboundBlockUpdatePacket;)V
public handleConfigurationStart(Lnet/minecraft/network/protocol/game/ClientboundStartConfigurationPacket;)V
public handleTakeItemEntity(Lnet/minecraft/network/protocol/game/ClientboundTakeItemEntityPacket;)V
public handleSystemChat(Lnet/minecraft/network/protocol/game/ClientboundSystemChatPacket;)V
public handlePlayerChat(Lnet/minecraft/network/protocol/game/ClientboundPlayerChatPacket;)V
public handleDisguisedChat(Lnet/minecraft/network/protocol/game/ClientboundDisguisedChatPacket;)V
public handleDeleteChat(Lnet/minecraft/network/protocol/game/ClientboundDeleteChatPacket;)V
public handleAnimate(Lnet/minecraft/network/protocol/game/ClientboundAnimatePacket;)V
public handleHurtAnimation(Lnet/minecraft/network/protocol/game/ClientboundHurtAnimationPacket;)V
public handleSwingAnimation(Lnet/minecraft/network/protocol/game/ClientboundSwingAnimationPacket;)V
public handleSetTime(Lnet/minecraft/network/protocol/game/ClientboundSetTimePacket;)V
public handleSetSpawn(Lnet/minecraft/network/protocol/game/ClientboundSetDefaultSpawnPositionPacket;)V
public handleSetEntityPassengersPacket(Lnet/minecraft/network/protocol/game/ClientboundSetPassengersPacket;)V
public handleAddTransientBlockPacket(Lnet/minecraft/network/protocol/game/ClientboundAddTransientBlockPacket;)V
public handleEntityLinkPacket(Lnet/minecraft/network/protocol/game/ClientboundSetEntityLinkPacket;)V
private static findTotem(Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/item/ItemStack;
public handleEntityEvent(Lnet/minecraft/network/protocol/game/ClientboundEntityEventPacket;)V
public handleDamageEvent(Lnet/minecraft/network/protocol/game/ClientboundDamageEventPacket;)V
public handleSetHealth(Lnet/minecraft/network/protocol/game/ClientboundSetHealthPacket;)V
public handleSetExperience(Lnet/minecraft/network/protocol/game/ClientboundSetExperiencePacket;)V
public handleRespawn(Lnet/minecraft/network/protocol/game/ClientboundRespawnPacket;)V
private determineLevelLoadingReason(ZLnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/client/gui/screens/LevelLoadingScreen$Reason;
public handleExplosion(Lnet/minecraft/network/protocol/game/ClientboundExplodePacket;)V
public handleMountScreenOpen(Lnet/minecraft/network/protocol/game/ClientboundMountScreenOpenPacket;)V
public handleOpenScreen(Lnet/minecraft/network/protocol/game/ClientboundOpenScreenPacket;)V
public handleContainerSetSlot(Lnet/minecraft/network/protocol/game/ClientboundContainerSetSlotPacket;)V
public handleSetCursorItem(Lnet/minecraft/network/protocol/game/ClientboundSetCursorItemPacket;)V
public handleSetPlayerInventory(Lnet/minecraft/network/protocol/game/ClientboundSetPlayerInventoryPacket;)V
public handleContainerContent(Lnet/minecraft/network/protocol/game/ClientboundContainerSetContentPacket;)V
public handleOpenSignEditor(Lnet/minecraft/network/protocol/game/ClientboundOpenSignEditorPacket;)V
public handleBlockEntityData(Lnet/minecraft/network/protocol/game/ClientboundBlockEntityDataPacket;)V
public handleContainerSetData(Lnet/minecraft/network/protocol/game/ClientboundContainerSetDataPacket;)V
public handleSetEquipment(Lnet/minecraft/network/protocol/game/ClientboundSetEquipmentPacket;)V
public handleContainerClose(Lnet/minecraft/network/protocol/game/ClientboundContainerClosePacket;)V
public handleBlockEvent(Lnet/minecraft/network/protocol/game/ClientboundBlockEventPacket;)V
public handleBlockDestruction(Lnet/minecraft/network/protocol/game/ClientboundBlockDestructionPacket;)V
public handleGameEvent(Lnet/minecraft/network/protocol/game/ClientboundGameEventPacket;)V
private openDemoIntroScreen(Lnet/minecraft/client/Options;)V
private startWaitingForNewLevel(Lnet/minecraft/client/player/LocalPlayer;Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/client/gui/screens/LevelLoadingScreen$Reason;)V
public handleMapItemData(Lnet/minecraft/network/protocol/game/ClientboundMapItemDataPacket;)V
public handleLevelEvent(Lnet/minecraft/network/protocol/game/ClientboundLevelEventPacket;)V
public handleUpdateAdvancementsPacket(Lnet/minecraft/network/protocol/game/ClientboundUpdateAdvancementsPacket;)V
public handleSelectAdvancementsTab(Lnet/minecraft/network/protocol/game/ClientboundSelectAdvancementsTabPacket;)V
public handleCommands(Lnet/minecraft/network/protocol/game/ClientboundCommandsPacket;)V
public handleStopSoundEvent(Lnet/minecraft/network/protocol/game/ClientboundStopSoundPacket;)V
public handleCommandSuggestions(Lnet/minecraft/network/protocol/game/ClientboundCommandSuggestionsPacket;)V
public handleUpdateRecipes(Lnet/minecraft/network/protocol/game/ClientboundUpdateRecipesPacket;)V
public handleLookAt(Lnet/minecraft/network/protocol/game/ClientboundPlayerLookAtPacket;)V
public handleTagQueryPacket(Lnet/minecraft/network/protocol/game/ClientboundTagQueryPacket;)V
public handleAwardStats(Lnet/minecraft/network/protocol/game/ClientboundAwardStatsPacket;)V
public handleRecipeBookAdd(Lnet/minecraft/network/protocol/game/ClientboundRecipeBookAddPacket;)V
public handleRecipeBookRemove(Lnet/minecraft/network/protocol/game/ClientboundRecipeBookRemovePacket;)V
public handleRecipeBookSettings(Lnet/minecraft/network/protocol/game/ClientboundRecipeBookSettingsPacket;)V
private refreshRecipeBook(Lnet/minecraft/client/ClientRecipeBook;)V
public handleUpdateMobEffect(Lnet/minecraft/network/protocol/game/ClientboundUpdateMobEffectPacket;)V
private updateTags(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagNetworkSerialization$NetworkPayload;)Lnet/minecraft/core/Registry$PendingTags;
public handleUpdateTags(Lnet/minecraft/network/protocol/common/ClientboundUpdateTagsPacket;)V
public handlePlayerCombatEnd(Lnet/minecraft/network/protocol/game/ClientboundPlayerCombatEndPacket;)V
public handlePlayerCombatEnter(Lnet/minecraft/network/protocol/game/ClientboundPlayerCombatEnterPacket;)V
public handlePlayerCombatKill(Lnet/minecraft/network/protocol/game/ClientboundPlayerCombatKillPacket;)V
public handleChangeDifficulty(Lnet/minecraft/network/protocol/game/ClientboundChangeDifficultyPacket;)V
public handleSetCamera(Lnet/minecraft/network/protocol/game/ClientboundSetCameraPacket;)V
public handleInitializeBorder(Lnet/minecraft/network/protocol/game/ClientboundInitializeBorderPacket;)V
public handleSetBorderCenter(Lnet/minecraft/network/protocol/game/ClientboundSetBorderCenterPacket;)V
public handleSetBorderLerpSize(Lnet/minecraft/network/protocol/game/ClientboundSetBorderLerpSizePacket;)V
public handleSetBorderSize(Lnet/minecraft/network/protocol/game/ClientboundSetBorderSizePacket;)V
public handleSetBorderWarningDistance(Lnet/minecraft/network/protocol/game/ClientboundSetBorderWarningDistancePacket;)V
public handleSetBorderWarningDelay(Lnet/minecraft/network/protocol/game/ClientboundSetBorderWarningDelayPacket;)V
public handleTitlesClear(Lnet/minecraft/network/protocol/game/ClientboundClearTitlesPacket;)V
public handleServerData(Lnet/minecraft/network/protocol/game/ClientboundServerDataPacket;)V
public handleCustomChatCompletions(Lnet/minecraft/network/protocol/game/ClientboundCustomChatCompletionsPacket;)V
public setActionBarText(Lnet/minecraft/network/protocol/game/ClientboundSetActionBarTextPacket;)V
public setTitleText(Lnet/minecraft/network/protocol/game/ClientboundSetTitleTextPacket;)V
public setSubtitleText(Lnet/minecraft/network/protocol/game/ClientboundSetSubtitleTextPacket;)V
public setTitlesAnimation(Lnet/minecraft/network/protocol/game/ClientboundSetTitlesAnimationPacket;)V
public handleTabListCustomisation(Lnet/minecraft/network/protocol/game/ClientboundTabListPacket;)V
public handleRemoveMobEffect(Lnet/minecraft/network/protocol/game/ClientboundRemoveMobEffectPacket;)V
public handlePlayerInfoRemove(Lnet/minecraft/network/protocol/game/ClientboundPlayerInfoRemovePacket;)V
public handlePlayerInfoUpdate(Lnet/minecraft/network/protocol/game/ClientboundPlayerInfoUpdatePacket;)V
private applyPlayerInfoUpdate(Lnet/minecraft/network/protocol/game/ClientboundPlayerInfoUpdatePacket$Action;Lnet/minecraft/network/protocol/game/ClientboundPlayerInfoUpdatePacket$Entry;Lnet/minecraft/client/multiplayer/PlayerInfo;)V
private initializeChatSession(Lnet/minecraft/network/protocol/game/ClientboundPlayerInfoUpdatePacket$Entry;Lnet/minecraft/client/multiplayer/PlayerInfo;)V
private enforcesSecureChat()Z
public onlineMode()Z
public handlePlayerAbilities(Lnet/minecraft/network/protocol/game/ClientboundPlayerAbilitiesPacket;)V
public handleGameRuleValues(Lnet/minecraft/network/protocol/game/ClientboundGameRuleValuesPacket;)V
public handleSoundEvent(Lnet/minecraft/network/protocol/game/ClientboundSoundPacket;)V
public handleSoundEntityEvent(Lnet/minecraft/network/protocol/game/ClientboundSoundEntityPacket;)V
public handleBossUpdate(Lnet/minecraft/network/protocol/game/ClientboundBossEventPacket;)V
public handleItemCooldown(Lnet/minecraft/network/protocol/game/ClientboundCooldownPacket;)V
public handleMoveVehicle(Lnet/minecraft/network/protocol/game/ClientboundMoveVehiclePacket;)V
public handleOpenBook(Lnet/minecraft/network/protocol/game/ClientboundOpenBookPacket;)V
public handleCustomPayload(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V
private handleUnknownCustomPayload(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V
public handleAddObjective(Lnet/minecraft/network/protocol/game/ClientboundSetObjectivePacket;)V
public handleSetScore(Lnet/minecraft/network/protocol/game/ClientboundSetScorePacket;)V
public handleResetScore(Lnet/minecraft/network/protocol/game/ClientboundResetScorePacket;)V
public handleSetDisplayObjective(Lnet/minecraft/network/protocol/game/ClientboundSetDisplayObjectivePacket;)V
public handleSetPlayerTeamPacket(Lnet/minecraft/network/protocol/game/ClientboundSetPlayerTeamPacket;)V
public handleParticleEvent(Lnet/minecraft/network/protocol/game/ClientboundLevelParticlesPacket;)V
private tryAddParticle(Lnet/minecraft/network/protocol/game/ClientboundLevelParticlesPacket;DDDDDD)Z
public handleUpdateAttributes(Lnet/minecraft/network/protocol/game/ClientboundUpdateAttributesPacket;)V
public handlePlaceRecipe(Lnet/minecraft/network/protocol/game/ClientboundPlaceGhostRecipePacket;)V
public handleLightUpdatePacket(Lnet/minecraft/network/protocol/game/ClientboundLightUpdatePacket;)V
private applyLightData(IILnet/minecraft/network/protocol/game/ClientboundLightUpdatePacketData;Z)V
public handleMerchantOffers(Lnet/minecraft/network/protocol/game/ClientboundMerchantOffersPacket;)V
public handleSetChunkCacheRadius(Lnet/minecraft/network/protocol/game/ClientboundSetChunkCacheRadiusPacket;)V
public handleSetSimulationDistance(Lnet/minecraft/network/protocol/game/ClientboundSetSimulationDistancePacket;)V
public handleSetChunkCacheCenter(Lnet/minecraft/network/protocol/game/ClientboundSetChunkCacheCenterPacket;)V
public handleBlockChangedAck(Lnet/minecraft/network/protocol/game/ClientboundBlockChangedAckPacket;)V
public handleBundlePacket(Lnet/minecraft/network/protocol/game/ClientboundBundlePacket;)V
public handleProjectilePowerPacket(Lnet/minecraft/network/protocol/game/ClientboundProjectilePowerPacket;)V
public handleChunkBatchStart(Lnet/minecraft/network/protocol/game/ClientboundChunkBatchStartPacket;)V
public handleChunkBatchFinished(Lnet/minecraft/network/protocol/game/ClientboundChunkBatchFinishedPacket;)V
public handleDebugSample(Lnet/minecraft/network/protocol/game/ClientboundDebugSamplePacket;)V
public handlePongResponse(Lnet/minecraft/network/protocol/ping/ClientboundPongResponsePacket;)V
public handleTestInstanceBlockStatus(Lnet/minecraft/network/protocol/game/ClientboundTestInstanceBlockStatus;)V
public handleWaypoint(Lnet/minecraft/network/protocol/game/ClientboundTrackedWaypointPacket;)V
public handleDebugChunkValue(Lnet/minecraft/network/protocol/game/ClientboundDebugChunkValuePacket;)V
public handleDebugBlockValue(Lnet/minecraft/network/protocol/game/ClientboundDebugBlockValuePacket;)V
public handleDebugEntityValue(Lnet/minecraft/network/protocol/game/ClientboundDebugEntityValuePacket;)V
public handleDebugEvent(Lnet/minecraft/network/protocol/game/ClientboundDebugEventPacket;)V
public handleGameTestHighlightPos(Lnet/minecraft/network/protocol/game/ClientboundGameTestHighlightPosPacket;)V
public handleLowDiskSpaceWarning(Lnet/minecraft/network/protocol/game/ClientboundLowDiskSpaceWarningPacket;)V
private readSectionList(IILnet/minecraft/world/level/lighting/LevelLightEngine;Lnet/minecraft/world/level/LightLayer;Ljava/util/BitSet;Ljava/util/BitSet;Ljava/util/Iterator;Z)V
public getConnection()Lnet/minecraft/network/Connection;
public isAcceptingMessages()Z
public getListedOnlinePlayers()Ljava/util/Collection;
public getOnlinePlayers()Ljava/util/Collection;
public getOnlinePlayerIds()Ljava/util/Collection;
public getPlayerInfo(Ljava/util/UUID;)Lnet/minecraft/client/multiplayer/PlayerInfo;
public getPlayerInfo(Ljava/lang/String;)Lnet/minecraft/client/multiplayer/PlayerInfo;
public getSeenPlayers()Ljava/util/Map;
public getPlayerInfoIgnoreCase(Ljava/lang/String;)Lnet/minecraft/client/multiplayer/PlayerInfo;
public getLocalGameProfile()Lcom/mojang/authlib/GameProfile;
public getAdvancements()Lnet/minecraft/client/multiplayer/ClientAdvancements;
public getCommands()Lcom/mojang/brigadier/CommandDispatcher;
public getLevel()Lnet/minecraft/client/multiplayer/ClientLevel;
public getDebugQueryHandler()Lnet/minecraft/client/DebugQueryHandler;
public levels()Ljava/util/Set;
public registryAccess()Lnet/minecraft/core/RegistryAccess$Frozen;
public markMessageAsProcessed(Lnet/minecraft/network/chat/MessageSignature;Z)V
private sendChatAcknowledgement()V
public sendChat(Ljava/lang/String;)V
public sendCommand(Ljava/lang/String;)V
public sendUnattendedCommand(Ljava/lang/String;Lnet/minecraft/client/gui/screens/Screen;)V
private verifyCommand(Ljava/lang/String;)Lnet/minecraft/client/multiplayer/ClientPacketListener$CommandCheckResult;
private static isValidCommand(Lcom/mojang/brigadier/ParseResults;)Z
private openSendConfirmationWindow(Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/network/chat/Component;Ljava/lang/Runnable;)V
private openCommandSendConfirmationWindow(Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/client/gui/screens/Screen;)V
private openSignedCommandSendConfirmationWindow(Ljava/lang/String;Ljava/lang/String;Lnet/minecraft/client/gui/screens/Screen;)V
public broadcastClientInformation(Lnet/minecraft/server/level/ClientInformation;)V
public tick()V
private notifyPlayerLoaded()V
public getPlayerCompiledSectionCallback()Ljava/lang/Runnable;
public prepareKeyPair()V
private setKeyPair(Lnet/minecraft/world/entity/player/ProfileKeyPair;)V
protected createDialogAccess()Lnet/minecraft/client/gui/screens/dialog/DialogConnectionAccess;
public getServerData()Lnet/minecraft/client/multiplayer/ServerData;
public enabledFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public isFeatureEnabled(Lnet/minecraft/world/flag/FeatureFlagSet;)Z
public scoreboard()Lnet/minecraft/world/scores/Scoreboard;
public updateSearchTrees()V
public searchTrees()Lnet/minecraft/client/multiplayer/SessionSearchTrees;
public registerForCleaning(Lnet/minecraft/client/multiplayer/CacheSlot;)V
public decoratedHashOpsGenenerator()Lnet/minecraft/network/HashedPatchMap$HashGenerator;
public getWaypointManager()Lnet/minecraft/client/waypoints/ClientWaypointManager;
public createDebugValueAccess()Lnet/minecraft/util/debug/DebugValueAccess;
public hasClientLoaded()Z
private setClientLoaded(Z)V
public clockManager()Lnet/minecraft/client/ClientClockManager;
private synthetic lambda$openSignedCommandSendConfirmationWindow$1(Ljava/lang/String;Lnet/minecraft/client/gui/screens/Screen;)V
private synthetic lambda$openSignedCommandSendConfirmationWindow$0(Ljava/lang/String;)V
private synthetic lambda$openCommandSendConfirmationWindow$0(Ljava/lang/String;Lnet/minecraft/client/gui/screens/Screen;)V
private synthetic lambda$openSendConfirmationWindow$0(Ljava/lang/Runnable;Lnet/minecraft/client/gui/screens/Screen;Z)V
private synthetic lambda$sendCommand$0(Ljava/time/Instant;JLnet/minecraft/network/chat/LastSeenMessagesTracker$Update;Ljava/lang/String;)Lnet/minecraft/network/chat/MessageSignature;
private synthetic lambda$handleLightUpdatePacket$0(IILnet/minecraft/network/protocol/game/ClientboundLightUpdatePacketData;)V
private static synthetic lambda$handleSetPlayerTeamPacket$0(Lnet/minecraft/world/scores/PlayerTeam;Lnet/minecraft/network/protocol/game/ClientboundSetPlayerTeamPacket$Parameters;)V
private synthetic lambda$handleUpdateTags$0(Ljava/util/List;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagNetworkSerialization$NetworkPayload;)V
private synthetic lambda$openDemoIntroScreen$1(Lnet/minecraft/client/gui/components/PopupScreen;)V
private static synthetic lambda$openDemoIntroScreen$0(Lnet/minecraft/client/gui/components/PopupScreen;)V
private synthetic lambda$handleGameEvent$0(Lnet/minecraft/client/player/LocalPlayer;)V
private static synthetic lambda$handleSetEquipment$0(Lnet/minecraft/world/entity/LivingEntity;Lcom/mojang/datafixers/util/Pair;)V
private synthetic lambda$handleBlockEntityData$0(Lnet/minecraft/network/protocol/game/ClientboundBlockEntityDataPacket;Lnet/minecraft/world/level/block/entity/BlockEntity;)V
private synthetic lambda$queueLightRemoval$0(Lnet/minecraft/world/level/ChunkPos;)V
private synthetic lambda$handleLevelChunkWithLight$0(IILnet/minecraft/network/protocol/game/ClientboundLightUpdatePacketData;)V
private synthetic lambda$handleChunkBlocksUpdate$0(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
private synthetic lambda$handleRemoveEntities$0(I)V
private static synthetic lambda$new$2(Lnet/minecraft/client/Minecraft;Lnet/minecraft/server/permissions/Permission;)Z
private static synthetic lambda$new$0(Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/core/component/TypedDataComponent;)Ljava/lang/Integer;
private static synthetic lambda$new$1(Lnet/minecraft/core/component/TypedDataComponent;Ljava/lang/String;)Ljava/lang/IllegalArgumentException;
private static synthetic lambda$static$0(Lnet/minecraft/server/permissions/Permission;)Z
static <clinit>()V
```
