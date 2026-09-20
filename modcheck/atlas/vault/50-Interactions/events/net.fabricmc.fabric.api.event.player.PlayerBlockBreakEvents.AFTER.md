---
type: "event"
event: "net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.AFTER"
callback: "net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents$After"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.AFTER

Callback interface: `net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents$After`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerPlayerGameModeMixin.onBlockBroken` @21 | [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]].`destroyBlock` @Inject INVOKE `Lnet/minecraft/world/level/block/Block;destroy(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
