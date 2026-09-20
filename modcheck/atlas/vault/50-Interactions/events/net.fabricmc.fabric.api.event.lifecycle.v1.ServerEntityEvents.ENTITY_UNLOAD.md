---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_UNLOAD"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents$Unload"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents.ENTITY_UNLOAD

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerEntityEvents$Unload`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ServerLevelEntityCallbacksMixin.invokeEntityUnloadEvent` | `ServerLevel$EntityCallbacks.onTrackingEnd(Lnet/minecraft/world/entity/Entity;)V` @Inject at HEAD | both | static_inference |
| `LifecycleEventsImpl.lambda$onInitialize$3` | (impl code, not a mixin) | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
