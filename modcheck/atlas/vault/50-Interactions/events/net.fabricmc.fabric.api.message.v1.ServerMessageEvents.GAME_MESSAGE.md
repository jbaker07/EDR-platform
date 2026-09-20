---
type: "event"
event: "net.fabricmc.fabric.api.message.v1.ServerMessageEvents.GAME_MESSAGE"
callback: "net.fabricmc.fabric.api.message.v1.ServerMessageEvents$GameMessage"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.message.v1.ServerMessageEvents.GAME_MESSAGE

Callback interface: `net.fabricmc.fabric.api.message.v1.ServerMessageEvents$GameMessage`

Module: [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `PlayerListMixin.onSendGameMessage` @44 | [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`broadcastSystemMessage` @Inject HEAD | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
