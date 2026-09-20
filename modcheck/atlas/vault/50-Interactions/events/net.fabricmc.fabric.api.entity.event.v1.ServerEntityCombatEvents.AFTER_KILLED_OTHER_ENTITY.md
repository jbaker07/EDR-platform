---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.ServerEntityCombatEvents.AFTER_KILLED_OTHER_ENTITY"
callback: "net.fabricmc.fabric.api.entity.event.v1.ServerEntityCombatEvents$AfterKilledOtherEntity"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.ServerEntityCombatEvents.AFTER_KILLED_OTHER_ENTITY

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.ServerEntityCombatEvents$AfterKilledOtherEntity`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.onEntityKilledOther` @50 | [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`die` @WrapOperation INVOKE `Lnet/minecraft/world/entity/Entity;killedEntity(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;)Z` | unknown | static_inference |
| `ServerPlayerMixin.callOnKillForPlayer` @42 | [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`die` @Inject INVOKE `Lnet/minecraft/server/level/ServerPlayer;getKillCredit()Lnet/minecraft/world/entity/LivingEntity;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
