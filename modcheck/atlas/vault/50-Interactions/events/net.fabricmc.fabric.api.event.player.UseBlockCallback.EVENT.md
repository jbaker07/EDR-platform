---
type: "event"
event: "net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT"
callback: "net.fabricmc.fabric.api.event.player.UseBlockCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.event.player.UseBlockCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ServerPlayerGameModeMixin.interactBlock` | `ServerPlayerGameMode.useItemOn` @Inject at HEAD | both | static_inference |
| `MultiPlayerGameModeMixin.interactBlock` | `MultiPlayerGameMode.useItemOn` @Inject at INVOKE Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;startPrediction(Lnet/minecraft/client/multiplayer/ClientLevel;Lnet/minecraft/client/multiplayer/prediction/PredictiveAction;)V | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.player.UseBlockCallback.EVENT|read it]].
