---
type: "event"
event: "net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.ALLOW_CHAT"
callback: "net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents$AllowChat"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.ALLOW_CHAT

Callback interface: `net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents$AllowChat`

Module: [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ChatListenerMixin.fabric_onChatMessage` | (handler is not itself an injector method: fired from a helper or impl class) | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
