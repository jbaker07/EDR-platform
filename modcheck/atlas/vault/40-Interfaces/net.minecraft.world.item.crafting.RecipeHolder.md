---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.RecipeHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.RecipeHolder

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/item/crafti` | exact | invokespecial@103 in `ClientboundRecipeSyncPayload$Entry.read` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@72 in `ClientboundRecipeSyncPayload$Entry.write` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@32 in `SynchronizedRecipesImpl.indexByKey` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `RecipeSyncImplClient.lambda$onRecipeSyncPacket$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/world/item/crafting/Recipe;` | exact | invokevirtual@7 in `SynchronizedRecipes.getFirstMatch` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/world/item/crafting/Recipe;` | exact | invokevirtual@13 in `SynchronizedRecipes.get` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/world/item/crafting/Recipe;` | exact | invokevirtual@82 in `ClientboundRecipeSyncPayload$Entry.write` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/world/item/crafting/Recipe;` | exact | invokevirtual@32 in `SynchronizedRecipesImpl.indexByType` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/world/item/crafting/Recipe;` | exact | invokevirtual@1 in `SynchronizedRecipesImpl.lambda$getAllMatches$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/world/item/crafting/Recipe;` | exact | invokevirtual@83 in `RecipeMapMixin.attachSerializerMap` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (3 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Lnet/minecraft/resources/ResourceKey;
private final value : Lnet/minecraft/world/item/crafting/Recipe;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/item/crafting/Recipe;)V
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
public id()Lnet/minecraft/resources/ResourceKey;
public value()Lnet/minecraft/world/item/crafting/Recipe;
static <clinit>()V
```
