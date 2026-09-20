---
type: "mechanism"
module: "fabric-loot-api-v3"
version: "4.0.8+4068fd645d"
sha256: "569540023c6d19e4b4854e14ea4bb99aed401946d5470f761ac1f1388cc7de2c"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-loot-api-v3

**Version** `4.0.8+4068fd645d` -- **artifact sha256** `569540023c6d19e4b4854e14ea4bb99aed401946d5470f761ac1f1388cc7de2c`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-resource-loader-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-loot-api-v3.mixins.json"]`
- access widener: `fabric-loot-api-v3.classtweaker`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.loot.v3.LootTableEvents.ALL_LOADED|LootTableEvents.ALL_LOADED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.loot.v3.LootTableEvents.MODIFY|LootTableEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.loot.v3.LootTableEvents.MODIFY_DROPS|LootTableEvents.MODIFY_DROPS]]
- [[50-Interactions/events/net.fabricmc.fabric.api.loot.v3.LootTableEvents.REPLACE|LootTableEvents.REPLACE]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.loot.v3.FabricLootPoolBuilder|FabricLootPoolBuilder]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.loot.v3.FabricLootTableBuilder|FabricLootTableBuilder]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.loot.v3.LootTableEvents|LootTableEvents]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.loot.v3.LootTableSource|LootTableSource]] (class, 8 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
