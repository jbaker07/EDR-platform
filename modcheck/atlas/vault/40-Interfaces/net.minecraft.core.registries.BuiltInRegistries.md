---
type: "interface"
fqcn: "net.minecraft.core.registries.BuiltInRegistries"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.registries.BuiltInRegistries

System: [[20-Systems/net.minecraft.core.registries|net.minecraft.core.registries]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `bootStrap` | `()V` | exact | invokestatic@24 in `MainMixin.afterModInit` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `bootStrap` | `()V` | exact | invokestatic@10 in `MinecraftMixin.afterModInit` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `createContents` | `()V` | exact | invokestatic@0 in `BootstrapMixin.delayRegistryFreeze` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `createContents` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `freeze` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `freeze` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `ACTIVITY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@509 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `ATTRIBUTE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@404 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `BIOME_SOURCE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@273 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@70 in `BlockApiLookupImpl.registerForBlocks` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@66 in `FlammableBlockRegistryImpl.getEntryMap` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@4 in `FabricBlockLootSubProvider.excludeFromStrictValidation` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@93 in `FabricBlockLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@148 in `FabricBlockLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@0 in `BlockLootSubProviderMixin.lambda$onlyVanillaBlocks$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@9 in `RecipeProviderMixin.adjustIdWaxRecipes` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@7 in `RecipeProviderMixin.dontGenerateNonVanillaWaxingRecipes` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@18 in `RecipeProviderMixin.dontGenerateNonVanillaWaxingRecipes` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@14 in `ModelLoadingPluginContextImpl.registerBlockStateResolver` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@117 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@0 in `BlockInitTracker.postFreeze` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@0 in `BlocksMixin.initShapeCache` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@12 in `BootstrapMixin.afterInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `BLOCK_ENTITY_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@280 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `BLOCK_STATE_PROVIDER_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@223 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `CARVER_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@209 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `CHUNK_STATUS` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@318 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `COMMAND_ARGUMENT_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@11 in `ArgumentTypeRegistry.registerArgumentType` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `COMMAND_ARGUMENT_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@367 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `CONSUME_EFFECT_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@643 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `CONTEXT_KEY_SET` | `Lnet/minecraft/core/Registry;` | exact | getstatic@0 in `SimpleFabricLootTableSubProvider.getName` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `CREATIVE_MODE_TAB` | `Lnet/minecraft/core/Registry;` | exact | getstatic@30 in `FabricCreativeGuiComponents.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `CREATIVE_MODE_TAB` | `Lnet/minecraft/core/Registry;` | exact | getstatic@5 in `CreativeModeTabMixin.getStacks` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `CREATIVE_MODE_TAB` | `Lnet/minecraft/core/Registry;` | exact | getstatic@109 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `CREATIVE_MODE_TAB` | `Lnet/minecraft/core/Registry;` | exact | getstatic@300 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `CREATIVE_MODE_TAB` | `Lnet/minecraft/core/Registry;` | exact | getstatic@337 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `CREATIVE_MODE_TAB` | `Lnet/minecraft/core/Registry;` | exact | getstatic@0 in `FabricLanguageProvider$TranslationBuilder.addCreativeModeTab` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `CUSTOM_STAT` | `Lnet/minecraft/core/Registry;` | exact | getstatic@303 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `DATA_COMPONENT_PREDICATE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@605 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `DATA_COMPONENT_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@582 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `DEBUG_SUBSCRIPTION` | `Lnet/minecraft/core/Registry;` | exact | getstatic@718 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@62 in `EntityApiLookupImpl.registerForTypes` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@61 in `EntityApiLookupImpl.lambda$checkSelfImplementingTypes$0` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@20 in `BiomeModifications.addSpawn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@31 in `BiomeModifications.addSpawn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@4 in `FabricEntityLootSubProvider.excludeFromStrictValidation` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@128 in `FabricEntityLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@186 in `FabricEntityLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@18 in `FabricDefaultAttributeRegistry.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@32 in `MinecartComparatorLogicRegistry.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@140 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `FEATURE_SIZE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@251 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `FEATURE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@216 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `FLUID` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@71 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `FLUID` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@26 in `BootstrapMixin.afterInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `FLUID` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@57 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `FLUID` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@34 in `FluidVariantAttributeHandler.getName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `FLUID` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@1 in `VariantCodecs.lambda$static$1` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `FLUID` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@52 in `FluidVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `FOLIAGE_PLACER_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@230 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GAME_EVENT` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@537 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `GAME_RULE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@5 in `GameRuleBuilder.buildAndRegister` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@79 in `ItemApiLookupImpl.registerForItems` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@32 in `ModelProviderItemInfoCollectorMixin.filterItemsForProcessingMod` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@9 in `RecipeProviderMixin.adjustIdStonecutter` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@0 in `DefaultItemComponentImpl$ModifyContextImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@163 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@40 in `BootstrapMixin.afterInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@76 in `ArmorRendererRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@1 in `VariantCodecs.lambda$static$0` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `LOOT_CONDITION_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@530 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `LOOT_FUNCTION_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@523 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `LOOT_POOL_ENTRY_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@516 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `MAP_DECORATION_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@620 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `MEMORY_MODULE_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@495 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `MENU` | `Lnet/minecraft/core/Registry;` | exact | getstatic@21 in `Networking.sendOpenPacket` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| reads | `MENU` | `Lnet/minecraft/core/Registry;` | exact | getstatic@15 in `Networking.onInitialize` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| reads | `MENU` | `Lnet/minecraft/core/Registry;` | exact | getstatic@16 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| reads | `MENU` | `Lnet/minecraft/core/Registry;` | exact | getstatic@49 in `ServerPlayerMixin.fabric_storeOpenedMenu` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| reads | `MENU` | `Lnet/minecraft/core/Registry;` | exact | getstatic@92 in `ServerPlayerMixin.fabric_replaceVanillaScreenPacket` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| reads | `MENU` | `Lnet/minecraft/core/Registry;` | exact | getstatic@382 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `MENU` | `Lnet/minecraft/core/Registry;` | exact | getstatic@7 in `CursorSlotWrapper.toString` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `MOB_EFFECT` | `Lnet/minecraft/core/Registry;` | exact | getstatic@94 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `NUMBER_FORMAT_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@552 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `PARTICLE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@7 in `ParticleProviderRegistryImpl$DirectParticleProviderRegistry.register` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `PARTICLE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@25 in `ParticleProviderRegistryImpl$DirectParticleProviderRegistry.register` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `PARTICLE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@258 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `PARTICLE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@0 in `ParticleResourcesMixin.onInit` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `POINT_OF_INTEREST_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@0 in `PoiHelper.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `POINT_OF_INTEREST_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@488 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `POINT_OF_INTEREST_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@703 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `POSITION_SOURCE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@567 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `POS_RULE_TEST` | `Lnet/minecraft/core/Registry;` | exact | getstatic@346 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `POTION` | `Lnet/minecraft/core/Registry;` | exact | getstatic@186 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `RECIPE_BOOK_CATEGORY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@688 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `RECIPE_DISPLAY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@658 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `RECIPE_SERIALIZER` | `Lnet/minecraft/core/Registry;` | exact | getstatic@5 in `ClientboundRecipeSyncPayload$Entry.read` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `RECIPE_SERIALIZER` | `Lnet/minecraft/core/Registry;` | exact | getstatic@1 in `ClientboundRecipeSyncPayload$Entry.write` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `RECIPE_SERIALIZER` | `Lnet/minecraft/core/Registry;` | exact | getstatic@38 in `RecipeSyncImpl.onRecipeSyncRequest` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `RECIPE_SERIALIZER` | `Lnet/minecraft/core/Registry;` | exact | getstatic@51 in `ClientConfigurationPacketListenerImplMixin.sendSupportedRecipeSerializers | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `RECIPE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@397 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@28 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@160 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@48 in `ClientRegistrySyncHandler.apply` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@69 in `ClientRegistrySyncHandler.checkRemoteRemap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@11 in `RegistrySyncManager.areAllRegistriesOptional` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@8 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@41 in `RegistrySyncManager.createAndPopulateRegistryMap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@0 in `MinecraftMixin.unmap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@33 in `MinecraftMixin.unmap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `REGISTRY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@60 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| reads | `RULE_TEST` | `Lnet/minecraft/core/Registry;` | exact | getstatic@339 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `SENSOR_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@502 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `SLOT_DISPLAY` | `Lnet/minecraft/core/Registry;` | exact | getstatic@673 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `SOUND_EVENT` | `Lnet/minecraft/core/Registry;` | exact | getstatic@61 in `SoundTypeBuilderImpl.build` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `SOUND_EVENT` | `Lnet/minecraft/core/Registry;` | exact | getstatic@56 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `STAT_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@1 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `STAT_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@427 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `STRUCTURE_PIECE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@332 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `STRUCTURE_POOL_ELEMENT` | `Lnet/minecraft/core/Registry;` | exact | getstatic@360 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `STRUCTURE_PROCESSOR` | `Lnet/minecraft/core/Registry;` | exact | getstatic@353 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `STRUCTURE_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@325 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `TEST_FUNCTION` | `Lnet/minecraft/core/Registry;` | exact | getstatic@63 in `FabricGameTestModInitializer.onInitialize` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `TREE_DECORATOR_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@244 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `TRUNK_PLACER_TYPE` | `Lnet/minecraft/core/Registry;` | exact | getstatic@237 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `VILLAGER_PROFESSION` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@465 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `VILLAGER_TYPE` | `Lnet/minecraft/core/DefaultedRegistry;` | exact | getstatic@442 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (100 fields, 49 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final LOADERS : Ljava/util/Map;
private static final WRITABLE_REGISTRY : Lnet/minecraft/core/WritableRegistry;
public static final DATA_COMPONENT_INITIALIZERS : Lnet/minecraft/core/component/DataComponentInitializers;
public static final GAME_EVENT : Lnet/minecraft/core/DefaultedRegistry;
public static final SOUND_EVENT : Lnet/minecraft/core/Registry;
public static final FLUID : Lnet/minecraft/core/DefaultedRegistry;
public static final MOB_EFFECT : Lnet/minecraft/core/Registry;
public static final BLOCK : Lnet/minecraft/core/DefaultedRegistry;
public static final DEBUG_SUBSCRIPTION : Lnet/minecraft/core/Registry;
public static final ENTITY_TYPE : Lnet/minecraft/core/DefaultedRegistry;
public static final ITEM : Lnet/minecraft/core/DefaultedRegistry;
public static final POTION : Lnet/minecraft/core/Registry;
public static final PARTICLE_TYPE : Lnet/minecraft/core/Registry;
public static final BLOCK_ENTITY_TYPE : Lnet/minecraft/core/Registry;
public static final CUSTOM_STAT : Lnet/minecraft/core/Registry;
public static final CHUNK_STATUS : Lnet/minecraft/core/DefaultedRegistry;
public static final RULE_TEST : Lnet/minecraft/core/Registry;
public static final RULE_BLOCK_ENTITY_MODIFIER : Lnet/minecraft/core/Registry;
public static final POS_RULE_TEST : Lnet/minecraft/core/Registry;
public static final MENU : Lnet/minecraft/core/Registry;
public static final RECIPE_TYPE : Lnet/minecraft/core/Registry;
public static final RECIPE_SERIALIZER : Lnet/minecraft/core/Registry;
public static final ATTRIBUTE : Lnet/minecraft/core/Registry;
public static final POSITION_SOURCE_TYPE : Lnet/minecraft/core/Registry;
public static final COMMAND_ARGUMENT_TYPE : Lnet/minecraft/core/Registry;
public static final STAT_TYPE : Lnet/minecraft/core/Registry;
public static final VILLAGER_TYPE : Lnet/minecraft/core/DefaultedRegistry;
public static final VILLAGER_PROFESSION : Lnet/minecraft/core/DefaultedRegistry;
public static final POINT_OF_INTEREST_TYPE : Lnet/minecraft/core/Registry;
public static final MEMORY_MODULE_TYPE : Lnet/minecraft/core/DefaultedRegistry;
public static final SENSOR_TYPE : Lnet/minecraft/core/DefaultedRegistry;
public static final ACTIVITY : Lnet/minecraft/core/Registry;
public static final LOOT_POOL_ENTRY_TYPE : Lnet/minecraft/core/Registry;
public static final LOOT_FUNCTION_TYPE : Lnet/minecraft/core/Registry;
public static final LOOT_CONDITION_TYPE : Lnet/minecraft/core/Registry;
public static final CONTEXT_FLOAT_PROVIDER_TYPE : Lnet/minecraft/core/Registry;
public static final CONTEXT_INT_PROVIDER_TYPE : Lnet/minecraft/core/Registry;
public static final LOOT_NBT_PROVIDER_TYPE : Lnet/minecraft/core/Registry;
public static final LOOT_SCORE_PROVIDER_TYPE : Lnet/minecraft/core/Registry;
public static final FLOAT_PROVIDER_TYPE : Lnet/minecraft/core/Registry;
public static final INT_PROVIDER_TYPE : Lnet/minecraft/core/Registry;
public static final HEIGHT_PROVIDER_TYPE : Lnet/minecraft/core/Registry;
public static final BLOCK_PREDICATE_TYPE : Lnet/minecraft/core/Registry;
public static final CARVER_TYPE : Lnet/minecraft/core/Registry;
public static final FEATURE_TYPE : Lnet/minecraft/core/Registry;
public static final STRUCTURE_PLACEMENT : Lnet/minecraft/core/Registry;
public static final STRUCTURE_PIECE : Lnet/minecraft/core/Registry;
public static final STRUCTURE_TYPE : Lnet/minecraft/core/Registry;
public static final PLACEMENT_MODIFIER_TYPE : Lnet/minecraft/core/Registry;
public static final BLOCK_STATE_PROVIDER_TYPE : Lnet/minecraft/core/Registry;
public static final FOLIAGE_PLACER_TYPE : Lnet/minecraft/core/Registry;
public static final TRUNK_PLACER_TYPE : Lnet/minecraft/core/Registry;
public static final ROOT_PLACER_TYPE : Lnet/minecraft/core/Registry;
public static final TREE_DECORATOR_TYPE : Lnet/minecraft/core/Registry;
public static final FEATURE_SIZE_TYPE : Lnet/minecraft/core/Registry;
public static final BIOME_SOURCE : Lnet/minecraft/core/Registry;
public static final CHUNK_GENERATOR : Lnet/minecraft/core/Registry;
public static final MATERIAL_CONDITION_TYPE : Lnet/minecraft/core/Registry;
public static final MATERIAL_RULE_TYPE : Lnet/minecraft/core/Registry;
public static final DENSITY_FUNCTION_TYPE : Lnet/minecraft/core/Registry;
public static final STRUCTURE_PROCESSOR : Lnet/minecraft/core/Registry;
public static final STRUCTURE_POOL_ELEMENT : Lnet/minecraft/core/Registry;
public static final POOL_ALIAS_BINDING_TYPE : Lnet/minecraft/core/Registry;
public static final CREATIVE_MODE_TAB : Lnet/minecraft/core/Registry;
public static final TRIGGER_TYPES : Lnet/minecraft/core/Registry;
public static final NUMBER_FORMAT_TYPE : Lnet/minecraft/core/Registry;
public static final DATA_COMPONENT_TYPE : Lnet/minecraft/core/Registry;
public static final GAME_RULE : Lnet/minecraft/core/Registry;
public static final ENTITY_SUB_PREDICATE_TYPE : Lnet/minecraft/core/Registry;
public static final DATA_COMPONENT_PREDICATE_TYPE : Lnet/minecraft/core/Registry;
public static final MAP_DECORATION_TYPE : Lnet/minecraft/core/Registry;
public static final ENCHANTMENT_EFFECT_COMPONENT_TYPE : Lnet/minecraft/core/Registry;
public static final ENCHANTMENT_LEVEL_BASED_VALUE_TYPE : Lnet/minecraft/core/Registry;
public static final ENCHANTMENT_ENTITY_EFFECT_TYPE : Lnet/minecraft/core/Registry;
public static final ENCHANTMENT_LOCATION_BASED_EFFECT_TYPE : Lnet/minecraft/core/Registry;
public static final ENCHANTMENT_VALUE_EFFECT_TYPE : Lnet/minecraft/core/Registry;
public static final ENCHANTMENT_PROVIDER_TYPE : Lnet/minecraft/core/Registry;
public static final CONSUME_EFFECT_TYPE : Lnet/minecraft/core/Registry;
public static final RECIPE_DISPLAY : Lnet/minecraft/core/Registry;
public static final SLOT_DISPLAY : Lnet/minecraft/core/Registry;
public static final RECIPE_BOOK_CATEGORY : Lnet/minecraft/core/Registry;
public static final TICKET_TYPE : Lnet/minecraft/core/Registry;
public static final INCOMING_RPC_METHOD : Lnet/minecraft/core/Registry;
public static final OUTGOING_RPC_METHOD : Lnet/minecraft/core/Registry;
public static final TEST_ENVIRONMENT_DEFINITION_TYPE : Lnet/minecraft/core/Registry;
public static final TEST_INSTANCE_TYPE : Lnet/minecraft/core/Registry;
public static final SPAWN_CONDITION_TYPE : Lnet/minecraft/core/Registry;
public static final DIALOG_TYPE : Lnet/minecraft/core/Registry;
public static final DIALOG_ACTION_TYPE : Lnet/minecraft/core/Registry;
public static final INPUT_CONTROL_TYPE : Lnet/minecraft/core/Registry;
public static final DIALOG_BODY_TYPE : Lnet/minecraft/core/Registry;
public static final PERMISSION_TYPE : Lnet/minecraft/core/Registry;
public static final PERMISSION_CHECK_TYPE : Lnet/minecraft/core/Registry;
public static final ENVIRONMENT_ATTRIBUTE : Lnet/minecraft/core/Registry;
public static final ATTRIBUTE_TYPE : Lnet/minecraft/core/Registry;
public static final SLOT_SOURCE_TYPE : Lnet/minecraft/core/Registry;
public static final CONTEXT_KEY_SET : Lnet/minecraft/core/Registry;
public static final TEST_FUNCTION : Lnet/minecraft/core/Registry;
public static final REGISTRY : Lnet/minecraft/core/Registry;
public <init>()V
private static registerSimple(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/registries/BuiltInRegistries$RegistryBootstrap;)Lnet/minecraft/core/Registry;
private static registerSimpleWithIntrusiveHolders(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/registries/BuiltInRegistries$RegistryBootstrap;)Lnet/minecraft/core/Registry;
private static registerDefaulted(Lnet/minecraft/resources/ResourceKey;Ljava/lang/String;Lnet/minecraft/core/registries/BuiltInRegistries$RegistryBootstrap;)Lnet/minecraft/core/DefaultedRegistry;
private static registerDefaultedWithIntrusiveHolders(Lnet/minecraft/resources/ResourceKey;Ljava/lang/String;Lnet/minecraft/core/registries/BuiltInRegistries$RegistryBootstrap;)Lnet/minecraft/core/DefaultedRegistry;
private static internalRegister(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/WritableRegistry;Lnet/minecraft/core/registries/BuiltInRegistries$RegistryBootstrap;)Lnet/minecraft/core/WritableRegistry;
public static bootStrap()V
private static createContents()V
private static freeze()V
private static validate(Lnet/minecraft/core/Registry;)V
public static acquireBootstrapRegistrationLookup(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/HolderGetter;
private static bindBootstrappedTagsToEmpty(Lnet/minecraft/core/Registry;)V
private static synthetic lambda$validate$0(Lnet/minecraft/core/Registry;Lnet/minecraft/core/Registry;)V
private static synthetic lambda$createContents$0(Lnet/minecraft/resources/Identifier;Ljava/util/function/Supplier;)V
private static synthetic lambda$internalRegister$1(Lnet/minecraft/core/registries/BuiltInRegistries$RegistryBootstrap;Lnet/minecraft/core/WritableRegistry;)Ljava/lang/Object;
private static synthetic lambda$internalRegister$0(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
private static synthetic lambda$static$31(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$30(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$29(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$28(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$27(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$26(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$25(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$24(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$23(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$22(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$21(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$20(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$19(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$18(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$17(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$16(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$15(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$14(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$13(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$12(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$11(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$10(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$9(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$8(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$7(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$6(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$5(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$4(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$3(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$2(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$1(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
private static synthetic lambda$static$0(Lnet/minecraft/core/Registry;)Ljava/lang/Object;
static <clinit>()V
```
