---
type: "interface"
fqcn: "net.minecraft.core.registries.Registries"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.registries.Registries

System: [[20-Systems/net.minecraft.core.registries|net.minecraft.core.registries]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `tagsDirPath` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;` | exact | invokestatic@1 in `ClientTagsLoader.getTagFiles` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `elementsDirPath` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `tagsDirPath` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `ADVANCEMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@11 in `ReloadableServerRegistriesMixin.lambda$modifyAdvancements$0` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| reads | `ADVANCEMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@34 in `ResourceManagerRegistryLoadTaskMixin.modifyAdvancement` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| reads | `ADVANCEMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@11 in `FabricAdvancementProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `ADVANCEMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@22 in `FabricRecipeProvider$2.lookup` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `ADVANCEMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@29 in `FabricRecipeProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `ADVANCEMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@90 in `FabricRecipeProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BIOME` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@16 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `BIOME` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@21 in `BiomeSelectionContextImpl.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `BIOME` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `BiomeSelectionContextImpl.hasTag` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `BIOME` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@3 in `BuiltInResourceKeys.biomeHolderGetter` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `BIOME` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `TheEndBiomeSourceMixin.lambda$modifyCodec$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `BIOME` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@30 in `TagRegistration.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `BIOME` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@733 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@0 in `BlockFunctionalityTags.create` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@17 in `TagRegistration.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `BLOCK` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@2 in `FabricTagsProvider$BlockTagsProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `BLOCK_ENTITY_TYPE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@2 in `FabricTagsProvider$BlockEntityTypeTagsProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `CARVER` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@22 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `CARVER` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `FabricDynamicRegistryProvider$Entries.configuredCarvers` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `DIMENSION` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@30 in `TestAnnotationLocator$TestMethod.testData` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `DIMENSION_TYPE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@12 in `DimensionModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| reads | `ENCHANTMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@82 in `TagRegistration.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `ENCHANTMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@748 in `FabricRegistryInit.onInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@69 in `TagRegistration.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `ENTITY_TYPE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@2 in `FabricTagsProvider$EntityTypeTagsProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `FEATURE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `BiomeSelectionContextImpl.getFeatureKey` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `FLUID` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@56 in `TagRegistration.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `FLUID` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@2 in `FabricTagsProvider$FluidTagsProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `FLUID` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@68 in `VariantCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `TagRegistration.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@8 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarning$0` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@2 in `FabricTagsProvider$ItemTagsProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@0 in `DataComponentInitializersPendingComponentsMixin.apply` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@11 in `RegistryContainsResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@11 in `TagsPopulatedResourceCondition.lambda$static$0` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| reads | `ITEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@27 in `VariantCodecs.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `LEVEL_STEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@267 in `BiomeModificationImpl.finalizeWorldGen` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `LEVEL_STEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `BiomeSelectionContextImpl.canGenerateIn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `LEVEL_STEM` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@5 in `WorldDimensionsMixin.useFailSoftMap` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| reads | `LOOT_TABLE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `FabricLootTableContext.accept` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `LOOT_TABLE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `FabricLootTableProviderImpl.getOutputPath` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `LOOT_TABLE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@12 in `LootUtil.getEntryOrDirect` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| reads | `LOOT_TABLE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `ReloadableServerRegistriesMixin.lambda$modifyLootTables$0` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| reads | `LOOT_TABLE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@34 in `ResourceManagerRegistryLoadTaskMixin.modifyLootTable` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| reads | `PLACED_FEATURE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@41 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `PLACED_FEATURE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `BiomeSelectionContextImpl.getPlacedFeatureKey` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `PLACED_FEATURE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `FabricDynamicRegistryProvider$Entries.placedFeatures` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `POINT_OF_INTEREST_TYPE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@3 in `PoiHelper.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `POTION` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@95 in `TagRegistration.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `RECIPE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `FabricRecipeProvider$2.lookup` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `RECIPE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@2 in `FabricRecipeProvider$RecipeBootstrapContext.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `RECIPE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@78 in `FabricRecipeProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `RECIPE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@70 in `ClientboundRecipeSyncPayload$Entry.read` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `STRUCTURE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `BiomeSelectionContextImpl.validForStructure` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `STRUCTURE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `BiomeSelectionContextImpl.getStructureKey` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `STRUCTURE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@43 in `TagRegistration.<clinit>` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| reads | `TEST_ENVIRONMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@77 in `FabricGameTestModInitializer.registerDynamicEntries` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `TEST_ENVIRONMENT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `TestAnnotationLocator$TestMethod.testData` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `TEST_FUNCTION` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `TestAnnotationLocator$TestMethod.testInstance` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `TEST_INSTANCE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@64 in `FabricGameTestModInitializer.registerDynamicEntries` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `TEST_INSTANCE` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@4 in `RegistryDataLoaderMixin.lambda$loadFromResources$0` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `WORLD_PRESET` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@7 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (157 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ROOT_REGISTRY_NAME : Lnet/minecraft/resources/Identifier;
public static final ACTIVITY : Lnet/minecraft/resources/ResourceKey;
public static final ATTRIBUTE : Lnet/minecraft/resources/ResourceKey;
public static final BIOME_SOURCE : Lnet/minecraft/resources/ResourceKey;
public static final BLOCK_ENTITY_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final BLOCK_PREDICATE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final BLOCK_STATE_PROVIDER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final BLOCK : Lnet/minecraft/resources/ResourceKey;
public static final CARVER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final CHUNK_GENERATOR : Lnet/minecraft/resources/ResourceKey;
public static final CHUNK_STATUS : Lnet/minecraft/resources/ResourceKey;
public static final COMMAND_ARGUMENT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final CONSUME_EFFECT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final CREATIVE_MODE_TAB : Lnet/minecraft/resources/ResourceKey;
public static final CUSTOM_STAT : Lnet/minecraft/resources/ResourceKey;
public static final DATA_COMPONENT_PREDICATE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final DATA_COMPONENT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final GAME_RULE : Lnet/minecraft/resources/ResourceKey;
public static final DEBUG_SUBSCRIPTION : Lnet/minecraft/resources/ResourceKey;
public static final DENSITY_FUNCTION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final DIALOG_BODY_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final DIALOG_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENCHANTMENT_EFFECT_COMPONENT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENCHANTMENT_ENTITY_EFFECT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENCHANTMENT_LEVEL_BASED_VALUE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENCHANTMENT_LOCATION_BASED_EFFECT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENCHANTMENT_PROVIDER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENCHANTMENT_VALUE_EFFECT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENTITY_SUB_PREDICATE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENTITY_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENVIRONMENT_ATTRIBUTE : Lnet/minecraft/resources/ResourceKey;
public static final ATTRIBUTE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final FEATURE_SIZE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final FEATURE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final FLOAT_PROVIDER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final FLUID : Lnet/minecraft/resources/ResourceKey;
public static final FOLIAGE_PLACER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final GAME_EVENT : Lnet/minecraft/resources/ResourceKey;
public static final HEIGHT_PROVIDER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final INPUT_CONTROL_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final INT_PROVIDER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ITEM : Lnet/minecraft/resources/ResourceKey;
public static final SLOT_SOURCE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final LOOT_CONDITION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final LOOT_FUNCTION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final LOOT_NBT_PROVIDER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final CONTEXT_FLOAT_PROVIDER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final CONTEXT_INT_PROVIDER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final LOOT_POOL_ENTRY_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final LOOT_SCORE_PROVIDER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final MAP_DECORATION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final MATERIAL_CONDITION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final MATERIAL_RULE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final MEMORY_MODULE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final MENU : Lnet/minecraft/resources/ResourceKey;
public static final MOB_EFFECT : Lnet/minecraft/resources/ResourceKey;
public static final NUMBER_FORMAT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final PARTICLE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final PLACEMENT_MODIFIER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final POINT_OF_INTEREST_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final POOL_ALIAS_BINDING : Lnet/minecraft/resources/ResourceKey;
public static final POSITION_SOURCE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final POS_RULE_TEST : Lnet/minecraft/resources/ResourceKey;
public static final POTION : Lnet/minecraft/resources/ResourceKey;
public static final RECIPE_BOOK_CATEGORY : Lnet/minecraft/resources/ResourceKey;
public static final RECIPE_DISPLAY : Lnet/minecraft/resources/ResourceKey;
public static final RECIPE_SERIALIZER : Lnet/minecraft/resources/ResourceKey;
public static final RECIPE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ROOT_PLACER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final RULE_BLOCK_ENTITY_MODIFIER : Lnet/minecraft/resources/ResourceKey;
public static final RULE_TEST : Lnet/minecraft/resources/ResourceKey;
public static final SENSOR_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final SLOT_DISPLAY : Lnet/minecraft/resources/ResourceKey;
public static final SOUND_EVENT : Lnet/minecraft/resources/ResourceKey;
public static final SPAWN_CONDITION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final STAT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final STRUCTURE_PIECE : Lnet/minecraft/resources/ResourceKey;
public static final STRUCTURE_PLACEMENT : Lnet/minecraft/resources/ResourceKey;
public static final STRUCTURE_POOL_ELEMENT : Lnet/minecraft/resources/ResourceKey;
public static final STRUCTURE_PROCESSOR : Lnet/minecraft/resources/ResourceKey;
public static final STRUCTURE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final DIALOG_ACTION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final TEST_ENVIRONMENT_DEFINITION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final TEST_FUNCTION : Lnet/minecraft/resources/ResourceKey;
public static final TEST_INSTANCE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final TICKET_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final TREE_DECORATOR_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final TRUNK_PLACER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final VILLAGER_PROFESSION : Lnet/minecraft/resources/ResourceKey;
public static final VILLAGER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final INCOMING_RPC_METHOD : Lnet/minecraft/resources/ResourceKey;
public static final OUTGOING_RPC_METHOD : Lnet/minecraft/resources/ResourceKey;
public static final PERMISSION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final PERMISSION_CHECK_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final CONTEXT_KEY_SET : Lnet/minecraft/resources/ResourceKey;
public static final BANNER_PATTERN : Lnet/minecraft/resources/ResourceKey;
public static final BIOME : Lnet/minecraft/resources/ResourceKey;
public static final BLOCK_STATE_PROVIDER : Lnet/minecraft/resources/ResourceKey;
public static final CAT_SOUND_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final CAT_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final CARVER : Lnet/minecraft/resources/ResourceKey;
public static final CHAT_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final CHICKEN_SOUND_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final CHICKEN_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final ZOMBIE_NAUTILUS_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final FEATURE : Lnet/minecraft/resources/ResourceKey;
public static final COW_SOUND_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final COW_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final DAMAGE_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final DENSITY_FUNCTION : Lnet/minecraft/resources/ResourceKey;
public static final DIALOG : Lnet/minecraft/resources/ResourceKey;
public static final DIMENSION_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final ENCHANTMENT_PROVIDER : Lnet/minecraft/resources/ResourceKey;
public static final ENCHANTMENT : Lnet/minecraft/resources/ResourceKey;
public static final FLAT_LEVEL_GENERATOR_PRESET : Lnet/minecraft/resources/ResourceKey;
public static final FROG_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final INSTRUMENT : Lnet/minecraft/resources/ResourceKey;
public static final JUKEBOX_SONG : Lnet/minecraft/resources/ResourceKey;
public static final MATERIAL_CONDITION : Lnet/minecraft/resources/ResourceKey;
public static final MATERIAL_RULE : Lnet/minecraft/resources/ResourceKey;
public static final MULTI_NOISE_BIOME_SOURCE_PARAMETER_LIST : Lnet/minecraft/resources/ResourceKey;
public static final NOISE_SETTINGS : Lnet/minecraft/resources/ResourceKey;
public static final NOISE : Lnet/minecraft/resources/ResourceKey;
public static final PAINTING_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final PIG_SOUND_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final PIG_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final PLACED_FEATURE : Lnet/minecraft/resources/ResourceKey;
public static final PROCESSOR_LIST : Lnet/minecraft/resources/ResourceKey;
public static final STRUCTURE_SET : Lnet/minecraft/resources/ResourceKey;
public static final STRUCTURE : Lnet/minecraft/resources/ResourceKey;
public static final SULFUR_CUBE_ARCHETYPE : Lnet/minecraft/resources/ResourceKey;
public static final TEMPLATE_POOL : Lnet/minecraft/resources/ResourceKey;
public static final TEST_ENVIRONMENT : Lnet/minecraft/resources/ResourceKey;
public static final TEST_INSTANCE : Lnet/minecraft/resources/ResourceKey;
public static final TIMELINE : Lnet/minecraft/resources/ResourceKey;
public static final TRADE_SET : Lnet/minecraft/resources/ResourceKey;
public static final TRIAL_SPAWNER_CONFIG : Lnet/minecraft/resources/ResourceKey;
public static final TRIGGER_TYPE : Lnet/minecraft/resources/ResourceKey;
public static final TRIM_MATERIAL : Lnet/minecraft/resources/ResourceKey;
public static final TRIM_PATTERN : Lnet/minecraft/resources/ResourceKey;
public static final VILLAGER_TRADE : Lnet/minecraft/resources/ResourceKey;
public static final WOLF_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final WOLF_SOUND_VARIANT : Lnet/minecraft/resources/ResourceKey;
public static final WORLD_CLOCK : Lnet/minecraft/resources/ResourceKey;
public static final WORLD_PRESET : Lnet/minecraft/resources/ResourceKey;
public static final DECORATED_POT_PATTERN : Lnet/minecraft/resources/ResourceKey;
public static final BLOCK_TRANSFORMER : Lnet/minecraft/resources/ResourceKey;
public static final DIMENSION : Lnet/minecraft/resources/ResourceKey;
public static final LEVEL_STEM : Lnet/minecraft/resources/ResourceKey;
public static final LOOT_TABLE : Lnet/minecraft/resources/ResourceKey;
public static final ITEM_MODIFIER : Lnet/minecraft/resources/ResourceKey;
public static final PREDICATE : Lnet/minecraft/resources/ResourceKey;
public static final SLOT_SOURCE : Lnet/minecraft/resources/ResourceKey;
public static final CONTEXT_FLOAT_PROVIDER : Lnet/minecraft/resources/ResourceKey;
public static final CONTEXT_INT_PROVIDER : Lnet/minecraft/resources/ResourceKey;
public static final ADVANCEMENT : Lnet/minecraft/resources/ResourceKey;
public static final RECIPE : Lnet/minecraft/resources/ResourceKey;
public <init>()V
public static levelStemToLevel(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/resources/ResourceKey;
public static levelToLevelStem(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/resources/ResourceKey;
private static createRegistryKey(Ljava/lang/String;)Lnet/minecraft/resources/ResourceKey;
private static registryDirPath(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
public static elementsDirPath(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
public static tagsDirPath(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
public static componentsDirPath(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
static <clinit>()V
```
