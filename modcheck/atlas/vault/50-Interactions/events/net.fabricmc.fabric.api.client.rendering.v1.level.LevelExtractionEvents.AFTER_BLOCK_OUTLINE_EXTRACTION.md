---
type: "event"
event: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelExtractionEvents.AFTER_BLOCK_OUTLINE_EXTRACTION"
callback: "net.fabricmc.fabric.api.client.rendering.v1.level.LevelExtractionEvents$AfterBlockOutlineExtraction"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.level.LevelExtractionEvents.AFTER_BLOCK_OUTLINE_EXTRACTION

Callback interface: `net.fabricmc.fabric.api.client.rendering.v1.level.LevelExtractionEvents$AfterBlockOutlineExtraction`

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LevelExtractorMixin.afterBlockOutlineExtraction` @20 | [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]].`extractBlockOutline` @Inject RETURN | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
