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

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `CapeLayerMixin.injectCapeRenderCheck` | `CapeLayer.submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/AvatarRenderState;FF)V` @Inject at HEAD | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
