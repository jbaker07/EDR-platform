---
type: "mechanism"
module: "fabric-block-getter-api-v2"
version: "2.0.9+3434d6d95d"
sha256: "0f819114e0eb2d7e2950b25fe4edc581eb453bd4f98f77ffc2ddabe07a557764"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-block-getter-api-v2

**Version** `2.0.9+3434d6d95d` -- **artifact sha256** `0f819114e0eb2d7e2950b25fe4edc581eb453bd4f98f77ffc2ddabe07a557764`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `null`
- mixin configs: `["fabric-block-getter-api-v2.mixins.json", {"config": "fabric-block-getter-api-v2.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-block-getter-api-v2.classtweaker`
- mixin classes: 6 found by annotation, 6 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.renderer.chunk.RenderRegionCache|RenderRegionCache]].`createRegion` | `(Lnet/minecraft/client/multiplayer/ClientLevel;J)Lnet/minecraft/client/renderer/chunk/RenderSectionRegion;` | name_only | @Inject | INVOKE `Lnet/minecraft/client/renderer/chunk/RenderRegionCache;getSectionDataCopy(Lnet/minecraft/world/level/Level;III)Lnet/minecraft/client/renderer/chunk/SectionCopy;` (exact) | client | 1000 (default) | `RenderRegionCacheMixin.copyDataForChunk` |
| [[40-Interfaces/net.minecraft.client.renderer.chunk.RenderRegionCache|RenderRegionCache]].`createRegion` | `(Lnet/minecraft/client/multiplayer/ClientLevel;J)Lnet/minecraft/client/renderer/chunk/RenderSectionRegion;` | name_only | @Inject | RETURN | client | 1000 (default) | `RenderRegionCacheMixin.createDataMap` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.blockgetter.v2.FabricBlockGetter|FabricBlockGetter]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.blockgetter.v2.RenderDataBlockEntity|RenderDataBlockEntity]] (interface, 1 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
