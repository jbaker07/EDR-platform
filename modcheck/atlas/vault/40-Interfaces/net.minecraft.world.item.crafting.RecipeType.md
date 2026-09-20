---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.RecipeType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.RecipeType

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `equals` | `(Ljava/lang/Object;)Z` | inherited_exact | invokeinterface@22 in `SynchronizedRecipes.get` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (8 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CRAFTING : Lnet/minecraft/world/item/crafting/RecipeType;
public static final SMELTING : Lnet/minecraft/world/item/crafting/RecipeType;
public static final BLASTING : Lnet/minecraft/world/item/crafting/RecipeType;
public static final SMOKING : Lnet/minecraft/world/item/crafting/RecipeType;
public static final CAMPFIRE_COOKING : Lnet/minecraft/world/item/crafting/RecipeType;
public static final STONECUTTING : Lnet/minecraft/world/item/crafting/RecipeType;
public static final SMITHING : Lnet/minecraft/world/item/crafting/RecipeType;
public static final BREWING : Lnet/minecraft/world/item/crafting/RecipeType;
public static register(Ljava/lang/String;)Lnet/minecraft/world/item/crafting/RecipeType;
static <clinit>()V
```
