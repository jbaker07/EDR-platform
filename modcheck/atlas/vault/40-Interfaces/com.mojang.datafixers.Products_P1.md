---
type: "interface"
fqcn: "com.mojang.datafixers.Products$P1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.datafixers.Products$P1

Package `com.mojang.datafixers`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Lcom/mojang/datafixers/kinds` | exact | invokevirtual@20 in `TheEndBiomeSourceMixin.lambda$modifyCodec$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/Function` | exact | invokevirtual@36 in `CompositeBlockStateModelImpl$Unbaked.lambda$static$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/Function` | exact | invokevirtual@33 in `AllModsLoadedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/Function` | exact | invokevirtual@33 in `AndResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/Function` | exact | invokevirtual@33 in `AnyModsLoadedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/Function` | exact | invokevirtual@33 in `FeaturesEnabledResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/Function` | exact | invokevirtual@28 in `NotResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/Function` | exact | invokevirtual@33 in `OrResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `apply` | `(Lcom/mojang/datafixers/kinds/Applicative;Ljava/util/function/Function` | exact | invokevirtual@34 in `DefaultResourcePackStorage.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
