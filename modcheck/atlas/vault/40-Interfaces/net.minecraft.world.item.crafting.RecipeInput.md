---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.RecipeInput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.RecipeInput

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isEmpty` | `()Z` | exact | invokeinterface@1 in `SynchronizedRecipesImpl.getAllMatches` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getItem(I)Lnet/minecraft/world/item/ItemStack;
public abstract size()I
public isEmpty()Z
```
