---
type: "interface"
fqcn: "net.minecraft.data.recipes.ShapelessRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.ShapelessRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/data/recipes/RecipeBuilder`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/Res` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (6 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final items : Lnet/minecraft/core/HolderGetter;
private final category : Lnet/minecraft/data/recipes/RecipeCategory;
private final result : Lnet/minecraft/world/item/ItemStackTemplate;
private final ingredients : Ljava/util/List;
private final advancementBuilder : Lnet/minecraft/data/recipes/RecipeUnlockAdvancementBuilder;
private group : Ljava/lang/String;
private <init>(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/ItemStackTemplate;)V
public static shapeless(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/item/ItemStackTemplate;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public static shapeless(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public static shapeless(Lnet/minecraft/core/HolderGetter;Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/world/level/ItemLike;I)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public requires(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public requires(Lnet/minecraft/world/level/ItemLike;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public requires(Lnet/minecraft/world/level/ItemLike;I)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public requires(Lnet/minecraft/world/item/crafting/Ingredient;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public requires(Lnet/minecraft/world/item/crafting/Ingredient;I)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public unlockedBy(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public group(Ljava/lang/String;)Lnet/minecraft/data/recipes/ShapelessRecipeBuilder;
public defaultId()Lnet/minecraft/resources/ResourceKey;
public save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V
public synthetic group(Ljava/lang/String;)Lnet/minecraft/data/recipes/RecipeBuilder;
public synthetic unlockedBy(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/data/recipes/RecipeBuilder;
```
