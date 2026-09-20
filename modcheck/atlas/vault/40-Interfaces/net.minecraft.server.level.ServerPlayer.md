---
type: "interface"
fqcn: "net.minecraft.server.level.ServerPlayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerPlayer

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`class` public; extends `net/minecraft/world/entity/player/Player`; implements `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/server/level/Ser` | exact | invokespecial@10 in `FakePlayer.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `closeContainer` | `()V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | declared |
| calls | `commandSource` | `()Lnet/minecraft/commands/CommandSource;` | exact | invokevirtual@97 in `EntityPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `distanceToSqr` | `(DDD)D` | inherited_exact | invokevirtual@16 in `PlayerLookup.lambda$around$1` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `distanceToSqr` | `(Lnet/minecraft/world/phys/Vec3;)D` | inherited_exact | invokevirtual@2 in `PlayerLookup.lambda$around$0` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getDisplayName` | `()Lnet/minecraft/network/chat/Component;` | inherited_exact | invokevirtual@9 in `ServerPlayNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getGameProfile` | `()Lcom/mojang/authlib/GameProfile;` | inherited_exact | invokevirtual@13 in `FakePlayerPacketListener.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@9 in `AttachmentSync.lambda$onInitialize$2` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@36 in `AttachmentSync.lambda$onInitialize$2` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | declared |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@29 in `PlayerListMixin.afterRespawn` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@34 in `PlayerListMixin.afterRespawn` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@52 in `PlayerListMixin.afterRespawn` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@57 in `PlayerListMixin.afterRespawn` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@2 in `FakePlayerPacketListener.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@28 in `PlayerMixin.onPlayerInteractEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@4 in `ServerGamePacketListenerImplMixin.handleInteract` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@7 in `ServerPlayNetworkAddon.schedule` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@33 in `RecipeSyncImpl.sendRecipes` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `calculateGameModeForNewPlayer` | `(Lnet/minecraft/world/level/GameType;)Lnet/minecraft/world/level/GameT` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `checkFallDamage` | `(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/co` | name_only | @ModifyExpressionValue at ['NEW'] | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| injects_into | `die` | `(Lnet/minecraft/world/damagesource/DamageSource;)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `die` | `(Lnet/minecraft/world/damagesource/DamageSource;)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `openMenu` | `(Lnet/minecraft/world/MenuProvider;)Ljava/util/OptionalInt;` | exact | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| injects_into | `restoreFrom` | `?` | ambiguous | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `triggerDimensionChangeTriggers` | `(Lnet/minecraft/server/level/ServerLevel;)V` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@1 in `AttachmentSync.trySync` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@9 in `AttachmentSync.trySync` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@25 in `AttachmentSync.trySync` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@40 in `InteractionEventsRouter.lambda$onInitialize$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@49 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@112 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@6 in `ServerPlayNetworking$Context.packetContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@8 in `ServerPlayNetworking.getReceived` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@8 in `ServerPlayNetworking.getSendable` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@8 in `ServerPlayNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@8 in `ServerPlayNetworking.canSend` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@8 in `ServerPlayNetworking.getSender` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@39 in `ServerPlayNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@8 in `ServerPlayNetworking.reconfigure` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@1 in `PlayerListMixin.handlePlayerConnection` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/server/network/ServerGamePacketListenerImpl;` | exact | getfield@12 in `RecipeSyncImpl.sendRecipes` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `containerCounter` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | declared |
| wraps | `openMenu` | `(Lnet/minecraft/world/MenuProvider;)Ljava/util/OptionalInt;` | exact | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| wraps | `openMenu` | `(Lnet/minecraft/world/MenuProvider;)Ljava/util/OptionalInt;` | exact | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| wraps | `startSleepInBed` | `(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/worl` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `startSleepInBed` | `(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/worl` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `startSleepInBed` | `(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/worl` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (73 fields, 216 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final NEUTRAL_MOB_DEATH_NOTIFICATION_RADII_XZ : I
private static final NEUTRAL_MOB_DEATH_NOTIFICATION_RADII_Y : I
private static final FLY_STAT_RECORDING_SPEED : I
public static final BLOCK_INTERACTION_DISTANCE_VERIFICATION_BUFFER : D
public static final ENTITY_INTERACTION_DISTANCE_VERIFICATION_BUFFER : D
public static final ENDER_PEARL_TICKET_RADIUS : I
public static final ENDER_PEARLS_TAG : Ljava/lang/String;
public static final ENDER_PEARL_DIMENSION_TAG : Ljava/lang/String;
public static final TAG_DIMENSION : Ljava/lang/String;
private static final CREATIVE_BLOCK_INTERACTION_RANGE_MODIFIER : Lnet/minecraft/world/entity/ai/attributes/AttributeModifier;
private static final CREATIVE_ENTITY_INTERACTION_RANGE_MODIFIER : Lnet/minecraft/world/entity/ai/attributes/AttributeModifier;
private static final SPAWN_SET_MESSAGE : Lnet/minecraft/network/chat/Component;
private static final WAYPOINT_TRANSMIT_RANGE_CROUCH_MODIFIER : Lnet/minecraft/world/entity/ai/attributes/AttributeModifier;
private static final DEFAULT_SEEN_CREDITS : Z
private static final DEFAULT_SPAWN_EXTRA_PARTICLES_ON_FALL : Z
public connection : Lnet/minecraft/server/network/ServerGamePacketListenerImpl;
private final server : Lnet/minecraft/server/MinecraftServer;
public final gameMode : Lnet/minecraft/server/level/ServerPlayerGameMode;
private final advancements : Lnet/minecraft/server/PlayerAdvancements;
private final stats : Lnet/minecraft/stats/ServerStatsCounter;
private lastRecordedHealthAndAbsorption : F
private lastRecordedFoodLevel : I
private lastRecordedAirLevel : I
private lastRecordedArmor : I
private lastRecordedLevel : I
private lastRecordedExperience : I
private lastSentHealth : F
private lastSentFood : I
private lastFoodSaturationZero : Z
private lastSentExp : I
private chatVisibility : Lnet/minecraft/world/entity/player/ChatVisiblity;
private particleStatus : Lnet/minecraft/server/level/ParticleStatus;
private canChatColor : Z
private lastActionTime : J
private camera : Lnet/minecraft/world/entity/Entity;
private isChangingDimension : Z
public seenCredits : Z
private final recipeBook : Lnet/minecraft/stats/ServerRecipeBook;
private levitationStartPos : Lnet/minecraft/world/phys/Vec3;
private levitationStartTime : I
private disconnected : Z
private requestedViewDistance : I
private language : Ljava/lang/String;
private startingToFallPosition : Lnet/minecraft/world/phys/Vec3;
private enteredNetherPosition : Lnet/minecraft/world/phys/Vec3;
private enteredLavaOnVehiclePosition : Lnet/minecraft/world/phys/Vec3;
private currentExplosionImpactPos : Lnet/minecraft/world/phys/Vec3;
private currentExplosionCause : Lnet/minecraft/world/entity/Entity;
private lastSectionPos : Lnet/minecraft/core/SectionPos;
private chunkTrackingView : Lnet/minecraft/server/level/ChunkTrackingView;
private respawnConfig : Lnet/minecraft/server/level/ServerPlayer$RespawnConfig;
private final textFilter : Lnet/minecraft/server/network/TextFilter;
private textFilteringEnabled : Z
private allowsListing : Z
private spawnExtraParticlesOnFall : Z
private wardenSpawnTracker : Lnet/minecraft/world/entity/monster/warden/WardenSpawnTracker;
private raidOmenPosition : Lnet/minecraft/core/BlockPos;
private lastKnownClientMovement : Lnet/minecraft/world/phys/Vec3;
private lastClientInput : Lnet/minecraft/world/entity/player/Input;
private final enderPearls : Ljava/util/Set;
private postEffectsDirty : Z
private timeEntitySatOnShoulder : J
private shoulderEntityLeft : Lnet/minecraft/nbt/CompoundTag;
private shoulderEntityRight : Lnet/minecraft/nbt/CompoundTag;
private final containerSynchronizer : Lnet/minecraft/world/inventory/ContainerSynchronizer;
private final containerListener : Lnet/minecraft/world/inventory/ContainerListener;
private chatSession : Lnet/minecraft/network/chat/RemoteChatSession;
public final object : Ljava/lang/Object;
private final commandSource : Lnet/minecraft/commands/CommandSource;
private requestedDebugSubscriptions : Ljava/util/Set;
private containerCounter : I
public wonGame : Z
public <init>(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/server/level/ServerLevel;Lcom/mojang/authlib/GameProfile;Lnet/minecraft/server/level/ClientInformation;)V
public adjustSpawnLocation(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
protected readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V
protected addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V
private saveParentVehicle(Lnet/minecraft/world/level/storage/ValueOutput;)V
public loadAndSpawnParentVehicle(Lnet/minecraft/world/level/storage/ValueInput;)V
private saveEnderPearls(Lnet/minecraft/world/level/storage/ValueOutput;)V
public loadAndSpawnEnderPearls(Lnet/minecraft/world/level/storage/ValueInput;)V
private loadAndSpawnEnderPearl(Lnet/minecraft/world/level/storage/ValueInput;)V
public setExperiencePoints(I)V
public setExperienceLevels(I)V
public giveExperienceLevels(I)V
public onEnchantmentPerformed(Lnet/minecraft/world/item/ItemStack;I)V
private initMenu(Lnet/minecraft/world/inventory/AbstractContainerMenu;)V
public initInventoryMenu()V
public onEnterCombat()V
public onLeaveCombat()V
public onInsideBlock(Lnet/minecraft/world/level/block/state/BlockState;)V
protected createItemCooldowns()Lnet/minecraft/world/item/ItemCooldowns;
public tick()V
private updatePlayerAttributes()V
public doTick()V
private synchronizeSpecialItemUpdates(Lnet/minecraft/world/item/ItemStack;)V
protected tickRegeneration()V
public handleShoulderEntities()V
private playShoulderEntityAmbientSound(Lnet/minecraft/nbt/CompoundTag;)V
public setEntityOnShoulder(Lnet/minecraft/nbt/CompoundTag;)Z
protected removeEntitiesOnShoulder()V
private respawnEntityOnShoulder(Lnet/minecraft/nbt/CompoundTag;)V
public resetFallDistance()V
public trackStartFallingPosition()V
public trackEnteredOrExitedLavaOnVehicle()V
private updateScoreForCriteria(Lnet/minecraft/world/scores/criteria/ObjectiveCriteria;I)V
public die(Lnet/minecraft/world/damagesource/DamageSource;)V
private tellNeutralMobsThatIDied()V
public awardKillScore(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)V
private handleTeamKill(Lnet/minecraft/world/scores/ScoreHolder;Lnet/minecraft/world/scores/ScoreHolder;Ljava/util/Map;)V
public hurtServer(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)Z
public canHarmPlayer(Lnet/minecraft/world/entity/player/Player;)Z
private isPvpAllowed()Z
public findRespawnPositionAndUseSpawnBlock(ZLnet/minecraft/world/level/portal/TeleportTransition$PostTeleportTransition;)Lnet/minecraft/world/level/portal/TeleportTransition;
public isReceivingWaypoints()Z
protected onAttributeUpdated(Lnet/minecraft/core/Holder;)V
public sendPostEffects()V
public addPostEffect(Lnet/minecraft/resources/Identifier;)Z
public clearPostEffects()Z
public getPostEffects()Ljava/util/List;
public removePostEffect(Lnet/minecraft/resources/Identifier;)Z
private static findRespawnAndUseSpawnBlock(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/level/ServerPlayer$RespawnConfig;Z)Ljava/util/Optional;
public showEndCredits()V
public teleport(Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/server/level/ServerPlayer;
public forceSetRotation(FZFZ)V
private triggerDimensionChangeTriggers(Lnet/minecraft/server/level/ServerLevel;)V
public broadcastToPlayer(Lnet/minecraft/server/level/ServerPlayer;)Z
public take(Lnet/minecraft/world/entity/Entity;I)V
public startSleepInBed(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/attribute/BedRule;Lnet/minecraft/core/BlockPos;)Lcom/mojang/datafixers/util/Either;
public startSleeping(Lnet/minecraft/core/BlockPos;)Z
private bedInRange(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z
private isReachableBedBlock(Lnet/minecraft/core/BlockPos;)Z
private bedBlocked(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z
public stopSleepInBed(ZZ)V
public isInvulnerableTo(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;)Z
protected onChangedBlock(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;)V
protected checkFallDamage(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V
public onExplosionHit(Lnet/minecraft/world/entity/Entity;)V
protected pushEntities()V
public openTextEdit(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraft/world/level/block/entity/SignTextSlot;)V
public openDialog(Lnet/minecraft/core/Holder;)V
private nextContainerCounter()V
public openMenu(Lnet/minecraft/world/MenuProvider;)Ljava/util/OptionalInt;
public sendMerchantOffers(ILnet/minecraft/world/item/trading/MerchantOffers;IIZZ)V
public openHorseInventory(Lnet/minecraft/world/entity/animal/equine/AbstractHorse;Lnet/minecraft/world/Container;)V
public openNautilusInventory(Lnet/minecraft/world/entity/animal/nautilus/AbstractNautilus;Lnet/minecraft/world/Container;)V
public openItemGui(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/InteractionHand;)V
public openCommandBlock(Lnet/minecraft/world/level/block/entity/CommandBlockEntity;)V
public closeContainer()V
public doCloseContainer()V
public rideTick()V
public checkMovementStatistics(DDD)V
private checkRidingStatistics(DDD)V
private static didNotMove(DDD)Z
public awardStat(Lnet/minecraft/stats/Stat;I)V
public resetStat(Lnet/minecraft/stats/Stat;)V
public awardRecipes(Ljava/util/Collection;)I
public triggerRecipeCrafted(Lnet/minecraft/world/item/crafting/RecipeHolder;Ljava/util/List;)V
public awardRecipesByKey(Ljava/util/List;)V
public resetRecipes(Ljava/util/Collection;)I
public jumpFromGround()V
public giveExperiencePoints(I)V
public disconnect()V
public hasDisconnected()Z
public resetSentInfo()V
protected completeUsingItem()V
public lookAt(Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;Lnet/minecraft/world/phys/Vec3;)V
public lookAt(Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;)V
public restoreFrom(Lnet/minecraft/server/level/ServerPlayer;Z)V
private transferInventoryXpAndScore(Lnet/minecraft/world/entity/player/Player;)V
protected onEffectAdded(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/entity/Entity;)V
protected onEffectUpdated(Lnet/minecraft/world/effect/MobEffectInstance;ZLnet/minecraft/world/entity/Entity;)V
protected onEffectsRemoved(Ljava/util/Collection;)V
public teleportTo(DDD)V
public teleportRelative(DDD)V
public teleportTo(Lnet/minecraft/server/level/ServerLevel;DDDLjava/util/Set;FFZ)Z
public snapTo(DDD)V
public crit(Lnet/minecraft/world/entity/Entity;)V
public magicCrit(Lnet/minecraft/world/entity/Entity;)V
public onUpdateAbilities()V
public level()Lnet/minecraft/server/level/ServerLevel;
public setGameMode(Lnet/minecraft/world/level/GameType;)Z
public gameMode()Lnet/minecraft/world/level/GameType;
public commandSource()Lnet/minecraft/commands/CommandSource;
public createCommandSourceStack()Lnet/minecraft/commands/CommandSourceStack;
public sendSystemMessage(Lnet/minecraft/network/chat/Component;)V
public sendOverlayMessage(Lnet/minecraft/network/chat/Component;)V
public sendBuildLimitMessage(ZI)V
public sendSpawnProtectionMessage(Lnet/minecraft/core/BlockPos;)V
public sendSystemMessage(Lnet/minecraft/network/chat/Component;Z)V
public sendChatMessage(Lnet/minecraft/network/chat/OutgoingChatMessage;ZLnet/minecraft/network/chat/ChatType$Bound;)V
public getIpAddress()Ljava/lang/String;
public updateOptions(Lnet/minecraft/server/level/ClientInformation;)V
public clientInformation()Lnet/minecraft/server/level/ClientInformation;
public canChatInColor()Z
public getChatVisibility()Lnet/minecraft/world/entity/player/ChatVisiblity;
private acceptsSystemMessages(Z)Z
private acceptsChatMessages()Z
public requestedViewDistance()I
public sendServerStatus(Lnet/minecraft/network/protocol/status/ServerStatus;)V
public permissions()Lnet/minecraft/server/permissions/PermissionSet;
public resetLastActionTime()V
public getStats()Lnet/minecraft/stats/ServerStatsCounter;
public getRecipeBook()Lnet/minecraft/stats/ServerRecipeBook;
protected updateInvisibilityStatus()V
public getCamera()Lnet/minecraft/world/entity/Entity;
public setCamera(Lnet/minecraft/world/entity/Entity;)V
protected processPortalCooldown()V
public getLastActionTime()J
public getTabListDisplayName()Lnet/minecraft/network/chat/Component;
public getTabListOrder()I
public isChangingDimension()Z
public hasChangedDimension()V
public getAdvancements()Lnet/minecraft/server/PlayerAdvancements;
public getRespawnConfig()Lnet/minecraft/server/level/ServerPlayer$RespawnConfig;
public copyRespawnPosition(Lnet/minecraft/server/level/ServerPlayer;)V
public setRespawnPosition(Lnet/minecraft/server/level/ServerPlayer$RespawnConfig;Z)V
public getLastSectionPos()Lnet/minecraft/core/SectionPos;
public setLastSectionPos(Lnet/minecraft/core/SectionPos;)V
public getChunkTrackingView()Lnet/minecraft/server/level/ChunkTrackingView;
public setChunkTrackingView(Lnet/minecraft/server/level/ChunkTrackingView;)V
public drop(Lnet/minecraft/world/item/ItemStack;ZLnet/minecraft/util/Prediction;)Lnet/minecraft/world/entity/item/ItemEntity;
public getTextFilter()Lnet/minecraft/server/network/TextFilter;
public setServerLevel(Lnet/minecraft/server/level/ServerLevel;)V
private static readPlayerMode(Lnet/minecraft/world/level/storage/ValueInput;Ljava/lang/String;)Lnet/minecraft/world/level/GameType;
private calculateGameModeForNewPlayer(Lnet/minecraft/world/level/GameType;)Lnet/minecraft/world/level/GameType;
private storeGameTypes(Lnet/minecraft/world/level/storage/ValueOutput;)V
public isTextFilteringEnabled()Z
public shouldFilterMessageTo(Lnet/minecraft/server/level/ServerPlayer;)Z
public mayInteract(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;)Z
protected updateUsingItem(Lnet/minecraft/world/item/ItemStack;)V
public drop(Z)V
public handleExtraItemsCreatedOnUse(Lnet/minecraft/world/item/ItemStack;)V
public allowsListing()Z
public getWardenSpawnTracker()Lnet/minecraft/world/entity/monster/warden/WardenSpawnTracker;
public setSpawnExtraParticlesOnFall(Z)V
public onItemPickup(Lnet/minecraft/world/entity/item/ItemEntity;)V
public setChatSession(Lnet/minecraft/network/chat/RemoteChatSession;)V
public getChatSession()Lnet/minecraft/network/chat/RemoteChatSession;
public indicateDamage(DD)V
public startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z
public removeVehicle()V
public createCommonSpawnInfo(Lnet/minecraft/server/level/ServerLevel;)Lnet/minecraft/network/protocol/game/CommonPlayerSpawnInfo;
public setRaidOmenPosition(Lnet/minecraft/core/BlockPos;)V
public clearRaidOmenPosition()V
public getRaidOmenPosition()Lnet/minecraft/core/BlockPos;
public getKnownMovement()Lnet/minecraft/world/phys/Vec3;
public getKnownSpeed()Lnet/minecraft/world/phys/Vec3;
public setKnownMovement(Lnet/minecraft/world/phys/Vec3;)V
protected getEnchantedDamage(Lnet/minecraft/world/entity/Entity;FLnet/minecraft/world/damagesource/DamageSource;)F
public onEquippedItemBroken(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;)V
public getLastClientInput()Lnet/minecraft/world/entity/player/Input;
public setLastClientInput(Lnet/minecraft/world/entity/player/Input;)V
public getLastClientMoveIntent()Lnet/minecraft/world/phys/Vec3;
public registerEnderPearl(Lnet/minecraft/world/entity/projectile/throwableitemprojectile/ThrownEnderpearl;)V
public deregisterEnderPearl(Lnet/minecraft/world/entity/projectile/throwableitemprojectile/ThrownEnderpearl;)V
public getEnderPearls()Ljava/util/Set;
public getShoulderEntityLeft()Lnet/minecraft/nbt/CompoundTag;
protected setShoulderEntityLeft(Lnet/minecraft/nbt/CompoundTag;)V
public getShoulderEntityRight()Lnet/minecraft/nbt/CompoundTag;
protected setShoulderEntityRight(Lnet/minecraft/nbt/CompoundTag;)V
public registerAndUpdateEnderPearlTicket(Lnet/minecraft/world/entity/projectile/throwableitemprojectile/ThrownEnderpearl;)J
public static placeEnderPearlTicket(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/ChunkPos;)J
public requestDebugSubscriptions(Ljava/util/Set;)V
public debugSubscriptions()Ljava/util/Set;
public swingAndResetAttackStrength(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/component/SwingAnimation;Z)V
public synthetic level()Lnet/minecraft/world/level/Level;
public synthetic teleport(Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/world/entity/Entity;
private synthetic lambda$drop$0(Lnet/minecraft/world/entity/player/Inventory;I)V
private synthetic lambda$sendSystemMessage$0(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/protocol/Packet;
private synthetic lambda$awardRecipesByKey$0(Lnet/minecraft/resources/ResourceKey;)Ljava/util/stream/Stream;
private static synthetic lambda$awardStat$0(ILnet/minecraft/world/scores/ScoreAccess;)V
private synthetic lambda$startSleepInBed$1(Lnet/minecraft/world/level/block/AbstractBedBlock;ZLnet/minecraft/util/Unit;)V
private synthetic lambda$startSleepInBed$0(Lnet/minecraft/world/entity/monster/Monster;)Z
private static synthetic lambda$findRespawnAndUseSpawnBlock$1(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/server/level/ServerPlayer$RespawnPosAngle;
private static synthetic lambda$findRespawnAndUseSpawnBlock$0(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/server/level/ServerPlayer$RespawnPosAngle;
private synthetic lambda$tellNeutralMobsThatIDied$1(Lnet/minecraft/world/entity/Mob;)V
private static synthetic lambda$tellNeutralMobsThatIDied$0(Lnet/minecraft/world/entity/Mob;)Z
private synthetic lambda$die$0(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/protocol/Packet;
private static synthetic lambda$die$1(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private static synthetic lambda$updateScoreForCriteria$0(ILnet/minecraft/world/scores/ScoreAccess;)V
private synthetic lambda$respawnEntityOnShoulder$1(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;)V
private static synthetic lambda$respawnEntityOnShoulder$0()Ljava/lang/String;
private static synthetic lambda$loadAndSpawnEnderPearl$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/entity/Entity;
private static synthetic lambda$loadAndSpawnParentVehicle$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/entity/Entity;
private synthetic lambda$readAdditionalSaveData$0(Lnet/minecraft/stats/ServerRecipeBook$Packed;)V
private synthetic lambda$readAdditionalSaveData$1(Lnet/minecraft/resources/ResourceKey;)Z
private static synthetic lambda$new$0(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/resources/ResourceKey;Ljava/util/function/Consumer;)V
static <clinit>()V
```
