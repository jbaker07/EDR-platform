---
type: "event"
event: "net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback.EVENT"
callback: "net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback`

Module: [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ClientPacketListenerMixin.onGameJoin` | `ClientPacketListener.handleLogin` @Inject at RETURN | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
