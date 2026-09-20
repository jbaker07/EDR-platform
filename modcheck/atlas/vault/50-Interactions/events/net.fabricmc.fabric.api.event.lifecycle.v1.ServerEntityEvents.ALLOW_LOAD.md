---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ALLOW_LOAD"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents$AllowLoad"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ALLOW_LOAD

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents$AllowLoad`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `PersistentEntitySectionManagerMixin.beforeAddEntity` @42 | [[40-Interfaces/net.minecraft.world.level.entity.PersistentEntitySectionManager|PersistentEntitySectionManager]].`addEntity` @Inject HEAD | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
