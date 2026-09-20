---
type: "interface"
fqcn: "net.minecraft.world.entity.Mob"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.Mob

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`abstract_class` public abstract; extends `net/minecraft/world/entity/LivingEntity`; implements `net/minecraft/world/entity/Targeting`, `net/minecraft/world/entity/EquipmentUser`, `net/minecraft/world/entity/Leashable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `convertTo` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/Co` | exact | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `registerDebugValues` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/util/debug/Deb` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |

## Declared members (45 fields, 163 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DATA_MOB_FLAGS_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final MOB_FLAG_NO_AI : I
private static final MOB_FLAG_LEFTHANDED : I
private static final MOB_FLAG_AGGRESSIVE : I
protected static final PICKUP_REACH : I
private static final ITEM_PICKUP_REACH : Lnet/minecraft/core/Vec3i;
private static final EQUIPMENT_POPULATION_ORDER : Ljava/util/List;
public static final MAX_WEARING_ARMOR_CHANCE : F
public static final WEARING_ARMOR_UPGRADE_MATERIAL_CHANCE : F
public static final WEARING_ARMOR_UPGRADE_MATERIAL_ATTEMPTS : F
public static final MAX_PICKUP_LOOT_CHANCE : F
public static final MAX_ENCHANTED_ARMOR_CHANCE : F
public static final MAX_ENCHANTED_WEAPON_CHANCE : F
public static final UPDATE_GOAL_SELECTOR_EVERY_N_TICKS : I
private static final DEFAULT_ATTACK_REACH : D
private static final DEFAULT_CAN_PICK_UP_LOOT : Z
private static final DEFAULT_PERSISTENCE_REQUIRED : Z
private static final DEFAULT_LEFT_HANDED : Z
private static final DEFAULT_NO_AI : Z
protected static final RANDOM_SPAWN_BONUS_ID : Lnet/minecraft/resources/Identifier;
public static final TAG_DROP_CHANCES : Ljava/lang/String;
public static final TAG_LEFT_HANDED : Ljava/lang/String;
public static final TAG_CAN_PICK_UP_LOOT : Ljava/lang/String;
public static final TAG_NO_AI : Ljava/lang/String;
public static final TAG_PERSISTENCE_REQUIRED : Ljava/lang/String;
public ambientSoundTime : I
protected xpReward : I
protected lookControl : Lnet/minecraft/world/entity/ai/control/LookControl;
protected moveControl : Lnet/minecraft/world/entity/ai/control/MoveControl;
protected jumpControl : Lnet/minecraft/world/entity/ai/control/JumpControl;
private final bodyRotationControl : Lnet/minecraft/world/entity/ai/control/BodyRotationControl;
protected navigation : Lnet/minecraft/world/entity/ai/navigation/PathNavigation;
protected final goalSelector : Lnet/minecraft/world/entity/ai/goal/GoalSelector;
protected final targetSelector : Lnet/minecraft/world/entity/ai/goal/GoalSelector;
private target : Lnet/minecraft/world/entity/LivingEntity;
private final sensing : Lnet/minecraft/world/entity/ai/sensing/Sensing;
private dropChances : Lnet/minecraft/world/entity/DropChances;
private canPickUpLoot : Z
private persistenceRequired : Z
private final pathfindingMalus : Ljava/util/Map;
private lootTable : Ljava/util/Optional;
private lootTableSeed : J
private leashData : Lnet/minecraft/world/entity/Leashable$LeashData;
private homePosition : Lnet/minecraft/core/BlockPos;
private homeRadius : I
protected <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V
protected registerGoals()V
public static createMobAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
protected createNavigation(Lnet/minecraft/world/level/Level;)Lnet/minecraft/world/entity/ai/navigation/PathNavigation;
protected shouldPassengersInheritMalus()Z
public getPathfindingMalus(Lnet/minecraft/world/level/pathfinder/PathType;)F
public setPathfindingMalus(Lnet/minecraft/world/level/pathfinder/PathType;F)V
public onPathfindingStart()V
public onPathfindingDone()V
protected createBodyControl()Lnet/minecraft/world/entity/ai/control/BodyRotationControl;
public getLookControl()Lnet/minecraft/world/entity/ai/control/LookControl;
public getMoveControl()Lnet/minecraft/world/entity/ai/control/MoveControl;
public getJumpControl()Lnet/minecraft/world/entity/ai/control/JumpControl;
public getNavigation()Lnet/minecraft/world/entity/ai/navigation/PathNavigation;
public getControllingPassenger()Lnet/minecraft/world/entity/LivingEntity;
public getSensing()Lnet/minecraft/world/entity/ai/sensing/Sensing;
public getTarget()Lnet/minecraft/world/entity/LivingEntity;
public getTargetUnchecked()Lnet/minecraft/world/entity/LivingEntity;
protected asValidTarget(Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/entity/LivingEntity;
protected final getTargetFromBrain()Lnet/minecraft/world/entity/LivingEntity;
public setTarget(Lnet/minecraft/world/entity/LivingEntity;)V
public canAttack(Lnet/minecraft/world/entity/LivingEntity;)Z
public canUseNonMeleeWeapon(Lnet/minecraft/world/item/ItemStack;)Z
public ate()V
protected defineSynchedData(Lnet/minecraft/network/syncher/SynchedEntityData$Builder;)V
public getAmbientSoundInterval()I
public playAmbientSound()V
public baseTick()V
protected playHurtSound(Lnet/minecraft/world/damagesource/DamageSource;)V
private resetAmbientSoundTime()V
protected getBaseExperienceReward(Lnet/minecraft/server/level/ServerLevel;)I
public spawnAnim()V
public handleEntityEvent(B)V
public tick()V
protected updateControlFlags()V
protected tickHeadTurn(F)V
protected getAmbientSound()Lnet/minecraft/sounds/SoundEvent;
protected addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V
protected readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V
protected dropFromLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;Z)V
public final getLootTable()Ljava/util/Optional;
public getLootTableSeed()J
public setZza(F)V
public setYya(F)V
public setXxa(F)V
public setSpeed(F)V
public stopInPlace()V
public aiStep()V
protected sunProtectionSlot()Lnet/minecraft/world/entity/EquipmentSlot;
private burnUndead()V
private isSunBurnTick()Z
protected getPickupReach()Lnet/minecraft/core/Vec3i;
protected pickUpItem(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/item/ItemEntity;)V
public equipItemIfPossible(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;
protected setItemSlotAndDropWhenKilled(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;)V
protected canShearEquipment(Lnet/minecraft/world/entity/player/Player;)Z
protected attemptToShearEquipment(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/ItemStack;)Z
protected shearItem(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;)V
public setGuaranteedDrop(Lnet/minecraft/world/entity/EquipmentSlot;)V
protected canReplaceCurrentItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;)Z
private compareArmor(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;)Z
private compareWeapons(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;)Z
private getApproximateAttributeWith(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/EquipmentSlot;)D
public canReplaceEqualItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z
public canHoldItem(Lnet/minecraft/world/item/ItemStack;)Z
public wantsToPickUp(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)Z
public getPreferredWeaponType()Lnet/minecraft/tags/TagKey;
public removeWhenFarAway(D)Z
public requiresCustomPersistence()Z
public checkDespawn()V
protected final serverAiStep()V
protected customServerAiStep(Lnet/minecraft/server/level/ServerLevel;)V
public getMaxHeadXRot()I
public getMaxHeadYRot()I
public clampHeadRotationToBody()V
public getHeadRotSpeed()I
public lookAt(Lnet/minecraft/world/entity/Entity;FF)V
private rotlerp(FFF)F
public static checkMobSpawnRules(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)Z
public checkSpawnRules(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/world/entity/EntitySpawnReason;)Z
public checkSpawnObstruction(Lnet/minecraft/world/level/LevelReader;)Z
public getMaxSpawnClusterSize()I
public isMaxGroupSizeReached(I)Z
public getMaxFallDistance()I
public getBodyArmorItem()Lnet/minecraft/world/item/ItemStack;
public isSaddled()Z
public isWearingBodyArmor()Z
private hasValidEquippableItemForSlot(Lnet/minecraft/world/entity/EquipmentSlot;)Z
public createEquipmentSlotContainer(Lnet/minecraft/world/entity/EquipmentSlot;)Lnet/minecraft/world/Container;
protected dropCustomDeathLoot(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;Z)V
public getDropChances()Lnet/minecraft/world/entity/DropChances;
public dropPreservedEquipment(Lnet/minecraft/server/level/ServerLevel;)V
public dropPreservedEquipment(Lnet/minecraft/server/level/ServerLevel;Ljava/util/function/Predicate;)Ljava/util/Set;
private createEquipmentParams(Lnet/minecraft/server/level/ServerLevel;)Lnet/minecraft/world/level/storage/loot/LootParams;
public equip(Lnet/minecraft/world/entity/EquipmentTable;)V
public equip(Lnet/minecraft/resources/ResourceKey;Ljava/util/Map;)V
protected populateDefaultEquipmentSlots(Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/DifficultyInstance;)V
public static getEquipmentForSlot(Lnet/minecraft/world/entity/EquipmentSlot;I)Lnet/minecraft/world/item/Item;
protected populateDefaultEquipmentEnchantments(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/DifficultyInstance;)V
protected enchantSpawnedWeapon(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/DifficultyInstance;)V
protected enchantSpawnedArmor(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/DifficultyInstance;)V
private enchantSpawnedEquipment(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/util/RandomSource;FLnet/minecraft/world/DifficultyInstance;)V
public finalizeSpawn(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/world/DifficultyInstance;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/SpawnGroupData;)Lnet/minecraft/world/entity/SpawnGroupData;
public setPersistenceRequired()V
public setDropChance(Lnet/minecraft/world/entity/EquipmentSlot;F)V
public canPickUpLoot()Z
public setCanPickUpLoot(Z)V
protected canDispenserEquipIntoSlot(Lnet/minecraft/world/entity/EquipmentSlot;)Z
public isPersistenceRequired()Z
public interact(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/InteractionResult;
private checkAndHandleImportantInteractions(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
protected onOffspringSpawnedFromEgg(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Mob;)V
protected mobInteract(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
protected usePlayerItem(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/ItemStack;)V
public isWithinHome()Z
public isWithinHome(Lnet/minecraft/core/BlockPos;)Z
public isWithinHome(Lnet/minecraft/world/phys/Vec3;)Z
public setHomeTo(Lnet/minecraft/core/BlockPos;I)V
public getHomePosition()Lnet/minecraft/core/BlockPos;
public getHomeRadius()I
public clearHome()V
public hasHome()Z
public convertTo(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/ConversionParams;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/ConversionParams$AfterConversion;)Lnet/minecraft/world/entity/Mob;
public convertTo(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/ConversionParams;Lnet/minecraft/world/entity/ConversionParams$AfterConversion;)Lnet/minecraft/world/entity/Mob;
public getLeashData()Lnet/minecraft/world/entity/Leashable$LeashData;
private resetAngularLeashMomentum()V
public setLeashData(Lnet/minecraft/world/entity/Leashable$LeashData;)V
public onLeashRemoved()V
public leashTooFarBehaviour()V
public canBeLeashed()Z
public startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z
public isEffectiveAi()Z
public setNoAi(Z)V
public setLeftHanded(Z)V
public setAggressive(Z)V
public isNoAi()Z
public isLeftHanded()Z
public isAggressive()Z
public setBaby(Z)V
public getMainArm()Lnet/minecraft/world/entity/HumanoidArm;
public isWithinMeleeAttackRange(Lnet/minecraft/world/entity/LivingEntity;)Z
protected getAttackBoundingBox(D)Lnet/minecraft/world/phys/AABB;
public doHurtTarget(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;)Z
protected jumpInLiquid(Lnet/minecraft/tags/TagKey;)V
public removeFreeWill()V
public getGoalSelector()Lnet/minecraft/world/entity/ai/goal/GoalSelector;
public removeAllGoals(Ljava/util/function/Predicate;)V
protected removeAfterChangingDimensions()V
public getPickResult()Lnet/minecraft/world/item/ItemStack;
protected onAttributeUpdated(Lnet/minecraft/core/Holder;)V
public registerDebugValues(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/util/debug/DebugValueSource$Registration;)V
public chargeSpeedModifier()F
public swingForAttack(Lnet/minecraft/world/InteractionHand;)V
public swing(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/component/SwingAnimation;)V
private synthetic lambda$registerDebugValues$3(Lnet/minecraft/server/level/ServerLevel;)Lnet/minecraft/util/debug/DebugBrainDump;
private synthetic lambda$registerDebugValues$1()Lnet/minecraft/util/debug/DebugGoalInfo;
private static synthetic lambda$registerDebugValues$2(Ljava/util/List;Lnet/minecraft/world/entity/ai/goal/WrappedGoal;)V
private synthetic lambda$registerDebugValues$0()Lnet/minecraft/util/debug/DebugPathInfo;
private static synthetic lambda$removeFreeWill$0(Lnet/minecraft/world/entity/ai/goal/Goal;)Z
private synthetic lambda$checkAndHandleImportantInteractions$0(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Mob;)V
private static synthetic lambda$dropPreservedEquipment$0(Lnet/minecraft/world/item/ItemStack;)Z
private static synthetic lambda$addAdditionalSaveData$0(Lnet/minecraft/world/level/storage/ValueOutput;Lnet/minecraft/resources/ResourceKey;)V
static <clinit>()V
```
