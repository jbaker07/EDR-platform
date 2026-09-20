---
type: "interface"
fqcn: "net.minecraft.world.entity.Entity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.Entity

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/Nameable`, `net/minecraft/world/level/entity/EntityAccess`, `net/minecraft/world/scores/ScoreHolder`, `net/minecraft/network/syncher/SyncedDataHolder`, `net/minecraft/core/component/DataComponentGetter`, `net/minecraft/world/entity/ItemOwner`, `net/minecraft/world/entity/SlotProvider`, `net/minecraft/util/debug/DebugValueSource`, `net/minecraft/core/TypedInstance`, `net/fabricmc/fabric/api/attachment/v1/AttachmentTarget`, `net/fabricmc/fabric/api/event/lifecycle/v1/EntityLoadData`, `net/fabricmc/fabric/api/permission/v1/PermissionContextOwner`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Lev` | exact | invokespecial@3 in `LivingEntityMixin.<init>` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Lev` | exact | invokespecial@3 in `LivingEntityMixin.<init>` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Lev` | exact | invokespecial@3 in `LivingEntityMixin.<init>` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `blockPosition` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@41 in `EntityPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getId` | `()I` | exact | invokevirtual@85 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getId` | `()I` | exact | invokevirtual@51 in `MultiPlayerGameModeMixin.attackEntity` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getId` | `()I` | exact | invokevirtual@40 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPlainTextName` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@11 in `EntityPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/entity/EntityType;` | exact | invokevirtual@24 in `EntityApiLookupImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/entity/EntityType;` | exact | invokevirtual@1 in `DetectorRailBlockMixin.lambda$getCustomComparatorOutput$0` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getUUID` | `()Ljava/util/UUID;` | exact | invokevirtual@4 in `EntityPermissionContext.uuid` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getUUID` | `()Ljava/util/UUID;` | exact | invokevirtual@128 in `CommandSourceStackMixin.storeOriginalSource` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getX` | `()D` | exact | invokevirtual@19 in `ServerGamePacketListenerImplMixin.handleInteract` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getX` | `()D` | exact | invokevirtual@57 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getY` | `()D` | exact | invokevirtual@23 in `ServerGamePacketListenerImplMixin.handleInteract` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getY` | `()D` | exact | invokevirtual@62 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getZ` | `()D` | exact | invokevirtual@27 in `ServerGamePacketListenerImplMixin.handleInteract` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getZ` | `()D` | exact | invokevirtual@67 in `MinecraftMixin.injectUseEntityCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `isInLava` | `()Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| calls | `isInWater` | `()Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| calls | `isPushedByFluid` | `()Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| calls | `killedEntity` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/L` | exact | invokevirtual@19 in `ServerPlayerMixin.callOnKillForPlayer` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@1 in `EntityDataAccessorMixin.setData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@62 in `EntityMixin.afterDimensionChanged` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@30 in `PersistentEntitySectionManagerMixin.beforeAddEntity` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@8 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@31 in `EntityPermissionContext.<init>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@41 in `EntityPermissionContext.<init>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | exact | invokevirtual@56 in `EntityPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `position` | `()Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@26 in `EntityPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `spawnReason` | `()Lnet/minecraft/world/entity/EntitySpawnReason;` | inherited_exact | invokevirtual@38 in `PersistentEntitySectionManagerMixin.beforeAddEntity` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Lev` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Lev` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| injects_into | `canSpawnSprintParticle` | `()Z` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `isInLiquid` | `()Z` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `isVisuallyCrawling` | `()Z` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `load` | `(Lnet/minecraft/world/level/storage/ValueInput;)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `registerDebugValues` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/util/debug/Deb` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |
| injects_into | `saveWithoutId` | `(Lnet/minecraft/world/level/storage/ValueOutput;)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `setId` | `(I)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `spawnSprintParticle` | `()V` | name_only | @ModifyExpressionValue at ['NEW'] | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| injects_into | `updateFluidInteraction` | `()Z` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `updateSwimming` | `()V` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `updateSwimming` | `()V` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `fallDistance` | `D` | exact | getfield@17 in `SimpleConfiguredFluidBehavior.handleFluidInteractionUpdate` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `firstTick` | `Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| reads | `fluidInteraction` | `Lnet/minecraft/world/entity/EntityFluidInteraction;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| reads | `id` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| reads | `level` | `Lnet/minecraft/world/level/Level;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | declared |
| wraps | `teleport` | `(Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `updateSwimming` | `()V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| writes | `fallDistance` | `D` | exact | putfield@26 in `SimpleConfiguredFluidBehavior.handleFluidInteractionUpdate` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (136 fields, 554 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final TAG_ID : Ljava/lang/String;
public static final TAG_UUID : Ljava/lang/String;
public static final TAG_PASSENGERS : Ljava/lang/String;
public static final TAG_DATA : Ljava/lang/String;
public static final TAG_POS : Ljava/lang/String;
public static final TAG_MOTION : Ljava/lang/String;
public static final TAG_ROTATION : Ljava/lang/String;
public static final TAG_PORTAL_COOLDOWN : Ljava/lang/String;
public static final TAG_NO_GRAVITY : Ljava/lang/String;
public static final TAG_AIR : Ljava/lang/String;
public static final TAG_ON_GROUND : Ljava/lang/String;
public static final TAG_FALL_DISTANCE : Ljava/lang/String;
public static final TAG_FIRE : Ljava/lang/String;
public static final TAG_SILENT : Ljava/lang/String;
public static final TAG_GLOWING : Ljava/lang/String;
public static final TAG_INVULNERABLE : Ljava/lang/String;
public static final TAG_INVULNERABLE_TIME : Ljava/lang/String;
public static final TAG_CUSTOM_NAME : Ljava/lang/String;
public static final INVALID_ENTITY_ID : I
public static final CONTENTS_SLOT_INDEX : I
public static final BOARDING_COOLDOWN : I
public static final TOTAL_AIR_SUPPLY : I
public static final MAX_ENTITY_TAG_COUNT : I
private static final TAG_LIST_CODEC : Lcom/mojang/serialization/Codec;
public static final DEFAULT_NAME_TAG_DISTANCE : D
public static final DEFAULT_BELOW_NAME_DISTANCE : D
public static final MAX_NAME_TAG_DISTANCE : D
public static final DELTA_AFFECTED_BY_BLOCKS_BELOW_0_2 : F
public static final DELTA_AFFECTED_BY_BLOCKS_BELOW_0_5 : D
public static final DELTA_AFFECTED_BY_BLOCKS_BELOW_1_0 : D
public static final BASE_TICKS_REQUIRED_TO_FREEZE : I
public static final FREEZE_HURT_FREQUENCY : I
public static final BASE_SAFE_FALL_DISTANCE : I
private static final INITIAL_AABB : Lnet/minecraft/world/phys/AABB;
private static final WATER_FLOW_SCALE : D
private static final LAVA_FAST_FLOW_SCALE : D
private static final LAVA_SLOW_FLOW_SCALE : D
private static final MAX_BLOCK_ITERATIONS_ALONG_TRAVEL_PER_TICK : I
private static final MAX_MOVEMENT_RESETTING_TRACE_DISTANCE : D
private static final FLUIDS_WITH_CURRENT : Ljava/util/Set;
private static viewScale : D
private final type : Lnet/minecraft/world/entity/EntityType;
private requiresPrecisePosition : Z
private id : I
public blocksBuilding : Z
private passengers : Lcom/google/common/collect/ImmutableList;
protected boardingCooldown : I
private vehicle : Lnet/minecraft/world/entity/Entity;
private level : Lnet/minecraft/world/level/Level;
public xo : D
public yo : D
public zo : D
private position : Lnet/minecraft/world/phys/Vec3;
private blockPosition : Lnet/minecraft/core/BlockPos;
private chunkPosition : Lnet/minecraft/world/level/ChunkPos;
private deltaMovement : Lnet/minecraft/world/phys/Vec3;
private yRot : F
private xRot : F
public yRotO : F
public xRotO : F
private bb : Lnet/minecraft/world/phys/AABB;
private onGround : Z
public horizontalCollision : Z
public verticalCollision : Z
public verticalCollisionBelow : Z
public minorHorizontalCollision : Z
protected stuckSpeedMultiplier : Lnet/minecraft/world/phys/Vec3;
private removalReason : Lnet/minecraft/world/entity/Entity$RemovalReason;
public static final DEFAULT_BB_WIDTH : F
public static final DEFAULT_BB_HEIGHT : F
public moveDist : F
public flyDist : F
public fallDistance : D
private nextStep : F
public xOld : D
public yOld : D
public zOld : D
public noPhysics : Z
protected final random : Lnet/minecraft/util/RandomSource;
public tickCount : I
private remainingFireTicks : I
private final fluidInteraction : Lnet/minecraft/world/entity/EntityFluidInteraction;
protected wasTouchingWater : Z
protected wasEyeInWater : Z
private invulnerableTime : I
protected firstTick : Z
protected final entityData : Lnet/minecraft/network/syncher/SynchedEntityData;
protected static final DATA_SHARED_FLAGS_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
protected static final FLAG_ONFIRE : I
private static final FLAG_SHIFT_KEY_DOWN : I
private static final FLAG_SPRINTING : I
private static final FLAG_SWIMMING : I
private static final FLAG_INVISIBLE : I
protected static final FLAG_GLOWING : I
protected static final FLAG_FALL_FLYING : I
private static final DATA_AIR_SUPPLY_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_CUSTOM_NAME : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_CUSTOM_NAME_VISIBLE : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_SILENT : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_NO_GRAVITY : Lnet/minecraft/network/syncher/EntityDataAccessor;
protected static final DATA_POSE : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_TICKS_FROZEN : Lnet/minecraft/network/syncher/EntityDataAccessor;
private levelCallback : Lnet/minecraft/world/level/entity/EntityInLevelCallback;
private final packetPositionCodec : Lnet/minecraft/network/protocol/game/VecDeltaCodec;
public needsSync : Z
public syncPosition : Z
public syncVelocity : Z
public portalProcess : Lnet/minecraft/world/entity/PortalProcessor;
private portalCooldown : I
private permanentlyInvulnerable : Z
protected uuid : Ljava/util/UUID;
protected stringUUID : Ljava/lang/String;
private hasGlowingTag : Z
private final tags : Ljava/util/Set;
private final pistonDeltas : [D
private pistonDeltasGameTime : J
private dimensions : Lnet/minecraft/world/entity/EntityDimensions;
private eyeHeight : F
public isInPowderSnow : Z
public wasInPowderSnow : Z
public mainSupportingBlockPos : Ljava/util/Optional;
private onGroundNoBlocks : Z
private crystalSoundIntensity : F
private lastCrystalSoundPlayTick : I
private hasVisualFire : Z
private lastKnownSpeed : Lnet/minecraft/world/phys/Vec3;
private lastKnownPosition : Lnet/minecraft/world/phys/Vec3;
private inBlockState : Lnet/minecraft/world/level/block/state/BlockState;
public static final MAX_MOVEMENTS_HANDELED_PER_TICK : I
private final movementThisTick : Ljava/util/ArrayDeque;
private final finalMovementsThisTick : Ljava/util/List;
private final visitedBlocks : Lit/unimi/dsi/fastutil/longs/LongSet;
protected final insideEffectCollector : Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;
protected final interpolationHandler : Lnet/minecraft/world/entity/InterpolationHandler;
private customData : Lnet/minecraft/world/item/component/CustomData;
public <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V
public isColliding(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z
public getTeamColor()I
public isSpectator()Z
public canInteractWithLevel()Z
public final unRide()V
public syncPacketPositionCodec(DDD)V
public getPositionCodec()Lnet/minecraft/network/protocol/game/VecDeltaCodec;
public getType()Lnet/minecraft/world/entity/EntityType;
public typeHolder()Lnet/minecraft/core/Holder;
public getRequiresPrecisePosition()Z
public setRequiresPrecisePosition(Z)V
public getId()I
public setId(I)V
public entityTags()Ljava/util/Set;
public addTag(Ljava/lang/String;)Z
public removeTag(Ljava/lang/String;)Z
public kill(Lnet/minecraft/server/level/ServerLevel;)V
public final discard()V
protected abstract defineSynchedData(Lnet/minecraft/network/syncher/SynchedEntityData$Builder;)V
public getEntityData()Lnet/minecraft/network/syncher/SynchedEntityData;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public remove(Lnet/minecraft/world/entity/Entity$RemovalReason;)V
public onClientRemoval()V
public onRemoval(Lnet/minecraft/world/entity/Entity$RemovalReason;)V
public setPose(Lnet/minecraft/world/entity/Pose;)V
public getPose()Lnet/minecraft/world/entity/Pose;
public hasPose(Lnet/minecraft/world/entity/Pose;)Z
public closerThan(Lnet/minecraft/world/entity/Entity;D)Z
public closerThan(Lnet/minecraft/world/entity/Entity;DD)Z
protected setRot(FF)V
public final setPos(Lnet/minecraft/world/phys/Vec3;)V
public setPos(DDD)V
protected final makeBoundingBox()Lnet/minecraft/world/phys/AABB;
protected makeBoundingBox(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/AABB;
protected reapplyPosition()V
public turn(DD)V
public updateDataBeforeSync()V
public tick()V
public final commonTick()V
public baseTick()V
protected computeSpeed()V
public setSharedFlagOnFire(Z)V
public checkBelowWorld()V
public setPortalCooldown()V
public setPortalCooldown(I)V
public getPortalCooldown()I
public isOnPortalCooldown()Z
protected processPortalCooldown()V
public lavaIgnite()V
public lavaHurt()V
protected shouldPlayLavaHurtSound()Z
public final igniteForSeconds(F)V
public igniteForTicks(I)V
public setRemainingFireTicks(I)V
public getRemainingFireTicks()I
public clearFire()V
protected onBelowWorld()V
public isFree(DDD)Z
private isFree(Lnet/minecraft/world/phys/AABB;)Z
public setOnGround(Z)V
public setOnGroundWithMovement(ZLnet/minecraft/world/phys/Vec3;)V
public setOnGroundWithMovement(ZZLnet/minecraft/world/phys/Vec3;)V
public isSupportedBy(Lnet/minecraft/core/BlockPos;)Z
protected checkSupportingBlock(ZLnet/minecraft/world/phys/Vec3;)V
public onGround()Z
public move(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V
public recordMovement(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V
private restituteMovementAfterCollisions(Lnet/minecraft/world/level/block/state/BlockState;ZZLnet/minecraft/world/phys/Vec3;)V
private getBlockBounciness(Lnet/minecraft/world/level/block/Block;)D
protected getEntityBounciness()D
protected getEffectiveGravity()D
protected omnidirectionalAirMover()Z
private applyMovementEmissionAndPlaySound(Lnet/minecraft/world/entity/Entity$MovementEmission;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
protected applyEffectsFromBlocks()V
protected applyEffectsFromBlocksForLastMovements()V
private addMovementThisTick(Lnet/minecraft/world/entity/Entity$Movement;)V
public removeLatestMovementRecording()V
protected clearMovementThisTick()V
public hasMovedHorizontallyRecently()Z
public applyEffectsFromBlocks(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;)V
private applyEffectsFromBlocks(Ljava/util/List;)V
protected isAffectedByBlocks()Z
private isStateClimbable(Lnet/minecraft/world/level/block/state/BlockState;)Z
private vibrationAndSoundEffectsFromBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;ZZLnet/minecraft/world/phys/Vec3;)Z
protected isHorizontalCollisionMinor(Lnet/minecraft/world/phys/Vec3;)Z
protected playEntityOnFireExtinguishedSound()V
public extinguishFire()V
protected processFlappingMovement()V
public getOnPosLegacy()Lnet/minecraft/core/BlockPos;
public getBlockPosBelowThatAffectsMyMovement()Lnet/minecraft/core/BlockPos;
public getOnPos()Lnet/minecraft/core/BlockPos;
protected getOnPos(F)Lnet/minecraft/core/BlockPos;
protected getBlockJumpFactor()F
protected getBlockSpeedFactor()F
protected maybeBackOffFromEdge(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/MoverType;)Lnet/minecraft/world/phys/Vec3;
protected limitPistonMovement(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
private applyPistonMovementRestriction(Lnet/minecraft/core/Direction$Axis;D)D
public getAvailableSpaceBelow(D)D
private collide(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
private static collectCandidateStepUpHeights(Lnet/minecraft/world/phys/AABB;Ljava/util/List;FF)[F
public static collideBoundingBox(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/AABB;Lnet/minecraft/world/level/Level;Ljava/util/List;)Lnet/minecraft/world/phys/Vec3;
public static collideBoundingBox(Lnet/minecraft/world/phys/shapes/CollisionContext;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/AABB;Lnet/minecraft/world/level/Level;Ljava/util/List;)Lnet/minecraft/world/phys/Vec3;
public static collectAllColliders(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;
private static collectCollidersIgnoringWorldBorder(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/level/Level;Ljava/util/List;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;
private static collectCollidersIgnoringWorldBorder(Lnet/minecraft/world/phys/shapes/CollisionContext;Lnet/minecraft/world/level/Level;Ljava/util/List;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;
private static collideWithShapes(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/AABB;Ljava/util/List;)Lnet/minecraft/world/phys/Vec3;
protected nextStep()F
protected getSwimSound()Lnet/minecraft/sounds/SoundEvent;
protected getSwimSplashSound()Lnet/minecraft/sounds/SoundEvent;
protected getSwimHighSpeedSplashSound()Lnet/minecraft/sounds/SoundEvent;
private checkInsideBlocks(Ljava/util/List;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;)V
private checkInsideBlocks(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lit/unimi/dsi/fastutil/longs/LongSet;I)I
private debugBlockIntersection(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;ZZ)V
public collidedWithFluid(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;)Z
public collidedWithShapeMovingFrom(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Ljava/util/List;)Z
protected onInsideBlock(Lnet/minecraft/world/level/block/state/BlockState;)V
public adjustSpawnLocation(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
public gameEvent(Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/Entity;)V
public gameEvent(Lnet/minecraft/core/Holder;)V
private walkingStepSound(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
protected waterSwimSound()V
protected getPrimaryStepSoundBlockPos(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
protected playCombinationStepSounds(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)V
protected playMuffledStepSound(Lnet/minecraft/world/level/block/state/BlockState;)V
protected playStepSound(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
private shouldPlayAmethystStepSound(Lnet/minecraft/world/level/block/state/BlockState;)Z
private playAmethystStepSound()V
protected playSwimSound(F)V
protected onFlap()V
protected isFlapping()Z
public playSound(Lnet/minecraft/sounds/SoundEvent;FF)V
public playSound(Lnet/minecraft/sounds/SoundEvent;)V
public isSilent()Z
public setSilent(Z)V
public isNoGravity()Z
public setNoGravity(Z)V
protected getDefaultGravity()D
public final getGravity()D
protected applyGravity()V
protected getAirDrag()F
protected getMovementEmission()Lnet/minecraft/world/entity/Entity$MovementEmission;
public dampensVibrations()Z
public final doCheckFallDamage(DDDZ)V
protected checkFallDamage(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V
public fireImmune()Z
public causeFallDamage(DFLnet/minecraft/world/damagesource/DamageSource;)Z
protected propagateFallToPassengers(DFLnet/minecraft/world/damagesource/DamageSource;)V
public isInWater()Z
private isInRain()Z
public isInWaterOrRain()Z
public isInLiquid()Z
public isInFloatableFluid()Z
public isUnderWater()Z
public isInShallowWater()Z
public isInClouds()Z
public updateSwimming()V
protected updateFluidInteraction()Z
protected doWaterSplashEffect()V
protected getBlockStateOnLegacy()Lnet/minecraft/world/level/block/state/BlockState;
public getBlockStateOn()Lnet/minecraft/world/level/block/state/BlockState;
public canSpawnSprintParticle()Z
protected spawnSprintParticle()V
public isEyeInFluid(Lnet/minecraft/tags/TagKey;)Z
public isInLava()Z
public moveRelative(FLnet/minecraft/world/phys/Vec3;)V
protected static getInputVector(Lnet/minecraft/world/phys/Vec3;FF)Lnet/minecraft/world/phys/Vec3;
public getLightLevelDependentMagicValue()F
public absSnapTo(DDDFF)V
public absSnapRotationTo(FF)V
public absSnapTo(DDD)V
public snapTo(Lnet/minecraft/world/phys/Vec3;)V
public snapTo(DDD)V
public snapTo(Lnet/minecraft/core/BlockPos;FF)V
public snapTo(Lnet/minecraft/world/phys/Vec3;FF)V
public snapTo(DDDFF)V
public final setOldPosAndRot()V
public final setOldPosAndRot(Lnet/minecraft/world/phys/Vec3;FF)V
protected setOldPos()V
public setOldRot()V
private setOldPos(Lnet/minecraft/world/phys/Vec3;)V
private setOldRot(FF)V
public final oldPosition()Lnet/minecraft/world/phys/Vec3;
public distanceTo(Lnet/minecraft/world/entity/Entity;)F
public distanceToSqr(DDD)D
public distanceToSqr(Lnet/minecraft/world/entity/Entity;)D
public distanceToSqr(Lnet/minecraft/world/phys/Vec3;)D
public playerTouch(Lnet/minecraft/world/entity/player/Player;)V
public push(Lnet/minecraft/world/entity/Entity;)V
public push(Lnet/minecraft/world/phys/Vec3;)V
public pushFromExplosion(Lnet/minecraft/world/phys/Vec3;)V
public push(DDD)V
protected markHurt()V
public final hurt(Lnet/minecraft/world/damagesource/DamageSource;F)V
public final hurtOrSimulate(Lnet/minecraft/world/damagesource/DamageSource;F)Z
public abstract hurtServer(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)Z
public hurtClient(Lnet/minecraft/world/damagesource/DamageSource;)Z
public final getViewVector(F)Lnet/minecraft/world/phys/Vec3;
public getNearestViewDirection()Lnet/minecraft/core/Direction;
public getViewXRot(F)F
public getViewYRot(F)F
public getXRot(F)F
public getYRot(F)F
public static calculateViewVector(FF)Lnet/minecraft/world/phys/Vec3;
public static calculateViewQuaternion(FF)Lorg/joml/Quaternionf;
public final getUpVector(F)Lnet/minecraft/world/phys/Vec3;
protected static calculateUpVector(FF)Lnet/minecraft/world/phys/Vec3;
public final getEyePosition()Lnet/minecraft/world/phys/Vec3;
public final getEyePosition(F)Lnet/minecraft/world/phys/Vec3;
public getLightProbePosition(F)Lnet/minecraft/world/phys/Vec3;
public final getPosition(F)Lnet/minecraft/world/phys/Vec3;
public pick(DFZ)Lnet/minecraft/world/phys/HitResult;
public canBeHitByProjectile()Z
public isPickable()Z
public canBePickedFromInside()Z
public isPushable()Z
public awardKillScore(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)V
public shouldRender(DDD)Z
public shouldRenderAtSqrDistance(D)Z
public saveAsPassenger(Lnet/minecraft/world/level/storage/ValueOutput;)Z
public save(Lnet/minecraft/world/level/storage/ValueOutput;)Z
public saveWithoutId(Lnet/minecraft/world/level/storage/ValueOutput;)V
public load(Lnet/minecraft/world/level/storage/ValueInput;)V
public postDataManipulated()V
protected repositionEntityAfterLoad()Z
protected final getEncodeId()Ljava/lang/String;
protected abstract readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V
protected abstract addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V
public spawnAtLocation(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/world/entity/item/ItemEntity;
public spawnAtLocation(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/entity/item/ItemEntity;
public spawnAtLocation(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/entity/item/ItemEntity;
public spawnAtLocation(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;F)Lnet/minecraft/world/entity/item/ItemEntity;
public isAlive()Z
public isInWall()Z
public interact(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/InteractionResult;
public shearOffAllLeashConnections(Lnet/minecraft/world/entity/player/Player;)Z
public dropAllLeashConnections(Lnet/minecraft/world/entity/player/Player;)Z
public canCollideWith(Lnet/minecraft/world/entity/Entity;)Z
public canBeCollidedWith(Lnet/minecraft/world/entity/Entity;)Z
public rideTick()V
public final positionRider(Lnet/minecraft/world/entity/Entity;)V
protected positionRider(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity$MoveFunction;)V
public onPassengerTurned(Lnet/minecraft/world/entity/Entity;)V
public getVehicleAttachmentPoint(Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/phys/Vec3;
public getPassengerRidingPosition(Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/phys/Vec3;
protected getPassengerAttachmentPoint(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/EntityDimensions;F)Lnet/minecraft/world/phys/Vec3;
protected static getDefaultPassengerAttachmentPoint(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/EntityAttachments;)Lnet/minecraft/world/phys/Vec3;
public final startRiding(Lnet/minecraft/world/entity/Entity;)Z
public showVehicleHealth()Z
public startRiding(Lnet/minecraft/world/entity/Entity;ZZ)Z
protected canRide(Lnet/minecraft/world/entity/Entity;)Z
public ejectPassengers()V
public final doTeamsAllowDamage(Lnet/minecraft/world/entity/Entity;)Z
public removeVehicle()V
public stopRiding()V
protected addPassenger(Lnet/minecraft/world/entity/Entity;)V
protected removePassenger(Lnet/minecraft/world/entity/Entity;)V
protected canAddPassenger(Lnet/minecraft/world/entity/Entity;)Z
protected couldAcceptPassenger()Z
public final isInterpolating()Z
public final moveOrInterpolateTo(Lnet/minecraft/world/entity/PositionPath;FF)V
public final moveOrInterpolateTo(Lnet/minecraft/world/phys/Vec3;FF)V
public final moveOrInterpolateTo(FF)V
public final moveOrInterpolateTo(Lnet/minecraft/world/entity/PositionPath;)V
public final moveOrInterpolateTo(Lnet/minecraft/world/entity/PositionPath;FFZ)V
public onInterpolationStart(Lnet/minecraft/world/entity/InterpolationHandler;)V
protected createInterpolationHandler()Lnet/minecraft/world/entity/InterpolationHandler;
public final getInterpolation()Lnet/minecraft/world/entity/InterpolationHandler;
public lerpHeadTo(FI)V
public getPickRadius()F
public getLookAngle()Lnet/minecraft/world/phys/Vec3;
public getLookQuaternion()Lorg/joml/Quaternionf;
public getHeadLookAngle()Lnet/minecraft/world/phys/Vec3;
public getHandHoldingItemAngle(Lnet/minecraft/world/item/Item;)Lnet/minecraft/world/phys/Vec3;
public getRotationVector()Lnet/minecraft/world/phys/Vec2;
public getForward()Lnet/minecraft/world/phys/Vec3;
public setAsInsidePortal(Lnet/minecraft/world/level/block/Portal;Lnet/minecraft/core/BlockPos;)V
protected handlePortal()V
public teleportToPortalDestination(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/portal/TeleportTransition;)V
public getDimensionChangingDelay()I
public lerpMotion(Lnet/minecraft/world/phys/Vec3;)V
public handleDamageEvent(Lnet/minecraft/world/damagesource/DamageSource;)V
public handleEntityEvent(B)V
public animateHurt(F)V
public isOnFire()Z
public isPassenger()Z
public isVehicle()Z
public dismountsUnderwater()Z
public canControlVehicle()Z
public setShiftKeyDown(Z)V
public isShiftKeyDown()Z
public isSteppingCarefully()Z
public isSuppressingBounce()Z
public isDiscrete()Z
public isDescending()Z
public isCrouching()Z
public isSprinting()Z
public setSprinting(Z)V
public isSwimming()Z
public isVisuallySwimming()Z
public isVisuallyCrawling()Z
public setSwimming(Z)V
public final hasGlowingTag()Z
public final setGlowingTag(Z)V
public isCurrentlyGlowing()Z
public isInvisible()Z
public isInvisibleTo(Lnet/minecraft/world/entity/player/Player;)Z
public isOnRails()Z
public updateDynamicGameEventListener(Ljava/util/function/BiConsumer;)V
public getTeam()Lnet/minecraft/world/scores/PlayerTeam;
public final isAlliedTo(Lnet/minecraft/world/entity/Entity;)Z
protected considersEntityAsAlly(Lnet/minecraft/world/entity/Entity;)Z
public isAlliedTo(Lnet/minecraft/world/scores/Team;)Z
public setInvisible(Z)V
protected getSharedFlag(I)Z
protected setSharedFlag(IZ)V
public getMaxAirSupply()I
public getAirSupply()I
public setAirSupply(I)V
public clearFreeze()V
public getTicksFrozen()I
public setTicksFrozen(I)V
public getPercentFrozen()F
public isFullyFrozen()Z
public getTicksRequiredToFreeze()I
public thunderHit(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LightningBolt;)V
public onAboveBubbleColumn(ZLnet/minecraft/core/BlockPos;)V
protected static handleOnAboveBubbleColumn(Lnet/minecraft/world/entity/Entity;ZLnet/minecraft/core/BlockPos;)V
protected static sendBubbleColumnParticles(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
public onInsideBubbleColumn(Z)V
protected static handleOnInsideBubbleColumn(Lnet/minecraft/world/entity/Entity;Z)V
public killedEntity(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;)Z
public projectileReceivesSideEffectsOnHit(Z)Z
public checkFallDistanceAccumulation()V
public resetFallDistance()V
protected moveTowardsClosestSpace(DDD)V
public makeStuckInBlock(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/phys/Vec3;)V
private static removeAction(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
public getName()Lnet/minecraft/network/chat/Component;
protected getTypeName()Lnet/minecraft/network/chat/Component;
public is(Lnet/minecraft/world/entity/Entity;)Z
public getYHeadRot()F
public setYHeadRot(F)V
public setYBodyRot(F)V
public isAttackable()Z
public skipAttackInteraction(Lnet/minecraft/world/entity/Entity;)Z
public toString()Ljava/lang/String;
protected final isInvulnerableToBase(Lnet/minecraft/world/damagesource/DamageSource;)Z
public isInvulnerable()Z
public isPermanentlyInvulnerable()Z
public isTemporarilyInvulnerable()Z
public isInvulnerableToPiercingWeapon()Z
public setPermanentlyInvulnerable(Z)V
public copyPosition(Lnet/minecraft/world/entity/Entity;)V
public restoreFrom(Lnet/minecraft/world/entity/Entity;)V
public teleport(Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/world/entity/Entity;
private teleportSameDimension(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/world/entity/Entity;
private teleportCrossDimension(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/world/entity/Entity;
protected teleportSpectators(Lnet/minecraft/world/level/portal/TeleportTransition;Lnet/minecraft/server/level/ServerLevel;)V
private calculatePassengerTransition(Lnet/minecraft/world/level/portal/TeleportTransition;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/level/portal/TeleportTransition;
private sendTeleportTransitionToPlayers(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/portal/TeleportTransition;)V
public teleportSetPosition(Lnet/minecraft/world/entity/PositionMoveRotation;Ljava/util/Set;)V
public teleportSetPosition(Lnet/minecraft/world/entity/PositionMoveRotation;Lnet/minecraft/world/entity/PositionMoveRotation;Ljava/util/Set;)V
public forceSetRotation(FZFZ)V
public placePortalTicket(Lnet/minecraft/core/BlockPos;)V
protected removeAfterChangingDimensions()V
public getRelativePortalPosition(Lnet/minecraft/core/Direction$Axis;Lnet/minecraft/util/BlockUtil$FoundRectangle;)Lnet/minecraft/world/phys/Vec3;
public canUsePortal(Z)Z
public canTeleport(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/Level;)Z
public getBlockExplosionResistance(Lnet/minecraft/world/level/Explosion;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;F)F
public shouldBlockExplode(Lnet/minecraft/world/level/Explosion;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;F)Z
public getMaxFallDistance()I
public isIgnoringBlockTriggers()Z
public fillCrashReportCategory(Lnet/minecraft/CrashReportCategory;)V
public displayFireAnimation()Z
public setUUID(Ljava/util/UUID;)V
public getUUID()Ljava/util/UUID;
public getStringUUID()Ljava/lang/String;
public getScoreboardName()Ljava/lang/String;
public isPushedByFluid()Z
public static getViewScale()D
public static setViewScale(D)V
public getDisplayName()Lnet/minecraft/network/chat/Component;
public setCustomName(Lnet/minecraft/network/chat/Component;)V
public getCustomName()Lnet/minecraft/network/chat/Component;
public hasCustomName()Z
public setCustomNameVisible(Z)V
public isCustomNameVisible()Z
public belowNameDisplay()Lnet/minecraft/network/chat/Component;
public teleportTo(Lnet/minecraft/server/level/ServerLevel;DDDLjava/util/Set;FFZ)Z
public dismountTo(DDD)V
public teleportTo(DDD)V
private teleportPassengers()V
public teleportRelative(DDD)V
public shouldShowName()Z
public onSyncedDataUpdated(Ljava/util/List;)V
public onSyncedDataUpdated(Lnet/minecraft/network/syncher/EntityDataAccessor;)V
protected fixupDimensions()V
public refreshDimensions()V
public fudgePositionAfterSizeChange(Lnet/minecraft/world/entity/EntityDimensions;)Z
public getDirection()Lnet/minecraft/core/Direction;
public getMotionDirection()Lnet/minecraft/core/Direction;
protected createHoverEvent()Lnet/minecraft/network/chat/HoverEvent;
public broadcastToPlayer(Lnet/minecraft/server/level/ServerPlayer;)Z
public final getBoundingBox()Lnet/minecraft/world/phys/AABB;
public getInterpolatedBoundingBox(F)Lnet/minecraft/world/phys/AABB;
public final setBoundingBox(Lnet/minecraft/world/phys/AABB;)V
public final getEyeHeight(Lnet/minecraft/world/entity/Pose;)F
public final getEyeHeight()F
public getSlot(I)Lnet/minecraft/world/entity/SlotAccess;
public ignoreExplosion(Lnet/minecraft/world/level/Explosion;)Z
public startSeenByPlayer(Lnet/minecraft/server/level/ServerPlayer;)V
public stopSeenByPlayer(Lnet/minecraft/server/level/ServerPlayer;)V
public rotate(Lnet/minecraft/world/level/block/Rotation;)F
public mirror(Lnet/minecraft/world/level/block/Mirror;)F
public deflection(Lnet/minecraft/world/entity/projectile/Projectile;)Lnet/minecraft/world/entity/projectile/ProjectileDeflection;
public getControllingPassenger()Lnet/minecraft/world/entity/LivingEntity;
public final hasControllingPassenger()Z
public final getPassengers()Ljava/util/List;
public getFirstPassenger()Lnet/minecraft/world/entity/Entity;
public hasPassenger(Lnet/minecraft/world/entity/Entity;)Z
public hasPassenger(Ljava/util/function/Predicate;)Z
private getIndirectPassengersStream()Ljava/util/stream/Stream;
public getSelfAndPassengers()Ljava/util/stream/Stream;
public getPassengersAndSelf()Ljava/util/stream/Stream;
public getIndirectPassengers()Ljava/lang/Iterable;
public countPlayerPassengers()I
public hasExactlyOnePlayerPassenger()Z
public getRootVehicle()Lnet/minecraft/world/entity/Entity;
public isPassengerOfSameVehicle(Lnet/minecraft/world/entity/Entity;)Z
public hasIndirectPassenger(Lnet/minecraft/world/entity/Entity;)Z
public final isLocalInstanceAuthoritative()Z
protected isLocalClientAuthoritative()Z
public isClientAuthoritative()Z
public getMoveSimulationType()Lnet/minecraft/world/entity/MoveSimulationType;
public final canSimulateMovement()Z
public isEffectiveAi()Z
protected static getCollisionHorizontalEscapeVector(DDF)Lnet/minecraft/world/phys/Vec3;
public getDismountLocationForPassenger(Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/phys/Vec3;
public getVehicle()Lnet/minecraft/world/entity/Entity;
public getControlledVehicle()Lnet/minecraft/world/entity/Entity;
public getPistonPushReaction()Lnet/minecraft/world/level/material/PushReaction;
public getSoundSource()Lnet/minecraft/sounds/SoundSource;
protected getFireImmuneTicks()I
public createCommandSourceStackForNameResolution(Lnet/minecraft/server/level/ServerLevel;)Lnet/minecraft/commands/CommandSourceStack;
public lookAt(Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;Lnet/minecraft/world/phys/Vec3;)V
public getPreciseBodyRotation(F)F
public touchingUnloadedChunk()Z
public getFluidHeight(Lnet/minecraft/tags/TagKey;)D
public getFluidJumpThreshold()D
public final getBbWidth()F
public final getBbHeight()F
public getAddEntityPacket(Lnet/minecraft/server/level/ServerEntity;)Lnet/minecraft/network/protocol/Packet;
public getDimensions(Lnet/minecraft/world/entity/Pose;)Lnet/minecraft/world/entity/EntityDimensions;
public final getAttachments()Lnet/minecraft/world/entity/EntityAttachments;
public position()Lnet/minecraft/world/phys/Vec3;
public trackingPosition()Lnet/minecraft/world/phys/Vec3;
public blockPosition()Lnet/minecraft/core/BlockPos;
public getInBlockState()Lnet/minecraft/world/level/block/state/BlockState;
public chunkPosition()Lnet/minecraft/world/level/ChunkPos;
public getDeltaMovement()Lnet/minecraft/world/phys/Vec3;
public setDeltaMovement(Lnet/minecraft/world/phys/Vec3;)V
public addDeltaMovement(Lnet/minecraft/world/phys/Vec3;)V
public setDeltaMovement(DDD)V
public final getBlockX()I
public final getX()D
public getX(D)D
public getRandomX(D)D
public final getBlockY()I
public final getY()D
public getY(D)D
public getRandomY(D)D
public getRandomY()D
public getEyeY()D
public final getBlockZ()I
public final getZ()D
public getZ(D)D
public getRandomZ(D)D
public final setPosRaw(DDD)V
public checkDespawn()V
public getQuadLeashHolderOffsets()[Lnet/minecraft/world/phys/Vec3;
public supportQuadLeashAsHolder()Z
public notifyLeashHolder(Lnet/minecraft/world/entity/Leashable;)V
public notifyLeasheeRemoved(Lnet/minecraft/world/entity/Leashable;)V
public getRopeHoldPosition(F)Lnet/minecraft/world/phys/Vec3;
public recreateFromPacket(Lnet/minecraft/network/protocol/game/ClientboundAddEntityPacket;)V
public getPickResult()Lnet/minecraft/world/item/ItemStack;
public setIsInPowderSnow(Z)V
public canFreeze()Z
public isFreezing()Z
public getYRot()F
public getVisualRotationYInDegrees()F
public setYRot(F)V
public getXRot()F
public setXRot(F)V
public storePositionAndRotation()Lnet/minecraft/core/PositionAndRotation;
public getClientPositionAndRotation()Lnet/minecraft/core/PositionAndRotation;
public getClientPosition()Lnet/minecraft/world/phys/Vec3;
public canSprint()Z
public maxUpStep()F
public onExplosionHit(Lnet/minecraft/world/entity/Entity;)V
public final isRemoved()Z
public getRemovalReason()Lnet/minecraft/world/entity/Entity$RemovalReason;
public final setRemoved(Lnet/minecraft/world/entity/Entity$RemovalReason;)V
protected unsetRemoved()V
public setLevelCallback(Lnet/minecraft/world/level/entity/EntityInLevelCallback;)V
public shouldBeSaved()Z
public isAlwaysTicking()Z
public mayInteract(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;)Z
public isFlyingVehicle()Z
public level()Lnet/minecraft/world/level/Level;
protected setLevel(Lnet/minecraft/world/level/Level;)V
public damageSources()Lnet/minecraft/world/damagesource/DamageSources;
public registryAccess()Lnet/minecraft/core/RegistryAccess;
protected lerpPositionAndRotationStep(IDDDDD)V
public getRandom()Lnet/minecraft/util/RandomSource;
public getKnownMovement()Lnet/minecraft/world/phys/Vec3;
public getKnownSpeed()Lnet/minecraft/world/phys/Vec3;
public getWeaponItem()Lnet/minecraft/world/item/ItemStack;
public getLootTable()Ljava/util/Optional;
protected applyImplicitComponents(Lnet/minecraft/core/component/DataComponentGetter;)V
public final applyComponentsFromItemStack(Lnet/minecraft/world/item/ItemStack;)V
public get(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;
protected static castComponentValue(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Ljava/lang/Object;
public setComponent(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)V
protected applyImplicitComponent(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Z
protected applyImplicitComponentIfPresent(Lnet/minecraft/core/component/DataComponentGetter;Lnet/minecraft/core/component/DataComponentType;)Z
public problemPath()Lnet/minecraft/util/ProblemReporter$PathElement;
public registerDebugValues(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/util/debug/DebugValueSource$Registration;)V
public getFluidInteractionBox()Lnet/minecraft/world/phys/AABB;
public setInvulnerableTime(I)V
public getInvulnerableTime()I
protected modifyPassengerFluidInteractionBox(Lnet/minecraft/world/phys/AABB;)Lnet/minecraft/world/phys/AABB;
private static synthetic lambda$countPlayerPassengers$0(Lnet/minecraft/world/entity/Entity;)Z
private synthetic lambda$getIndirectPassengers$0()Ljava/util/Iterator;
private static synthetic lambda$teleportPassengers$0(Lnet/minecraft/world/entity/Entity;)V
private synthetic lambda$getDisplayName$0(Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private synthetic lambda$fillCrashReportCategory$3()Ljava/lang/String;
private synthetic lambda$fillCrashReportCategory$2()Ljava/lang/String;
private synthetic lambda$fillCrashReportCategory$1()Ljava/lang/String;
private synthetic lambda$fillCrashReportCategory$0()Ljava/lang/String;
private static synthetic lambda$sendTeleportTransitionToPlayers$0(Ljava/util/List;Lnet/minecraft/server/level/ServerPlayer;)Z
private static synthetic lambda$removePassenger$0(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$startRiding$1(Lnet/minecraft/world/entity/Entity;)V
private static synthetic lambda$startRiding$0(Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$interact$0(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/Leashable;)Z
private synthetic lambda$isInWall$0(Lnet/minecraft/world/phys/AABB;Lnet/minecraft/core/BlockPos;)Z
private static synthetic lambda$getEncodeId$0()Ljava/lang/IllegalStateException;
private synthetic lambda$load$1(Ljava/lang/String;)V
private synthetic lambda$load$0(Ljava/util/UUID;)V
private synthetic lambda$checkFallDamage$0(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
private synthetic lambda$checkInsideBlocks$0(ILjava/util/concurrent/atomic/AtomicInteger;ZLnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lit/unimi/dsi/fastutil/longs/LongSet;ZLnet/minecraft/world/phys/AABB;Lnet/minecraft/world/entity/InsideBlockEffectApplier$StepBasedCollector;Lnet/minecraft/core/BlockPos;I)Z
static <clinit>()V
```
