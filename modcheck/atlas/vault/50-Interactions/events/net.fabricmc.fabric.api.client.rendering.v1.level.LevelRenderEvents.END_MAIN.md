---
type: "event"
event: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.END_MAIN"
callback: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents$EndMain"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.END_MAIN

Callback interface: `net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents$EndMain`

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LevelRendererMixin.endMainRender` @13 | [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]].`lambda$addMainPass$0` @Inject RETURN | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
