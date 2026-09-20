---
type: "event"
event: "net.fabricmc.fabric.api.event.player.BlockEvents.USE_ITEM_ON"
callback: "net.fabricmc.fabric.api.event.player.BlockEvents$UseItemOnCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.BlockEvents.USE_ITEM_ON

Callback interface: `net.fabricmc.fabric.api.event.player.BlockEvents$UseItemOnCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `BlockBehaviourBlockStateBaseMixin.callUseItemOnEvent` | `BlockBehaviour$BlockStateBase.useItemOn` @Inject at HEAD | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
