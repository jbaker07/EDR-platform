---
type: "interface"
fqcn: "com.mojang.serialization.codecs.RecordCodecBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.serialization.codecs.RecordCodecBuilder

Package `com.mojang.serialization.codecs`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `create` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@5 in `SoundTypeBuilderImpl$Entry.<clinit>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `create` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@5 in `SoundTypeBuilderImpl$SoundType.<clinit>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `create` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@90 in `CustomUnbakedBlockStateModelRegistry.<clinit>` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `create` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@5 in `OverlayConditionsMetadata$Entry.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `create` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@58 in `DefaultResourcePackStorage.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `create` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@6 in `TagRemovalInternals.modifyTagFileCodec` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `create` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@5 in `VariantCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `create` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | exact | invokestatic@62 in `VariantCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `TheEndBiomeSourceMixin.modifyCodec` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@6 in `GameRulesServiceGameRuleUpdateMixin.fabric_createTypedCodec` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `CompositeBlockStateModelImpl$Unbaked.<clinit>` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@15 in `ComponentsIngredient$Serializer.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@15 in `CustomDataIngredient$Serializer.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@15 in `DifferenceIngredient$Serializer.<clinit>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `AllModsLoadedResourceCondition.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `AndResourceCondition.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `AnyModsLoadedResourceCondition.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `FeaturesEnabledResourceCondition.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `NotResourceCondition.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `OrResourceCondition.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `RegistryContainsResourceCondition.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `mapCodec` | `(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;` | exact | invokestatic@5 in `TagsPopulatedResourceCondition.<clinit>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
