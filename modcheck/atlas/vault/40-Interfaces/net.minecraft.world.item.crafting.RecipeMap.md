---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.RecipeMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.RecipeMap

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `byType` | `(Lnet/minecraft/world/item/crafting/RecipeType;)Ljava/util/Collection;` | exact | invokevirtual@5 in `RecipeManagerMixin.getAllOfType` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getRecipesFor` | `(Lnet/minecraft/world/item/crafting/RecipeType;Lnet/minecraft/world/it` | exact | invokevirtual@7 in `RecipeManagerMixin.getAllMatches` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `values` | `()Ljava/util/Collection;` | exact | invokevirtual@2 in `SynchronizedRecipesImpl.<init>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `values` | `()Ljava/util/Collection;` | exact | invokevirtual@9 in `SynchronizedRecipesImpl.<init>` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `values` | `()Ljava/util/Collection;` | exact | invokevirtual@53 in `RecipeMapMixin.attachSerializerMap` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| injects_into | `create` | `(Lnet/minecraft/core/HolderLookup;)Lnet/minecraft/world/item/crafting/` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (3 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/world/item/crafting/RecipeMap;
private final byType : Lcom/google/common/collect/Multimap;
private final byKey : Ljava/util/Map;
private <init>(Lcom/google/common/collect/Multimap;Ljava/util/Map;)V
public static create(Lnet/minecraft/core/HolderLookup;)Lnet/minecraft/world/item/crafting/RecipeMap;
public byType(Lnet/minecraft/world/item/crafting/RecipeType;)Ljava/util/Collection;
public values()Ljava/util/Collection;
public byKey(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/item/crafting/RecipeHolder;
public getRecipesFor(Lnet/minecraft/world/item/crafting/RecipeType;Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecraft/world/level/Level;)Ljava/util/stream/Stream;
private static synthetic lambda$getRecipesFor$0(Lnet/minecraft/world/item/crafting/RecipeInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/crafting/RecipeHolder;)Z
private static synthetic lambda$create$0(Lcom/google/common/collect/ImmutableMultimap$Builder;Lcom/google/common/collect/ImmutableMap$Builder;Lnet/minecraft/core/Holder$Reference;)V
static <clinit>()V
```
