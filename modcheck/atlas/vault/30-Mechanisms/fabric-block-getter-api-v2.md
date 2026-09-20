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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.renderer.chunk.RenderRegionCache|RenderRegionCache]] | `createRegion` | injects_into `@Inject at INVOKE Lnet/minecraft/client/renderer/chunk/RenderRegionCache;getSectionDataCopy(Lnet/minecraft/world/level/Level;III)Lnet/minecraft/client/renderer/chunk/SectionCopy;` | client | `RenderRegionCacheMixin.copyDataForChunk` |
| [[40-Interfaces/net.minecraft.client.renderer.chunk.RenderRegionCache|RenderRegionCache]] | `createRegion` | injects_into `@Inject at RETURN` | client | `RenderRegionCacheMixin.createDataMap` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.blockgetter.v2.FabricBlockGetter|FabricBlockGetter]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.blockgetter.v2.RenderDataBlockEntity|RenderDataBlockEntity]] (interface, 1 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
