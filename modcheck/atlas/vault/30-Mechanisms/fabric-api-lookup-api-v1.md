---
type: "mechanism"
module: "fabric-api-lookup-api-v1"
version: "2.0.24+3434d6d95d"
sha256: "4ff3be674760c602b4ed59c10d74d2d52597e8a562489ecd4b68ebf7f71d466c"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-api-lookup-api-v1

**Version** `2.0.24+3434d6d95d` -- **artifact sha256** `4ff3be674760c602b4ed59c10d74d2d52597e8a562489ecd4b68ebf7f71d466c`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-lifecycle-events-v1": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.lookup.ApiLookupImpl"]}`
- mixin configs: `["fabric-api-lookup-api-v1.mixins.json"]`

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.lookup.v1.block.BlockApiCache|BlockApiCache]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup|BlockApiLookup]] (interface, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.lookup.v1.custom.ApiLookupMap|ApiLookupMap]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.lookup.v1.custom.ApiProviderMap|ApiProviderMap]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.lookup.v1.entity.EntityApiLookup|EntityApiLookup]] (interface, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.lookup.v1.item.ItemApiLookup|ItemApiLookup]] (interface, 9 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
