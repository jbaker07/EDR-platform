---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.CraftingRecipe"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.CraftingRecipe

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `defaultCraftingReminder` | `@Redirect at INVOKE Lnet/minecraft/world/item/Item;getCraftingRemainder()Lnet/mi` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.item.crafting.CraftingRecipe extends net.minecraft.world.item.crafting.Recipe<net.minecraft.world.item.crafting.CraftingInput> {
    public default net.minecraft.world.item.crafting.RecipeType<net.minecraft.world.item.crafting.CraftingRecipe> getType();
    public abstract net.minecraft.world.item.crafting.RecipeSerializer<? extends net.minecraft.world.item.crafting.CraftingRecipe> getSerializer();
    public abstract net.minecraft.world.item.crafting.CraftingBookCategory category();
    public default net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getRemainingItems(net.minecraft.world.item.crafting.CraftingInput);
    public static net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> defaultCraftingReminder(net.minecraft.world.item.crafting.CraftingInput);
    public default net.minecraft.world.item.crafting.RecipeBookCategory recipeBookCategory();
}
```
