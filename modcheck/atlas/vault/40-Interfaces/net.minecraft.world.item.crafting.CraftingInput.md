---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.CraftingInput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.CraftingInput

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/item/crafting/RecipeInput`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getItem` | `(I)Lnet/minecraft/world/item/ItemStack;` | exact | invokevirtual@35 in `ShapelessRecipeMixin.customIngredientMatch` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `ingredientCount` | `()I` | exact | invokevirtual@12 in `ShapelessRecipeMixin.customIngredientMatch` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `size` | `()I` | exact | invokevirtual@26 in `ShapelessRecipeMixin.customIngredientMatch` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (6 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/world/item/crafting/CraftingInput;
private final width : I
private final height : I
private final items : Ljava/util/List;
private final stackedContents : Lnet/minecraft/world/entity/player/StackedItemContents;
private final ingredientCount : I
private <init>(IILjava/util/List;)V
public static of(IILjava/util/List;)Lnet/minecraft/world/item/crafting/CraftingInput;
public static ofPositioned(IILjava/util/List;)Lnet/minecraft/world/item/crafting/CraftingInput$Positioned;
public getItem(I)Lnet/minecraft/world/item/ItemStack;
public getItem(II)Lnet/minecraft/world/item/ItemStack;
public size()I
public isEmpty()Z
public stackedContents()Lnet/minecraft/world/entity/player/StackedItemContents;
public items()Ljava/util/List;
public ingredientCount()I
public width()I
public height()I
public equals(Ljava/lang/Object;)Z
public hashCode()I
static <clinit>()V
```
