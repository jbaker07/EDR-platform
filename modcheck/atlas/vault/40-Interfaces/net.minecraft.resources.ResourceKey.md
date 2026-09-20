---
type: "interface"
fqcn: "net.minecraft.resources.ResourceKey"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.ResourceKey

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `codec` | `(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/Codec` | exact | invokestatic@8 in `WorldDimensionsMixin.useFailSoftMap` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@14 in `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@14 in `SmithingTransformRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@14 in `SmithingTrimRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@14 in `SpecialRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@16 in `TestAnnotationLocator$TestMethod.testData` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@45 in `TestAnnotationLocator$TestMethod.testData` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@11 in `TestAnnotationLocator$TestMethod.testInstance` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@7 in `PoiHelper.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@175 in `MappedRegistryMixin.unmap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@36 in `MappedRegistryMixin.aliasResourceKeyParameter` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `create` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identif` | exact | invokestatic@84 in `ResourceConditionsImpl.registryContains` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `createRegistryKey` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Resourc` | exact | invokestatic@21 in `FabricEntityDataRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `createRegistryKey` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Resourc` | exact | invokestatic@1 in `FabricRegistryBuilder.create` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `createRegistryKey` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Resourc` | exact | invokestatic@1 in `FabricRegistryBuilder.createDefaulted` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `createRegistryKey` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Resourc` | exact | invokestatic@1 in `RegistrySyncPayload.lambda$getRegistryAttributeMap$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `createRegistryKey` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Resourc` | exact | invokestatic@65 in `RegistryCustomContentState.validate` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `createRegistryKey` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Resourc` | exact | invokestatic@18 in `ResourceConditionsImpl.tagsPopulated` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `createRegistryKey` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Resourc` | exact | invokestatic@18 in `ResourceConditionsImpl.registryContains` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `ResourceManagerRegistryLoadTaskMixin.lambda$modifyAdvancement$0` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `BiomeModifications.addFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `BiomeModifications.addCarver` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@6 in `BiomeSelectors.lambda$vanilla$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@205 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@15 in `VibrationFrequencyRegistry.register` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@61 in `VibrationFrequencyRegistry.register` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@44 in `VillagerInteractionRegistries.registerGiftLootTable` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@24 in `FabricTagKey.getTranslationKey` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@35 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@35 in `CreativeModeTabsMixin.lambda$paginateTabs$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@42 in `CreativeModeTabsMixin.lambda$paginateTabs$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `AttachmentSavedData.lambda$codec$1` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@106 in `AttachmentChange.tryApply` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@18 in `FabricSoundsProvider$SoundExporter.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@182 in `FabricBlockLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `FabricDynamicRegistryProvider$Entries.lambda$addAll$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `FabricDynamicRegistryProvider$Entries.lambda$new$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@8 in `FabricDynamicRegistryProvider.writeHolders` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@49 in `FabricDynamicRegistryProvider.writeHolders` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@60 in `FabricDynamicRegistryProvider.writeHolders` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@68 in `FabricDynamicRegistryProvider.writeHolders` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@154 in `FabricDynamicRegistryProvider.writeHolders` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@2 in `FabricEntityLootSubProvider.lambda$generate$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `FabricLanguageProvider$TranslationBuilder.addEnchantment` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@50 in `FabricRecipeProvider.lambda$run$2` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@50 in `FabricRecipeProvider.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `TagAliasGenerator.getDirectory` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@27 in `SoundTypeBuilderImpl$RegistrationBuilderImpl.ofEvent` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@9 in `FabricLootTableProviderImpl.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@20 in `FabricLootTableProviderImpl.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@43 in `FabricLootTableProviderImpl.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@24 in `BlockItemTagAppenderMixin.remove` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@2 in `BlockItemTagAppenderMixin.lambda$removeAll$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@2 in `BlockItemTagAppenderMixin.lambda$removeAll$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@2 in `BlockItemTagAppenderMixin.lambda$remove$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@8 in `TagAppenderMixin$TagAppender1Mixin.remove` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@8 in `TagAppenderMixin$TagAppender1Mixin.lambda$removeAll$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@8 in `TagAppenderMixin$TagAppender1Mixin.lambda$removeAll$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `VanillaAdventureAdvancementsMixin.lambda$onlyCheckVanillaEntities$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `ModelProviderBlockStateGeneratorCollectorMixin.lambda$filterBlocksForP | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `ModelProviderItemInfoCollectorMixin.lambda$filterItemsForProcessingMod | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `EntityLootSubProviderMixin.lambda$onlyVanillaEntities$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@6 in `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@6 in `SmithingTransformRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@6 in `SmithingTrimRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@6 in `SpecialRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `WorldDimensionsMixin.betterModdedStabilityCheck` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@132 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@121 in `BlockTransformerHelperImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@129 in `EnchantmentUtil.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@3 in `DataComponentInitializersPendingComponentsMixin.apply` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@10 in `DataComponentInitializersPendingComponentsMixin.apply` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@2 in `EntityTypeBuilderMixin.allowNoModdedDatafixers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `RecipeSyncImplClient.lambda$onRecipeSyncPacket$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@17 in `ListenableRegistry.get` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@9 in `StateIdTracker.register` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@30 in `StateIdTracker.register` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@64 in `RegistryCustomContentState.lambda$construct$1` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@10 in `MappedRegistryMixin.onChange` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@48 in `MappedRegistryMixin.onChange` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@59 in `MappedRegistryMixin.onChange` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@33 in `MappedRegistryMixin.set` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@652 in `MappedRegistryMixin.remap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@738 in `MappedRegistryMixin.remap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@758 in `MappedRegistryMixin.remap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@11 in `MappedRegistryMixin.aliasResourceKeyParameter` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `RegistriesMixin.prependDirectoryWithNamespace` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@1 in `RegistriesMixin.prependTagDirectoryWithNamespace` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@5 in `ResourceConditions.tagsPopulated` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@5 in `ResourceConditions.registryContains` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@14 in `RegistryContainsResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@7 in `TagsPopulatedResourceCondition.<init>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@14 in `TagsPopulatedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@21 in `RegistryLoadTaskPendingRegistrationMixin.loadFromResource` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@14 in `ClientTags.isInLocal` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@34 in `ClientTags.isInLocal` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `TagAliasLoader.getDirectory` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@327 in `TagAliasLoader.apply` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@91 in `ClientTagsImpl.isInWithLocalFallback` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@67 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@145 in `MappedRegistryMixin.fabric_applyPendingTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `identifier` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@306 in `MappedRegistryMixin.fabric_applyPendingTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `isFor` | `(Lnet/minecraft/resources/ResourceKey;)Z` | exact | invokevirtual@37 in `ResourceManagerRegistryLoadTaskMixin.modifyAdvancement` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `isFor` | `(Lnet/minecraft/resources/ResourceKey;)Z` | exact | invokevirtual@37 in `ResourceManagerRegistryLoadTaskMixin.modifyLootTable` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@5 in `FabricDynamicRegistryProvider$Entries.getQueuedEntries` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@26 in `FabricDynamicRegistryProvider$Entries.getQueuedEntries` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `RegistryContainsResourceCondition.<init>` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@14 in `RegistryLoadTaskPendingRegistrationMixin.loadFromResource` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `registry` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@18 in `ClientTags.isInLocal` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registryKey` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@5 in `AdvancementHolderProvider.get` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `registryKey` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@5 in `AdvancementHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `registryKey` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `registryKey` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `SmithingTransformRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `registryKey` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `SmithingTrimRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `registryKey` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `SpecialRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `registryKey` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@5 in `LootTableHolderProvider.get` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registryKey` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@5 in `LootTableHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registryKey` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@32 in `MappedRegistryMixin.aliasResourceKeyParameter` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (4 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final REGISTRY_STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private static final VALUES : Ljava/util/concurrent/ConcurrentMap;
private final registryName : Lnet/minecraft/resources/Identifier;
private final identifier : Lnet/minecraft/resources/Identifier;
public static codec(Lnet/minecraft/resources/ResourceKey;)Lcom/mojang/serialization/Codec;
public static streamCodec(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/network/codec/StreamCodec;
public static create(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/ResourceKey;
public static createRegistryKey(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/ResourceKey;
private static create(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/ResourceKey;
private <init>(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;)V
public toString()Ljava/lang/String;
public isFor(Lnet/minecraft/resources/ResourceKey;)Z
public cast(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public dependent(Lnet/minecraft/resources/ResourceKey;Ljava/lang/String;)Lnet/minecraft/resources/ResourceKey;
public dependent(Lnet/minecraft/resources/ResourceKey;Ljava/util/function/UnaryOperator;)Lnet/minecraft/resources/ResourceKey;
public identifier()Lnet/minecraft/resources/Identifier;
public registry()Lnet/minecraft/resources/Identifier;
public registryKey()Lnet/minecraft/resources/ResourceKey;
private static synthetic lambda$create$0(Lnet/minecraft/resources/ResourceKey$InternKey;)Lnet/minecraft/resources/ResourceKey;
private static synthetic lambda$streamCodec$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/ResourceKey;
private static synthetic lambda$codec$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/ResourceKey;
static <clinit>()V
```
