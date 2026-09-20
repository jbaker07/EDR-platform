---
type: "interface"
fqcn: "com.mojang.serialization.DataResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.serialization.DataResult

Package `com.mojang.serialization`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `error` | `()Ljava/util/Optional;` | exact | invokeinterface@136 in `ResourceConditionsImpl.applyResourceConditions` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@15 in `AttachmentSerializingImpl.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@36 in `AttachmentSerializingImpl.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@17 in `GameRuleBuilder$EnumRuleBuilder.lambda$createEnumCodec$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@61 in `GameRuleMixin.deserializeEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@79 in `GameRuleMixin.deserializeEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@108 in `CustomUnbakedBlockStateModelRegistry.lambda$static$9` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@177 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@6 in `CustomIngredientImpl.lambda$static$1` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@6 in `ResourceConditionType.lambda$static$1` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `error` | `(Ljava/util/function/Supplier;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@27 in `OverlayConditionsMetadata$Entry.validateDirectory` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `flatMap` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/DataResult;` | exact | invokeinterface@22 in `FailSoftMapCodec.decode` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `getOrThrow` | `()Ljava/lang/Object;` | exact | invokeinterface@26 in `FabricCodecDataProvider.convert` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOrThrow` | `()Ljava/lang/Object;` | exact | invokeinterface@56 in `FabricDataGenHelper.addConditions` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOrThrow` | `()Ljava/lang/Object;` | exact | invokeinterface@42 in `DoubleRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getOrThrow` | `()Ljava/lang/Object;` | exact | invokeinterface@50 in `ResourceConditionsImpl.applyResourceConditions` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `getOrThrow` | `()Ljava/lang/Object;` | exact | invokeinterface@44 in `DefaultResourcePackStorage.read` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getOrThrow` | `()Ljava/lang/Object;` | exact | invokeinterface@15 in `DefaultResourcePackStorage.write` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getOrThrow` | `()Ljava/lang/Object;` | exact | invokeinterface@17 in `ModPackResourcesUtil.getMetadataPackJson` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getOrThrow` | `()Ljava/lang/Object;` | exact | invokeinterface@10 in `SpecialCodecs$1.lambda$decode$0` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Ljava/util/function/Function;)Ljava/lang/Object;` | exact | invokeinterface@132 in `FabricAdvancementProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Ljava/util/function/Function;)Ljava/lang/Object;` | exact | invokeinterface@16 in `FabricRecipeProvider.lambda$run$2` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Ljava/util/function/Function;)Ljava/lang/Object;` | exact | invokeinterface@16 in `FabricRecipeProvider.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Ljava/util/function/Function;)Ljava/lang/Object;` | exact | invokeinterface@93 in `FabricLootTableProviderImpl.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Ljava/util/function/Function;)Ljava/lang/Object;` | exact | invokeinterface@111 in `GameRuleBuilder.build` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `isSuccess` | `()Z` | exact | invokeinterface@9 in `DoubleRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `isSuccess` | `()Z` | exact | invokeinterface@40 in `ResourceConditionsImpl.applyResourceConditions` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/DataResult;` | exact | invokeinterface@33 in `FailSoftMapCodec.decode` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/DataResult;` | exact | invokeinterface@27 in `CustomUnbakedBlockStateModelRegistry$KeyExistsCodec.decode` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/DataResult;` | exact | invokeinterface@47 in `CustomUnbakedBlockStateModelRegistry$KeyExistsCodec.decode` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `map` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/DataResult;` | exact | invokeinterface@27 in `VariantCodecs.validateComponents` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `mapError` | `(Ljava/util/function/UnaryOperator;)Lcom/mojang/serialization/DataResu` | exact | invokeinterface@21 in `FabricCodecDataProvider.convert` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `result` | `()Ljava/util/Optional;` | exact | invokeinterface@34 in `FailSoftMapCodec.lambda$decode$2` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `result` | `()Ljava/util/Optional;` | exact | invokeinterface@43 in `FailSoftMapCodec.lambda$decode$2` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `result` | `()Ljava/util/Optional;` | exact | invokeinterface@158 in `DefaultResourcePackStorage.read` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `result` | `()Ljava/util/Optional;` | exact | invokeinterface@90 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `resultOrPartial` | `()Ljava/util/Optional;` | exact | invokeinterface@80 in `FailSoftMapCodec.lambda$decode$2` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `resultOrPartial` | `()Ljava/util/Optional;` | exact | invokeinterface@121 in `FailSoftMapCodec.lambda$decode$2` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `resultOrPartial` | `(Ljava/util/function/Consumer;)Ljava/util/Optional;` | exact | invokeinterface@15 in `FabricDynamicRegistryProvider.writeToPath` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `setLifecycle` | `(Lcom/mojang/serialization/Lifecycle;)Lcom/mojang/serialization/DataRe` | exact | invokeinterface@10 in `FailSoftMapCodec.decode` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@46 in `AttachmentSavedData$1.encode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@74 in `AttachmentSavedData$2.decode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@43 in `AttachmentSerializingImpl.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@31 in `FailSoftMapCodec.decode` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@39 in `TaggedChoiceTaggedChoiceTypeMixin.onGetCodec` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@5 in `GameRuleBuilder$EnumRuleBuilder.lambda$createEnumCodec$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@67 in `GameRuleMixin.deserializeEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@60 in `CustomUnbakedBlockStateModelRegistry.lambda$static$9` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@80 in `CustomUnbakedBlockStateModelRegistry.lambda$static$9` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@97 in `CustomUnbakedBlockStateModelRegistry.lambda$static$9` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@185 in `CustomUnbakedBlockStateModelRegistry.lambda$static$5` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@6 in `CustomIngredientImpl.lambda$static$3` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@16 in `OverlayConditionsMetadata$Entry.validateDirectory` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@80 in `SimpleJsonResourceReloadListenerMixin.applyResourceConditions` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@22 in `SpecialCodecs$1.decode` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `success` | `(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` | exact | invokestatic@21 in `SpecialCodecs$2.decode` | unknown | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
