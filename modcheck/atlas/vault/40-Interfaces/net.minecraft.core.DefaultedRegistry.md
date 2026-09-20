---
type: "interface"
fqcn: "net.minecraft.core.DefaultedRegistry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.DefaultedRegistry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/Registry`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@75 in `BlockApiLookupImpl.registerForBlocks` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@67 in `EntityApiLookupImpl.registerForTypes` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@66 in `EntityApiLookupImpl.lambda$checkSelfImplementingTypes$0` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@84 in `ItemApiLookupImpl.registerForItems` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@24 in `BiomeModifications.addSpawn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@8 in `FabricBlockLootSubProvider.excludeFromStrictValidation` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@8 in `FabricEntityLootSubProvider.excludeFromStrictValidation` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@37 in `ModelProviderItemInfoCollectorMixin.filterItemsForProcessingMod` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@4 in `BlockLootSubProviderMixin.lambda$onlyVanillaBlocks$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@18 in `RecipeProviderMixin.adjustIdStonecutter` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@13 in `RecipeProviderMixin.adjustIdWaxRecipes` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@11 in `RecipeProviderMixin.dontGenerateNonVanillaWaxingRecipes` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@22 in `RecipeProviderMixin.dontGenerateNonVanillaWaxingRecipes` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@22 in `FabricDefaultAttributeRegistry.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@36 in `MinecartComparatorLogicRegistry.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@86 in `ArmorRendererRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@66 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@43 in `FluidVariantAttributeHandler.getName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@56 in `FluidVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getResourceKey` | `(Ljava/lang/Object;)Ljava/util/Optional;` | inherited_exact | invokeinterface@35 in `BiomeModifications.addSpawn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getResourceKey` | `(Ljava/lang/Object;)Ljava/util/Optional;` | inherited_exact | invokeinterface@18 in `ModelLoadingPluginContextImpl.registerBlockStateResolver` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getTagOrEmpty` | `(Lnet/minecraft/tags/TagKey;)Ljava/lang/Iterable;` | inherited_exact | invokeinterface@70 in `FlammableBlockRegistryImpl.getEntryMap` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@153 in `FabricBlockLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@191 in `FabricEntityLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `holderByNameCodec` | `()Lcom/mojang/serialization/Codec;` | inherited_exact | invokeinterface@4 in `VariantCodecs.lambda$static$1` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `holderByNameCodec` | `()Lcom/mojang/serialization/Codec;` | inherited_exact | invokeinterface@4 in `VariantCodecs.lambda$static$0` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | inherited_exact | invokeinterface@3 in `DefaultItemComponentImpl$ModifyContextImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `keySet` | `()Ljava/util/Set;` | inherited_exact | invokeinterface@96 in `FabricBlockLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `keySet` | `()Ljava/util/Set;` | inherited_exact | invokeinterface@131 in `FabricEntityLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `stream` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@3 in `BlockInitTracker.postFreeze` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;
public abstract getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;
public abstract byId(I)Ljava/lang/Object;
public abstract getDefaultKey()Lnet/minecraft/resources/Identifier;
```
