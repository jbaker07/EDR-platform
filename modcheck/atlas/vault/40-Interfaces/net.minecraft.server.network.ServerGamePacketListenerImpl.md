---
type: "interface"
fqcn: "net.minecraft.server.network.ServerGamePacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerGamePacketListenerImpl

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`class` public; extends `net/minecraft/server/network/ServerCommonPacketListenerImpl`; implements `net/minecraft/network/protocol/game/ServerGamePacketListener`, `net/minecraft/server/network/ServerPlayerConnection`, `net/minecraft/network/TickablePacketListener`, `net/minecraft/network/protocol/game/GameProtocols$Context`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connecti` | exact | invokespecial@20 in `FakePlayerPacketListener.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@12 in `AttachmentSync.trySync` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@28 in `AttachmentSync.trySync` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@9 in `ServerPlayNetworking$Context.packetContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `ServerPlayNetworkAddon$ContextImpl.packetContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `ServerPlayerMixin.getPacketContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPlayer` | `()Lnet/minecraft/server/level/ServerPlayer;` | exact | invokevirtual@4 in `ServerPlayNetworkAddon$ContextImpl.player` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;)V` | inherited_exact | invokevirtual@62 in `InteractionEventsRouter.lambda$onInitialize$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;)V` | inherited_exact | invokevirtual@64 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;)V` | inherited_exact | invokevirtual@117 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;)V` | inherited_exact | invokevirtual@130 in `ServerPlayerMixin.fabric_replaceVanillaScreenPacket` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;)V` | inherited_exact | invokevirtual@46 in `ServerPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `switchToConfig` | `()V` | exact | invokevirtual@26 in `ServerPlayNetworkAddon.reconfigure` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `tryPickItem` | `(Lnet/minecraft/world/item/ItemStack;)V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| injects_into | `<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connecti` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleCustomPayload` | `(Lnet/minecraft/network/protocol/common/ServerboundCustomPayloadPacket` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleInteract` | `(Lnet/minecraft/network/protocol/game/ServerboundInteractPacket;)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `player` | `Lnet/minecraft/server/level/ServerPlayer;` | exact | getfield@31 in `GlobalAttachmentsImpl.lambda$fabric_syncChange$0` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/server/level/ServerPlayer;` | exact | getfield@45 in `GlobalAttachmentsImpl.lambda$fabric_syncChange$0` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/server/level/ServerPlayer;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| reads | `player` | `Lnet/minecraft/server/level/ServerPlayer;` | exact | getfield@6 in `ServerPlayNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/server/level/ServerPlayer;` | exact | getfield@4 in `ServerPlayNetworkAddon.schedule` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `handleConfigurationAcknowledged` | `(Lnet/minecraft/network/protocol/game/ServerboundConfigurationAcknowle` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `handlePickItemFromBlock` | `(Lnet/minecraft/network/protocol/game/ServerboundPickItemFromBlockPack` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| wraps | `handlePickItemFromEntity` | `(Lnet/minecraft/network/protocol/game/ServerboundPickItemFromEntityPac` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (54 fields, 147 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final NO_BLOCK_UPDATES_TO_ACK : I
private static final TRACKED_MESSAGE_DISCONNECT_THRESHOLD : I
private static final MAXIMUM_FLYING_TICKS : I
private static final ATTACK_INDICATOR_TOLERANCE_TICKS : I
public static final CLIENT_LOADED_TIMEOUT_TIME : I
private static final MIN_SUPPORTED_BLOCKS_RESEND_TICKS : I
private static final MAX_DISTANCE_TO_VALID_POSITION : I
private static final INVALID_POSITION_CORRECTION_MAX_INTERVAL_TICKS : I
private static final CHAT_VALIDATION_FAILED : Lnet/minecraft/network/chat/Component;
private static final INVALID_COMMAND_SIGNATURE : Lnet/minecraft/network/chat/Component;
public player : Lnet/minecraft/server/level/ServerPlayer;
public final chunkSender : Lnet/minecraft/server/network/PlayerChunkSender;
private tickCount : I
private ackBlockChangesUpTo : I
private final dropSpamThrottler : Lnet/minecraft/util/TickThrottler;
private final chatSpamThrottler : Lnet/minecraft/util/TickThrottler;
private final commandSpamThrottler : Lnet/minecraft/util/TickThrottler;
private firstGoodX : D
private firstGoodY : D
private firstGoodZ : D
private lastGoodX : D
private lastGoodY : D
private lastGoodZ : D
private lastVehicle : Lnet/minecraft/world/entity/Entity;
private vehicleFirstGoodX : D
private vehicleFirstGoodY : D
private vehicleFirstGoodZ : D
private vehicleLastGoodX : D
private vehicleLastGoodY : D
private vehicleLastGoodZ : D
private awaitingPositionFromClient : Lnet/minecraft/world/phys/Vec3;
private awaitingTeleport : I
private awaitingTeleportTime : I
private clientIsFloating : Z
private aboveGroundTickCount : I
private clientVehicleIsFloating : Z
private aboveGroundVehicleTickCount : I
private receivedMovePacketCount : I
private knownMovePacketCount : I
private receivedMovementThisTick : Z
private receivedPositionThisTick : Z
private chatSession : Lnet/minecraft/network/chat/RemoteChatSession;
private signedMessageDecoder : Lnet/minecraft/network/chat/SignedMessageChain$Decoder;
private final lastSeenMessages : Lnet/minecraft/network/chat/LastSeenMessagesValidator;
private nextChatIndex : I
private final messageSignatureCache : Lnet/minecraft/network/chat/MessageSignatureCache;
private final chatMessageChain : Lnet/minecraft/util/FutureChain;
private waitingForSwitchToConfig : Z
private waitingForRespawn : Z
private clientLoadedTimeoutTimer : I
private lastSupportedBlocksSend : I
private final commandSuggestionsProvider : Lnet/minecraft/server/network/ServerCommandSuggestionsProvider;
private vehiclePositionLastResetAt : I
public <init>(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/network/CommonListenerCookie;)V
public tick()V
private tickPlayer()Z
private getMaximumFlyingTicks(Lnet/minecraft/world/entity/Entity;)I
public resetFlyingTicks()V
public resetPosition()V
public isAcceptingMessages()Z
public shouldHandleMessage(Lnet/minecraft/network/protocol/Packet;)Z
protected playerProfile()Lcom/mojang/authlib/GameProfile;
private filterTextPacket(Ljava/lang/Object;Ljava/util/function/BiFunction;)Ljava/util/concurrent/CompletableFuture;
private filterTextPacket(Ljava/lang/String;)Ljava/util/concurrent/CompletableFuture;
private filterTextPacket(Ljava/util/List;)Ljava/util/concurrent/CompletableFuture;
public handlePlayerInput(Lnet/minecraft/network/protocol/game/ServerboundPlayerInputPacket;)V
private static containsInvalidValues(DDDFF)Z
private static clampHorizontal(D)D
private static clampVertical(D)D
public handleMoveVehicle(Lnet/minecraft/network/protocol/game/ServerboundMoveVehiclePacket;)V
private noBlocksAround(Lnet/minecraft/world/entity/Entity;)Z
public handleAcceptTeleportPacket(Lnet/minecraft/network/protocol/game/ServerboundAcceptTeleportationPacket;)V
public handleAcceptPlayerLoad(Lnet/minecraft/network/protocol/game/ServerboundPlayerLoadedPacket;)V
public handleRecipeBookSeenRecipePacket(Lnet/minecraft/network/protocol/game/ServerboundRecipeBookSeenRecipePacket;)V
public handleBundleItemSelectedPacket(Lnet/minecraft/network/protocol/game/ServerboundSelectBundleItemPacket;)V
public handleRecipeBookChangeSettingsPacket(Lnet/minecraft/network/protocol/game/ServerboundRecipeBookChangeSettingsPacket;)V
public handleSeenAdvancements(Lnet/minecraft/network/protocol/game/ServerboundSeenAdvancementsPacket;)V
public handleCustomCommandSuggestions(Lnet/minecraft/network/protocol/game/ServerboundCommandSuggestionPacket;)V
public handleSetCommandBlock(Lnet/minecraft/network/protocol/game/ServerboundSetCommandBlockPacket;)V
public handleSetCommandMinecart(Lnet/minecraft/network/protocol/game/ServerboundSetCommandMinecartPacket;)V
public handlePickItemFromBlock(Lnet/minecraft/network/protocol/game/ServerboundPickItemFromBlockPacket;)V
private static addBlockDataToItem(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/item/ItemStack;)V
public handlePickItemFromEntity(Lnet/minecraft/network/protocol/game/ServerboundPickItemFromEntityPacket;)V
private tryPickItem(Lnet/minecraft/world/item/ItemStack;)V
public handleRenameItem(Lnet/minecraft/network/protocol/game/ServerboundRenameItemPacket;)V
public handleSetBeaconPacket(Lnet/minecraft/network/protocol/game/ServerboundSetBeaconPacket;)V
public handleSetGameRule(Lnet/minecraft/network/protocol/game/ServerboundSetGameRulePacket;)V
private setGameRuleValue(Lnet/minecraft/world/level/gamerules/GameRules;Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/String;)V
private broadcastGameRuleChangeToOperators(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V
public handleSetStructureBlock(Lnet/minecraft/network/protocol/game/ServerboundSetStructureBlockPacket;)V
public handleSetTestBlock(Lnet/minecraft/network/protocol/game/ServerboundSetTestBlockPacket;)V
public handleTestInstanceBlockAction(Lnet/minecraft/network/protocol/game/ServerboundTestInstanceBlockActionPacket;)V
public handleSetJigsawBlock(Lnet/minecraft/network/protocol/game/ServerboundSetJigsawBlockPacket;)V
public handleJigsawGenerate(Lnet/minecraft/network/protocol/game/ServerboundJigsawGeneratePacket;)V
public handleSelectTrade(Lnet/minecraft/network/protocol/game/ServerboundSelectTradePacket;)V
public handleEditBook(Lnet/minecraft/network/protocol/game/ServerboundEditBookPacket;)V
private updateBookContents(Ljava/util/List;I)V
private signBook(Lnet/minecraft/server/network/FilteredText;Ljava/util/List;I)V
private filterableFromOutgoing(Lnet/minecraft/server/network/FilteredText;)Lnet/minecraft/server/network/Filterable;
public handleEntityTagQuery(Lnet/minecraft/network/protocol/game/ServerboundEntityTagQueryPacket;)V
public handleContainerSlotStateChanged(Lnet/minecraft/network/protocol/game/ServerboundContainerSlotStateChangedPacket;)V
public handleBlockEntityTagQuery(Lnet/minecraft/network/protocol/game/ServerboundBlockEntityTagQueryPacket;)V
public handleMovePlayer(Lnet/minecraft/network/protocol/game/ServerboundMovePlayerPacket;)V
private handlePlayerPositionChange(DDDFFZZ)V
private shouldCheckPlayerMovement(Z)Z
private forceSendPlayerSupportBlocks()V
private updateAwaitingTeleport()Z
private isEntityCollidingWithAnythingNew(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;DDD)Z
public teleport(DDDFF)V
public teleport(Lnet/minecraft/world/entity/PositionMoveRotation;Ljava/util/Set;)V
public handlePlayerAction(Lnet/minecraft/network/protocol/game/ServerboundPlayerActionPacket;)V
private static wasBlockPlacementAttempt(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/item/ItemStack;)Z
public handleUseItemOn(Lnet/minecraft/network/protocol/game/ServerboundUseItemOnPacket;)V
public handleUseItem(Lnet/minecraft/network/protocol/game/ServerboundUseItemPacket;)V
public handleTeleportToEntityPacket(Lnet/minecraft/network/protocol/game/ServerboundTeleportToEntityPacket;)V
public handlePaddleBoat(Lnet/minecraft/network/protocol/game/ServerboundPaddleBoatPacket;)V
public onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V
private removePlayerFromWorld()V
public ackBlockChangesUpTo(I)V
public handleSetCarriedItem(Lnet/minecraft/network/protocol/game/ServerboundSetCarriedItemPacket;)V
public handleChat(Lnet/minecraft/network/protocol/game/ServerboundChatPacket;)V
public handleChatCommand(Lnet/minecraft/network/protocol/game/ServerboundChatCommandPacket;)V
private performUnsignedChatCommand(Ljava/lang/String;)V
public handleSignedChatCommand(Lnet/minecraft/network/protocol/game/ServerboundChatCommandSignedPacket;)V
private performSignedChatCommand(Lnet/minecraft/network/protocol/game/ServerboundChatCommandSignedPacket;Lnet/minecraft/network/chat/LastSeenMessages;)V
private handleMessageDecodeFailure(Lnet/minecraft/network/chat/SignedMessageChain$DecodeException;)V
private collectSignedArguments(Lnet/minecraft/network/protocol/game/ServerboundChatCommandSignedPacket;Lnet/minecraft/network/chat/SignableCommand;Lnet/minecraft/network/chat/LastSeenMessages;)Ljava/util/Map;
private collectUnsignedArguments(Ljava/util/List;)Ljava/util/Map;
private static createSignedArgumentMismatchException(Ljava/lang/String;Ljava/util/List;Ljava/util/List;)Lnet/minecraft/network/chat/SignedMessageChain$DecodeException;
private parseCommand(Ljava/lang/String;)Lcom/mojang/brigadier/ParseResults;
private tryHandleChat(Ljava/lang/String;ZLjava/lang/Runnable;)V
private unpackAndApplyLastSeen(Lnet/minecraft/network/chat/LastSeenMessages$Update;)Ljava/util/Optional;
private static isChatMessageIllegal(Ljava/lang/String;)Z
private getSignedMessage(Lnet/minecraft/network/protocol/game/ServerboundChatPacket;Lnet/minecraft/network/chat/LastSeenMessages;)Lnet/minecraft/network/chat/PlayerChatMessage;
private broadcastChatMessage(Lnet/minecraft/network/chat/PlayerChatMessage;)V
private detectRateSpam(Lnet/minecraft/util/TickThrottler;)V
private detectCommandRateSpam()V
private detectChatRateSpam()V
public handleChatAck(Lnet/minecraft/network/protocol/game/ServerboundChatAckPacket;)V
public handlePunch(Lnet/minecraft/network/protocol/game/ServerboundPunchPacket;)V
public handlePlayerCommand(Lnet/minecraft/network/protocol/game/ServerboundPlayerCommandPacket;)V
public sendPlayerChatMessage(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/network/chat/ChatType$Bound;)V
public sendDisguisedChatMessage(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/ChatType$Bound;)V
public getRemoteAddress()Ljava/net/SocketAddress;
public switchToConfig()V
public handlePingRequest(Lnet/minecraft/network/protocol/ping/ServerboundPingRequestPacket;)V
public handleAttack(Lnet/minecraft/network/protocol/game/ServerboundAttackPacket;)V
public handleInteract(Lnet/minecraft/network/protocol/game/ServerboundInteractPacket;)V
public handleSpectatorAction(Lnet/minecraft/network/protocol/game/ServerboundSpectatorActionPacket;)V
public handleClientCommand(Lnet/minecraft/network/protocol/game/ServerboundClientCommandPacket;)V
private sendGameRuleValues()V
private static addGameRuleValue(Lnet/minecraft/world/level/gamerules/GameRules;Ljava/util/Map;Lnet/minecraft/world/level/gamerules/GameRule;)V
public handleContainerClose(Lnet/minecraft/network/protocol/game/ServerboundContainerClosePacket;)V
public handleContainerClick(Lnet/minecraft/network/protocol/game/ServerboundContainerClickPacket;)V
public handlePlaceRecipe(Lnet/minecraft/network/protocol/game/ServerboundPlaceRecipePacket;)V
public handleContainerButtonClick(Lnet/minecraft/network/protocol/game/ServerboundContainerButtonClickPacket;)V
public handleSetCreativeModeSlot(Lnet/minecraft/network/protocol/game/ServerboundSetCreativeModeSlotPacket;)V
public handleSignUpdate(Lnet/minecraft/network/protocol/game/ServerboundSignUpdatePacket;)V
private updateSignText(Lnet/minecraft/network/protocol/game/ServerboundSignUpdatePacket;Ljava/util/List;)V
public handlePlayerAbilities(Lnet/minecraft/network/protocol/game/ServerboundPlayerAbilitiesPacket;)V
public handleClientInformation(Lnet/minecraft/network/protocol/common/ServerboundClientInformationPacket;)V
public handleChangeDifficulty(Lnet/minecraft/network/protocol/game/ServerboundChangeDifficultyPacket;)V
public handleChangeGameMode(Lnet/minecraft/network/protocol/game/ServerboundChangeGameModePacket;)V
public handleLockDifficulty(Lnet/minecraft/network/protocol/game/ServerboundLockDifficultyPacket;)V
public handleChatSessionUpdate(Lnet/minecraft/network/protocol/game/ServerboundChatSessionUpdatePacket;)V
public handleConfigurationAcknowledged(Lnet/minecraft/network/protocol/game/ServerboundConfigurationAcknowledgedPacket;)V
public handleChunkBatchReceived(Lnet/minecraft/network/protocol/game/ServerboundChunkBatchReceivedPacket;)V
public handleDebugSubscriptionRequest(Lnet/minecraft/network/protocol/game/ServerboundDebugSubscriptionRequestPacket;)V
private resetPlayerChatState(Lnet/minecraft/network/chat/RemoteChatSession;)V
public handleCustomPayload(Lnet/minecraft/network/protocol/common/ServerboundCustomPayloadPacket;)V
public handleClientTickEnd(Lnet/minecraft/network/protocol/game/ServerboundClientTickEndPacket;)V
private handlePlayerKnownMovement(Lnet/minecraft/world/phys/Vec3;)V
public hasInfiniteMaterials()Z
public canUseCommandBlocks()Z
public getPlayer()Lnet/minecraft/server/level/ServerPlayer;
public hasClientLoaded()Z
public tickClientLoadTimeout()V
private markClientLoaded()V
public markClientUnloadedAfterDeath()V
private restartClientLoadTimerAfterRespawn()V
private synthetic lambda$resetPlayerChatState$0(Lnet/minecraft/network/chat/RemoteChatSession;)V
private synthetic lambda$handleSignUpdate$0(Lnet/minecraft/network/protocol/game/ServerboundSignUpdatePacket;Ljava/util/List;)V
private static synthetic lambda$addGameRuleValue$0(Ljava/util/Map;Lnet/minecraft/world/level/gamerules/GameRule;Lnet/minecraft/world/level/gamerules/GameRules;Lnet/minecraft/resources/ResourceKey;)V
private static synthetic lambda$sendGameRuleValues$0(Lnet/minecraft/world/level/gamerules/GameRules;Ljava/util/Map;Lnet/minecraft/world/level/gamerules/GameRule;)V
private synthetic lambda$performSignedChatCommand$0(Lnet/minecraft/commands/CommandSigningContext;Lnet/minecraft/commands/CommandSourceStack;)Lnet/minecraft/commands/CommandSourceStack;
private synthetic lambda$handleSignedChatCommand$0(Lnet/minecraft/network/protocol/game/ServerboundChatCommandSignedPacket;Ljava/util/Optional;)V
private synthetic lambda$handleChatCommand$0(Lnet/minecraft/network/protocol/game/ServerboundChatCommandPacket;)V
private synthetic lambda$handleChat$0(Lnet/minecraft/network/protocol/game/ServerboundChatPacket;Ljava/util/Optional;)V
private synthetic lambda$handleChat$1(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/network/chat/Component;Lnet/minecraft/server/network/FilteredText;)V
private synthetic lambda$forceSendPlayerSupportBlocks$0(Lnet/minecraft/core/BlockPos;)V
private synthetic lambda$signBook$0(Lnet/minecraft/server/network/FilteredText;)Lnet/minecraft/server/network/Filterable;
private synthetic lambda$handleEditBook$1(ILjava/util/List;)V
private synthetic lambda$handleEditBook$0(ILjava/util/List;)V
private synthetic lambda$handleTestInstanceBlockAction$0(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
private static synthetic lambda$broadcastGameRuleChangeToOperators$1(Lnet/minecraft/network/chat/Component;Lnet/minecraft/server/level/ServerPlayer;)V
private static synthetic lambda$broadcastGameRuleChangeToOperators$0(Lnet/minecraft/server/players/PlayerList;Lnet/minecraft/server/level/ServerPlayer;)Z
private synthetic lambda$setGameRuleValue$0(Lnet/minecraft/world/level/gamerules/GameRules;Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V
private synthetic lambda$handleCustomCommandSuggestions$0(ILcom/mojang/brigadier/suggestion/Suggestions;)V
private synthetic lambda$filterTextPacket$0(Ljava/lang/Object;)Ljava/lang/Object;
static <clinit>()V
```
