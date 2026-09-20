---
type: "interface"
fqcn: "net.minecraft.world.entity.vehicle.minecart.AbstractMinecart"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.vehicle.minecart.AbstractMinecart

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`abstract_class` public abstract; extends `net/minecraft/world/entity/vehicle/VehicleEntity`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getType` | `()Lnet/minecraft/world/entity/EntityType;` | inherited_exact | invokevirtual@64 in `DetectorRailBlockMixin.getCustomComparatorOutput` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (10 fields, 65 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOWERED_PASSENGER_ATTACHMENT : Lnet/minecraft/world/phys/Vec3;
private static final DATA_ID_CUSTOM_DISPLAY_BLOCK : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_ID_DISPLAY_OFFSET : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final POSE_DISMOUNT_HEIGHTS : Lcom/google/common/collect/ImmutableMap;
protected static final WATER_SLOWDOWN_FACTOR : F
private static final DEFAULT_FLIPPED_ROTATION : Z
private onRails : Z
private flipped : Z
private final behavior : Lnet/minecraft/world/entity/vehicle/minecart/MinecartBehavior;
private static final EXITS : Ljava/util/Map;
protected <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V
protected <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;DDD)V
public setInitialPos(DDD)V
public static createMinecart(Lnet/minecraft/world/level/Level;DDDLnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/entity/vehicle/minecart/AbstractMinecart;
public getBehavior()Lnet/minecraft/world/entity/vehicle/minecart/MinecartBehavior;
protected getMovementEmission()Lnet/minecraft/world/entity/Entity$MovementEmission;
protected defineSynchedData(Lnet/minecraft/network/syncher/SynchedEntityData$Builder;)V
public canCollideWith(Lnet/minecraft/world/entity/Entity;)Z
public isPushable()Z
public getRelativePortalPosition(Lnet/minecraft/core/Direction$Axis;Lnet/minecraft/util/BlockUtil$FoundRectangle;)Lnet/minecraft/world/phys/Vec3;
protected getPassengerAttachmentPoint(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/EntityDimensions;F)Lnet/minecraft/world/phys/Vec3;
public getDismountLocationForPassenger(Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/phys/Vec3;
protected getBlockSpeedFactor()F
public animateHurt(F)V
public isPickable()Z
public static exits(Lnet/minecraft/world/level/block/state/properties/RailShape;)Lcom/mojang/datafixers/util/Pair;
public getMotionDirection()Lnet/minecraft/core/Direction;
protected getDefaultGravity()D
public tick()V
public isFirstTick()Z
public getCurrentBlockPosOrRailBelow()Lnet/minecraft/core/BlockPos;
protected getMaxSpeed(Lnet/minecraft/server/level/ServerLevel;)D
public activateMinecart(Lnet/minecraft/server/level/ServerLevel;IIIZ)V
public lerpPositionAndRotationStep(IDDDDD)V
public applyGravity()V
public reapplyPosition()V
public updateFluidInteraction()Z
public getKnownMovement()Lnet/minecraft/world/phys/Vec3;
protected createInterpolationHandler()Lnet/minecraft/world/entity/InterpolationHandler;
public onInterpolationStart(Lnet/minecraft/world/entity/InterpolationHandler;)V
public recreateFromPacket(Lnet/minecraft/network/protocol/game/ClientboundAddEntityPacket;)V
public lerpMotion(Lnet/minecraft/world/phys/Vec3;)V
protected moveAlongTrack(Lnet/minecraft/server/level/ServerLevel;)V
protected comeOffTrack(Lnet/minecraft/server/level/ServerLevel;)V
protected getAirDrag()F
protected makeStepAlongTrack(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/properties/RailShape;D)D
public move(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;)V
public applyEffectsFromBlocks()V
public isOnRails()Z
public setOnRails(Z)V
public isFlipped()Z
public setFlipped(Z)V
public getRedstoneDirection(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/Vec3;
public isRedstoneConductor(Lnet/minecraft/core/BlockPos;)Z
protected applyNaturalSlowdown(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
protected readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V
protected addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V
public push(Lnet/minecraft/world/entity/Entity;)V
private pushOtherMinecart(Lnet/minecraft/world/entity/vehicle/minecart/AbstractMinecart;DD)V
public getDisplayBlockState()Lnet/minecraft/world/level/block/state/BlockState;
private getCustomDisplayBlockState()Ljava/util/Optional;
public getDefaultDisplayBlockState()Lnet/minecraft/world/level/block/state/BlockState;
public getDisplayOffset()I
public getDefaultDisplayOffset()I
public setCustomDisplayBlockState(Ljava/util/Optional;)V
public setDisplayOffset(I)V
public static useExperimentalMovement(Lnet/minecraft/world/level/Level;)Z
public abstract getPickResult()Lnet/minecraft/world/item/ItemStack;
public isRideable()Z
public isFurnace()Z
private static synthetic lambda$addAdditionalSaveData$0(Lnet/minecraft/world/level/storage/ValueOutput;Lnet/minecraft/world/level/block/state/BlockState;)V
private static synthetic lambda$static$0()Lcom/google/common/collect/ImmutableMap;
private synthetic lambda$getDismountLocationForPassenger$1(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
private synthetic lambda$getDismountLocationForPassenger$0(Lnet/minecraft/core/BlockPos$MutableBlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
static <clinit>()V
```
