---
type: "event"
event: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_TRANSLUCENT_FEATURES"
callback: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents$AfterTranslucentFeatures"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_TRANSLUCENT_FEATURES

Callback interface: `net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents$AfterTranslucentFeatures`

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LevelRendererMixin.afterRenderClassicTranslucentFeatures` @13 | [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]].`executeClassicTransparency` @Inject INVOKE `Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;executeTranslucent(Lcom/mojang/renderpearl/api/commands/RenderPass;)V` | unknown | static_inference |
| `LevelRendererMixin.afterRenderOitTranslucentFeatures` @13 | [[40-Interfaces/net.minecraft.client.renderer.LevelRenderer|LevelRenderer]].`executeOit` @Inject RETURN | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
