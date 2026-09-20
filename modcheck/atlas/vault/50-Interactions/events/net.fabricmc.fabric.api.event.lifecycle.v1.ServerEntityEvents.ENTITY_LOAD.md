---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents$Load"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents$Load`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerLevelEntityCallbacksMixin.invokeEntityLoadEvent` @14 | [[40-Interfaces/net.minecraft.server.level.ServerLevel_EntityCallbacks|ServerLevel$EntityCallbacks]].`onTrackingStart` @Inject TAIL | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_LOAD|read it]].
