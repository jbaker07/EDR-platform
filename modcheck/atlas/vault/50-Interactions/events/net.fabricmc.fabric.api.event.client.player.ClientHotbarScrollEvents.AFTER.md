---
type: "event"
event: "net.fabricmc.fabric.api.event.client.player.ClientHotbarScrollEvents.AFTER"
callback: "net.fabricmc.fabric.api.event.client.player.ClientHotbarScrollEvents$After"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.client.player.ClientHotbarScrollEvents.AFTER

Callback interface: `net.fabricmc.fabric.api.event.client.player.ClientHotbarScrollEvents$After`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `MouseHandlerMixin.wrapSelectedSlot` @96 | [[40-Interfaces/net.minecraft.client.MouseHandler|MouseHandler]].`onScroll` @WrapOperation INVOKE `Lnet/minecraft/world/entity/player/Inventory;setSelectedSlot(I)V` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
