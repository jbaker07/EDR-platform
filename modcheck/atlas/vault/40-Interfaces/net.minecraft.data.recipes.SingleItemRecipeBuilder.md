---
type: "interface"
fqcn: "net.minecraft.data.recipes.SingleItemRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.SingleItemRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/data/recipes/RecipeBuilder`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/Res` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (5 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final category : Lnet/minecraft/data/recipes/RecipeCategory;
private final result : Lnet/minecraft/world/item/ItemStackTemplate;
private final ingredient : Lnet/minecraft/world/item/crafting/Ingredient;
private final advancementBuilder : Lnet/minecraft/data/recipes/RecipeUnlockAdvancementBuilder;
private final factory : Lnet/minecraft/world/item/crafting/SingleItemRecipe$Factory;
private <init>(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/SingleItemRecipe$Factory;Lnet/minecraft/world/item/crafting/Ingredient;Lnet/minecraft/world/item/ItemStackTemplate;)V
public <init>(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/crafting/SingleItemRecipe$Factory;Lnet/minecraft/world/item/crafting/Ingredient;Lnet/minecraft/world/level/ItemLike;I)V
public static stonecutting(Lnet/minecraft/world/item/crafting/Ingredient;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;I)Lnet/minecraft/data/recipes/SingleItemRecipeBuilder;
public unlockedBy(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/data/recipes/SingleItemRecipeBuilder;
public group(Ljava/lang/String;)Lnet/minecraft/data/recipes/SingleItemRecipeBuilder;
public defaultId()Lnet/minecraft/resources/ResourceKey;
public save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V
public synthetic group(Ljava/lang/String;)Lnet/minecraft/data/recipes/RecipeBuilder;
public synthetic unlockedBy(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/data/recipes/RecipeBuilder;
```
