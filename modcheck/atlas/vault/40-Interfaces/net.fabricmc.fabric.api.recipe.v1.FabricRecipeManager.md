---
type: "interface"
fqcn: "net.fabricmc.fabric.api.recipe.v1.FabricRecipeManager"
module: "fabric-recipe-api-v1"
sha256: "7d63b44a449ddd84d4988ef17cec1894ef1890b2844079500ea47c3dc678d967"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.recipe.v1.FabricRecipeManager

Module: [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] -- kind: interface

```java
public default <I extends net.minecraft.world.item.crafting.RecipeInput, T extends net.minecraft.world.item.crafting.Recipe<I>> java.util.stream.Stream<net.minecraft.world.item.crafting.RecipeHolder<T>> getAllMatches(net.minecraft.world.item.crafting.RecipeType<T>, I, net.minecraft.world.level.Level)
public default <I extends net.minecraft.world.item.crafting.RecipeInput, T extends net.minecraft.world.item.crafting.Recipe<I>> java.util.Collection<net.minecraft.world.item.crafting.RecipeHolder<T>> getAllOfType(net.minecraft.world.item.crafting.RecipeType<T>)
```
