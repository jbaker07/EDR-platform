---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientRecipeContainer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientRecipeContainer

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getSynchronizedRecipes()Lnet/fabricmc/fabric/api/recipe/v1/sync/SynchronizedRecipe` | `` | client | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.multiplayer.ClientRecipeContainer implements net.minecraft.world.item.crafting.RecipeAccess {
    private final java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.RecipePropertySet>, net.minecraft.world.item.crafting.RecipePropertySet> itemSets;
    private final net.minecraft.world.item.crafting.SelectableRecipe$SingleInputSet<net.minecraft.world.item.crafting.StonecutterRecipe> stonecutterRecipes;
    public net.minecraft.client.multiplayer.ClientRecipeContainer(java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.RecipePropertySet>, net.minecraft.world.item.crafting.RecipePropertySet>, net.minecraft.world.item.crafting.SelectableRecipe$SingleInputSet<net.minecraft.world.item.crafting.StonecutterRecipe>);
    public net.minecraft.world.item.crafting.RecipePropertySet propertySet(net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.RecipePropertySet>);
    public net.minecraft.world.item.crafting.SelectableRecipe$SingleInputSet<net.minecraft.world.item.crafting.StonecutterRecipe> stonecutterRecipes();
}
```
