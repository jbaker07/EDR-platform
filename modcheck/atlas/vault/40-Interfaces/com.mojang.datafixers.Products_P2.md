---
type: "interface"
fqcn: "com.mojang.datafixers.Products$P2"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.datafixers.Products$P2

Package `com.mojang.datafixers`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@51 in `GameRulesServiceGameRuleUpdateMixin.lambda$fabric_createTypedCodec$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@43 in `CustomUnbakedBlockStateModelRegistry.lambda$static$1` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@46 in `ComponentsIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@46 in `CustomDataIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@46 in `DifferenceIngredient$Serializer.lambda$static$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@56 in `OverlayConditionsMetadata$Entry.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@60 in `RegistryContainsResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@60 in `TagsPopulatedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@46 in `TagRemovalInternals.lambda$modifyTagFileCodec$0` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@54 in `VariantCodecs.lambda$static$1` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/BiFuncti` | exact | invokevirtual@54 in `VariantCodecs.lambda$static$0` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
