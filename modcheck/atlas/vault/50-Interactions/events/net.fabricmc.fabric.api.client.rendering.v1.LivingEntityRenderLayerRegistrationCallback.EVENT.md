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

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `EntityRenderersMixin.onRegisterRenderers` | `EntityRenderers.<clinit>*` @Inject at RETURN | client | static_inference |
| `EntityRenderersMixin.createAvatarRenderer` | (handler is not itself an injector method: fired from a helper or impl class) | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
