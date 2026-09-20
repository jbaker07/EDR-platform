---
type: "interface"
fqcn: "net.minecraft.data.recipes.SimpleCookingRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.SimpleCookingRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/data/recipes/RecipeBuilder`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/Res` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (9 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final craftingCategory : Lnet/minecraft/data/recipes/RecipeCategory;
private final cookingCategory : Lnet/minecraft/world/item/crafting/CookingBookCategory;
private final result : Lnet/minecraft/world/item/ItemStackTemplate;
private final ingredient : Lnet/minecraft/world/item/crafting/Ingredient;
private final experience : F
private final cookingTime : I
private final advancementBuilder : Lnet/minecraft/data/recipes/RecipeUnlockAdvancementBuilder;
private group : Ljava/lang/String;
private final factory : Lnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;
private <init>(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/item/ItemStackTemplate;Lnet/minecraft/world/item/crafting/Ingredient;FILnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;)V
private <init>(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;Lnet/minecraft/world/item/crafting/Ingredient;FILnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;)V
public static generic(Lnet/minecraft/world/item/crafting/Ingredient;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;FILnet/minecraft/world/item/crafting/AbstractCookingRecipe$Factory;)Lnet/minecraft/data/recipes/SimpleCookingRecipeBuilder;
public static campfireCooking(Lnet/minecraft/world/item/crafting/Ingredient;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;FI)Lnet/minecraft/data/recipes/SimpleCookingRecipeBuilder;
public static blasting(Lnet/minecraft/world/item/crafting/Ingredient;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;FI)Lnet/minecraft/data/recipes/SimpleCookingRecipeBuilder;
public static smelting(Lnet/minecraft/world/item/crafting/Ingredient;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/CookingBookCategory;Lnet/minecraft/world/level/ItemLike;FI)Lnet/minecraft/data/recipes/SimpleCookingRecipeBuilder;
public static smoking(Lnet/minecraft/world/item/crafting/Ingredient;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;FI)Lnet/minecraft/data/recipes/SimpleCookingRecipeBuilder;
public unlockedBy(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/data/recipes/SimpleCookingRecipeBuilder;
public group(Ljava/lang/String;)Lnet/minecraft/data/recipes/SimpleCookingRecipeBuilder;
public defaultId()Lnet/minecraft/resources/ResourceKey;
public save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V
public synthetic group(Ljava/lang/String;)Lnet/minecraft/data/recipes/RecipeBuilder;
public synthetic unlockedBy(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/data/recipes/RecipeBuilder;
```
