---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.MODIFY_WAKE_UP_POSITION"
callback: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$ModifyWakeUpPosition"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.MODIFY_WAKE_UP_POSITION

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$ModifyWakeUpPosition`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.modifyWakeUpPosition` | `LivingEntity.lambda$stopSleeping$0` @Redirect at INVOKE Lnet/minecraft/world/level/block/AbstractBedBlock;findStandUpPosition(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/CollisionGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;F)Ljava/util/Optional; | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
