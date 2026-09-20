---
type: "mechanism"
module: "fabric-recipe-api-v1"
version: "10.0.8+fcdff87f5d"
sha256: "7d63b44a449ddd84d4988ef17cec1894ef1890b2844079500ea47c3dc678d967"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-recipe-api-v1

**Version** `10.0.8+fcdff87f5d` -- **artifact sha256** `7d63b44a449ddd84d4988ef17cec1894ef1890b2844079500ea47c3dc678d967`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-networking-api-v1": "*", "fabric-lifecycle-events-v1": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.recipe.ingredient.CustomIngredientInit", "net.fabricmc.fabric.impl.recipe.ingredient.CustomIngredientSync", "net.fabricmc.fabric.impl.recipe.sync.RecipeSyncImpl"], "client": ["net.fabricmc.fabric.impl.recipe.ingredient.client.CustomIngredientSyncClient", "net.fabricmc.fabric.impl.recipe.sync.client.RecipeSyncImplClient"]}`
- mixin configs: `["fabric-recipe-api-v1.mixins.json", {"config": "fabric-recipe-api-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-recipe-api-v1.classtweaker`
- mixin classes: 11 found by annotation, 11 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.recipe.v1.sync.ClientRecipeSynchronizedEvent.EVENT|ClientRecipeSynchronizedEvent.EVENT]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]].`handleSelectKnownPacks` | `(Lnet/minecraft/network/protocol/configuration/ClientboundSelectKnownPacks;)V` | name_only | @Inject | TAIL | client | 1000 (default) | `ClientConfigurationPacketListenerImplMixin.sendSupportedRecipeSerializers` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleUpdateRecipes` | `(Lnet/minecraft/network/protocol/game/ClientboundUpdateRecipesPacket;)V` | name_only | @WrapOperation | FIELD `Lnet/minecraft/client/multiplayer/ClientPacketListener;recipes:Lnet/minecraft/client/multiplayer/ClientRecipeContainer;` (exact) | client | 1000 (default) | `ClientPacketListenerMixin.copyPreviousRecipes` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]].`<clinit>` | `()V` | exact | @ModifyExpressionValue | INVOKE `Lnet/minecraft/network/codec/StreamCodec;map(Ljava/util/function/Function;Ljava/util/function/Function;)Lnet/minecraft/network/codec/StreamCodec;` (exact) | both | 1000 (default) | `IngredientMixin.useCustomIngredientStreamCodec` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]].`<clinit>` | `()V` | exact | @ModifyExpressionValue | INVOKE `Lnet/minecraft/network/codec/StreamCodec;map(Ljava/util/function/Function;Ljava/util/function/Function;)Lnet/minecraft/network/codec/StreamCodec;` (exact) | both | 1000 (default) | `IngredientMixin.useOptionalCustomIngredientStreamCodec` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]].`<clinit>` | `()V` | exact | @Inject | TAIL | both | 1000 (default) | `IngredientMixin.injectCodec` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]].`equals` | `(Ljava/lang/Object;)Z` | exact | @Inject | HEAD | both | 1000 (default) | `IngredientMixin.onHeadEquals` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]].`lambda$static$0` | `(Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/core/HolderSet;` | name_only | @Inject | HEAD | both | 1000 (default) | `IngredientMixin.onGetEntries` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]].`lambda$static$2` | `(Ljava/util/Optional;)Lnet/minecraft/core/HolderSet;` | name_only | @Inject | HEAD | both | 1000 (default) | `IngredientMixin.onGetEntries` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]].`lambda$static$4` | `(Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/core/HolderSet;` | name_only | @Inject | HEAD | both | 1000 (default) | `IngredientMixin.onGetEntries` |
| [[40-Interfaces/net.minecraft.world.item.crafting.RecipeManager|RecipeManager]].`<init>` | `(Lnet/minecraft/core/HolderLookup$Provider;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `RecipeManagerMixin.updateSynchronizedRecipes` |
| [[40-Interfaces/net.minecraft.world.item.crafting.RecipeMap|RecipeMap]].`create` | `(Lnet/minecraft/core/HolderLookup;)Lnet/minecraft/world/item/crafting/RecipeMap;` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `RecipeMapMixin.attachSerializerMap` |
| [[40-Interfaces/net.minecraft.world.item.crafting.ShapelessRecipe|ShapelessRecipe]].`<init>` | `(Lnet/minecraft/world/item/crafting/Recipe$CommonInfo;Lnet/minecraft/world/item/crafting/CraftingRecipe$CraftingBookInfo;Lnet/minecraft/world/item/ItemStackTemplate;Ljava/util/List;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `ShapelessRecipeMixin.cacheRequiresTesting` |
| [[40-Interfaces/net.minecraft.world.item.crafting.ShapelessRecipe|ShapelessRecipe]].`matches` | `(Lnet/minecraft/world/item/crafting/CraftingInput;Lnet/minecraft/world/level/Level;)Z` | exact | @Inject | HEAD | both | 1000 (default) | `ShapelessRecipeMixin.customIngredientMatch` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.recipe.v1.sync.ClientRecipeSynchronizedEvent|ClientRecipeSynchronizedEvent]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.FabricRecipeAccess|FabricRecipeAccess]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.FabricRecipeManager|FabricRecipeManager]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredient|CustomIngredient]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredientSerializer|CustomIngredientSerializer]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.ingredient.DefaultCustomIngredients|DefaultCustomIngredients]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.ingredient.FabricIngredient|FabricIngredient]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.sync.RecipeSynchronization|RecipeSynchronization]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.sync.SynchronizedRecipes|SynchronizedRecipes]] (interface, 8 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
