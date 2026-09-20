---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.CraftingRecipe"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.CraftingRecipe

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/item/crafting/Recipe`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `defaultCraftingReminder` | `(Lnet/minecraft/world/item/crafting/CraftingInput;)Lnet/minecraft/core` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| wraps | `defaultCraftingReminder` | `(Lnet/minecraft/world/item/crafting/CraftingInput;)Lnet/minecraft/core` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (0 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public getType()Lnet/minecraft/world/item/crafting/RecipeType;
public abstract getSerializer()Lnet/minecraft/world/item/crafting/RecipeSerializer;
public abstract category()Lnet/minecraft/world/item/crafting/CraftingBookCategory;
public getRemainingItems(Lnet/minecraft/world/item/crafting/CraftingInput;)Lnet/minecraft/core/NonNullList;
public static defaultCraftingReminder(Lnet/minecraft/world/item/crafting/CraftingInput;)Lnet/minecraft/core/NonNullList;
public recipeBookCategory()Lnet/minecraft/world/item/crafting/RecipeBookCategory;
```
