---
type: "event"
event: "net.fabricmc.fabric.api.client.rendering.v1.LivingEntityFeatureRenderEvents.ALLOW_CAPE_RENDER"
callback: "net.fabricmc.fabric.api.client.rendering.v1.LivingEntityFeatureRenderEvents$AllowCapeRender"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.LivingEntityFeatureRenderEvents.ALLOW_CAPE_RENDER

Callback interface: `net.fabricmc.fabric.api.client.rendering.v1.LivingEntityFeatureRenderEvents$AllowCapeRender`

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `CapeLayerMixin.injectCapeRenderCheck` @11 | [[40-Interfaces/net.minecraft.client.renderer.entity.layers.CapeLayer|CapeLayer]].`submit` @Inject HEAD | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
