---
type: "event"
event: "net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.MODIFY_GAME"
callback: "net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents$ModifyGame"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents.MODIFY_GAME

Callback interface: `net.fabricmc.fabric.api.client.message.v1.ClientReceiveMessageEvents$ModifyGame`

Module: [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ChatListenerMixin.fabric_allowSystemMessage` | `ChatListener.handleSystemMessage` @Inject at HEAD | client | static_inference |
| `ChatListenerMixin.fabric_allowOverlayMessage` | `ChatListener.handleOverlay` @Inject at HEAD | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
