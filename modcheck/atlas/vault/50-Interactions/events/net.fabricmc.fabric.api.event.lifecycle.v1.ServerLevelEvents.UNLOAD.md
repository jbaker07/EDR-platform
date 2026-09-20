---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLevelEvents.UNLOAD"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLevelEvents$Unload"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerLevelEvents.UNLOAD

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerLevelEvents$Unload`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `MinecraftServerMixin.onUnloadWorldAtShutdown` | `MinecraftServer.stopServer` @Inject at INVOKE Lnet/minecraft/server/level/ServerLevel;close()V | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
