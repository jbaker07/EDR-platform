---
type: "event"
event: "net.fabricmc.fabric.api.event.player.BlockEvents.USE_WITHOUT_ITEM"
callback: "net.fabricmc.fabric.api.event.player.BlockEvents$UseWithoutItemCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.player.BlockEvents.USE_WITHOUT_ITEM

Callback interface: `net.fabricmc.fabric.api.event.player.BlockEvents$UseWithoutItemCallback`

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `BlockBehaviourBlockStateBaseMixin.callUseWithoutItemEvent` | `BlockBehaviour$BlockStateBase.useWithoutItem` @Inject at HEAD | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
