---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.MODIFY_SLEEPING_DIRECTION"
callback: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$ModifySleepingDirection"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.MODIFY_SLEEPING_DIRECTION

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$ModifySleepingDirection`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.onGetSleepingDirection` @39 | [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`getBedOrientation` @WrapOperation INVOKE `Lnet/minecraft/world/level/block/BedBlock;getBedOrientation(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/Direction;` | unknown | static_inference |
| `ServerPlayerMixin.redirectSleepDirection` @55 | [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`startSleepInBed` @WrapOperation INVOKE `Lnet/minecraft/world/level/block/state/BlockState;getValue(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lang/Comparable;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
