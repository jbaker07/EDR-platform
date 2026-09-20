---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STOPPING"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$ServerStopping"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STOPPING

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$ServerStopping`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `MinecraftServerMixin.beforeShutdownServer` @13 | [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`stopServer` @Inject HEAD | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STOPPING|read it]].
