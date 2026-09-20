---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.SET_BED_OCCUPATION_STATE"
callback: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$SetBedOccupationState"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.SET_BED_OCCUPATION_STATE

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$SetBedOccupationState`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.setOccupiedState` @40 | [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`lambda$stopSleeping$0` @Redirect INVOKE `Lnet/minecraft/world/level/Level;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z`; [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`startSleeping` @Redirect INVOKE `Lnet/minecraft/world/level/Level;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
