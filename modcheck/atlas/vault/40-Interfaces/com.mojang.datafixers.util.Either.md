---
type: "interface"
fqcn: "com.mojang.datafixers.util.Either"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.datafixers.util.Either

Package `com.mojang.datafixers.util`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `left` | `()Ljava/util/Optional;` | exact | invokevirtual@4 in `TestScreenshotComparisonOptionsImpl.getTemplateImagePath` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `left` | `()Ljava/util/Optional;` | exact | invokevirtual@1 in `ResourceManagerRegistryLoadTaskMixin.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `left` | `()Ljava/util/Optional;` | exact | invokevirtual@11 in `ResourceManagerRegistryLoadTaskMixin.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@18 in `TestScreenshotComparisonOptionsImpl.<init>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@65 in `SoundTypeBuilderImpl$Entry.lambda$static$2` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@31 in `PlayerMixin.onStartSleepInBed` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@72 in `ServerPlayerMixin.redirectSleepDirection` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@20 in `GameRulesServiceGameRuleUpdateMixin.lambda$getValueAndFabricTypeCodec$ | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@111 in `ResourceManagerRegistryLoadTaskMixin.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@54 in `CustomUnbakedBlockStateModelRegistry.lambda$static$9` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@94 in `CustomUnbakedBlockStateModelRegistry.lambda$static$9` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@118 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `left` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@17 in `IngredientMixin.lambda$injectCodec$2` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@62 in `TheEndBiomeData$ResourceKeyHashStrategy.equals` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@22 in `TheEndBiomeData$ResourceKeyHashStrategy.hashCode` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@14 in `TestScreenshotComparisonOptionsImpl.getGrayscaleTemplateImage` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@14 in `TestScreenshotComparisonOptionsImpl.getColorTemplateImage` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@7 in `GameRulesServiceGameRuleUpdateMixin.lambda$getValueAndFabricTypeCodec$ | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@17 in `CustomUnbakedBlockStateModelRegistry$KeyExistsCodec.encode` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@9 in `CustomUnbakedBlockStateModelRegistry.lambda$static$7` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@7 in `CustomUnbakedBlockStateModelRegistry.lambda$static$8` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@7 in `CustomUnbakedBlockStateModelRegistry.lambda$static$4` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang` | exact | invokevirtual@11 in `IngredientMixin.lambda$injectCodec$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `mapLeft` | `(Ljava/util/function/Function;)Lcom/mojang/datafixers/util/Either;` | exact | invokevirtual@101 in `ResourceManagerRegistryLoadTaskMixin.modifyAdvancement` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `mapLeft` | `(Ljava/util/function/Function;)Lcom/mojang/datafixers/util/Either;` | exact | invokevirtual@101 in `ResourceManagerRegistryLoadTaskMixin.modifyLootTable` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `right` | `()Ljava/util/Optional;` | exact | invokevirtual@4 in `ResourceManagerRegistryLoadTaskMixin.load` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `right` | `()Ljava/util/Optional;` | exact | invokevirtual@17 in `ResourceManagerRegistryLoadTaskMixin.load` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `right` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@18 in `TestScreenshotComparisonOptionsImpl.<init>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `right` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@60 in `SoundTypeBuilderImpl$Entry.lambda$static$2` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `right` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@13 in `GameRulesServiceGameRuleUpdateMixin.lambda$getValueAndFabricTypeCodec$ | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `right` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@57 in `CustomUnbakedBlockStateModelRegistry.lambda$static$9` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `right` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@74 in `CustomUnbakedBlockStateModelRegistry.lambda$static$9` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `right` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@77 in `CustomUnbakedBlockStateModelRegistry.lambda$static$9` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `right` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@152 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `right` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@10 in `IngredientMixin.lambda$injectCodec$2` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `right` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/util/Either;` | exact | invokestatic@39 in `RegistryLoadTaskPendingRegistrationMixin.loadFromResource` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
