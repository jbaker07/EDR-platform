---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.Recipe"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.Recipe

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getSerializer` | `()Lnet/minecraft/world/item/crafting/RecipeSerializer;` | exact | invokeinterface@86 in `RecipeMapMixin.attachSerializerMap` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/item/crafting/RecipeType;` | exact | invokeinterface@16 in `SynchronizedRecipes.get` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/item/crafting/RecipeType;` | exact | invokeinterface@35 in `SynchronizedRecipesImpl.indexByType` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `matches` | `(Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecraft/world/l` | exact | invokeinterface@12 in `SynchronizedRecipes.getFirstMatch` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `matches` | `(Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecraft/world/l` | exact | invokeinterface@6 in `SynchronizedRecipesImpl.lambda$getAllMatches$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| reads | `DIRECT_CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@0 in `FabricRecipeProvider.lambda$run$1` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (5 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final KEY_CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final LIST_CODEC : Lcom/mojang/serialization/Codec;
public abstract matches(Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecraft/world/level/Level;)Z
public abstract assemble(Lnet/minecraft/world/item/crafting/RecipeInput;)Lnet/minecraft/world/item/ItemStack;
public isSpecial()Z
public abstract showNotification()Z
public abstract group()Ljava/lang/String;
public abstract getSerializer()Lnet/minecraft/world/item/crafting/RecipeSerializer;
public abstract getType()Lnet/minecraft/world/item/crafting/RecipeType;
public abstract placementInfo()Lnet/minecraft/world/item/crafting/PlacementInfo;
public display()Ljava/util/List;
public abstract recipeBookCategory()Lnet/minecraft/world/item/crafting/RecipeBookCategory;
static <clinit>()V
```
