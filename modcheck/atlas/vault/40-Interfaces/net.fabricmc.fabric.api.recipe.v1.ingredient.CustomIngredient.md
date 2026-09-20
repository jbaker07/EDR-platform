---
type: "interface"
fqcn: "net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredient"
module: "fabric-recipe-api-v1"
sha256: "7d63b44a449ddd84d4988ef17cec1894ef1890b2844079500ea47c3dc678d967"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredient

Module: [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] -- kind: interface

```java
public abstract boolean test(net.minecraft.world.item.ItemStack)
public abstract java.util.stream.Stream items()
public abstract boolean requiresTesting()
public abstract net.fabricmc.fabric.api.recipe.v1.ingredient.CustomIngredientSerializer getSerializer()
public net.minecraft.world.item.crafting.display.SlotDisplay display()
public net.minecraft.world.item.crafting.Ingredient toVanilla()
```
