---
type: "index"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Modification mechanisms

Four families, kept apart because they compose differently:

1. **Data packs / resource packs** -- no code. See [[00-Scope/Data_Driven_Surface|the data-driven surface]].
2. **Registries** -- code registers new content. See [[30-Mechanisms/Registries]].
3. **Fabric API modules** -- events, hooks and helpers, each implemented by mixins into vanilla. Listed below with what each actually hooks.
4. **Direct mixins by a mod** -- the same mechanism Fabric API uses, applied by a mod itself. See [[30-Mechanisms/Mixin]] and [[30-Mechanisms/Transformation_Tests|the transformation tests]].

| module | lifecycle | env | mixins | injections | by injector | events published | api classes |
|---|---|---|---|---|---|---|---|
| [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | stable | * | 3 | 4 | WrapOperation 2, Inject 1, ModifyReceiver 1 | 3 | 3 |
| [[30-Mechanisms/fabric-api|fabric-api]] | ? | * | 0 | 0 |  | 0 | 0 |
| [[30-Mechanisms/fabric-api-base|fabric-api-base]] | stable | * | 0 | 0 |  | 0 | 7 |
| [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | stable | * | 2 | 0 |  | 0 | 6 |
| [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | stable | * | 8 | 9 | Inject 8, Redirect 1 | 0 | 8 |
| [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | stable | * | 10 | 8 | Inject 4, Redirect 2, WrapOperation 1, ModifyArg 1 | 1 | 4 |
| [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | stable | * | 6 | 2 | Inject 2 | 0 | 2 |
| [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | experimental | client | 25 | 41 | Inject 33, WrapOperation 3, WrapMethod 3, WrapWithCondition 1, ModifyExpressionValue 1 | 0 | 14 |
| [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | stable | * | 8 | 7 | Inject 7 | 2 | 7 |
| [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | stable | * | 20 | 33 | ModifyExpressionValue 11, Inject 9, ModifyReturnValue 5, WrapOperation 4, ModifyArg 3, WrapWithCondition 1 | 0 | 10 |
| [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | stable | * | 1 | 0 |  | 0 | 11 |
| [[30-Mechanisms/fabric-crash-report-info-v1|fabric-crash-report-info-v1]] | stable | * | 2 | 2 | ModifyArg 1, Inject 1 | 0 | 0 |
| [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | stable | * | 4 | 9 | Inject 9 | 1 | 4 |
| [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | stable | * | 20 | 20 | Inject 14, WrapOperation 4, ModifyExpressionValue 1, ModifyArg 1 | 0 | 6 |
| [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | stable | * | 27 | 34 | ModifyVariable 10, Inject 7, ModifyArg 6, ModifyExpressionValue 5, WrapOperation 5, Redirect 1 | 0 | 23 |
| [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | stable | * | 4 | 5 | Inject 4, WrapOperation 1 | 0 | 5 |
| [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | stable | * | 7 | 7 | Inject 3, Redirect 3, WrapMethod 1 | 1 | 1 |
| [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | stable | * | 13 | 42 | Inject 24, WrapOperation 6, Redirect 5, WrapMethod 4, ModifyVariable 2, ModifyArg 1 | 31 | 9 |
| [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | stable | * | 11 | 27 | Inject 22, WrapOperation 5 | 19 | 14 |
| [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | stable | * | 8 | 6 | Inject 3, WrapMethod 1, ModifyReturnValue 1, WrapOperation 1 | 0 | 3 |
| [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | stable | * | 4 | 7 | Inject 6, ModifyExpressionValue 1 | 0 | 2 |
| [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | stable | * | 23 | 29 | Inject 13, Redirect 8, ModifyArg 3, WrapOperation 3, WrapMethod 1, ModifyExpressionValue 1 | 7 | 16 |
| [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | stable | client | 3 | 2 | Inject 2 | 0 | 1 |
| [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | stable | * | 23 | 57 | Inject 51, Redirect 3, ModifyExpressionValue 2, WrapOperation 1 | 39 | 14 |
| [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | stable | * | 7 | 3 | WrapOperation 2, WrapMethod 1 | 4 | 4 |
| [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | stable | * | 2 | 3 | Redirect 2, Inject 1 | 0 | 3 |
| [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | stable | * | 4 | 11 | Inject 11 | 22 | 4 |
| [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | stable | client | 6 | 18 | Inject 4, ModifyArg 4, Redirect 3, ModifyExpressionValue 3, ModifyReturnValue 2, WrapOperation 2 | 0 | 16 |
| [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | stable | * | 37 | 47 | Inject 32, WrapOperation 7, Redirect 3, ModifyArg 2, WrapMethod 2, ModifyVariable 1 | 30 | 25 |
| [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | stable | * | 14 | 13 | Inject 9, WrapOperation 4 | 1 | 10 |
| [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | stable | * | 12 | 14 | ModifyExpressionValue 8, Inject 3, WrapOperation 1, Redirect 1, ModifyReturnValue 1 | 1 | 6 |
| [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | stable | * | 2 | 3 | Inject 2, ModifyReturnValue 1 | 2 | 6 |
| [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | stable | * | 11 | 13 | Inject 9, ModifyExpressionValue 2, WrapOperation 1, ModifyReturnValue 1 | 1 | 9 |
| [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | stable | * | 25 | 37 | Inject 12, ModifyVariable 8, Redirect 7, WrapOperation 4, ModifyArg 3, ModifyReturnValue 2, ModifyExpressionValue 1 | 1 | 9 |
| [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | stable | client | 26 | 27 | Inject 13, Redirect 9, WrapOperation 3, ModifyReturnValue 1, Overwrite 1 | 0 | 29 |
| [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | stable | client | 2 | 0 |  | 0 | 0 |
| [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | stable | * | 2 | 3 | Inject 1, ModifyExpressionValue 1, WrapMethod 1 | 0 | 3 |
| [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | stable | client | 57 | 104 | Inject 48, WrapOperation 39, ModifyExpressionValue 5, ModifyReturnValue 4, WrapWithCondition 4, Redirect 2, ModifyVariable 1, WrapMethod 1 | 18 | 38 |
| [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | stable | * | 8 | 7 | Inject 4, ModifyVariable 1, ModifyExpressionValue 1, WrapOperation 1 | 0 | 3 |
| [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] | deprecated | * | 0 | 0 |  | 0 | 7 |
| [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | stable | * | 25 | 38 | Inject 17, Redirect 8, ModifyArg 6, WrapOperation 2, ModifyReturnValue 2, ModifyVariable 2, ModifyExpressionValue 1 | 0 | 8 |
| [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | stable | client | 8 | 26 | Inject 18, WrapOperation 8 | 2 | 4 |
| [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | stable | * | 4 | 0 |  | 0 | 3 |
| [[30-Mechanisms/fabric-sound-api-v1|fabric-sound-api-v1]] | stable | client | 2 | 1 | Redirect 1 | 0 | 1 |
| [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | stable | * | 8 | 11 | Inject 6, WrapOperation 3, ModifyExpressionValue 1, ModifyArg 1 | 0 | 2 |
| [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | stable | * | 18 | 12 | Inject 8, WrapOperation 2, ModifyVariable 1, Redirect 1 | 1 | 39 |
| [[30-Mechanisms/fabric-transitive-access-wideners-v1|fabric-transitive-access-wideners-v1]] | stable | * | 0 | 0 |  | 0 | 0 |
