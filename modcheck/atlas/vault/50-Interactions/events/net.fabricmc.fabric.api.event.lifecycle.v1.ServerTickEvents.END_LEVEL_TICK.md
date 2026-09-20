---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents$EndLevelTick"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents$EndLevelTick`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ServerLevelMixin.endLevelTick` | `ServerLevel.tick` @Inject at TAIL | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK|read it]].
