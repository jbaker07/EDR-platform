---
type: "event"
event: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLevelEvents.AFTER_CLIENT_LEVEL_CHANGE"
callback: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLevelEvents$AfterClientLevelChange"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLevelEvents.AFTER_CLIENT_LEVEL_CHANGE

Callback interface: `net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientLevelEvents$AfterClientLevelChange`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `MinecraftMixin.afterClientLevelChange` @22 | [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`updateLevelInEngines` @Inject TAIL | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
