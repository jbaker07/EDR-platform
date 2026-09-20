---
type: "interface"
fqcn: "com.mojang.serialization.JsonOps"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.serialization.JsonOps

Package `com.mojang.serialization`: a library outside the Minecraft jar (no system note).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@27 in `FabricAdvancementProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@9 in `FabricCodecDataProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@1 in `FabricDynamicRegistryProvider.lambda$run$2` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@53 in `FabricRecipeProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@44 in `FabricDataGenHelper.addConditions` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@16 in `FabricLootTableProviderImpl.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@22 in `ResourceConditionsImpl.applyResourceConditions` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@32 in `DefaultResourcePackStorage.read` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@6 in `DefaultResourcePackStorage.write` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@4 in `ModPackResourcesUtil.getMetadataPackJson` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@159 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| reads | `INSTANCE` | `Lcom/mojang/serialization/JsonOps;` | exact | getstatic@77 in `ClientTagsLoader.loadTag` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Members

Not in the processed Minecraft jar: this type belongs to a library Minecraft depends on (see [[00-Scope/Corpus]], group minecraft_library) and its members were not extracted.
