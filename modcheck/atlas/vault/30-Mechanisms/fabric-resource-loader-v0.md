---
type: "mechanism"
module: "fabric-resource-loader-v0"
version: "3.3.26+4fc5413f5d"
sha256: "18afa6466d69ff68e1eeb74088d210a55004d11380ec14aad6ea66faa7da1922"
lifecycle: "deprecated"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-resource-loader-v0

**Version** `3.3.26+4fc5413f5d` -- **artifact sha256** `18afa6466d69ff68e1eeb74088d210a55004d11380ec14aad6ea66faa7da1922`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-resource-loader-v1": "*"}`
- entrypoints: `null`
- mixin configs: `null`

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.resource.IdentifiableResourceReloadListener|IdentifiableResourceReloadListener]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.ModResourcePack|ModResourcePack]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.ResourceManagerHelper|ResourceManagerHelper]] (interface, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.ResourcePackActivationType|ResourcePackActivationType]] (class, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.ResourceReloadListenerKeys|ResourceReloadListenerKeys]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.SimpleResourceReloadListener|SimpleResourceReloadListener]] (interface, 3 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
