---
type: "mechanism"
module: "fabric-data-generation-api-v1"
version: "27.2.4+427eab975d"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-data-generation-api-v1

**Version** `27.2.4+427eab975d` -- **artifact sha256** `2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-convention-tags-v2": "*", "fabric-recipe-api-v1": "*", "fabric-registry-sync-v0": "*", "fabric-resource-conditions-api-v1": "*", "fabric-tag-api-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-data-generation-api-v1.mixins.json", {"config": "fabric-data-generation-api-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-data-generation-api-v1.classtweaker`
- mixin classes: 27 found by annotation, 27 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`<init>` | `(Lnet/minecraft/client/main/GameConfig;)V` | name_only | @Inject | INVOKE `Lcom/mojang/blaze3d/systems/RenderSystem;getBackendDescription()Ljava/lang/String;` (exact) | client | 1000 (default) | `MinecraftMixin.main` |
| [[40-Interfaces/net.minecraft.client.data.models.ModelProvider|ModelProvider]].`<init>` | `(Lnet/minecraft/data/PackOutput;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `ModelProviderMixin.init` |
| [[40-Interfaces/net.minecraft.client.data.models.ModelProvider|ModelProvider]].`run` | `(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFuture;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/data/models/BlockModelGenerators;run()V` (exact) | client | 1000 (default) | `ModelProviderMixin.registerBlockStateModels` |
| [[40-Interfaces/net.minecraft.client.data.models.ModelProvider|ModelProvider]].`run` | `(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFuture;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/data/models/ItemModelGenerators;run()V` (exact) | client | 1000 (default) | `ModelProviderMixin.registerItemModels` |
| [[40-Interfaces/net.minecraft.client.data.models.ModelProvider|ModelProvider]].`run` | `(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Inject | INVOKE `Lnet/minecraft/client/data/models/BlockModelGenerators;run()V` (exact) | client | 1000 (default) | `ModelProviderMixin.setFabricPackOutput` |
| [[40-Interfaces/net.minecraft.client.data.models.ModelProvider_BlockStateGeneratorCollector|ModelProvider$BlockStateGeneratorCollector]].`validate` | `()V` | name_only | @ModifyArg | INVOKE `Ljava/util/stream/Stream;filter(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;` (exact) | client | 1000 (default) | `ModelProviderBlockStateGeneratorCollectorMixin.filterBlocksForProcessingMod` |
| [[40-Interfaces/net.minecraft.client.data.models.ModelProvider_ItemInfoCollector|ModelProvider$ItemInfoCollector]].`finalizeAndValidate` | `()V` | name_only | @ModifyArg | INVOKE `Ljava/util/stream/Stream;filter(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;` (exact) | client | 1000 (default) | `ModelProviderItemInfoCollectorMixin.filterItemsForProcessingMod` |
| [[40-Interfaces/net.minecraft.client.data.models.ModelProvider_ItemInfoCollector|ModelProvider$ItemInfoCollector]].`lambda$finalizeAndValidate$0` | `(Lnet/minecraft/world/item/Item;)V` | name_only | @WrapOperation | INVOKE `Ljava/util/Map;containsKey(Ljava/lang/Object;)Z` (exact) | client | 1000 (default) | `ModelProviderItemInfoCollectorMixin.filterItemsForProcessingMod` |
| [[40-Interfaces/net.minecraft.data.DataProvider|DataProvider]].`lambda$static$0` | `(Lit/unimi/dsi/fastutil/objects/Object2IntOpenHashMap;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `DataProviderMixin.addFabricKeySortOrders` |
| [[40-Interfaces/net.minecraft.data.HashCache|HashCache]].`lambda$purgeStaleAndWrite$0` | `(Ljava/util/Set;Ljava/lang/String;Lnet/minecraft/data/HashCache$ProviderCache;)V` | name_only | @Redirect | INVOKE `Ljava/time/ZonedDateTime;now()Ljava/time/ZonedDateTime;` (exact) | both | 1000 (default) | `HashCacheMixin.constantTime` |
| [[40-Interfaces/net.minecraft.data.HashCache_ProviderCache|HashCache$ProviderCache]].`save` | `(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/lang/String;)V` | name_only | @ModifyExpressionValue | INVOKE `Lcom/google/common/collect/ImmutableMap;entrySet()Lcom/google/common/collect/ImmutableSet;` (exact) | both | 1000 (default) | `HashCacheProviderCacheMixin.sortPaths` |
| [[40-Interfaces/net.minecraft.data.HashCache_ProviderCache|HashCache$ProviderCache]].`save` | `(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/lang/String;)V` | name_only | @ModifyExpressionValue | INVOKE `Ljava/nio/file/Path;toString()Ljava/lang/String;` (exact) | both | 1000 (default) | `HashCacheProviderCacheMixin.pathToString` |
| [[40-Interfaces/net.minecraft.data.advancements.packs.VanillaAdventureAdvancements|VanillaAdventureAdvancements]].`validateMobsToKill` | `(Ljava/util/List;)Ljava/util/List;` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/data/worldgen/BootstrapContext;listContextElements(Lnet/minecraft/resources/ResourceKey;)Ljava/util/stream/Stream;` (inherited_exact) | both | 1000 (default) | `VanillaAdventureAdvancementsMixin.onlyCheckVanillaEntities` |
| [[40-Interfaces/net.minecraft.data.loot.BlockLootSubProvider|BlockLootSubProvider]].`run` | `()V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/core/DefaultedRegistry;iterator()Ljava/util/Iterator;` (inherited_exact) | both | 1000 (default) | `BlockLootSubProviderMixin.onlyVanillaBlocks` |
| [[40-Interfaces/net.minecraft.data.loot.EntityLootSubProvider|EntityLootSubProvider]].`run` | `()V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/data/loot/LootTableSubProvider$Context;listContextElements(Lnet/minecraft/resources/ResourceKey;)Ljava/util/stream/Stream;` (inherited_exact) | both | 1000 (default) | `EntityLootSubProviderMixin.onlyVanillaEntities` |
| [[40-Interfaces/net.minecraft.data.recipes.BrewingProvider|BrewingProvider]].`buildTransformations` | `()V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/data/recipes/BrewingProvider;save(Lnet/minecraft/data/recipes/BrewingRecipeBuilder;)V` (exact) | both | 1000 (default) | `BrewingProviderMixin.preventDuplicatingDefaultTransformations` |
| [[40-Interfaces/net.minecraft.data.recipes.BrewingRecipeBuilder|BrewingRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.recipes.CustomCraftingRecipeBuilder|CustomCraftingRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]].`lambda$waxRecipes$0` | `(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;save(Lnet/minecraft/data/recipes/RecipeOutput;Ljava/lang/String;)V` (inherited_exact) | both | 1000 (default) | `RecipeProviderMixin.adjustIdWaxRecipes` |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]].`lambda$waxRecipes$0` | `(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/block/Block;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `RecipeProviderMixin.dontGenerateNonVanillaWaxingRecipes` |
| [[40-Interfaces/net.minecraft.data.recipes.RecipeProvider|RecipeProvider]].`stonecutterResultFromBase` | `(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/level/ItemLike;I)V` | exact | @ModifyArg | INVOKE `Lnet/minecraft/data/recipes/SingleItemRecipeBuilder;save(Lnet/minecraft/data/recipes/RecipeOutput;Ljava/lang/String;)V` (inherited_exact) | both | 1000 (default) | `RecipeProviderMixin.adjustIdStonecutter` |
| [[40-Interfaces/net.minecraft.data.recipes.ShapedRecipeBuilder|ShapedRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.recipes.ShapelessRecipeBuilder|ShapelessRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.recipes.SimpleCookingRecipeBuilder|SimpleCookingRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.recipes.SingleItemRecipeBuilder|SingleItemRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.recipes.SmithingTransformRecipeBuilder|SmithingTransformRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `SmithingTransformRecipeBuilderMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.recipes.SmithingTrimRecipeBuilder|SmithingTrimRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `SmithingTrimRecipeBuilderMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.recipes.SpecialRecipeBuilder|SpecialRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `SpecialRecipeBuilderMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.recipes.TransmuteRecipeBuilder|TransmuteRecipeBuilder]].`save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` |
| [[40-Interfaces/net.minecraft.data.tags.TagsProvider|TagsProvider]].`<init>` | `(Lnet/minecraft/data/PackOutput;Lnet/minecraft/resources/ResourceKey;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;)V` | exact | @Inject | RETURN | both | 1000 (default) | `TagsProviderMixin.initPathResolver` |
| [[40-Interfaces/net.minecraft.data.tags.TagsProvider|TagsProvider]].`lambda$run$2` | `(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/data/tags/TagsProvider$1CombinedData;)Ljava/util/concurrent/CompletionStage;` | name_only | @WrapOperation | INVOKE `Ljava/util/concurrent/CompletableFuture;allOf([Ljava/util/concurrent/CompletableFuture;)Ljava/util/concurrent/CompletableFuture;` (exact) | both | 1000 (default) | `TagsProviderMixin.addTagAliasGroupBuilders` |
| [[40-Interfaces/net.minecraft.data.tags.TagsProvider|TagsProvider]].`lambda$run$5` | `(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Lnet/minecraft/data/CachedOutput;Lnet/minecraft/data/tags/TagsProvider$1CombinedData;Ljava/util/Map$Entry;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/data/DataProvider;saveStable(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/core/HolderLookup$Provider;Lcom/mojang/serialization/Codec;Ljava/lang/Object;Ljava/nio/file/Path;)Ljava/util/concurrent/CompletableFuture;` (exact) | both | 1000 (default) | `TagsProviderMixin.addRemove` |
| [[40-Interfaces/net.minecraft.data.tags.TagsProvider|TagsProvider]].`lambda$run$5` | `(Ljava/util/function/Predicate;Ljava/util/function/Predicate;Lnet/minecraft/data/CachedOutput;Lnet/minecraft/data/tags/TagsProvider$1CombinedData;Ljava/util/Map$Entry;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/tags/TagFile;<init>(Ljava/util/List;Z)V` (exact) | both | 1000 (default) | `TagsProviderMixin.addReplaced` |
| [[40-Interfaces/net.minecraft.server.Main|Main]].`main` | `([Ljava/lang/String;)V` | name_only | @Inject | NEW `net/minecraft/server/dedicated/DedicatedServerSettings` (exact) | server | 1000 (default) | `MainMixin.main` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.datagen.v1.builder.SoundTypeBuilder|SoundTypeBuilder]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.datagen.v1.provider.FabricModelProvider|FabricModelProvider]] (abstract_class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.datagen.v1.provider.FabricSoundsProvider|FabricSoundsProvider]] (abstract_class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.DataGeneratorEntrypoint|DataGeneratorEntrypoint]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.FabricDataGenerator|FabricDataGenerator]] (class, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.FabricPackOutput|FabricPackOutput]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.JsonKeySortOrderCallback|JsonKeySortOrderCallback]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.advancement.FabricAdvancementBuilder|FabricAdvancementBuilder]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.loot.FabricBlockLootSubProvider|FabricBlockLootSubProvider]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.loot.FabricEntityLootSubProvider|FabricEntityLootSubProvider]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricAdvancementProvider|FabricAdvancementProvider]] (abstract_class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricBlockLootSubProvider|FabricBlockLootSubProvider]] (abstract_class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricBrewingProvider|FabricBrewingProvider]] (abstract_class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricCodecDataProvider|FabricCodecDataProvider]] (abstract_class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider|FabricDynamicRegistryProvider]] (abstract_class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricEntityLootSubProvider|FabricEntityLootSubProvider]] (abstract_class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider|FabricLanguageProvider]] (abstract_class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricLootTableSubProvider|FabricLootTableSubProvider]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricRecipeProvider|FabricRecipeProvider]] (abstract_class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricTagAppender|FabricTagAppender]] (interface, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider|FabricTagsProvider]] (abstract_class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.provider.SimpleFabricLootTableSubProvider|SimpleFabricLootTableSubProvider]] (abstract_class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.datagen.v1.recipe.FabricRecipeOutput|FabricRecipeOutput]] (interface, 1 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
