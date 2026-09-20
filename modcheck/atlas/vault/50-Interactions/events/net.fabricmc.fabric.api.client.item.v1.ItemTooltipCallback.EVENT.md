---
type: "event"
event: "net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback.EVENT"
callback: "net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback`

Module: [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ItemStackMixin.getTooltip` | `ItemStack.getTooltipLines` @Inject at RETURN | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
