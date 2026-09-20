---
type: "mechanism"
module: "fabric-entity-events-v1"
version: "6.0.4+3434d6d95d"
sha256: "a5a9e382e4f9875f7450dbf0d45221ca70afc00cf84aa02023104bb15a530ade"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-entity-events-v1

**Version** `6.0.4+3434d6d95d` -- **artifact sha256** `a5a9e382e4f9875f7450dbf0d45221ca70afc00cf84aa02023104bb15a530ade`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `null`
- mixin configs: `["fabric-entity-events-v1.mixins.json"]`
- access widener: `fabric-entity-events-v1.classtweaker`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents.ALLOW|EntityElytraEvents.ALLOW]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents.CUSTOM|EntityElytraEvents.CUSTOM]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_BED|EntitySleepEvents.ALLOW_BED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_NEARBY_MONSTERS|EntitySleepEvents.ALLOW_NEARBY_MONSTERS]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_RESETTING_TIME|EntitySleepEvents.ALLOW_RESETTING_TIME]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_SETTING_SPAWN|EntitySleepEvents.ALLOW_SETTING_SPAWN]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_SLEEPING|EntitySleepEvents.ALLOW_SLEEPING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.MODIFY_SLEEPING_DIRECTION|EntitySleepEvents.MODIFY_SLEEPING_DIRECTION]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.MODIFY_WAKE_UP_POSITION|EntitySleepEvents.MODIFY_WAKE_UP_POSITION]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.SET_BED_OCCUPATION_STATE|EntitySleepEvents.SET_BED_OCCUPATION_STATE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.START_SLEEPING|EntitySleepEvents.START_SLEEPING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.STOP_SLEEPING|EntitySleepEvents.STOP_SLEEPING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerEntityCombatEvents.AFTER_KILLED_OTHER_ENTITY|ServerEntityCombatEvents.AFTER_KILLED_OTHER_ENTITY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerEntityLevelChangeEvents.AFTER_ENTITY_CHANGE_LEVEL|ServerEntityLevelChangeEvents.AFTER_ENTITY_CHANGE_LEVEL]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerEntityLevelChangeEvents.AFTER_PLAYER_CHANGE_LEVEL|ServerEntityLevelChangeEvents.AFTER_PLAYER_CHANGE_LEVEL]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.AFTER_DAMAGE|ServerLivingEntityEvents.AFTER_DAMAGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.AFTER_DEATH|ServerLivingEntityEvents.AFTER_DEATH]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DAMAGE|ServerLivingEntityEvents.ALLOW_DAMAGE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH|ServerLivingEntityEvents.ALLOW_DEATH]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.MOB_CONVERSION|ServerLivingEntityEvents.MOB_CONVERSION]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.AFTER_RESPAWN|ServerPlayerEvents.AFTER_RESPAWN]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.ALLOW_DEATH|ServerPlayerEvents.ALLOW_DEATH]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.COPY_FROM|ServerPlayerEvents.COPY_FROM]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.JOIN|ServerPlayerEvents.JOIN]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.LEAVE|ServerPlayerEvents.LEAVE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.AFTER_ADD|ServerMobEffectEvents.AFTER_ADD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.AFTER_REMOVE|ServerMobEffectEvents.AFTER_REMOVE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.ALLOW_ADD|ServerMobEffectEvents.ALLOW_ADD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.ALLOW_EARLY_REMOVE|ServerMobEffectEvents.ALLOW_EARLY_REMOVE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.BEFORE_ADD|ServerMobEffectEvents.BEFORE_ADD]]
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.BEFORE_REMOVE|ServerMobEffectEvents.BEFORE_REMOVE]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `die` | injects_into `@Inject at INVOKE Lnet/minecraft/server/level/ServerPlayer;getKillCredit()Lnet/minecraft/world/entity/LivingEntity;` | both | `ServerPlayerMixin.callOnKillForPlayer` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `die` | injects_into `@Inject at TAIL` | both | `ServerPlayerMixin.notifyDeath` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `restoreFrom` | injects_into `@Inject at TAIL` | both | `ServerPlayerMixin.onCopyFrom` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `startSleepInBed` | wraps `@Redirect at INVOKE Ljava/util/List;isEmpty()Z` | both | `ServerPlayerMixin.hasNoMonstersNearby` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]] | `triggerDimensionChangeTriggers(Lnet/minecraft/server/level/ServerLevel;)V` | injects_into `@Inject at TAIL` | both | `ServerPlayerMixin.afterLevelChanged` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]] | `placeNewPlayer` | injects_into `@Inject at RETURN` | both | `PlayerListMixin.firePlayerJoinEvent` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]] | `remove` | injects_into `@Inject at HEAD` | both | `PlayerListMixin.firePlayerLeaveEvent` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]] | `respawn` | injects_into `@Inject at TAIL` | both | `PlayerListMixin.afterRespawn` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `addEffect(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/entity/Entity;)Z` | injects_into `@Inject at INVOKE Ljava/util/Map;get(Ljava/lang/Object;)Ljava/lang/Object;` | both | `LivingEntityMixin.beforeAddEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `canGlide` | injects_into `@Inject at FIELD Lnet/minecraft/world/entity/EquipmentSlot;VALUES:Ljava/util/List;` | both | `LivingEntityMixin.injectElytraCheck` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `checkBedExists` | injects_into `@Inject at RETURN` | both | `LivingEntityMixin.onIsSleepingInBed` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `die` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/Level;broadcastEntityEvent(Lnet/minecraft/world/entity/Entity;B)V` | both | `LivingEntityMixin.notifyDeath` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `forceAddEffect` | injects_into `@Inject at INVOKE Lnet/minecraft/world/entity/LivingEntity;canBeAffected(Lnet/minecraft/world/effect/MobEffectInstance;)Z` | both | `LivingEntityMixin.beforeForceAddEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `hurtServer` | wraps `@Redirect at INVOKE Lnet/minecraft/world/entity/LivingEntity;isDeadOrDying()Z` | both | `LivingEntityMixin.beforeEntityKilled` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `hurtServer` | injects_into `@Inject at INVOKE Lnet/minecraft/world/entity/LivingEntity;isSleeping()Z` | both | `LivingEntityMixin.beforeDamage` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `hurtServer` | injects_into `@Inject at TAIL` | both | `LivingEntityMixin.afterDamage` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `lambda$stopSleeping$0` | injects_into `@ModifyVariable at INVOKE_ASSIGN Lnet/minecraft/world/level/Level;getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;` | both | `LivingEntityMixin.modifyBedForOccupiedState` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `lambda$stopSleeping$0` | wraps `@Redirect at INVOKE Lnet/minecraft/world/level/Level;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z` | both | `LivingEntityMixin.setOccupiedState` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `lambda$stopSleeping$0` | wraps `@Redirect at INVOKE Lnet/minecraft/world/level/block/AbstractBedBlock;findStandUpPosition(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/CollisionGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;F)Ljava/util/Optional;` | both | `LivingEntityMixin.modifyWakeUpPosition` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `onEffectAdded` | injects_into `@Inject at RETURN` | both | `LivingEntityMixin.afterAddEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `onEffectsRemoved` | injects_into `@Inject at RETURN` | both | `LivingEntityMixin.afterRemoveEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `removeAllEffects` | injects_into `@Inject at INVOKE Lcom/google/common/collect/Maps;newHashMap(Ljava/util/Map;)Ljava/util/HashMap;` | both | `LivingEntityMixin.beforeRemoveAllEffects` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `removeEffect` | injects_into `@Inject at HEAD` | both | `LivingEntityMixin.beforeRemoveEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `startSleeping` | injects_into `@Inject at RETURN` | both | `LivingEntityMixin.onSleep` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `startSleeping` | injects_into `@ModifyVariable at INVOKE_ASSIGN Lnet/minecraft/world/level/Level;getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;` | both | `LivingEntityMixin.modifyBedForOccupiedState` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `startSleeping` | wraps `@Redirect at INVOKE Lnet/minecraft/world/level/Level;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z` | both | `LivingEntityMixin.setOccupiedState` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `stopSleeping` | injects_into `@Inject at HEAD` | both | `LivingEntityMixin.onWakeUp` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `tickEffects` | injects_into `@Inject at INVOKE Ljava/util/Iterator;remove()V` | both | `LivingEntityMixin.beforeExpireRemoveEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] | `updateFallFlying()V` | injects_into `@Inject at INVOKE Lnet/minecraft/util/Util;getRandom(Ljava/util/List;Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;` | both | `LivingEntityMixin.injectElytraTick` |
| [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]] | `convertTo(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/ConversionParams;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/ConversionParams$AfterConversion;)Lnet/minecraft/world/entity/Mob;` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/server/level/ServerLevel;addFreshEntity(Lnet/minecraft/world/entity/Entity;)Z` | both | `MobMixin.afterEntityConverted` |
| [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]] | `isSleepingLongEnough` | injects_into `@Inject at RETURN` | both | `PlayerMixin.onIsSleepingLongEnough` |
| [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]] | `startSleepInBed` | injects_into `@Inject at HEAD` | both | `PlayerMixin.onStartSleepInBed` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents|EntityElytraEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents|EntitySleepEvents]] (class, 11 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.ServerEntityCombatEvents|ServerEntityCombatEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.ServerEntityLevelChangeEvents|ServerEntityLevelChangeEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents|ServerLivingEntityEvents]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents|ServerPlayerEvents]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.effect.EffectEventContext|EffectEventContext]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.effect.FabricMobEffect|FabricMobEffect]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents|ServerMobEffectEvents]] (class, 7 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
