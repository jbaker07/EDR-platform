---
type: "interface"
fqcn: "net.minecraft.data.recipes.CustomCraftingRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.CustomCraftingRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/Res` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (4 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final category : Lnet/minecraft/data/recipes/RecipeCategory;
private final advancementBuilder : Lnet/minecraft/data/recipes/RecipeUnlockAdvancementBuilder;
private group : Ljava/lang/String;
private final factory : Lnet/minecraft/data/recipes/CustomCraftingRecipeBuilder$Factory;
public <init>(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/data/recipes/CustomCraftingRecipeBuilder$Factory;)V
public static customCrafting(Lnet/minecraft/data/recipes/RecipeCategory;Lnet/minecraft/data/recipes/CustomCraftingRecipeBuilder$Factory;)Lnet/minecraft/data/recipes/CustomCraftingRecipeBuilder;
public unlockedBy(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/data/recipes/CustomCraftingRecipeBuilder;
public group(Ljava/lang/String;)Lnet/minecraft/data/recipes/CustomCraftingRecipeBuilder;
public save(Lnet/minecraft/data/recipes/RecipeOutput;Ljava/lang/String;)V
public save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V
```
