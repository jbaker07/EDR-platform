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

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.recipe.v1.sync.ClientRecipeSynchronizedEvent.EVENT|ClientRecipeSynchronizedEvent.EVENT]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]] | `handleSelectKnownPacks` | injects_into `@Inject at TAIL` | client | `ClientConfigurationPacketListenerImplMixin.sendSupportedRecipeSerializers` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]] | `<clinit>` | injects_into `@Inject at TAIL` | both | `IngredientMixin.injectCodec` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]] | `equals(Ljava/lang/Object;)Z` | injects_into `@Inject at HEAD` | both | `IngredientMixin.onHeadEquals` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]] | `lambda$static$0` | injects_into `@Inject at HEAD` | both | `IngredientMixin.onGetEntries` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]] | `lambda$static$2` | injects_into `@Inject at HEAD` | both | `IngredientMixin.onGetEntries` |
| [[40-Interfaces/net.minecraft.world.item.crafting.Ingredient|Ingredient]] | `lambda$static$4` | injects_into `@Inject at HEAD` | both | `IngredientMixin.onGetEntries` |
| [[40-Interfaces/net.minecraft.world.item.crafting.RecipeManager|RecipeManager]] | `<init>` | injects_into `@Inject at TAIL` | both | `RecipeManagerMixin.updateSynchronizedRecipes` |
| [[40-Interfaces/net.minecraft.world.item.crafting.ShapelessRecipe|ShapelessRecipe]] | `<init>` | injects_into `@Inject at RETURN` | both | `ShapelessRecipeMixin.cacheRequiresTesting` |
| [[40-Interfaces/net.minecraft.world.item.crafting.ShapelessRecipe|ShapelessRecipe]] | `matches(Lnet/minecraft/world/item/crafting/CraftingInput;Lnet/minecraft/world/level/Level;)Z` | injects_into `@Inject at HEAD` | both | `ShapelessRecipeMixin.customIngredientMatch` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.recipe.v1.sync.ClientRecipeSynchronizedEvent|ClientRecipeSynchronizedEvent]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.FabricRecipeAccess|FabricRecipeAccess]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.FabricRecipeManager|FabricRecipeManager]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredient|CustomIngredient]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredientSerializer|CustomIngredientSerializer]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.ingredient.DefaultCustomIngredients|DefaultCustomIngredients]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.ingredient.FabricIngredient|FabricIngredient]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.sync.RecipeSynchronization|RecipeSynchronization]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.recipe.v1.sync.SynchronizedRecipes|SynchronizedRecipes]] (interface, 8 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
