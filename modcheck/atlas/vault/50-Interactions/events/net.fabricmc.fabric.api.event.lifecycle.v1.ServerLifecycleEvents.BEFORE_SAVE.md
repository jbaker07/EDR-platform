---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.BEFORE_SAVE"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$BeforeSave"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.BEFORE_SAVE

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents$BeforeSave`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `MinecraftServerMixin.startSave` @15 | [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`saveAllChunks` @Inject HEAD | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
