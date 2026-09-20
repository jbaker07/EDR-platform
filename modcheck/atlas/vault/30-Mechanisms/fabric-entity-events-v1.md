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
- mixin classes: 13 found by annotation, 13 declared in configs; extraction failures: 0

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

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/com.mojang.brigadier.context.ContextChain|ContextChain]].`runExecutable` | `(Lcom/mojang/brigadier/context/CommandContext;Ljava/lang/Object;Lcom/mojang/brigadier/ResultConsumer;Z)I` | name_only | @WrapMethod | - | both | 1000 (default) | `ContextChainMixin.onRunExecutable` |
| [[40-Interfaces/net.minecraft.commands.execution.CommandQueueEntry|CommandQueueEntry]].`execute` | `(Lnet/minecraft/commands/execution/ExecutionContext;)V` | name_only | @WrapMethod | - | both | 1000 (default) | `CommandQueueEntryMixin.onExecute` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`die` | `(Lnet/minecraft/world/damagesource/DamageSource;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/level/ServerPlayer;getKillCredit()Lnet/minecraft/world/entity/LivingEntity;` (inherited_exact) | both | 1000 (default) | `ServerPlayerMixin.callOnKillForPlayer` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`die` | `(Lnet/minecraft/world/damagesource/DamageSource;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `ServerPlayerMixin.notifyDeath` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`restoreFrom` | `?` | ambiguous | @Inject | TAIL | both | 1000 (default) | `ServerPlayerMixin.onCopyFrom` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`startSleepInBed` | `(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/attribute/BedRule;Lnet/minecraft/core/BlockPos;)Lcom/mojang/datafixers/util/Either;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/level/block/state/BlockState;getValue(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lang/Comparable;` (inherited_exact) | both | 1000 (default) | `ServerPlayerMixin.redirectSleepDirection` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`startSleepInBed` | `(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/attribute/BedRule;Lnet/minecraft/core/BlockPos;)Lcom/mojang/datafixers/util/Either;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/server/level/ServerPlayer;setRespawnPosition(Lnet/minecraft/server/level/ServerPlayer$RespawnConfig;Z)V` (exact) | both | 1000 (default) | `ServerPlayerMixin.onSetSpawnPoint` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`startSleepInBed` | `(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/attribute/BedRule;Lnet/minecraft/core/BlockPos;)Lcom/mojang/datafixers/util/Either;` | name_only | @Redirect | INVOKE `Ljava/util/List;isEmpty()Z` (exact) | both | 1000 (default) | `ServerPlayerMixin.hasNoMonstersNearby` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`triggerDimensionChangeTriggers` | `(Lnet/minecraft/server/level/ServerLevel;)V` | exact | @Inject | TAIL | both | 1000 (default) | `ServerPlayerMixin.afterLevelChanged` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`placeNewPlayer` | `(Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/network/CommonListenerCookie;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `PlayerListMixin.firePlayerJoinEvent` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`remove` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `PlayerListMixin.firePlayerLeaveEvent` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`respawn` | `(Lnet/minecraft/server/level/ServerPlayer;ZLnet/minecraft/world/entity/Entity$RemovalReason;)Lnet/minecraft/server/level/ServerPlayer;` | name_only | @Inject | TAIL | both | 1000 (default) | `PlayerListMixin.afterRespawn` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`teleport` | `(Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/world/entity/Entity;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/entity/Entity;teleportCrossDimension(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/portal/TeleportTransition;)Lnet/minecraft/world/entity/Entity;` (exact) | both | 1000 (default) | `EntityMixin.afterDimensionChanged` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`addEffect` | `(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/entity/Entity;)Z` | exact | @Inject | INVOKE `Ljava/util/Map;get(Ljava/lang/Object;)Ljava/lang/Object;` (exact) | both | 1000 (default) | `LivingEntityMixin.beforeAddEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`canBeAffected` | `(Lnet/minecraft/world/effect/MobEffectInstance;)Z` | name_only | @WrapMethod | - | both | 1000 (default) | `LivingEntityMixin.allowAddEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`canGlide` | `()Z` | name_only | @Inject | FIELD `Lnet/minecraft/world/entity/EquipmentSlot;VALUES:Ljava/util/List;` (exact) | both | 1000 (default) | `LivingEntityMixin.injectElytraCheck` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`checkBedExists` | `()Z` | name_only | @Inject | RETURN | both | 1000 (default) | `LivingEntityMixin.onIsSleepingInBed` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`die` | `(Lnet/minecraft/world/damagesource/DamageSource;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/entity/Entity;killedEntity(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;)Z` (exact) | both | 1000 (default) | `LivingEntityMixin.onEntityKilledOther` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`die` | `(Lnet/minecraft/world/damagesource/DamageSource;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/Level;broadcastEntityEvent(Lnet/minecraft/world/entity/Entity;B)V` (exact) | both | 1000 (default) | `LivingEntityMixin.notifyDeath` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`forceAddEffect` | `(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/entity/Entity;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/entity/LivingEntity;canBeAffected(Lnet/minecraft/world/effect/MobEffectInstance;)Z` (exact) | both | 1000 (default) | `LivingEntityMixin.beforeForceAddEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`getBedOrientation` | `()Lnet/minecraft/core/Direction;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/world/level/block/BedBlock;getBedOrientation(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/Direction;` (inherited_exact) | both | 1000 (default) | `LivingEntityMixin.onGetSleepingDirection` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`hurtServer` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)Z` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/entity/LivingEntity;isDeadOrDying()Z` (exact) | both | 1000 (default) | `LivingEntityMixin.beforeEntityKilled` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`hurtServer` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)Z` | name_only | @Inject | INVOKE `Lnet/minecraft/world/entity/LivingEntity;isSleeping()Z` (exact) | both | 1000 (default) | `LivingEntityMixin.beforeDamage` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`hurtServer` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/damagesource/DamageSource;F)Z` | name_only | @Inject | TAIL | both | 1000 (default) | `LivingEntityMixin.afterDamage` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`lambda$stopSleeping$0` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @ModifyVariable | INVOKE_ASSIGN `Lnet/minecraft/world/level/Level;getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;` (exact) | both | 1000 (default) | `LivingEntityMixin.modifyBedForOccupiedState` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`lambda$stopSleeping$0` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/level/Level;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z` (inherited_exact) | both | 1000 (default) | `LivingEntityMixin.setOccupiedState` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`lambda$stopSleeping$0` | `(Lnet/minecraft/core/BlockPos;)V` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/level/block/AbstractBedBlock;findStandUpPosition(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/CollisionGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;F)Ljava/util/Optional;` (exact) | both | 1000 (default) | `LivingEntityMixin.modifyWakeUpPosition` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`onEffectAdded` | `(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/entity/Entity;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `LivingEntityMixin.afterAddEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`onEffectsRemoved` | `(Ljava/util/Collection;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `LivingEntityMixin.afterRemoveEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`removeAllEffects` | `()Z` | name_only | @WrapOperation | INVOKE `Ljava/util/Map;clear()V` (exact) | both | 1000 (default) | `LivingEntityMixin.allowRemoveAllEffects` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`removeAllEffects` | `()Z` | name_only | @Inject | INVOKE `Lcom/google/common/collect/Maps;newHashMap(Ljava/util/Map;)Ljava/util/HashMap;` (exact) | both | 1000 (default) | `LivingEntityMixin.beforeRemoveAllEffects` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`removeEffect` | `(Lnet/minecraft/core/Holder;)Z` | name_only | @WrapMethod | - | both | 1000 (default) | `LivingEntityMixin.allowRemoveEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`removeEffect` | `(Lnet/minecraft/core/Holder;)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `LivingEntityMixin.beforeRemoveEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`startSleeping` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Inject | RETURN | both | 1000 (default) | `LivingEntityMixin.onSleep` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`startSleeping` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @ModifyVariable | INVOKE_ASSIGN `Lnet/minecraft/world/level/Level;getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;` (exact) | both | 1000 (default) | `LivingEntityMixin.modifyBedForOccupiedState` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`startSleeping` | `(Lnet/minecraft/core/BlockPos;)Z` | name_only | @Redirect | INVOKE `Lnet/minecraft/world/level/Level;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z` (inherited_exact) | both | 1000 (default) | `LivingEntityMixin.setOccupiedState` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`stopSleeping` | `()V` | name_only | @Inject | HEAD | both | 1000 (default) | `LivingEntityMixin.onWakeUp` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`tickEffects` | `()V` | name_only | @Inject | INVOKE `Ljava/util/Iterator;remove()V` (exact) | both | 1000 (default) | `LivingEntityMixin.beforeExpireRemoveEffect` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`updateFallFlying` | `()V` | exact | @Inject | INVOKE `Lnet/minecraft/util/Util;getRandom(Ljava/util/List;Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;` (exact) | both | 1000 (default) | `LivingEntityMixin.injectElytraTick` |
| [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]].`convertTo` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/ConversionParams;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/ConversionParams$AfterConversion;)Lnet/minecraft/world/entity/Mob;` | exact | @ModifyArg | INVOKE `Lnet/minecraft/server/level/ServerLevel;addFreshEntity(Lnet/minecraft/world/entity/Entity;)Z` (exact) | both | 1000 (default) | `MobMixin.afterEntityConverted` |
| [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]].`isSleepingLongEnough` | `()Z` | name_only | @Inject | RETURN | both | 1000 (default) | `PlayerMixin.onIsSleepingLongEnough` |
| [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]].`startSleepInBed` | `(Lnet/minecraft/world/level/block/AbstractBedBlock;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/attribute/BedRule;Lnet/minecraft/core/BlockPos;)Lcom/mojang/datafixers/util/Either;` | name_only | @Inject | HEAD | both | 1000 (default) | `PlayerMixin.onStartSleepInBed` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents|EntityElytraEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents|EntitySleepEvents]] (class, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.ServerEntityCombatEvents|ServerEntityCombatEvents]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.ServerEntityLevelChangeEvents|ServerEntityLevelChangeEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents|ServerLivingEntityEvents]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents|ServerPlayerEvents]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.effect.EffectEventContext|EffectEventContext]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.effect.FabricMobEffect|FabricMobEffect]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents|ServerMobEffectEvents]] (class, 6 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
