---
type: "interface"
fqcn: "com.mojang.serialization.codecs.RecordCodecBuilder$Instance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.serialization.codecs.RecordCodecBuilder$Instance

Package `com.mojang.serialization.codecs`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@7 in `TheEndBiomeSourceMixin.lambda$modifyCodec$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@30 in `WorldDimensionsMixin.useFailSoftMap` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@27 in `CompositeBlockStateModelImpl$Unbaked.lambda$static$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@24 in `AllModsLoadedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@24 in `AndResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@24 in `AnyModsLoadedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@24 in `FeaturesEnabledResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@19 in `NotResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@24 in `OrResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;)Lcom/mojang/datafixers/Products$P1;` | inherited_exact | invokevirtual@25 in `DefaultResourcePackStorage.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@41 in `GameRulesServiceGameRuleUpdateMixin.lambda$fabric_createTypedCodec$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@34 in `CustomUnbakedBlockStateModelRegistry.lambda$static$1` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@37 in `ComponentsIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@37 in `CustomDataIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@37 in `DifferenceIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@47 in `OverlayConditionsMetadata$Entry.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@51 in `RegistryContainsResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@51 in `TagsPopulatedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@37 in `TagRemovalInternals.lambda$modifyTagFileCodec$0` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@45 in `VariantCodecs.lambda$static$1` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;)Lc` | inherited_exact | invokevirtual@45 in `VariantCodecs.lambda$static$0` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;Lco` | inherited_exact | invokevirtual@64 in `SoundTypeBuilderImpl$SoundType.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `group` | `(Lcom/mojang/datafixers/kinds/App;Lcom/mojang/datafixers/kinds/App;Lco` | inherited_exact | invokevirtual@182 in `SoundTypeBuilderImpl$Entry.lambda$static$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `stable` | `(Ljava/lang/Object;)Lcom/mojang/datafixers/kinds/App;` | exact | invokevirtual@17 in `TheEndBiomeSourceMixin.lambda$modifyCodec$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
