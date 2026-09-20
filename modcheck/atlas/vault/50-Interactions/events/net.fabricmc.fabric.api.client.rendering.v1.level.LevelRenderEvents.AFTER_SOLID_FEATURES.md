---
type: "event"
event: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_SOLID_FEATURES"
callback: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents$AfterSolidFeatures"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_SOLID_FEATURES

Callback interface: `net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents$AfterSolidFeatures`

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `LevelRendererMixin.afterRenderSolidFeatures` | `LevelRenderer.executeSolid` @Inject at INVOKE Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;executeSolid(Lcom/mojang/renderpearl/api/commands/RenderPass;)V | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
