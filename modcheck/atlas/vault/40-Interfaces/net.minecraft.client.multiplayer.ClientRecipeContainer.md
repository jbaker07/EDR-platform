---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientRecipeContainer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientRecipeContainer

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/item/crafting/RecipeAccess`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getSynchronizedRecipes` | `()Lnet/fabricmc/fabric/api/recipe/v1/sync/SynchronizedRecipes;` | inherited_exact | invokevirtual@8 in `ClientPacketListenerMixin.copyPreviousRecipes` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (2 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final itemSets : Ljava/util/Map;
private final stonecutterRecipes : Lnet/minecraft/world/item/crafting/SelectableRecipe$SingleInputSet;
public <init>(Ljava/util/Map;Lnet/minecraft/world/item/crafting/SelectableRecipe$SingleInputSet;)V
public propertySet(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/crafting/RecipePropertySet;
public stonecutterRecipes()Lnet/minecraft/world/item/crafting/SelectableRecipe$SingleInputSet;
```
