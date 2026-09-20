---
type: "event"
event: "net.fabricmc.fabric.api.event.client.player.ClientPreAttackCallback.EVENT"
callback: "net.fabricmc.fabric.api.event.client.player.ClientPreAttackCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.client.player.ClientPreAttackCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.event.client.player.ClientPreAttackCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `MinecraftMixin.injectHandleInputEventsForPreAttackCallback` @52 | [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`handleKeybinds` @Inject INVOKE `Lnet/minecraft/client/player/LocalPlayer;isUsingItem()Z` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
