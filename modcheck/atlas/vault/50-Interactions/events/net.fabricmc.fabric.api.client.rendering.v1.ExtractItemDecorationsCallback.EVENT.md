---
type: "event"
event: "net.fabricmc.fabric.api.client.rendering.v1.ExtractItemDecorationsCallback.EVENT"
callback: "net.fabricmc.fabric.api.client.rendering.v1.ExtractItemDecorationsCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.ExtractItemDecorationsCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.client.rendering.v1.ExtractItemDecorationsCallback`

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `GuiGraphicsExtractorMixin.drawStackOverlay` | `GuiGraphicsExtractor.itemDecorations(Lnet/minecraft/client/gui/Font;Lnet/minecraft/world/item/ItemStack;IILjava/lang/String;)V` @Inject at RETURN | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
