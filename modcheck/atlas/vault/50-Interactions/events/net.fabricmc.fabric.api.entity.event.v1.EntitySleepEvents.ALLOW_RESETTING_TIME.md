---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_RESETTING_TIME"
callback: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$AllowResettingTime"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_RESETTING_TIME

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$AllowResettingTime`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `PlayerMixin.onIsSleepingLongEnough` @21 | [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]].`isSleepingLongEnough` @Inject RETURN | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
