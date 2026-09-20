---
type: "interface"
fqcn: "net.minecraft.world.entity.monster.warden.Warden"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.monster.warden.Warden

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `net/minecraft/world/entity/monster/Monster`; implements `net/minecraft/world/level/gameevent/vibrations/VibrationSystem`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `clientDiggingParticles` | `(Lnet/minecraft/world/entity/AnimationState;)V` | name_only | @ModifyExpressionValue at ['NEW'] | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (38 fields, 60 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final VIBRATION_COOLDOWN_TICKS : I
private static final TIME_TO_USE_MELEE_UNTIL_SONIC_BOOM : I
private static final MAX_HEALTH : I
private static final MOVEMENT_SPEED_WHEN_FIGHTING : F
private static final KNOCKBACK_RESISTANCE : F
private static final ATTACK_KNOCKBACK : F
private static final ATTACK_DAMAGE : I
private static final FOLLOW_RANGE : I
private static final CLIENT_ANGER_LEVEL : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DARKNESS_DISPLAY_LIMIT : I
private static final DARKNESS_DURATION : I
private static final DARKNESS_RADIUS : I
private static final DARKNESS_INTERVAL : I
private static final ANGERMANAGEMENT_TICK_DELAY : I
private static final DEFAULT_ANGER : I
private static final PROJECTILE_ANGER : I
private static final ON_HURT_ANGER_BOOST : I
private static final RECENT_PROJECTILE_TICK_THRESHOLD : I
private static final TOUCH_COOLDOWN_TICKS : I
private static final DIGGING_PARTICLES_AMOUNT : I
private static final DIGGING_PARTICLES_DURATION : F
private static final DIGGING_PARTICLES_OFFSET : F
private static final PROJECTILE_ANGER_DISTANCE : I
private static final BRAIN_PROVIDER : Lnet/minecraft/world/entity/ai/Brain$Provider;
private tendrilAnimation : I
private tendrilAnimationO : I
private heartAnimation : I
private heartAnimationO : I
public final roarAnimationState : Lnet/minecraft/world/entity/AnimationState;
public final sniffAnimationState : Lnet/minecraft/world/entity/AnimationState;
public final emergeAnimationState : Lnet/minecraft/world/entity/AnimationState;
public final diggingAnimationState : Lnet/minecraft/world/entity/AnimationState;
public final attackAnimationState : Lnet/minecraft/world/entity/AnimationState;
public final sonicBoomAnimationState : Lnet/minecraft/world/entity/AnimationState;
private final dynamicGameEventListener : Lnet/minecraft/world/level/gameevent/DynamicGameEventListener;
private final vibrationUser : Lnet/minecraft/world/level/gameevent/vibrations/VibrationSystem$User;
private vibrationData : Lnet/minecraft/world/level/gameevent/vibrations/VibrationSystem$Data;
private angerManagement : Lnet/minecraft/world/entity/monster/warden/AngerManagement;
public <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V
public getAddEntityPacket(Lnet/minecraft/server/level/ServerEntity;)Lnet/minecraft/network/protocol/Packet;
public recreateFromPacket(Lnet/minecraft/network/protocol/game/ClientboundAddEntityPacket;)V
public checkSpawnObstruction(Lnet/minecraft/world/level/LevelReader;)Z
public getWalkTargetValue(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/LevelReader;)F
public isInvulnerableTo(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;)Z
private isDiggingOrEmerging()Z
protected canRide(Lnet/minecraft/world/entity/Entity;)Z
public getSecondsToDisableBlocking()F
protected nextStep()F
public static createAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
public dampensVibrations()Z
protected getSoundVolume()F
protected getAmbientSound()Lnet/minecraft/sounds/SoundEvent;
protected getHurtSound(Lnet/minecraft/world/damagesource/DamageSource;)Lnet/minecraft/sounds/SoundEvent;
protected getDeathSound()Lnet/minecraft/sounds/SoundEvent;
protected playStepSound(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public doHurtTarget(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;)Z
protected defineSynchedData(Lnet/minecraft/network/syncher/SynchedEntityData$Builder;)V
public getClientAngerLevel()I
private syncClientAngerLevel()V
public tick()V
protected customServerAiStep(Lnet/minecraft/server/level/ServerLevel;)V
public handleEntityEvent(B)V
private getHeartBeatDelay()I
public getTendrilAnimation(F)F
public getHeartAnimation(F)F
private clientDiggingParticles(Lnet/minecraft/world/entity/AnimationState;)V
public onSyncedDataUpdated(Lnet/minecraft/network/syncher/EntityDataAccessor;)V
public ignoreExplosion(Lnet/minecraft/world/level/Explosion;)Z
protected makeBrain(Lnet/minecraft/world/entity/ai/Brain$Packed;)Lnet/minecraft/world/entity/ai/Brain;
public getBrain()Lnet/minecraft/world/entity/ai/Brain;
public updateDynamicGameEventListener(Ljava/util/function/BiConsumer;)V
public canAttack(Lnet/minecraft/world/entity/LivingEntity;)Z
public canTargetEntity(Lnet/minecraft/world/entity/Entity;)Z
public static applyDarknessAround(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/Entity;I)V
protected addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V
protected readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V
private playListeningSound()V
public getAngerLevel()Lnet/minecraft/world/entity/monster/warden/AngerLevel;
private getActiveAnger()I
public clearAnger(Lnet/minecraft/world/entity/Entity;)V
public increaseAngerAt(Lnet/minecraft/world/entity/Entity;)V
public increaseAngerAt(Lnet/minecraft/world/entity/Entity;IZ)V
public getEntityAngryAt()Ljava/util/Optional;
public getTarget()Lnet/minecraft/world/entity/LivingEntity;
public removeWhenFarAway(D)Z
public finalizeSpawn(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/world/DifficultyInstance;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/SpawnGroupData;)Lnet/minecraft/world/entity/SpawnGroupData;
public hurtServer(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)Z
public setAttackTarget(Lnet/minecraft/world/entity/LivingEntity;)V
public getDefaultDimensions(Lnet/minecraft/world/entity/Pose;)Lnet/minecraft/world/entity/EntityDimensions;
public isPushable()Z
protected doPush(Lnet/minecraft/world/entity/Entity;)V
public getAngerManagement()Lnet/minecraft/world/entity/monster/warden/AngerManagement;
protected createNavigation(Lnet/minecraft/world/level/Level;)Lnet/minecraft/world/entity/ai/navigation/PathNavigation;
public getVibrationData()Lnet/minecraft/world/level/gameevent/vibrations/VibrationSystem$Data;
public getVibrationUser()Lnet/minecraft/world/level/gameevent/vibrations/VibrationSystem$User;
static synthetic access$000(Lnet/minecraft/world/entity/monster/warden/Warden;)Lnet/minecraft/world/entity/ai/Brain;
private synthetic lambda$readAdditionalSaveData$0()Lnet/minecraft/world/entity/monster/warden/AngerManagement;
static <clinit>()V
```
