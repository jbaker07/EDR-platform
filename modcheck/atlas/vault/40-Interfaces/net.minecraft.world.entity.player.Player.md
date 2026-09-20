---
type: "interface"
fqcn: "net.minecraft.world.entity.player.Player"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.player.Player

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`abstract_class` public abstract; extends `net/minecraft/world/entity/Avatar`; implements `net/minecraft/world/entity/ContainerUser`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/Level;Lcom/mojang/authlib/GameProfile;)V` | exact | invokespecial@3 in `ServerPlayerMixin.<init>` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `createItemStackToDrop` | `(Lnet/minecraft/world/item/ItemStack;ZZ)Lnet/minecraft/world/entity/it` | inherited_exact | invokevirtual@84 in `PlayerInventoryStorageImpl$DroppedStacks.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getAbsorptionAmount` | `()F` | exact | invokevirtual@48 in `HudStatusBarHeightRegistryImpl.lambda$static$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getAirSupply` | `()I` | inherited_exact | invokevirtual@6 in `HudStatusBarHeightRegistryImpl.lambda$static$5` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getArmorValue` | `()I` | inherited_exact | invokevirtual@1 in `HudStatusBarHeightRegistryImpl.lambda$static$2` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getAttributeValue` | `(Lnet/minecraft/core/Holder;)D` | inherited_exact | invokevirtual@32 in `HudStatusBarHeightRegistryImpl.lambda$static$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getDisplayName` | `()Lnet/minecraft/network/chat/Component;` | exact | invokevirtual@1 in `DebugMessages.forPlayer` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getEyeY` | `()D` | inherited_exact | invokevirtual@255 in `FluidStorageUtil.moveWithSound` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getHealth` | `()F` | inherited_exact | invokevirtual@8 in `HudStatusBarHeightRegistryImpl.lambda$static$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInventory` | `()Lnet/minecraft/world/entity/player/Inventory;` | exact | invokevirtual@108 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getInventory` | `()Lnet/minecraft/world/entity/player/Inventory;` | exact | invokevirtual@120 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getInventory` | `()Lnet/minecraft/world/entity/player/Inventory;` | exact | invokevirtual@1 in `PlayerInventoryStorage.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getItemInHand` | `(Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemS` | inherited_exact | invokevirtual@10 in `ContainerItemContext.forPlayerInteraction` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getItemInHand` | `(Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemS` | inherited_exact | invokevirtual@25 in `FluidStorageUtil.interactWithFluidStorage` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getMaxAirSupply` | `()I` | inherited_exact | invokevirtual@1 in `HudStatusBarHeightRegistryImpl.lambda$static$5` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getStringUUID` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@8 in `DebugMessages.forPlayer` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getX` | `()D` | inherited_exact | invokevirtual@251 in `FluidStorageUtil.moveWithSound` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getXRot` | `()F` | inherited_exact | invokevirtual@11 in `MultiPlayerGameModeMixin.lambda$interactItem$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getYRot` | `()F` | inherited_exact | invokevirtual@7 in `MultiPlayerGameModeMixin.lambda$interactItem$0` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getZ` | `()D` | inherited_exact | invokevirtual@259 in `FluidStorageUtil.moveWithSound` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `hasInfiniteMaterials` | `()Z` | exact | invokevirtual@1 in `ContainerItemContext.forPlayerInteraction` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isEyeInFluid` | `(Lnet/minecraft/tags/TagKey;)Z` | inherited_exact | invokevirtual@38 in `HudMixin.popTheBubbleForCustomFluids` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isEyeInFluid` | `(Lnet/minecraft/tags/TagKey;)Z` | inherited_exact | invokevirtual@20 in `HudStatusBarHeightRegistryImpl.lambda$static$5` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@11 in `MultiPlayerGameModeMixin.interactItem` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@40 in `MultiPlayerGameModeMixin.interactItem` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@11 in `MultiPlayerGameModeMixin.attackEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@246 in `FluidStorageUtil.moveWithSound` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@104 in `PlayerInventoryStorageImpl$DroppedStacks.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@18 in `PlayerInventoryStorageImpl.drop` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `permissions` | `()Lnet/minecraft/server/permissions/PermissionSet;` | exact | invokevirtual@18 in `EntityPermissionContext.permissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| injects_into | `attack` | `(Lnet/minecraft/world/entity/Entity;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `isSleepingLongEnough` | `()Z` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `startSleepInBed` | `(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/worl` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| writes | `containerMenu` | `Lnet/minecraft/world/inventory/AbstractContainerMenu;` | exact | putfield@144 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (44 fields, 227 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAX_HEALTH : I
public static final SLEEP_DURATION : I
public static final WAKE_UP_DURATION : I
public static final ENDER_SLOT_OFFSET : I
public static final HELD_ITEM_SLOT : I
public static final CRAFTING_SLOT_OFFSET : I
public static final DEFAULT_BLOCK_INTERACTION_RANGE : F
public static final DEFAULT_ENTITY_INTERACTION_RANGE : F
private static final DATA_PLAYER_ABSORPTION_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_SCORE_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_SHOULDER_PARROT_LEFT : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_SHOULDER_PARROT_RIGHT : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DEFAULT_SLEEP_TIMER : S
private static final DEFAULT_EXPERIENCE_PROGRESS : F
private static final DEFAULT_EXPERIENCE_LEVEL : I
private static final DEFAULT_TOTAL_EXPERIENCE : I
private static final NO_ENCHANTMENT_SEED : I
private static final DEFAULT_SELECTED_SLOT : I
private static final DEFAULT_SCORE : I
public static final CREATIVE_ENTITY_INTERACTION_RANGE_MODIFIER_VALUE : F
private final inventory : Lnet/minecraft/world/entity/player/Inventory;
protected enderChestInventory : Lnet/minecraft/world/inventory/PlayerEnderChestContainer;
public final inventoryMenu : Lnet/minecraft/world/inventory/InventoryMenu;
public containerMenu : Lnet/minecraft/world/inventory/AbstractContainerMenu;
protected foodData : Lnet/minecraft/world/food/FoodData;
protected jumpTriggerTime : I
public takeXpDelay : I
private sleepCounter : I
protected wasUnderwater : Z
private final abilities : Lnet/minecraft/world/entity/player/Abilities;
public experienceLevel : I
public totalExperience : I
public experienceProgress : F
protected enchantmentSeed : I
protected final defaultFlySpeed : F
private lastLevelUpTime : I
private final gameProfile : Lcom/mojang/authlib/GameProfile;
private reducedDebugInfo : Z
private lastItemInMainHand : Lnet/minecraft/world/item/ItemStack;
private final cooldowns : Lnet/minecraft/world/item/ItemCooldowns;
private lastDeathLocation : Ljava/util/Optional;
protected final postEffects : Ljava/util/List;
public fishing : Lnet/minecraft/world/entity/projectile/FishingHook;
protected hurtDir : F
public <init>(Lnet/minecraft/world/level/Level;Lcom/mojang/authlib/GameProfile;)V
protected createEquipment()Lnet/minecraft/world/entity/EntityEquipment;
public blockActionRestricted(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/GameType;)Z
public static createAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
protected defineSynchedData(Lnet/minecraft/network/syncher/SynchedEntityData$Builder;)V
public tick()V
protected getMaxHeadRotationRelativeToBody()F
public isSecondaryUseActive()Z
protected wantsToStopRiding()Z
protected isStayingOnGroundSurface()Z
protected updateIsUnderwater()Z
public onAboveBubbleColumn(ZLnet/minecraft/core/BlockPos;)V
public onInsideBubbleColumn(Z)V
private turtleHelmetTick()V
private isEquipped(Lnet/minecraft/world/item/Item;)Z
protected createItemCooldowns()Lnet/minecraft/world/item/ItemCooldowns;
protected updatePlayerPose()V
private getDesiredPose()Lnet/minecraft/world/entity/Pose;
protected canPlayerFitWithinBlocksAndEntitiesWhen(Lnet/minecraft/world/entity/Pose;)Z
protected getSwimSound()Lnet/minecraft/sounds/SoundEvent;
protected getSwimSplashSound()Lnet/minecraft/sounds/SoundEvent;
protected getSwimHighSpeedSplashSound()Lnet/minecraft/sounds/SoundEvent;
public getDimensionChangingDelay()I
public playSound(Lnet/minecraft/sounds/SoundEvent;FF)V
public getSoundSource()Lnet/minecraft/sounds/SoundSource;
protected getFireImmuneTicks()I
public handleEntityEvent(B)V
protected closeContainer()V
protected doCloseContainer()V
public rideTick()V
public aiStep()V
protected tickRegeneration()V
public handleShoulderEntities()V
protected removeEntitiesOnShoulder()V
private touch(Lnet/minecraft/world/entity/Entity;)V
public getScore()I
public setScore(I)V
public increaseScore(I)V
public startAutoSpinAttack(IFLnet/minecraft/world/item/ItemStack;)V
public getWeaponItem()Lnet/minecraft/world/item/ItemStack;
public die(Lnet/minecraft/world/damagesource/DamageSource;)V
protected dropEquipment(Lnet/minecraft/server/level/ServerLevel;)V
protected destroyVanishingCursedItems()V
protected getHurtSound(Lnet/minecraft/world/damagesource/DamageSource;)Lnet/minecraft/sounds/SoundEvent;
protected getDeathSound()Lnet/minecraft/sounds/SoundEvent;
public handleCreativeModeItemDrop(Lnet/minecraft/world/item/ItemStack;)V
public getDestroySpeed(Lnet/minecraft/world/level/block/state/BlockState;)F
public hasCorrectToolForDrops(Lnet/minecraft/world/level/block/state/BlockState;)Z
protected readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V
protected addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V
public isInvulnerableTo(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;)Z
public hurtServer(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)Z
protected blockUsingItem(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;FZ)V
public canBeSeenAsEnemy()Z
public canHarmPlayer(Lnet/minecraft/world/entity/player/Player;)Z
protected hurtArmor(Lnet/minecraft/world/damagesource/DamageSource;F)V
protected hurtHelmet(Lnet/minecraft/world/damagesource/DamageSource;F)V
protected actuallyHurt(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)V
public isTextFilteringEnabled()Z
public openTextEdit(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraft/world/level/block/entity/SignTextSlot;)V
public openMinecartCommandBlock(Lnet/minecraft/world/entity/vehicle/minecart/MinecartCommandBlock;)V
public openCommandBlock(Lnet/minecraft/world/level/block/entity/CommandBlockEntity;)V
public openStructureBlock(Lnet/minecraft/world/level/block/entity/StructureBlockEntity;)V
public openTestBlock(Lnet/minecraft/world/level/block/entity/TestBlockEntity;)V
public openTestInstanceBlock(Lnet/minecraft/world/level/block/entity/TestInstanceBlockEntity;)V
public openJigsawBlock(Lnet/minecraft/world/level/block/entity/JigsawBlockEntity;)V
public openHorseInventory(Lnet/minecraft/world/entity/animal/equine/AbstractHorse;Lnet/minecraft/world/Container;)V
public openNautilusInventory(Lnet/minecraft/world/entity/animal/nautilus/AbstractNautilus;Lnet/minecraft/world/Container;)V
public openMenu(Lnet/minecraft/world/MenuProvider;)Ljava/util/OptionalInt;
public openDialog(Lnet/minecraft/core/Holder;)V
public sendMerchantOffers(ILnet/minecraft/world/item/trading/MerchantOffers;IIZZ)V
public openItemGui(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/InteractionHand;)V
public interactOn(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/InteractionResult;
public removeVehicle()V
protected isImmobile()Z
public isAffectedByFluids()Z
protected maybeBackOffFromEdge(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/MoverType;)Lnet/minecraft/world/phys/Vec3;
private isAboveGround(F)Z
private canFallAtLeast(DDD)Z
public attack(Lnet/minecraft/world/entity/Entity;)V
private playServerSideSound(Lnet/minecraft/sounds/SoundEvent;)V
private createAttackSource(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/damagesource/DamageSource;
private cannotAttack(Lnet/minecraft/world/entity/Entity;)Z
private deflectProjectile(Lnet/minecraft/world/entity/Entity;)Z
private canCriticalAttack(Lnet/minecraft/world/entity/Entity;)Z
private isSweepAttack(ZZZ)Z
private attackVisualEffects(Lnet/minecraft/world/entity/Entity;ZZZZF)V
private damageStatsAndHearts(Lnet/minecraft/world/entity/Entity;F)V
private itemAttackInteraction(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/damagesource/DamageSource;Z)V
public causeExtraKnockback(Lnet/minecraft/world/entity/Entity;FLnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/damagesource/DamageSource;FZ)V
public getVoicePitch()F
private doSweepAttack(Lnet/minecraft/world/entity/Entity;FLnet/minecraft/world/damagesource/DamageSource;F)V
protected getEnchantedDamage(Lnet/minecraft/world/entity/Entity;FLnet/minecraft/world/damagesource/DamageSource;)F
protected doAutoAttackOnTouch(Lnet/minecraft/world/entity/LivingEntity;)V
public crit(Lnet/minecraft/world/entity/Entity;)V
private baseDamageScaleFactor()F
public stabAttack(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/entity/Entity;FZZZ)Z
public magicCrit(Lnet/minecraft/world/entity/Entity;)V
public remove(Lnet/minecraft/world/entity/Entity$RemovalReason;)V
public isClientAuthoritative()Z
protected isLocalClientAuthoritative()Z
public isLocalPlayer()Z
public getMoveSimulationType()Lnet/minecraft/world/entity/MoveSimulationType;
public isEffectiveAi()Z
public getGameProfile()Lcom/mojang/authlib/GameProfile;
public nameAndId()Lnet/minecraft/server/players/NameAndId;
public getInventory()Lnet/minecraft/world/entity/player/Inventory;
public getAbilities()Lnet/minecraft/world/entity/player/Abilities;
public hasInfiniteMaterials()Z
public preventsBlockDrops()Z
public updateTutorialInventoryAction(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/inventory/ClickAction;)V
public hasContainerOpen()Z
public canDropItems()Z
public startSleepInBed(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/attribute/BedRule;Lnet/minecraft/core/BlockPos;)Lcom/mojang/datafixers/util/Either;
public stopSleepInBed(ZZ)V
public stopSleeping()V
public isSleepingLongEnough()Z
public getSleepTimer()I
public sendSystemMessage(Lnet/minecraft/network/chat/Component;)V
public sendOverlayMessage(Lnet/minecraft/network/chat/Component;)V
public awardStat(Lnet/minecraft/resources/Identifier;)V
public awardStat(Lnet/minecraft/resources/Identifier;I)V
public awardStat(Lnet/minecraft/stats/Stat;)V
public awardStat(Lnet/minecraft/stats/Stat;I)V
public resetStat(Lnet/minecraft/stats/Stat;)V
public awardRecipes(Ljava/util/Collection;)I
public triggerRecipeCrafted(Lnet/minecraft/world/item/crafting/RecipeHolder;Ljava/util/List;)V
public travel(Lnet/minecraft/world/phys/Vec3;)V
protected canGlide()Z
public updateSwimming()V
protected freeAt(Lnet/minecraft/core/BlockPos;)Z
public getSpeed()F
public causeFallDamage(DFLnet/minecraft/world/damagesource/DamageSource;)Z
public tryToStartFallFlying()Z
public startFallFlying()V
protected doWaterSplashEffect()V
protected playStepSound(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public getFallSounds()Lnet/minecraft/world/entity/LivingEntity$Fallsounds;
public killedEntity(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;)Z
public makeStuckInBlock(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/phys/Vec3;)V
public giveExperiencePoints(I)V
public getEnchantmentSeed()I
public onEnchantmentPerformed(Lnet/minecraft/world/item/ItemStack;I)V
public giveExperienceLevels(I)V
public getXpNeededForNextLevel()I
public causeFoodExhaustion(F)V
protected hasEnoughFoodToDoExhaustiveManoeuvres()Z
public getFoodData()Lnet/minecraft/world/food/FoodData;
public canEat(Z)Z
public isHurt()Z
public mayBuild()Z
public mayUseItemAt(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/world/item/ItemStack;)Z
protected getBaseExperienceReward(Lnet/minecraft/server/level/ServerLevel;)I
protected isAlwaysExperienceDropper()Z
public shouldShowName()Z
protected getMovementEmission()Lnet/minecraft/world/entity/Entity$MovementEmission;
public onUpdateAbilities()V
public getName()Lnet/minecraft/network/chat/Component;
public getPlainTextName()Ljava/lang/String;
public getEnderChestInventory()Lnet/minecraft/world/inventory/PlayerEnderChestContainer;
protected doesEmitEquipEvent(Lnet/minecraft/world/entity/EquipmentSlot;)Z
public addItem(Lnet/minecraft/world/item/ItemStack;)Z
public abstract gameMode()Lnet/minecraft/world/level/GameType;
public isSpectator()Z
public isPickable()Z
public isSwimming()Z
public isCreative()Z
public isPushedByFluid()Z
public getDisplayName()Lnet/minecraft/network/chat/Component;
private decorateDisplayNameComponent(Lnet/minecraft/network/chat/MutableComponent;)Lnet/minecraft/network/chat/MutableComponent;
public getScoreboardName()Ljava/lang/String;
protected internalSetAbsorptionAmount(F)V
public getAbsorptionAmount()F
public getSlot(I)Lnet/minecraft/world/entity/SlotAccess;
public isReducedDebugInfo()Z
public setReducedDebugInfo(Z)V
public setRemainingFireTicks(I)V
protected static extractParrotVariant(Lnet/minecraft/nbt/CompoundTag;)Ljava/util/Optional;
protected static convertParrotVariant(Ljava/util/Optional;)Ljava/util/OptionalInt;
private static convertParrotVariant(Ljava/util/OptionalInt;)Ljava/util/Optional;
public setShoulderParrotLeft(Ljava/util/Optional;)V
public getShoulderParrotLeft()Ljava/util/Optional;
public setShoulderParrotRight(Ljava/util/Optional;)V
public getShoulderParrotRight()Ljava/util/Optional;
public getCurrentItemAttackStrengthDelay()F
public cannotAttackWithItem(Lnet/minecraft/world/item/ItemStack;I)Z
public getAttackStrengthScale(F)F
public getItemSwapScale(F)F
public resetAttackStrengthTicker()V
public onAttack()V
public resetOnlyAttackStrengthTicker()V
public getCooldowns()Lnet/minecraft/world/item/ItemCooldowns;
protected getBlockSpeedFactor()F
public getLuck()F
public canUseGameMasterBlocks()Z
public permissions()Lnet/minecraft/server/permissions/PermissionSet;
public getDismountPoses()Lcom/google/common/collect/ImmutableList;
public getProjectile(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;
public getRopeHoldPosition(F)Lnet/minecraft/world/phys/Vec3;
public isAlwaysTicking()Z
public isScoping()Z
public shouldBeSaved()Z
public getLastDeathLocation()Ljava/util/Optional;
public setLastDeathLocation(Ljava/util/Optional;)V
public getHurtDir()F
public animateHurt(F)V
public isMobilityRestricted()Z
public canSprint()Z
protected getFlyingSpeed()F
public hasContainerOpen(Lnet/minecraft/world/level/block/entity/ContainerOpenersCounter;Lnet/minecraft/core/BlockPos;)Z
public getContainerInteractionRange()D
public blockInteractionRange()D
public entityInteractionRange()D
public isWithinEntityInteractionRange(Lnet/minecraft/world/entity/Entity;D)Z
public isWithinEntityInteractionRange(Lnet/minecraft/world/phys/AABB;D)Z
public isWithinAttackRange(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/phys/AABB;D)Z
public isWithinBlockInteractionRange(Lnet/minecraft/core/BlockPos;D)Z
public shouldRotateWithMinecart()Z
public onClimbable()Z
public debugInfo()Ljava/lang/String;
private static printPlayerPermissions(Lnet/minecraft/server/permissions/PermissionSet;)Ljava/lang/String;
public getProfile()Lnet/minecraft/world/item/component/ResolvableProfile;
public createDamageSource()Lnet/minecraft/world/damagesource/DamageSource;
private static synthetic lambda$convertParrotVariant$0(Lnet/minecraft/world/entity/animal/parrot/Parrot$Variant;)Ljava/util/OptionalInt;
private synthetic lambda$decorateDisplayNameComponent$0(Ljava/lang/String;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private static synthetic lambda$addAdditionalSaveData$0(Lnet/minecraft/world/level/storage/ValueOutput;Lnet/minecraft/core/GlobalPos;)V
static <clinit>()V
```
