---
type: "event"
event: "net.fabricmc.fabric.api.event.player.UseItemCallback.EVENT"
callback: "net.fabricmc.fabric.api.event.player.UseItemCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.UseItemCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.event.player.UseItemCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerPlayerGameModeMixin.interactItem` @13 | [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]].`useItem` @Inject HEAD | unknown | static_inference |
| `MultiPlayerGameModeMixin.interactItem` @15 | [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]].`useItem` @Inject INVOKE `Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;ensureHasSentCarriedItem()V` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
