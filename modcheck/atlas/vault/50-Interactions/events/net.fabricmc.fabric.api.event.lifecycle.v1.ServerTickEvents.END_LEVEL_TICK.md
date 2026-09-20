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

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerLevelMixin.endLevelTick` @13 | [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]].`tick` @Inject TAIL | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_LEVEL_TICK|read it]].
