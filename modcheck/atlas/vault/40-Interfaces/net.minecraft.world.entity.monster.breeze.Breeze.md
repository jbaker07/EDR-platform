---
type: "interface"
fqcn: "net.minecraft.world.entity.monster.breeze.Breeze"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.monster.breeze.Breeze

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `net/minecraft/world/entity/monster/Monster`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `emitGroundParticles` | `(I)V` | name_only | @ModifyExpressionValue at ['NEW'] | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| injects_into | `emitJumpTrailParticles` | `()V` | name_only | @ModifyExpressionValue at ['NEW'] | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (18 fields, 34 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final SLIDE_PARTICLES_AMOUNT : I
private static final IDLE_PARTICLES_AMOUNT : I
private static final JUMP_TRAIL_PARTICLES_AMOUNT : I
private static final JUMP_TRAIL_DURATION_TICKS : I
private static final JUMP_CIRCLE_DISTANCE_Y : I
private static final FALL_DISTANCE_SOUND_TRIGGER_THRESHOLD : F
private static final WHIRL_SOUND_FREQUENCY_MIN : I
private static final WHIRL_SOUND_FREQUENCY_MAX : I
private static final BRAIN_PROVIDER : Lnet/minecraft/world/entity/ai/Brain$Provider;
public final idle : Lnet/minecraft/world/entity/AnimationState;
public final slide : Lnet/minecraft/world/entity/AnimationState;
public final slideBack : Lnet/minecraft/world/entity/AnimationState;
public final longJump : Lnet/minecraft/world/entity/AnimationState;
public final shoot : Lnet/minecraft/world/entity/AnimationState;
public final inhale : Lnet/minecraft/world/entity/AnimationState;
private jumpTrailStartedTick : I
private soundTick : I
private static final PROJECTILE_DEFLECTION : Lnet/minecraft/world/entity/projectile/ProjectileDeflection;
public static createAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
public <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V
public getBrain()Lnet/minecraft/world/entity/ai/Brain;
protected makeBrain(Lnet/minecraft/world/entity/ai/Brain$Packed;)Lnet/minecraft/world/entity/ai/Brain;
public onSyncedDataUpdated(Lnet/minecraft/network/syncher/EntityDataAccessor;)V
private resetAnimations()V
public tick()V
public resetJumpTrail()Lnet/minecraft/world/entity/monster/breeze/Breeze;
public emitJumpTrailParticles()V
public emitGroundParticles(I)V
public playAmbientSound()V
public playWhirlSound()V
public deflection(Lnet/minecraft/world/entity/projectile/Projectile;)Lnet/minecraft/world/entity/projectile/ProjectileDeflection;
protected getDeathSound()Lnet/minecraft/sounds/SoundEvent;
protected getHurtSound(Lnet/minecraft/world/damagesource/DamageSource;)Lnet/minecraft/sounds/SoundEvent;
protected getAmbientSound()Lnet/minecraft/sounds/SoundEvent;
public getHurtBy()Ljava/util/Optional;
public withinInnerCircleRange(Lnet/minecraft/world/phys/Vec3;)Z
protected customServerAiStep(Lnet/minecraft/server/level/ServerLevel;)V
public canAttack(Lnet/minecraft/world/entity/LivingEntity;)Z
public getMaxHeadYRot()I
public getHeadRotSpeed()I
public getFiringYPosition()D
public isInvulnerableTo(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;)Z
public getFluidJumpThreshold()D
public causeFallDamage(DFLnet/minecraft/world/damagesource/DamageSource;)Z
protected getMovementEmission()Lnet/minecraft/world/entity/Entity$MovementEmission;
public getTarget()Lnet/minecraft/world/entity/LivingEntity;
public registerDebugValues(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/util/debug/DebugValueSource$Registration;)V
private synthetic lambda$registerDebugValues$0()Lnet/minecraft/util/debug/DebugBreezeInfo;
private static synthetic lambda$getHurtBy$1(Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/entity/LivingEntity;
private static synthetic lambda$getHurtBy$0(Lnet/minecraft/world/entity/Entity;)Z
private static synthetic lambda$static$0(Lnet/minecraft/world/entity/projectile/Projectile;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/phys/Vec3;)V
static <clinit>()V
```
