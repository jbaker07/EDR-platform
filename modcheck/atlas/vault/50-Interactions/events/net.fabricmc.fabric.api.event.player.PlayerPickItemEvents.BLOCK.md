---
type: "event"
event: "net.fabricmc.fabric.api.event.player.PlayerPickItemEvents.BLOCK"
callback: "net.fabricmc.fabric.api.event.player.PlayerPickItemEvents$PickItemFromBlock"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.PlayerPickItemEvents.BLOCK

Callback interface: `net.fabricmc.fabric.api.event.player.PlayerPickItemEvents$PickItemFromBlock`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerGamePacketListenerImplMixin.onPickItemFromBlock` @20 | [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]].`handlePickItemFromBlock` @WrapOperation INVOKE `Lnet/minecraft/world/level/block/state/BlockState;getCloneItemStack(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;Z)Lnet/minecraft/world/item/ItemStack;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
