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

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ClientLevelEntityCallbacksMixin.invokeUnloadEntity` @14 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel_EntityCallbacks|ClientLevel$EntityCallbacks]].`onTrackingEnd` @Inject HEAD | unknown | static_inference |
| `ClientPacketListenerMixin.onPlayerRespawn` @55 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleRespawn` @Inject NEW `net/minecraft/client/multiplayer/ClientLevel` | unknown | static_inference |
| `ClientPacketListenerMixin.onGameJoin` @55 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleLogin` @Inject NEW `net/minecraft/client/multiplayer/ClientLevel` | unknown | static_inference |
| `ClientPacketListenerMixin.onClearLevel` @53 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`clearLevel` @Inject HEAD | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
