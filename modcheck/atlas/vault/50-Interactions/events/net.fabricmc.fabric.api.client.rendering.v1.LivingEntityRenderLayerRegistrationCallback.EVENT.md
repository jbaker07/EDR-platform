---
type: "event"
event: "net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback.EVENT"
callback: "net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback`

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `EntityRenderersMixin.createEntityRenderer` @60 | [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderers|EntityRenderers]].`lambda$createEntityRenderers$0` @Redirect INVOKE `Lnet/minecraft/client/renderer/entity/EntityRendererProvider;create(Lnet/minecraft/client/renderer/entity/EntityRendererProvider$Context;)Lnet/minecraft/client/renderer/entity/EntityRenderer;` | unknown | static_inference |
| `EntityRenderersMixin.createAvatarRenderer` @64 | [[40-Interfaces/net.minecraft.client.renderer.entity.EntityRenderers|EntityRenderers]].`createAvatarRenderers` @WrapOperation NEW `(Lnet/minecraft/client/renderer/entity/EntityRendererProvider$Context;Z)Lnet/minecraft/client/renderer/entity/player/AvatarRenderer;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
