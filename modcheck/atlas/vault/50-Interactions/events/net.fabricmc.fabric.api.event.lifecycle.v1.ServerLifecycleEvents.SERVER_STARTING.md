---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTING"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$ServerStarting"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTING

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$ServerStarting`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `MinecraftServerMixin.beforeSetupServer` | `MinecraftServer.runServer` @Inject at INVOKE Lnet/minecraft/server/MinecraftServer;initServer()Z | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTING|read it]].
