---
type: "event"
event: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_LEVEL_TICK"
callback: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents$EndLevelTick"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_LEVEL_TICK

Callback interface: `net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents$EndLevelTick`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ClientLevelMixin.endLevelTick` | `ClientLevel.tick` @Inject at RETURN | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
