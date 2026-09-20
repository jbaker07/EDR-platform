---
type: "interface"
fqcn: "net.minecraft.data.recipes.BrewingProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.BrewingProvider

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/data/recipes/RecipeOutput;)V` | exact | invokespecial@2 in `FabricBrewingProvider.<init>` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| wraps | `buildTransformations` | `()V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (4 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final containers : Ljava/util/List;
private final containerTransformations : Ljava/util/List;
private final potions : Ljava/util/Set;
private final output : Lnet/minecraft/data/recipes/RecipeOutput;
protected <init>(Lnet/minecraft/data/recipes/RecipeOutput;)V
protected addContainerTransformation(Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;Lnet/minecraft/world/item/Item;)V
protected addContainer(Lnet/minecraft/world/item/Item;)V
protected buildMix(Lnet/minecraft/core/Holder;Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder;)V
protected buildStartMix(Lnet/minecraft/world/item/Item;Lnet/minecraft/core/Holder;)V
protected buildTransformations()V
protected save(Lnet/minecraft/data/recipes/BrewingRecipeBuilder;)V
public final buildRecipes()V
protected abstract addContainers()V
protected abstract addContainerTransformations()V
protected abstract buildMixes()V
```
