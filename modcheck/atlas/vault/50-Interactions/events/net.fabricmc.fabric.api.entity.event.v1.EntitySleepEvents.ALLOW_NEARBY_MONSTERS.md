---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_NEARBY_MONSTERS"
callback: "net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$AllowNearbyMonsters"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents.ALLOW_NEARBY_MONSTERS

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.EntitySleepEvents$AllowNearbyMonsters`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerPlayerMixin.hasNoMonstersNearby` @25 | [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`startSleepInBed` @Redirect INVOKE `Ljava/util/List;isEmpty()Z` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
