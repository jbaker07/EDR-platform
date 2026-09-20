---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_BED"
callback: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$AllowBed"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_BED

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$AllowBed`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.onIsSleepingInBed` | `LivingEntity.checkBedExists` @Inject at RETURN | both | static_inference |
| `LivingEntityMixin.modifyBedForOccupiedState` | `LivingEntity.lambda$stopSleeping$0` @ModifyVariable at INVOKE_ASSIGN Lnet/minecraft/world/level/Level;getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;; `LivingEntity.startSleeping` @ModifyVariable at INVOKE_ASSIGN Lnet/minecraft/world/level/Level;getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState; | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
