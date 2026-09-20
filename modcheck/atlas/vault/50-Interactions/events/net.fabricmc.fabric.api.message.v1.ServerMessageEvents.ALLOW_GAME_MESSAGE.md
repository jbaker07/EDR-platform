---
type: "event"
event: "net.fabricmc.fabric.api.message.v1.ServerMessageEvents.ALLOW_GAME_MESSAGE"
callback: "net.fabricmc.fabric.api.message.v1.ServerMessageEvents$AllowGameMessage"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.message.v1.ServerMessageEvents.ALLOW_GAME_MESSAGE

Callback interface: `net.fabricmc.fabric.api.message.v1.ServerMessageEvents$AllowGameMessage`

Module: [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `PlayerListMixin.onSendGameMessage` | `PlayerList.broadcastSystemMessage(Lnet/minecraft/network/chat/Component;Ljava/util/function/Function;Z)V` @Inject at HEAD | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
