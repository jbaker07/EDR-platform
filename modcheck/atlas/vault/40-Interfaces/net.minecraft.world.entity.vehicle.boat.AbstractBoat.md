---
type: "interface"
fqcn: "net.minecraft.world.entity.vehicle.boat.AbstractBoat"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.vehicle.boat.AbstractBoat

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`abstract_class` public abstract; extends `net/minecraft/world/entity/vehicle/VehicleEntity`; implements `net/minecraft/world/entity/Leashable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `checkInWater` | `()Z` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| wraps | `isUnderwater` | `()Lnet/minecraft/world/entity/vehicle/boat/AbstractBoat$Status;` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (28 fields, 63 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DATA_ID_PADDLE_LEFT : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_ID_PADDLE_RIGHT : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_ID_BUBBLE_TIME : Lnet/minecraft/network/syncher/EntityDataAccessor;
public static final PADDLE_LEFT : I
public static final PADDLE_RIGHT : I
private static final TIME_TO_EJECT : I
private static final PADDLE_SPEED : F
public static final PADDLE_SOUND_TIME : D
public static final BUBBLE_TIME : I
private final paddlePositions : [F
private outOfControlTicks : F
private deltaRotation : F
private inputLeft : Z
private inputRight : Z
private inputUp : Z
private inputDown : Z
private waterLevel : D
private landFriction : F
private status : Lnet/minecraft/world/entity/vehicle/boat/AbstractBoat$Status;
private oldStatus : Lnet/minecraft/world/entity/vehicle/boat/AbstractBoat$Status;
private lastYd : D
private isAboveBubbleColumn : Z
private bubbleColumnDirectionIsDown : Z
private bubbleMultiplier : F
private bubbleAngle : F
private bubbleAngleO : F
private leashData : Lnet/minecraft/world/entity/Leashable$LeashData;
private final dropItem : Ljava/util/function/Supplier;
public <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;Ljava/util/function/Supplier;)V
public setInitialPos(DDD)V
protected getMovementEmission()Lnet/minecraft/world/entity/Entity$MovementEmission;
protected defineSynchedData(Lnet/minecraft/network/syncher/SynchedEntityData$Builder;)V
public canCollideWith(Lnet/minecraft/world/entity/Entity;)Z
public static canVehicleCollide(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;)Z
public canBeCollidedWith(Lnet/minecraft/world/entity/Entity;)Z
public isPushable()Z
public getRelativePortalPosition(Lnet/minecraft/core/Direction$Axis;Lnet/minecraft/util/BlockUtil$FoundRectangle;)Lnet/minecraft/world/phys/Vec3;
protected abstract rideHeight(Lnet/minecraft/world/entity/EntityDimensions;)D
protected getPassengerAttachmentPoint(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/EntityDimensions;F)Lnet/minecraft/world/phys/Vec3;
public onAboveBubbleColumn(ZLnet/minecraft/core/BlockPos;)V
public push(Lnet/minecraft/world/entity/Entity;)V
public animateHurt(F)V
public isPickable()Z
protected createInterpolationHandler()Lnet/minecraft/world/entity/InterpolationHandler;
public getMotionDirection()Lnet/minecraft/core/Direction;
public tick()V
private tickBubbleColumn()V
public handleEntityEvent(B)V
protected handleBubbleColumnEffect(Z)V
protected getPaddleSound()Lnet/minecraft/sounds/SoundEvent;
public setPaddleState(ZZ)V
public getRowingTime(IF)F
public getLeashData()Lnet/minecraft/world/entity/Leashable$LeashData;
public setLeashData(Lnet/minecraft/world/entity/Leashable$LeashData;)V
public getLeashOffset()Lnet/minecraft/world/phys/Vec3;
public supportQuadLeash()Z
public getQuadLeashOffsets()[Lnet/minecraft/world/phys/Vec3;
private getStatus()Lnet/minecraft/world/entity/vehicle/boat/AbstractBoat$Status;
public getWaterLevelAbove()F
public getGroundFriction()F
private checkInWater()Z
private isUnderwater()Lnet/minecraft/world/entity/vehicle/boat/AbstractBoat$Status;
protected getDefaultGravity()D
private floatBoat()V
protected getAirDrag()F
private controlBoat()V
protected getSinglePassengerXOffset()F
public hasEnoughSpaceFor(Lnet/minecraft/world/entity/Entity;)Z
protected positionRider(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity$MoveFunction;)V
public getDismountLocationForPassenger(Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/phys/Vec3;
protected clampRotation(Lnet/minecraft/world/entity/Entity;)F
private calculatePassengerBodyYRot(Lnet/minecraft/world/entity/Entity;)F
public onPassengerTurned(Lnet/minecraft/world/entity/Entity;)V
protected addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V
protected readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V
public interact(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/InteractionResult;
public remove(Lnet/minecraft/world/entity/Entity$RemovalReason;)V
protected checkFallDamage(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V
public getPaddleState(I)Z
private setBubbleTime(I)V
private getBubbleTime()I
public getBubbleAngle(F)F
protected canAddPassenger(Lnet/minecraft/world/entity/Entity;)Z
protected getMaxPassengers()I
public getControllingPassenger()Lnet/minecraft/world/entity/LivingEntity;
public setInput(ZZZZ)V
public isUnderWater()Z
protected final getDropItem()Lnet/minecraft/world/item/Item;
public final getPickResult()Lnet/minecraft/world/item/ItemStack;
protected modifyPassengerFluidInteractionBox(Lnet/minecraft/world/phys/AABB;)Lnet/minecraft/world/phys/AABB;
static <clinit>()V
```
