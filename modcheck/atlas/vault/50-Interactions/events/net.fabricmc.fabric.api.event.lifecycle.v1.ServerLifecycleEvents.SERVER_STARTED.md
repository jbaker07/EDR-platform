---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTED"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$ServerStarted"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTED

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$ServerStarted`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `MinecraftServerMixin.afterSetupServer` @13 | [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`runServer` @Inject INVOKE `Lnet/minecraft/server/MinecraftServer;buildServerStatus()Lnet/minecraft/network/protocol/status/ServerStatus;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTED|read it]].
