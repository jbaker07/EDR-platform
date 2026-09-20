---
type: "interface"
fqcn: "net.minecraft.world.entity.LivingEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.LivingEntity

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`abstract_class` public abstract; extends `net/minecraft/world/entity/Entity`; implements `net/minecraft/world/entity/Attackable`, `net/minecraft/world/waypoints/WaypointTransmitter`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getActiveEffects` | `()Ljava/util/Collection;` | exact | invokevirtual@12 in `LivingEntityMixin.beforeRemoveAllEffects` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getAttributeValue` | `(Lnet/minecraft/core/Holder;)D` | exact | invokevirtual@27 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getAttributeValue` | `(Lnet/minecraft/core/Holder;)D` | exact | invokevirtual@7 in `SimpleConfiguredFluidBehavior.lambda$static$0` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getDeltaMovement` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@12 in `FluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getDeltaMovement` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@20 in `FluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getDeltaMovement` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@24 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getDeltaMovement` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@37 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getDeltaMovement` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@88 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getDeltaMovement` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@162 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getDeltaMovement` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@24 in `SimpleConfiguredFluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getDeltaMovement` | `()Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@37 in `SimpleConfiguredFluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getEffect` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/effect/MobEffectInst` | exact | invokevirtual@33 in `LivingEntityMixin.allowRemoveEffect` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getEffect` | `(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/effect/MobEffectInst` | exact | invokevirtual@13 in `LivingEntityMixin.beforeRemoveEffect` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getFluidFallingAdjustedMovement` | `(DZLnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;` | exact | invokevirtual@133 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getFluidHeight` | `(Lnet/minecraft/tags/TagKey;)D` | inherited_exact | invokevirtual@42 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getFluidHeight` | `(Lnet/minecraft/tags/TagKey;)D` | inherited_exact | invokevirtual@149 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getFluidJumpThreshold` | `()D` | inherited_exact | invokevirtual@46 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getFluidJumpThreshold` | `()D` | inherited_exact | invokevirtual@153 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getSleepingPos` | `()Ljava/util/Optional;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | declared |
| calls | `getSpeed` | `()F` | exact | invokevirtual@32 in `SimpleConfiguredFluidBehavior.lambda$static$0` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `hasEffect` | `(Lnet/minecraft/core/Holder;)Z` | exact | invokevirtual@71 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `hasInfiniteMaterials` | `()Z` | exact | invokevirtual@21 in `ItemStackMixin.hookDamage` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `is` | `(Lnet/minecraft/tags/TagKey;)Z` | inherited_exact | invokevirtual@130 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isDeadOrDying` | `()Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | declared |
| calls | `isEyeInFluid` | `(Lnet/minecraft/tags/TagKey;)Z` | inherited_exact | invokevirtual@2 in `SimpleConfiguredFluidBehavior.lambda$static$2` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isSprinting` | `()Z` | inherited_exact | invokevirtual@1 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `isVehicle` | `()Z` | inherited_exact | invokevirtual@141 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@21 in `LivingEntityMixin.onIsSleepingInBed` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@15 in `FluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@27 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/world/entity/MoverType;Lnet/minecraft/world/phys/Vec3;` | inherited_exact | invokevirtual@27 in `SimpleConfiguredFluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `moveRelative` | `(FLnet/minecraft/world/phys/Vec3;)V` | inherited_exact | invokevirtual@4 in `FluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `moveRelative` | `(FLnet/minecraft/world/phys/Vec3;)V` | inherited_exact | invokevirtual@16 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `moveRelative` | `(FLnet/minecraft/world/phys/Vec3;)V` | inherited_exact | invokevirtual@16 in `SimpleConfiguredFluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `onClimbable` | `()Z` | exact | invokevirtual@89 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `onGround` | `()Z` | inherited_exact | invokevirtual@34 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `onGround` | `()Z` | inherited_exact | invokevirtual@13 in `SimpleConfiguredFluidBehavior.lambda$static$0` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `setDeltaMovement` | `(Lnet/minecraft/world/phys/Vec3;)V` | inherited_exact | invokevirtual@29 in `FluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `setDeltaMovement` | `(Lnet/minecraft/world/phys/Vec3;)V` | inherited_exact | invokevirtual@67 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `setDeltaMovement` | `(Lnet/minecraft/world/phys/Vec3;)V` | inherited_exact | invokevirtual@105 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `setDeltaMovement` | `(Lnet/minecraft/world/phys/Vec3;)V` | inherited_exact | invokevirtual@173 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `setDeltaMovement` | `(Lnet/minecraft/world/phys/Vec3;)V` | inherited_exact | invokevirtual@48 in `SimpleConfiguredFluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `addEffect` | `(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/en` | exact | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `aiStep` | `()V` | name_only | @ModifyExpressionValue at ['MIXINEXTRAS:EXPRESSION'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `aiStep` | `()V` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `aiStep` | `()V` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `aiStep` | `()V` | name_only | @ModifyArg at ['MIXINEXTRAS:EXPRESSION'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `baseTick` | `()V` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `canGlide` | `()Z` | name_only | @Inject at ['FIELD'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `checkBedExists` | `()Z` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `checkFallDamage` | `(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/co` | name_only | @ModifyExpressionValue at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `checkFallDamage` | `(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/co` | name_only | @ModifyExpressionValue at ['NEW'] | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| injects_into | `collectEquipmentChanges` | `(Ljava/util/Map;)Ljava/util/Map;` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `die` | `(Lnet/minecraft/world/damagesource/DamageSource;)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `forceAddEffect` | `(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/en` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `getEquipmentSlotForItem` | `(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/entity/Equi` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `hurtServer` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damageso` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `hurtServer` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damageso` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `lambda$stopSleeping$0` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @ModifyVariable at ['INVOKE_ASSIGN'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `onEffectAdded` | `(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/en` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `onEffectsRemoved` | `(Ljava/util/Collection;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `removeAllEffects` | `()Z` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `removeEffect` | `(Lnet/minecraft/core/Holder;)Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `startSleeping` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `startSleeping` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @ModifyVariable at ['INVOKE_ASSIGN'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `stopSleeping` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `tickEffects` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `trapdoorUsableAsLadder` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/B` | name_only | @Inject at ['INVOKE_ASSIGN'] | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| injects_into | `travelFlying` | `(Lnet/minecraft/world/phys/Vec3;FFF)V` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `updateFallFlying` | `()V` | exact | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| reads | `horizontalCollision` | `Z` | inherited_exact | getfield@82 in `SimpleConfiguredFluidBehavior.lambda$static$1` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| wraps | `canBeAffected` | `(Lnet/minecraft/world/effect/MobEffectInstance;)Z` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `die` | `(Lnet/minecraft/world/damagesource/DamageSource;)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `getBedOrientation` | `()Lnet/minecraft/core/Direction;` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `hurtServer` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damageso` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `lambda$stopSleeping$0` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `lambda$stopSleeping$0` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `removeAllEffects` | `()Z` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `removeEffect` | `(Lnet/minecraft/core/Holder;)Z` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `startSleeping` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| wraps | `travelInFluid` | `(Lnet/minecraft/world/phys/Vec3;)V` | name_only | @WrapWithCondition at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (124 fields, 409 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final TAG_ACTIVE_EFFECTS : Ljava/lang/String;
public static final TAG_ATTRIBUTES : Ljava/lang/String;
public static final TAG_SLEEPING_POS : Ljava/lang/String;
public static final TAG_EQUIPMENT : Ljava/lang/String;
public static final TAG_BRAIN : Ljava/lang/String;
public static final TAG_FALL_FLYING : Ljava/lang/String;
public static final TAG_HURT_TIME : Ljava/lang/String;
public static final TAG_DEATH_TIME : Ljava/lang/String;
public static final TAG_HEALTH : Ljava/lang/String;
private static final SPEED_MODIFIER_POWDER_SNOW_ID : Lnet/minecraft/resources/Identifier;
private static final SPRINTING_MODIFIER_ID : Lnet/minecraft/resources/Identifier;
private static final SPEED_MODIFIER_SPRINTING : Lnet/minecraft/world/entity/ai/attributes/AttributeModifier;
public static final EQUIPMENT_SLOT_OFFSET : I
public static final ARMOR_SLOT_OFFSET : I
public static final BODY_ARMOR_OFFSET : I
public static final SADDLE_OFFSET : I
public static final PLAYER_HURT_EXPERIENCE_TIME : I
private static final DAMAGE_SOURCE_TIMEOUT : I
public static final MIN_MOVEMENT_DISTANCE : D
public static final DEFAULT_BASE_GRAVITY : D
public static final DEATH_DURATION : I
protected static final INPUT_FRICTION : F
private static final TICKS_PER_ELYTRA_FREE_FALL_EVENT : I
private static final FREE_FALL_EVENTS_PER_ELYTRA_BREAK : I
public static final BASE_JUMP_POWER : F
protected static final DEFAULT_KNOCKBACK : F
protected static final DAMAGE_COOLDOWN_DURATION : I
protected static final HURT_DURATION_TICKS : I
private static final CLIMBING_VERTICAL_SPEED : D
private static final SWIM_AMOUNT_PER_TICK : F
private static final MAX_LINE_OF_SIGHT_TEST_RANGE : D
protected static final LIVING_ENTITY_FLAG_IS_USING : I
protected static final LIVING_ENTITY_FLAG_OFF_HAND : I
protected static final LIVING_ENTITY_FLAG_SPIN_ATTACK : I
protected static final DATA_LIVING_ENTITY_FLAGS : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_HEALTH_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_EFFECT_PARTICLES : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_EFFECT_AMBIENCE_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_ARROW_COUNT_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_STINGER_COUNT_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final SLEEPING_POS_ID : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final PARTICLE_FREQUENCY_WHEN_INVISIBLE : I
protected static final SLEEPING_DIMENSIONS : Lnet/minecraft/world/entity/EntityDimensions;
public static final EXTRA_RENDER_CULLING_SIZE_WITH_BIG_HAT : F
public static final DEFAULT_BABY_SCALE : F
protected static final SWIMMING_VERTICAL_SPEED : F
private static final CURRENT_IMPULSE_CONTEXT_RESET_GRACE_TIME_TICKS : I
private static final DEFAULT_CURRENT_IMPULSE_CONTEXT_RESET_GRACE_TIME : I
public static final BASE_HORIZONTAL_AIR_DRAG : F
public static final BASE_VERTICAL_AIR_DRAG : F
public static final WATER_DRAG : F
public static final SPRINTING_WATER_DRAG : F
public static final LAVA_DRAG : F
public static final LAVA_SHALLOW_VERTICAL_DRAG : F
public static final DOLPHINS_GRACE_WATER_DRAG : F
public static final FLYING_AIR_DRAG : F
public static final FLYING_VERTICAL_AIR_DRAG : F
public static final FLYING_LAVA_DRAG : F
public static final FLYING_WATER_DRAG : F
public static final ELYTRA_HORIZONTAL_AIR_DRAG : F
public static final ELYTRA_VERTICAL_AIR_DRAG : F
public static final BASE_SWIM_SPEED : F
private currentImpulseContextResetGraceTime : I
public static final PLAYER_NOT_WEARING_DISGUISE_ITEM : Ljava/util/function/Predicate;
private final attributes : Lnet/minecraft/world/entity/ai/attributes/AttributeMap;
private final combatTracker : Lnet/minecraft/world/damagesource/CombatTracker;
private final activeEffects : Ljava/util/Map;
private final lastEquipmentItems : Ljava/util/Map;
private final swingState : Lnet/minecraft/world/entity/LivingEntity$SwingState;
private discardFriction : Z
public removeArrowTime : I
public removeStingerTime : I
public hurtTime : I
public hurtDuration : I
public deathTime : I
public damageCooldownTime : I
protected attackStrengthTicker : I
protected itemSwapTicker : I
public final walkAnimation : Lnet/minecraft/world/entity/WalkAnimationState;
public yBodyRot : F
public yBodyRotO : F
public yHeadRot : F
public yHeadRotO : F
public final elytraAnimationState : Lnet/minecraft/world/entity/ElytraAnimationState;
protected lastHurtByPlayer : Lnet/minecraft/world/entity/EntityReference;
protected lastHurtByPlayerMemoryTime : I
protected dead : Z
protected noActionTime : I
protected lastHurt : F
protected jumping : Z
public xxa : F
public yya : F
public zza : F
protected lerpYHeadRot : D
protected lerpHeadSteps : I
private effectsDirty : Z
private lastHurtByMob : Lnet/minecraft/world/entity/EntityReference;
private lastHurtByMobTimestamp : I
private lastHurtMob : Lnet/minecraft/world/entity/LivingEntity;
private lastHurtMobTimestamp : I
private speed : F
private noJumpDelay : I
private absorptionAmount : F
protected useItem : Lnet/minecraft/world/item/ItemStack;
protected useItemRemaining : I
protected fallFlyTicks : I
private lastKineticHitFeedbackTime : J
private lastPos : Lnet/minecraft/core/BlockPos;
private lastClimbablePos : Ljava/util/Optional;
private lastDamageSource : Lnet/minecraft/world/damagesource/DamageSource;
private lastDamageStamp : J
protected autoSpinAttackTicks : I
protected autoSpinAttackDmg : F
protected autoSpinAttackItemStack : Lnet/minecraft/world/item/ItemStack;
protected recentKineticEnemies : Lit/unimi/dsi/fastutil/objects/Object2LongMap;
private swimAmount : F
private swimAmountO : F
protected brain : Lnet/minecraft/world/entity/ai/Brain;
private skipDropExperience : Z
private final activeLocationDependentEnchantments : Ljava/util/EnumMap;
protected final equipment : Lnet/minecraft/world/entity/EntityEquipment;
private locatorBarIcon : Lnet/minecraft/world/waypoints/Waypoint$Icon;
public currentImpulseImpactPos : Lnet/minecraft/world/phys/Vec3;
protected <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V
public asLivingEntity()Lnet/minecraft/world/entity/LivingEntity;
protected createEquipment()Lnet/minecraft/world/entity/EntityEquipment;
public getBrain()Lnet/minecraft/world/entity/ai/Brain;
protected makeBrain(Lnet/minecraft/world/entity/ai/Brain$Packed;)Lnet/minecraft/world/entity/ai/Brain;
public kill(Lnet/minecraft/server/level/ServerLevel;)V
protected defineSynchedData(Lnet/minecraft/network/syncher/SynchedEntityData$Builder;)V
public static createLivingAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
protected checkFallDamage(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V
public canBreatheUnderwater()Z
public getSwimAmount(F)F
public hasLandedInLiquid()Z
public baseTick()V
protected shouldTakeDrowningDamage()Z
protected getBlockSpeedFactor()F
private static computeModifiedFriction(FF)F
public getLuck()F
protected removeFrost()V
protected tryAddFrost()V
protected onChangedBlock(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;)V
public isBaby()Z
public getAgeScale()F
public final getScale()F
protected sanitizeScale(F)F
public isAffectedByFluids()Z
protected tickDeath()V
public shouldDropExperience()Z
protected shouldDropLoot(Lnet/minecraft/server/level/ServerLevel;)Z
protected decreaseAirSupply(I)I
protected increaseAirSupply(I)I
public final getExperienceReward(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;)I
protected getBaseExperienceReward(Lnet/minecraft/server/level/ServerLevel;)I
protected isAlwaysExperienceDropper()Z
public getLastHurtByMob()Lnet/minecraft/world/entity/LivingEntity;
public getLastHurtByPlayer()Lnet/minecraft/world/entity/player/Player;
public getLastAttacker()Lnet/minecraft/world/entity/LivingEntity;
public getLastHurtByMobTimestamp()I
public setLastHurtByPlayer(Lnet/minecraft/world/entity/player/Player;I)V
public setLastHurtByPlayer(Ljava/util/UUID;I)V
private setLastHurtByPlayer(Lnet/minecraft/world/entity/EntityReference;I)V
public setLastHurtByMob(Lnet/minecraft/world/entity/LivingEntity;)V
public getLastHurtMob()Lnet/minecraft/world/entity/LivingEntity;
public getLastHurtMobTimestamp()I
public setLastHurtMob(Lnet/minecraft/world/entity/Entity;)V
public getNoActionTime()I
public setNoActionTime(I)V
public shouldDiscardFriction()Z
public setDiscardFriction(Z)V
protected doesEmitEquipEvent(Lnet/minecraft/world/entity/EquipmentSlot;)Z
public onEquipItem(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)V
protected getEquipSound(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/equipment/Equippable;)Lnet/minecraft/core/Holder;
public remove(Lnet/minecraft/world/entity/Entity$RemovalReason;)V
public onRemoval(Lnet/minecraft/world/entity/Entity$RemovalReason;)V
protected triggerOnDeathMobEffects(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity$RemovalReason;)V
protected addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V
public drop(Lnet/minecraft/world/item/ItemStack;ZLnet/minecraft/util/Prediction;)Lnet/minecraft/world/entity/item/ItemEntity;
protected readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V
public updateDataBeforeSync()V
protected tickEffects()V
private updateDirtyEffects()V
protected updateInvisibilityStatus()V
private updateSynchronizedMobEffectParticles()V
private updateGlowingStatus()V
public getVisibilityPercent(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;)D
public canAttack(Lnet/minecraft/world/entity/LivingEntity;)Z
public canBeSeenAsEnemy()Z
public canBeSeenByAnyone()Z
public static areAllEffectsAmbient(Ljava/util/Collection;)Z
protected removeEffectParticles()V
public removeAllEffects()Z
public getActiveEffects()Ljava/util/Collection;
public getActiveEffectsMap()Ljava/util/Map;
public hasEffect(Lnet/minecraft/core/Holder;)Z
public getEffect(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/effect/MobEffectInstance;
public getEffectBlendFactor(Lnet/minecraft/core/Holder;F)F
public final addEffect(Lnet/minecraft/world/effect/MobEffectInstance;)Z
public addEffect(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/entity/Entity;)Z
public canBeAffected(Lnet/minecraft/world/effect/MobEffectInstance;)Z
public forceAddEffect(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/entity/Entity;)V
public isInvertedHealAndHarm()Z
public final removeEffectNoUpdate(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/effect/MobEffectInstance;
public removeEffect(Lnet/minecraft/core/Holder;)Z
protected onEffectAdded(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/entity/Entity;)V
public sendEffectToPassengers(Lnet/minecraft/world/effect/MobEffectInstance;)V
protected onEffectUpdated(Lnet/minecraft/world/effect/MobEffectInstance;ZLnet/minecraft/world/entity/Entity;)V
protected onEffectsRemoved(Ljava/util/Collection;)V
private refreshDirtyAttributes()V
protected onAttributeUpdated(Lnet/minecraft/core/Holder;)V
public heal(F)V
public getHealth()F
public setHealth(F)V
public isDeadOrDying()Z
public wasHurtRecently()Z
public hurtServer(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)Z
public dealDefaultKnockback(Lnet/minecraft/world/damagesource/DamageSource;FZ)V
public applyItemBlocking(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)F
private playSecondaryHurtSound(Lnet/minecraft/world/damagesource/DamageSource;)V
protected resolveMobResponsibleForDamage(Lnet/minecraft/world/damagesource/DamageSource;)V
protected resolvePlayerResponsibleForDamage(Lnet/minecraft/world/damagesource/DamageSource;)Lnet/minecraft/world/entity/player/Player;
protected blockUsingItem(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;FZ)V
protected blockedByItem(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;FZ)V
private checkTotemDeathProtection(Lnet/minecraft/world/damagesource/DamageSource;)Z
public getLastDamageSource()Lnet/minecraft/world/damagesource/DamageSource;
public getLastDamageSource(I)Lnet/minecraft/world/damagesource/DamageSource;
protected playHurtSound(Lnet/minecraft/world/damagesource/DamageSource;)V
public makeSound(Lnet/minecraft/sounds/SoundEvent;)V
private breakItem(Lnet/minecraft/world/item/ItemStack;)V
public die(Lnet/minecraft/world/damagesource/DamageSource;)V
protected handleKillingBlow()V
protected createWitherRose(Lnet/minecraft/world/entity/LivingEntity;)V
protected dropAllDeathLoot(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;)V
protected dropEquipment(Lnet/minecraft/server/level/ServerLevel;)V
protected dropExperience(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;)V
protected dropCustomDeathLoot(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;Z)V
public getLootTableSeed()J
protected getKnockback(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)F
protected dropFromLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;Z)V
public dropFromLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;ZLnet/minecraft/resources/ResourceKey;)V
public dropFromLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;ZLnet/minecraft/resources/ResourceKey;Ljava/util/function/Consumer;)V
public dropFromEntityInteractLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/ItemInstance;Ljava/util/function/BiConsumer;)Z
public dropFromGiftLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/resources/ResourceKey;Ljava/util/function/BiConsumer;)Z
protected dropFromShearingLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/item/ItemInstance;Ljava/util/function/BiConsumer;)V
protected dropFromLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/resources/ResourceKey;Ljava/util/function/Function;Ljava/util/function/BiConsumer;)Z
public knockback(DDDLnet/minecraft/world/damagesource/DamageSource;FZ)V
public knockback(DDDLnet/minecraft/world/damagesource/DamageSource;F)V
public indicateDamage(DD)V
protected getHurtSound(Lnet/minecraft/world/damagesource/DamageSource;)Lnet/minecraft/sounds/SoundEvent;
protected getDeathSound()Lnet/minecraft/sounds/SoundEvent;
private getFallDamageSound(I)Lnet/minecraft/sounds/SoundEvent;
public skipDropExperience()V
public wasExperienceConsumed()Z
public getHurtDir()F
protected getHitbox()Lnet/minecraft/world/phys/AABB;
public activeLocationDependentEnchantments(Lnet/minecraft/world/entity/EquipmentSlot;)Ljava/util/Map;
public postPiercingAttack()V
public getFallSounds()Lnet/minecraft/world/entity/LivingEntity$Fallsounds;
public getLastClimbablePos()Ljava/util/Optional;
public onClimbable()Z
private trapdoorUsableAsLadder(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z
public isAlive()Z
public isLookingAtMe(Lnet/minecraft/world/entity/LivingEntity;DZZ[D)Z
public getMaxFallDistance()I
protected final getComfortableFallDistance(F)I
public causeFallDamage(DFLnet/minecraft/world/damagesource/DamageSource;)Z
public setIgnoreFallDamageFromCurrentImpulse(ZLnet/minecraft/world/phys/Vec3;)V
public applyPostImpulseGraceTime(I)V
public isIgnoringFallDamageFromCurrentImpulse()Z
public tryResetCurrentImpulseContext()V
public isInPostImpulseGraceTime()Z
public resetCurrentImpulseContext()V
protected calculateFallDamage(DF)I
private calculateFallPower(D)D
protected playBlockFallSound()V
public animateHurt(F)V
public getArmorValue()I
public hurtArmor(Lnet/minecraft/world/damagesource/DamageSource;F)V
public hurtHelmet(Lnet/minecraft/world/damagesource/DamageSource;F)V
protected doHurtEquipment(Lnet/minecraft/world/damagesource/DamageSource;F[Lnet/minecraft/world/entity/EquipmentSlot;)V
protected getDamageAfterArmorAbsorb(Lnet/minecraft/world/damagesource/DamageSource;F)F
protected getDamageAfterMagicAbsorb(Lnet/minecraft/world/damagesource/DamageSource;F)F
protected actuallyHurt(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)V
public getCombatTracker()Lnet/minecraft/world/damagesource/CombatTracker;
public getKillCredit()Lnet/minecraft/world/entity/LivingEntity;
public final getMaxHealth()F
public final getMaxAbsorption()F
public final getArrowCount()I
public final setArrowCount(I)V
public final getStingerCount()I
public final setStingerCount(I)V
private getModifiedSwingDuration(Lnet/minecraft/world/item/component/SwingAnimation;)I
public swingAndResetAttackStrength(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/component/SwingAnimation;Z)V
public swing(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/component/SwingAnimation;Z)Z
public handleDamageEvent(Lnet/minecraft/world/damagesource/DamageSource;)V
public handleEntityEvent(B)V
public getTicksSinceLastKineticHitFeedback(F)F
public makePoofParticles()V
private makeDrownParticles()V
private onKineticHit()V
private swapHandItems()V
protected onBelowWorld()V
protected getEntityBounciness()D
public getAttribute(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/entity/ai/attributes/AttributeInstance;
public getAttributeValue(Lnet/minecraft/core/Holder;)D
public getAttributeBaseValue(Lnet/minecraft/core/Holder;)D
public getAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeMap;
public getMainHandItem()Lnet/minecraft/world/item/ItemStack;
public getOffhandItem()Lnet/minecraft/world/item/ItemStack;
public getItemHeldByArm(Lnet/minecraft/world/entity/HumanoidArm;)Lnet/minecraft/world/item/ItemStack;
public getWeaponItem()Lnet/minecraft/world/item/ItemStack;
public getAttackRangeWith(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/component/AttackRange;
public getActiveItem()Lnet/minecraft/world/item/ItemStack;
public isHolding(Lnet/minecraft/world/item/Item;)Z
public isHolding(Ljava/util/function/Predicate;)Z
public getItemInHand(Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/item/ItemStack;
public setItemInHand(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/item/ItemStack;)V
public hasItemInSlot(Lnet/minecraft/world/entity/EquipmentSlot;)Z
public canUseSlot(Lnet/minecraft/world/entity/EquipmentSlot;)Z
public getItemBySlot(Lnet/minecraft/world/entity/EquipmentSlot;)Lnet/minecraft/world/item/ItemStack;
public setItemSlot(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;)V
public getArmorCoverPercentage()F
public setSprinting(Z)V
protected getSoundVolume()F
public getVoicePitch()F
protected isImmobile()Z
public push(Lnet/minecraft/world/entity/Entity;)V
private dismountVehicle(Lnet/minecraft/world/entity/Entity;)V
public shouldShowName()Z
protected getJumpPower()F
protected getJumpPower(F)F
public getJumpBoostPower()F
public jumpFromGround()V
protected goDownInWater()V
protected jumpInLiquid(Lnet/minecraft/tags/TagKey;)V
protected getWaterSlowDown()F
public canStandOnFluid(Lnet/minecraft/world/level/material/FluidState;)Z
protected getDefaultGravity()D
protected getEffectiveGravity()D
public travel(Lnet/minecraft/world/phys/Vec3;)V
public getLiquidCollisionShape()Lnet/minecraft/world/phys/shapes/VoxelShape;
protected shouldTravelInFluid(Lnet/minecraft/world/level/material/FluidState;)Z
protected travelFlying(Lnet/minecraft/world/phys/Vec3;F)V
protected travelFlying(Lnet/minecraft/world/phys/Vec3;FFF)V
private travelInAir(Lnet/minecraft/world/phys/Vec3;)V
protected getAirDrag()F
protected travelInFluid(Lnet/minecraft/world/phys/Vec3;)V
protected travelInWater(Lnet/minecraft/world/phys/Vec3;DZD)V
protected isInShallowFluid(Lnet/minecraft/tags/TagKey;)Z
private travelInLava(Lnet/minecraft/world/phys/Vec3;DZD)V
private jumpOutOfFluid(D)V
private floatInLiquidWhileRidden(Lnet/minecraft/tags/TagKey;)V
public isInFluidDeeperThan(DLnet/minecraft/tags/TagKey;)Z
private travelFallFlying(Lnet/minecraft/world/phys/Vec3;)V
public stopFallFlying()V
private updateFallFlyingMovement(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
private handleFallFlyingCollisions(DD)V
private travelRidden(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/phys/Vec3;)V
protected tickRidden(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/phys/Vec3;)V
protected getRiddenInput(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
protected getRiddenSpeed(Lnet/minecraft/world/entity/player/Player;)F
public calculateEntityAnimation(Z)V
protected updateWalkAnimation(F)V
private handleRelativeFrictionAndCalculateMovement(Lnet/minecraft/world/phys/Vec3;F)Lnet/minecraft/world/phys/Vec3;
public getFluidFallingAdjustedMovement(DZLnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
private handleOnClimbable(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
private getFrictionInfluencedSpeed(F)F
protected getFlyingSpeed()F
public getSpeed()F
public setSpeed(F)V
public doHurtTarget(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;)Z
public causeExtraKnockback(Lnet/minecraft/world/entity/Entity;FLnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/damagesource/DamageSource;FZ)V
protected playAttackSound()V
public tick()V
public wasRecentlyStabbed(Lnet/minecraft/world/entity/Entity;I)Z
public rememberStabbedEntity(Lnet/minecraft/world/entity/Entity;)V
public stabbedEntities(Ljava/util/function/Predicate;)I
public stabAttack(Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/entity/Entity;FZZZ)Z
public onAttack()V
private detectEquipmentUpdates()V
protected collectEquipmentChanges(Ljava/util/Map;)Ljava/util/Map;
public equipmentHasChanged(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z
private handleHandSwap(Ljava/util/Map;)V
private handleEquipmentChanges(Ljava/util/Map;)V
private updatePlayersWithNewEquipment(Lnet/minecraft/server/level/ServerLevel;Ljava/util/List;)V
protected tickHeadTurn(F)V
protected getMaxHeadRotationRelativeToBody()F
public aiStep()V
protected applyInput()V
public isSensitiveToWater()Z
public isJumping()Z
protected updateFallFlying()V
protected canGlide()Z
protected serverAiStep()V
protected pushEntities()V
protected checkAutoSpinAttack(Lnet/minecraft/world/phys/AABB;Lnet/minecraft/world/phys/AABB;)V
protected doPush(Lnet/minecraft/world/entity/Entity;)V
protected doAutoAttackOnTouch(Lnet/minecraft/world/entity/LivingEntity;)V
public isAutoSpinAttack()Z
public stopRiding()V
public rideTick()V
protected createInterpolationHandler()Lnet/minecraft/world/entity/InterpolationHandler;
public lerpHeadTo(FI)V
public setJumping(Z)V
public onItemPickup(Lnet/minecraft/world/entity/item/ItemEntity;)V
public take(Lnet/minecraft/world/entity/Entity;I)V
public hasLineOfSight(Lnet/minecraft/world/entity/Entity;)Z
public hasLineOfSight(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/level/ClipContext$Block;Lnet/minecraft/world/level/ClipContext$Fluid;D)Z
public getViewYRot(F)F
public getCurrentSwing()Lnet/minecraft/world/entity/LivingEntity$SwingDescription;
public getSwingAnimation(F)F
public isSwinging()Z
public isPickable()Z
public isPushable()Z
public getYHeadRot()F
public setYHeadRot(F)V
public setYBodyRot(F)V
public getRelativePortalPosition(Lnet/minecraft/core/Direction$Axis;Lnet/minecraft/util/BlockUtil$FoundRectangle;)Lnet/minecraft/world/phys/Vec3;
public static resetForwardDirectionOfRelativePortalPosition(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
public getAbsorptionAmount()F
public final setAbsorptionAmount(F)V
protected internalSetAbsorptionAmount(F)V
public onEnterCombat()V
public onLeaveCombat()V
protected updateEffectVisibility()V
public abstract getMainArm()Lnet/minecraft/world/entity/HumanoidArm;
public isUsingItem()Z
public getUsedItemHand()Lnet/minecraft/world/InteractionHand;
private updatingUsingItem()V
public createItemStackToDrop(Lnet/minecraft/world/item/ItemStack;ZZ)Lnet/minecraft/world/entity/item/ItemEntity;
protected updateUsingItem(Lnet/minecraft/world/item/ItemStack;)V
private updateSwimAmount()V
protected setLivingEntityFlag(IZ)V
public startUsingItem(Lnet/minecraft/world/InteractionHand;)V
public onSyncedDataUpdated(Lnet/minecraft/network/syncher/EntityDataAccessor;)V
public lookAt(Lnet/minecraft/commands/arguments/EntityAnchorArgument$Anchor;Lnet/minecraft/world/phys/Vec3;)V
public getPreciseBodyRotation(F)F
public spawnItemParticles(Lnet/minecraft/world/item/ItemStack;I)V
protected completeUsingItem()V
public handleExtraItemsCreatedOnUse(Lnet/minecraft/world/item/ItemStack;)V
public getUseItem()Lnet/minecraft/world/item/ItemStack;
public getUseItemRemainingTicks()I
public getTicksUsingItem()I
public getTicksUsingItem(F)F
public releaseUsingItem()V
public stopUsingItem()V
public isBlocking()Z
public getItemBlockingWith()Lnet/minecraft/world/item/ItemStack;
public isSuppressingSlidingDownLadder()Z
public isFallFlying()Z
public isVisuallySwimming()Z
public getFallFlyingTicks()I
public randomTeleport(DDDZLnet/minecraft/tags/TagKey;)Z
public randomTeleport(DDDZLjava/util/function/Predicate;)Z
private checkPositionAndTeleport(DDDZLjava/util/function/Predicate;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;)Z
public canRandomlyTeleportTo(DDD)Z
public isAffectedByPotions()Z
public attackable()Z
public setRecordPlayingNearby(Lnet/minecraft/core/BlockPos;Z)V
public canPickUpLoot()Z
public final getDimensions(Lnet/minecraft/world/entity/Pose;)Lnet/minecraft/world/entity/EntityDimensions;
protected getDefaultDimensions(Lnet/minecraft/world/entity/Pose;)Lnet/minecraft/world/entity/EntityDimensions;
public getDismountPoses()Lcom/google/common/collect/ImmutableList;
public getLocalBoundsForPose(Lnet/minecraft/world/entity/Pose;)Lnet/minecraft/world/phys/AABB;
protected wouldNotSuffocateAtTargetPose(Lnet/minecraft/world/entity/Pose;)Z
public canUsePortal(Z)Z
public getSleepingPos()Ljava/util/Optional;
public setSleepingPos(Lnet/minecraft/core/BlockPos;)V
public clearSleepingPos()V
public isSleeping()Z
public startSleeping(Lnet/minecraft/core/BlockPos;)Z
private setPosToBed(Lnet/minecraft/core/BlockPos;)Z
private setPosToBed(DLnet/minecraft/core/BlockPos;)V
private checkBedExists()Z
public stopSleeping()V
public getBedOrientation()Lnet/minecraft/core/Direction;
public isInWall()Z
public getProjectile(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;
private static entityEventForEquipmentBreak(Lnet/minecraft/world/entity/EquipmentSlot;)B
public onEquippedItemBroken(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;)V
private stopLocationBasedEffects(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/entity/ai/attributes/AttributeMap;)V
public final canEquipWithDispenser(Lnet/minecraft/world/item/ItemStack;)Z
protected canDispenserEquipIntoSlot(Lnet/minecraft/world/entity/EquipmentSlot;)Z
public getEquipmentSlotForItem(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/entity/EquipmentSlot;
public isEquippableInSlot(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;)Z
private static createEquipmentSlotAccess(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/EquipmentSlot;)Lnet/minecraft/world/entity/SlotAccess;
private static getEquipmentSlot(I)Lnet/minecraft/world/entity/EquipmentSlot;
public getSlot(I)Lnet/minecraft/world/entity/SlotAccess;
public canFreeze()Z
public isCurrentlyGlowing()Z
public getVisualRotationYInDegrees()F
public recreateFromPacket(Lnet/minecraft/network/protocol/game/ClientboundAddEntityPacket;)V
public getSecondsToDisableBlocking()F
public maxUpStep()F
public getPassengerRidingPosition(Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/phys/Vec3;
protected lerpHeadRotationStep(ID)V
public igniteForTicks(I)V
public hasInfiniteMaterials()Z
public isInvulnerableTo(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;)Z
public static canGlideUsing(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;)Z
public getLastHurtByPlayerMemoryTime()I
public isTransmittingWaypoint()Z
public makeWaypointConnectionWith(Lnet/minecraft/server/level/ServerPlayer;)Ljava/util/Optional;
public waypointIcon()Lnet/minecraft/world/waypoints/Waypoint$Icon;
public createDamageSource()Lnet/minecraft/world/damagesource/DamageSource;
private static synthetic lambda$createEquipmentSlotAccess$0(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;)Z
private static synthetic lambda$stopLocationBasedEffects$0(Lnet/minecraft/world/entity/ai/attributes/AttributeMap;Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/ai/attributes/AttributeModifier;)V
private synthetic lambda$stopSleeping$0(Lnet/minecraft/core/BlockPos;)V
private static synthetic lambda$stopSleeping$1(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/Vec3;
private synthetic lambda$checkBedExists$0(Lnet/minecraft/core/BlockPos;)Ljava/lang/Boolean;
private static synthetic lambda$randomTeleport$0(Lnet/minecraft/tags/TagKey;Lnet/minecraft/world/level/block/state/BlockState;)Z
private synthetic lambda$updateFallFlying$0(Lnet/minecraft/world/entity/EquipmentSlot;)Z
private synthetic lambda$handleEquipmentChanges$0(Ljava/util/List;Lnet/minecraft/world/entity/EquipmentSlot;Lnet/minecraft/world/item/ItemStack;)V
private synthetic lambda$collectEquipmentChanges$0(Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/ai/attributes/AttributeModifier;)V
private static synthetic lambda$dismountVehicle$0(DLnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
private static synthetic lambda$isHolding$0(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/ItemStack;)Z
private static synthetic lambda$activeLocationDependentEnchantments$0(Lnet/minecraft/world/entity/EquipmentSlot;)Lit/unimi/dsi/fastutil/objects/Reference2ObjectMap;
private static synthetic lambda$dropFromLootTable$1(Ljava/util/function/BiConsumer;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)V
private synthetic lambda$dropFromShearingLootTable$0(Lnet/minecraft/world/item/ItemInstance;Lnet/minecraft/world/level/storage/loot/LootParams$Builder;)Lnet/minecraft/world/level/storage/loot/LootParams;
private synthetic lambda$dropFromGiftLootTable$0(Lnet/minecraft/world/level/storage/loot/LootParams$Builder;)Lnet/minecraft/world/level/storage/loot/LootParams;
private synthetic lambda$dropFromEntityInteractLootTable$0(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/ItemInstance;Lnet/minecraft/world/level/storage/loot/LootParams$Builder;)Lnet/minecraft/world/level/storage/loot/LootParams;
private synthetic lambda$dropFromLootTable$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$applyItemBlocking$0(Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/core/HolderSet;)Ljava/lang/Boolean;
private synthetic lambda$tickEffects$0(Lnet/minecraft/world/effect/MobEffectInstance;)V
private synthetic lambda$readAdditionalSaveData$1(Lnet/minecraft/world/entity/ai/Brain$Packed;)V
private synthetic lambda$readAdditionalSaveData$0(Lnet/minecraft/core/BlockPos;)V
private static synthetic lambda$addAdditionalSaveData$0(Lnet/minecraft/world/level/storage/ValueOutput;Lnet/minecraft/core/BlockPos;)V
private synthetic lambda$baseTick$0(Lnet/minecraft/core/BlockPos;)V
private static synthetic lambda$new$0(Lnet/minecraft/world/entity/EquipmentSlot;)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$static$0(Lnet/minecraft/world/entity/LivingEntity;)Z
static <clinit>()V
```
