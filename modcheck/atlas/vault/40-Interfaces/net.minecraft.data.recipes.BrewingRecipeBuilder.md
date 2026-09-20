---
type: "interface"
fqcn: "net.minecraft.data.recipes.BrewingRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.BrewingRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/data/recipes/RecipeBuilder`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/Res` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (3 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final input : Lnet/minecraft/world/item/crafting/PotionIngredient;
private final reagent : Lnet/minecraft/world/item/crafting/PotionIngredient;
private final output : Lnet/minecraft/world/item/ItemStackTemplate;
private <init>(Lnet/minecraft/world/item/crafting/PotionIngredient;Lnet/minecraft/world/item/crafting/PotionIngredient;Lnet/minecraft/world/item/ItemStackTemplate;)V
private static potionIngredient(Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/item/crafting/PotionIngredient;
private static potionOutput(Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/item/ItemStackTemplate;
public static brewingMix(Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder;Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder;)Lnet/minecraft/data/recipes/BrewingRecipeBuilder;
public static brewingContainerTransform(Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)Lnet/minecraft/data/recipes/BrewingRecipeBuilder;
public unlockedBy(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/data/recipes/RecipeBuilder;
public group(Ljava/lang/String;)Lnet/minecraft/data/recipes/BrewingRecipeBuilder;
public static getExactPotion(Lnet/minecraft/core/component/predicates/PotionsPredicate;)Ljava/util/Optional;
private getIngredientPotion(Lnet/minecraft/world/item/crafting/PotionIngredient;)Ljava/util/Optional;
public defaultId()Lnet/minecraft/resources/ResourceKey;
public save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V
public synthetic group(Ljava/lang/String;)Lnet/minecraft/data/recipes/RecipeBuilder;
private static synthetic lambda$defaultId$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/ResourceKey;Ljava/lang/String;)Ljava/lang/String;
```
