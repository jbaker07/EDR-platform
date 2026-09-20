---
type: "event"
event: "net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE"
callback: "net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents$Before"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE

Callback interface: `net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents$Before`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerPlayerGameModeMixin.breakBlock` @21 | [[40-Interfaces/net.minecraft.server.level.ServerPlayerGameMode|ServerPlayerGameMode]].`destroyBlock` @Inject INVOKE `Lnet/minecraft/world/level/block/Block;playerWillDestroy(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/level/block/state/BlockState;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.BEFORE|read it]].
