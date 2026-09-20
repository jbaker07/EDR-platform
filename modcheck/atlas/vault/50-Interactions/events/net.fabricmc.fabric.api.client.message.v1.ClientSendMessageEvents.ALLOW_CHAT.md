---
type: "event"
event: "net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.ALLOW_CHAT"
callback: "net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents$AllowChat"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents.ALLOW_CHAT

Callback interface: `net.fabricmc.fabric.api.client.message.v1.ClientSendMessageEvents$AllowChat`

Module: [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ClientPacketListenerMixin.fabric_allowSendChatMessage` @18 | [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`sendChat` @Inject HEAD | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
