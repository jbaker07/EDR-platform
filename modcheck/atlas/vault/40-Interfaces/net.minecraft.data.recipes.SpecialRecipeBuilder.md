---
type: "interface"
fqcn: "net.minecraft.data.recipes.SpecialRecipeBuilder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.SpecialRecipeBuilder

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `save` | `(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/Res` | exact | @ModifyVariable at ['HEAD'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private advancementBuilder : Lnet/minecraft/data/recipes/RecipeUnlockAdvancementBuilder;
private final factory : Ljava/util/function/Supplier;
public <init>(Ljava/util/function/Supplier;)V
public static special(Ljava/util/function/Supplier;)Lnet/minecraft/data/recipes/SpecialRecipeBuilder;
public unlockedBy(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/data/recipes/SpecialRecipeBuilder;
public save(Lnet/minecraft/data/recipes/RecipeOutput;Ljava/lang/String;)V
public save(Lnet/minecraft/data/recipes/RecipeOutput;Lnet/minecraft/resources/ResourceKey;)V
```
