---
type: "event"
event: "net.fabricmc.fabric.api.event.player.ItemEvents.USE"
callback: "net.fabricmc.fabric.api.event.player.ItemEvents$UseCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.ItemEvents.USE

Callback interface: `net.fabricmc.fabric.api.event.player.ItemEvents$UseCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ItemStackMixin.handleUseEvent` @13 | [[40-Interfaces/net.minecraft.world.item.ItemStack|ItemStack]].`use` @WrapOperation INVOKE `Lnet/minecraft/world/item/Item;use(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
