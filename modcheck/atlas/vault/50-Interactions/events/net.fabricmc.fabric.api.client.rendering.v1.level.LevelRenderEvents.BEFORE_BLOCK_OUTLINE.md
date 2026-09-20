---
type: "event"
event: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.BEFORE_BLOCK_OUTLINE"
callback: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents$BeforeBlockOutline"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.BEFORE_BLOCK_OUTLINE

Callback interface: `net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents$BeforeBlockOutline`

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LevelRendererMixin.beforeRenderBlockOutline` @23 | [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]].`submitBlockOutline` @Inject FIELD `Lnet/minecraft/client/renderer/state/level/CameraRenderState;pos:Lnet/minecraft/world/phys/Vec3;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
