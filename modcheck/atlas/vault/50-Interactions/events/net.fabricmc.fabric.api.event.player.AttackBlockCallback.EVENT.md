---
type: "event"
event: "net.fabricmc.fabric.api.event.player.AttackBlockCallback.EVENT"
callback: "net.fabricmc.fabric.api.event.player.AttackBlockCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.AttackBlockCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.event.player.AttackBlockCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerPlayerGameModeMixin.startBlockBreak` @30 | [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]].`handleBlockBreakAction` @Inject HEAD | unknown | static_inference |
| `MultiPlayerGameModeMixin.fabric_fireAttackBlockCallback` @28 | (handler is not itself an injector: fired from a helper or impl method) | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
