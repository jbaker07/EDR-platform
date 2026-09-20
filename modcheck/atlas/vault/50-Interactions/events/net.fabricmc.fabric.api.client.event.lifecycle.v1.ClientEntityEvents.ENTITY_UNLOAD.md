---
type: "event"
event: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientEntityEvents.ENTITY_UNLOAD"
callback: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientEntityEvents$Unload"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientEntityEvents.ENTITY_UNLOAD

Callback interface: `net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientEntityEvents$Unload`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ClientLevelEntityCallbacksMixin.invokeUnloadEntity` | `ClientLevel$EntityCallbacks.onTrackingEnd(Lnet/minecraft/world/entity/Entity;)V` @Inject at HEAD | client | static_inference |
| `ClientPacketListenerMixin.onPlayerRespawn` | `ClientPacketListener.handleRespawn` @Inject at NEW net/minecraft/client/multiplayer/ClientLevel | client | static_inference |
| `ClientPacketListenerMixin.onGameJoin` | `ClientPacketListener.handleLogin` @Inject at NEW net/minecraft/client/multiplayer/ClientLevel | client | static_inference |
| `ClientPacketListenerMixin.onClearLevel` | `ClientPacketListener.clearLevel` @Inject at HEAD | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
