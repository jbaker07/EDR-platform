---
type: "interface"
fqcn: "net.fabricmc.fabric.api.recipe.v1.sync.SynchronizedRecipes"
module: "fabric-recipe-api-v1"
sha256: "7d63b44a449ddd84d4988ef17cec1894ef1890b2844079500ea47c3dc678d967"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.recipe.v1.sync.SynchronizedRecipes

Module: [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] -- kind: interface

```java
public abstract java.util.stream.Stream getAllMatches(net.minecraft.world.item.crafting.RecipeType, net.minecraft.world.item.crafting.RecipeInput, net.minecraft.world.level.Level)
public abstract java.util.Collection getAllOfType(net.minecraft.world.item.crafting.RecipeType)
public java.util.Optional getFirstMatch(net.minecraft.world.item.crafting.RecipeType, net.minecraft.world.item.crafting.RecipeInput, net.minecraft.world.level.Level, net.minecraft.resources.ResourceKey)
public java.util.Optional getFirstMatch(net.minecraft.world.item.crafting.RecipeType, net.minecraft.world.item.crafting.RecipeInput, net.minecraft.world.level.Level, net.minecraft.world.item.crafting.RecipeHolder)
public abstract java.util.Optional getFirstMatch(net.minecraft.world.item.crafting.RecipeType, net.minecraft.world.item.crafting.RecipeInput, net.minecraft.world.level.Level)
public abstract net.minecraft.world.item.crafting.RecipeHolder get(net.minecraft.resources.ResourceKey)
public net.minecraft.world.item.crafting.RecipeHolder get(net.minecraft.world.item.crafting.RecipeType, net.minecraft.resources.ResourceKey)
public abstract java.util.Collection recipes()
```
