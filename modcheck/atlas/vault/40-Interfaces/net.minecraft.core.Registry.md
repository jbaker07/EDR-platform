---
type: "interface"
fqcn: "net.minecraft.core.Registry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Registry

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/IdMap`, `com/mojang/serialization/Keyable`, `net/minecraft/core/HolderLookup$RegistryLookup`, `net/fabricmc/fabric/api/event/registry/FabricRegistry`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `byId` | `(I)Ljava/lang/Object;` | inherited_exact | invokeinterface@624 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `byId` | `(I)Ljava/lang/Object;` | inherited_exact | invokeinterface@651 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `byId` | `(I)Ljava/lang/Object;` | inherited_exact | invokeinterface@69 in `RemapStateImpl.<init>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `containsKey` | `(Lnet/minecraft/resources/Identifier;)Z` | exact | invokeinterface@148 in `ClientRegistrySyncHandler.checkRemoteRemap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `containsKey` | `(Lnet/minecraft/resources/Identifier;)Z` | exact | invokeinterface@132 in `RegistryCustomContentState.validate` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `entrySet` | `()Ljava/util/Set;` | exact | invokeinterface@28 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `entrySet` | `()Ljava/util/Set;` | exact | invokeinterface@22 in `DimensionModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `forEach` | `(Ljava/util/function/Consumer;)V` | inherited_exact | invokeinterface@36 in `StateIdTracker.recalcStateMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | exact | invokeinterface@3 in `RegistryEntryAddedCallback.lambda$allEntries$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | inherited_exact | invokeinterface@2 in `BiomeModificationContextImpl.getHolder` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | inherited_exact | invokeinterface@40 in `ClientTagsImpl.isInWithLocalFallback` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@86 in `TagAliasLoader.applyToDynamicRegistries` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getId` | `(Ljava/lang/Object;)I` | exact | invokeinterface@11 in `BiomeModificationImpl.lambda$finalizeWorldGen$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getId` | `(Ljava/lang/Object;)I` | exact | invokeinterface@11 in `DimensionModificationImpl.lambda$finalizeWorldGen$0` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `getId` | `(Ljava/lang/Object;)I` | exact | invokeinterface@11 in `ParticleProviderRegistryImpl$DirectParticleProviderRegistry.registe | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getId` | `(Ljava/lang/Object;)I` | exact | invokeinterface@255 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getId` | `(Ljava/lang/Object;)I` | exact | invokeinterface@557 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getId` | `(Ljava/lang/Object;)I` | exact | invokeinterface@41 in `StateIdTracker.recalcHighestId` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getId` | `(Ljava/lang/Object;)I` | exact | invokeinterface@5 in `StateIdTracker.lambda$recalcStateMap$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@5 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@7 in `SimpleFabricLootTableSubProvider.getName` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@28 in `Networking.sendOpenPacket` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@27 in `Networking.forEachEntry` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@56 in `ServerPlayerMixin.fabric_storeOpenedMenu` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@100 in `ServerPlayerMixin.fabric_replaceVanillaScreenPacket` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@53 in `FabricEntityDataRegistryImpl.storeExternalHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@19 in `FabricEntityDataRegistryImpl.getId` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@29 in `ParticleProviderRegistryImpl$DirectParticleProviderRegistry.registe | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@8 in `ClientboundRecipeSyncPayload$Entry.write` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@56 in `ClientConfigurationPacketListenerImplMixin.sendSupportedRecipeSeria | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@237 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@539 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@74 in `RemapStateImpl.<init>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getKey` | `(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;` | exact | invokeinterface@17 in `CursorSlotWrapper.toString` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOptional` | `(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | exact | invokeinterface@38 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `getOptional` | `(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | exact | invokeinterface@68 in `SoundTypeBuilderImpl.build` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOptional` | `(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | exact | invokeinterface@43 in `RecipeSyncImpl.onRecipeSyncRequest` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getOptional` | `(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | exact | invokeinterface@70 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | inherited_exact | invokeinterface@33 in `BiomeSelectionContextImpl.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | inherited_exact | invokeinterface@18 in `BiomeSelectionContextImpl.hasTag` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | inherited_exact | invokeinterface@18 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | inherited_exact | invokeinterface@107 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | inherited_exact | invokeinterface@95 in `DimensionModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | inherited_exact | invokeinterface@19 in `TestAnnotationLocator$TestMethod.testData` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `getResourceKey` | `(Ljava/lang/Object;)Ljava/util/Optional;` | exact | invokeinterface@15 in `BiomeSelectionContextImpl.getFeatureKey` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getResourceKey` | `(Ljava/lang/Object;)Ljava/util/Optional;` | exact | invokeinterface@15 in `BiomeSelectionContextImpl.getPlacedFeatureKey` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getResourceKey` | `(Ljava/lang/Object;)Ljava/util/Optional;` | exact | invokeinterface@15 in `BiomeSelectionContextImpl.getStructureKey` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getResourceKey` | `(Ljava/lang/Object;)Ljava/util/Optional;` | exact | invokeinterface@82 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `getResourceKey` | `(Ljava/lang/Object;)Ljava/util/Optional;` | exact | invokeinterface@9 in `CreativeModeTabMixin.getStacks` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getResourceKey` | `(Ljava/lang/Object;)Ljava/util/Optional;` | exact | invokeinterface@45 in `ClientTagsImpl.getHolder` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getTags` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@26 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWar | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@20 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@19 in `FabricEntityDataRegistryImpl.get` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@9 in `ClientboundRecipeSyncPayload$Entry.read` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@52 in `ClientRegistrySyncHandler.apply` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@74 in `ClientRegistrySyncHandler.checkRemoteRemap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@45 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@573 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@603 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | invokeinterface@37 in `MinecraftMixin.unmap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | exact | invokeinterface@13 in `BiomeSelectionContextImpl.validForStructure` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getValue` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | exact | invokeinterface@13 in `BiomeSelectionContextImpl.canGenerateIn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getValueOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | exact | invokeinterface@120 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getValueOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | exact | invokeinterface@3 in `BiomeModificationImpl.lambda$finalizeWorldGen$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getValueOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | exact | invokeinterface@342 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `getValueOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | exact | invokeinterface@4 in `FabricLanguageProvider$TranslationBuilder.addCreativeModeTab` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getValueOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | exact | invokeinterface@3 in `DimensionModificationImpl.lambda$finalizeWorldGen$0` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | inherited_exact | invokeinterface@1 in `Networking.forEachEntry` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | inherited_exact | invokeinterface@71 in `FabricEntityDataRegistryImpl.reorderHandlers` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | inherited_exact | invokeinterface@187 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | inherited_exact | invokeinterface@510 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | inherited_exact | invokeinterface@9 in `StateIdTracker.recalcHighestId` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@59 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@1 in `RegistryAttributeHolder.get` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@5 in `DynamicRegistryViewImpl$1.entry` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@12 in `ListenableRegistry.get` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@386 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@4 in `StateIdTracker.register` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@25 in `StateIdTracker.register` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@5 in `RegistrySynchronizationMixin.filterNonSyncedEntriesAgain` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@20 in `ClientTagsImpl.getHolder` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `keySet` | `()Ljava/util/Set;` | exact | invokeinterface@11 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `keySet` | `()Ljava/util/Set;` | exact | invokeinterface@3 in `MinecraftMixin.unmap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `listElements` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@112 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `listElements` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@27 in `ReloadableServerRegistriesMixin.lambda$modifyLootTables$0` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `listElements` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@15 in `RegistryEntryAddedCallback.allEntries` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `register` | `(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/Identifier;Ljav` | exact | invokestatic@16 in `ArgumentTypeRegistry.registerArgumentType` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `register` | `(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/Identifier;Ljav` | exact | invokestatic@10 in `GameRuleBuilder.buildAndRegister` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `register` | `(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/Identifier;Ljav` | exact | invokestatic@74 in `FabricGameTestModInitializer.onInitialize` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `register` | `(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/Identifier;Ljav` | exact | invokestatic@146 in `FabricGameTestModInitializer.registerDynamicEntries` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `register` | `(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/Identifier;Ljav` | exact | invokestatic@94 in `FabricEntityDataRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `registrationInfo` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@35 in `WorldDimensionsMixin.betterModdedStabilityCheck` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `registryKeySet` | `()Ljava/util/Set;` | exact | invokeinterface@303 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `size` | `()I` | inherited_exact | invokeinterface@21 in `RegistrySynchronizationMixin.filterNonSyncedEntries` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `size` | `()I` | inherited_exact | invokeinterface@24 in `RegistrySynchronizationMixin.filterNonSyncedEntriesAgain` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `stream` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@275 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (0 fields, 47 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract key()Lnet/minecraft/resources/ResourceKey;
public byNameCodec()Lcom/mojang/serialization/Codec;
public holderByNameCodec()Lcom/mojang/serialization/Codec;
private referenceHolderWithLifecycle()Lcom/mojang/serialization/Codec;
private safeCastToReference(Lnet/minecraft/core/Holder;)Lcom/mojang/serialization/DataResult;
public keys(Lcom/mojang/serialization/DynamicOps;)Ljava/util/stream/Stream;
public abstract getKey(Ljava/lang/Object;)Lnet/minecraft/resources/Identifier;
public abstract getResourceKey(Ljava/lang/Object;)Ljava/util/Optional;
public abstract getId(Ljava/lang/Object;)I
public abstract getValue(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;
public abstract getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;
public abstract registrationInfo(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public getOptional(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;
public getOptional(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public abstract getAny()Ljava/util/Optional;
public getValueOrThrow(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;
public abstract keySet()Ljava/util/Set;
public abstract entrySet()Ljava/util/Set;
public abstract registryKeySet()Ljava/util/Set;
public abstract getRandom(Lnet/minecraft/util/RandomSource;)Ljava/util/Optional;
public stream()Ljava/util/stream/Stream;
public abstract containsKey(Lnet/minecraft/resources/Identifier;)Z
public abstract containsKey(Lnet/minecraft/resources/ResourceKey;)Z
public static register(Lnet/minecraft/core/Registry;Ljava/lang/String;Ljava/lang/Object;)Ljava/lang/Object;
public static register(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/Identifier;Ljava/lang/Object;)Ljava/lang/Object;
public static register(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/ResourceKey;Ljava/lang/Object;)Ljava/lang/Object;
public static registerForHolder(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/ResourceKey;Ljava/lang/Object;)Lnet/minecraft/core/Holder$Reference;
public static registerForHolder(Lnet/minecraft/core/Registry;Lnet/minecraft/resources/Identifier;Ljava/lang/Object;)Lnet/minecraft/core/Holder$Reference;
public abstract freeze()Lnet/minecraft/core/Registry;
public abstract createIntrusiveHolder(Ljava/lang/Object;)Lnet/minecraft/core/Holder$Reference;
public abstract get(I)Ljava/util/Optional;
public abstract get(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;
public abstract wrapAsHolder(Ljava/lang/Object;)Lnet/minecraft/core/Holder;
public getTagOrEmpty(Lnet/minecraft/tags/TagKey;)Ljava/lang/Iterable;
public abstract getTags()Ljava/util/stream/Stream;
public asHolderIdMap()Lnet/minecraft/core/IdMap;
public abstract prepareTagReload(Lnet/minecraft/tags/TagLoader$LoadResult;)Lnet/minecraft/core/Registry$PendingTags;
public abstract componentLookup()Lnet/minecraft/core/component/DataComponentLookup;
private static synthetic lambda$keys$0(Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;
private synthetic lambda$safeCastToReference$0(Lnet/minecraft/core/Holder;)Ljava/lang/String;
private synthetic lambda$referenceHolderWithLifecycle$4(Lnet/minecraft/core/Holder$Reference;)Lcom/mojang/serialization/Lifecycle;
private static synthetic lambda$referenceHolderWithLifecycle$3(Lnet/minecraft/core/Holder$Reference;)Lnet/minecraft/resources/Identifier;
private synthetic lambda$referenceHolderWithLifecycle$0(Lnet/minecraft/resources/Identifier;)Lcom/mojang/serialization/DataResult;
private synthetic lambda$referenceHolderWithLifecycle$1(Lnet/minecraft/resources/Identifier;)Lcom/mojang/serialization/DataResult;
private synthetic lambda$referenceHolderWithLifecycle$2(Lnet/minecraft/resources/Identifier;)Ljava/lang/String;
private static synthetic lambda$holderByNameCodec$0(Lnet/minecraft/core/Holder$Reference;)Lnet/minecraft/core/Holder;
private synthetic lambda$byNameCodec$0(Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;
```
