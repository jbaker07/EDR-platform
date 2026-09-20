---
type: "event"
event: "net.fabricmc.fabric.api.event.player.AttackEntityCallback.EVENT"
callback: "net.fabricmc.fabric.api.event.player.AttackEntityCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.AttackEntityCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.event.player.AttackEntityCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `PlayerMixin.onPlayerInteractEntity` | `Player.attack` @Inject at HEAD | both | static_inference |
| `MultiPlayerGameModeMixin.attackEntity` | `MultiPlayerGameMode.attack` @Inject at INVOKE Lnet/minecraft/client/multiplayer/ClientPacketListener;send(Lnet/minecraft/network/protocol/Packet;)V | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
