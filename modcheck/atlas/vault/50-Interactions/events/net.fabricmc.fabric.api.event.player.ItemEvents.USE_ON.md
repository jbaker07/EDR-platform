---
type: "event"
event: "net.fabricmc.fabric.api.event.player.ItemEvents.USE_ON"
callback: "net.fabricmc.fabric.api.event.player.ItemEvents$UseOnCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.ItemEvents.USE_ON

Callback interface: `net.fabricmc.fabric.api.event.player.ItemEvents$UseOnCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ItemStackMixin.handleUseOnEvent` | (handler is not itself an injector method: fired from a helper or impl class) | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
