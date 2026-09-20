---
type: "index"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Modification mechanisms

Three families, deliberately kept apart because they compose differently:

1. **Data packs / resource packs** -- no code. See [[00-Scope/Data_Driven_Surface|the data-driven surface]].
2. **Registries** -- code registers new content. See [[30-Mechanisms/Registries]].
3. **Fabric API modules** -- events, hooks and helpers, each implemented by mixins into vanilla. Listed below with what each actually hooks.

4. **Direct mixins by a mod** -- the same mechanism Fabric API uses, applied by a mod itself. See [[30-Mechanisms/Mixin]].

| module | lifecycle | env | mixin classes | injections | events published | api classes |
|---|---|---|---|---|---|---|
| [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | stable | * | 4 | 1 | 3 | 3 |
| [[30-Mechanisms/fabric-api|fabric-api]] | ? | * | 0 | 0 | 0 | 0 |
| [[30-Mechanisms/fabric-api-base|fabric-api-base]] | stable | * | 0 | 0 | 0 | 7 |
| [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | stable | * | 3 | 0 | 0 | 6 |
| [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | stable | * | 10 | 9 | 0 | 8 |
| [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | stable | * | 11 | 7 | 1 | 4 |
| [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | stable | * | 8 | 2 | 0 | 2 |
| [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | experimental | client | 32 | 33 | 0 | 14 |
| [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | stable | * | 10 | 7 | 2 | 7 |
| [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | stable | * | 23 | 12 | 0 | 10 |
| [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | stable | * | 2 | 0 | 0 | 11 |
| [[30-Mechanisms/fabric-crash-report-info-v1|fabric-crash-report-info-v1]] | stable | * | 3 | 2 | 0 | 0 |
| [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | stable | * | 6 | 9 | 1 | 4 |
| [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | stable | * | 22 | 15 | 0 | 6 |
| [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | stable | * | 32 | 24 | 0 | 23 |
| [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | stable | * | 6 | 4 | 0 | 5 |
| [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | stable | * | 8 | 6 | 1 | 1 |
| [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | stable | * | 16 | 32 | 30 | 9 |
| [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | stable | * | 13 | 22 | 19 | 14 |
| [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | stable | * | 10 | 3 | 0 | 3 |
| [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | stable | * | 6 | 6 | 0 | 2 |
| [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | stable | * | 25 | 24 | 6 | 16 |
| [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | stable | client | 4 | 2 | 0 | 1 |
| [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | stable | * | 26 | 54 | 39 | 14 |
| [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | stable | * | 8 | 0 | 4 | 4 |
| [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | stable | * | 3 | 3 | 0 | 3 |
| [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | stable | * | 6 | 11 | 21 | 4 |
| [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | stable | client | 7 | 11 | 0 | 16 |
| [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | stable | * | 41 | 38 | 30 | 25 |
| [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | stable | * | 16 | 9 | 1 | 10 |
| [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | stable | * | 14 | 4 | 1 | 6 |
| [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | stable | * | 3 | 2 | 0 | 6 |
| [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | stable | * | 15 | 9 | 1 | 9 |
| [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | stable | * | 27 | 30 | 1 | 9 |
| [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | stable | client | 32 | 22 | 0 | 29 |
| [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | stable | client | 3 | 0 | 0 | 0 |
| [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | stable | * | 3 | 1 | 0 | 3 |
| [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | stable | client | 60 | 51 | 18 | 38 |
| [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | stable | * | 9 | 5 | 0 | 3 |
| [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] | deprecated | * | 0 | 0 | 0 | 7 |
| [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | stable | * | 28 | 33 | 0 | 8 |
| [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | stable | client | 9 | 18 | 2 | 4 |
| [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | stable | * | 5 | 0 | 0 | 3 |
| [[30-Mechanisms/fabric-sound-api-v1|fabric-sound-api-v1]] | stable | client | 3 | 1 | 0 | 1 |
| [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | stable | * | 9 | 7 | 0 | 2 |
| [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | stable | * | 19 | 10 | 1 | 39 |
| [[30-Mechanisms/fabric-transitive-access-wideners-v1|fabric-transitive-access-wideners-v1]] | stable | * | 0 | 0 | 0 | 0 |
