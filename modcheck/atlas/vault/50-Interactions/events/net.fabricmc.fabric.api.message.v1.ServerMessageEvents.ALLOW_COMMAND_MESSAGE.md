---
type: "event"
event: "net.fabricmc.fabric.api.message.v1.ServerMessageEvents.ALLOW_COMMAND_MESSAGE"
callback: "net.fabricmc.fabric.api.message.v1.ServerMessageEvents$AllowCommandMessage"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.message.v1.ServerMessageEvents.ALLOW_COMMAND_MESSAGE

Callback interface: `net.fabricmc.fabric.api.message.v1.ServerMessageEvents$AllowCommandMessage`

Module: [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `PlayerListMixin.onSendCommandMessage` | `PlayerList.broadcastChatMessage(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/commands/CommandSourceStack;Lnet/minecraft/network/chat/ChatType$Bound;)V` @Inject at HEAD | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
