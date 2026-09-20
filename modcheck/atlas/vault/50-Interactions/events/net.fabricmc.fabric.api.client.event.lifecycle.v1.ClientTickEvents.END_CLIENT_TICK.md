---
type: "event"
event: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_CLIENT_TICK"
callback: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents$EndTick"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_CLIENT_TICK

Callback interface: `net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents$EndTick`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `MinecraftMixin.onEndTick` @13 | [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`tick` @Inject RETURN | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_CLIENT_TICK|read it]].
