---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.RecipeSerializer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.RecipeSerializer

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `streamCodec` | `()Lnet/minecraft/network/codec/StreamCodec;` | exact | invokevirtual@8 in `RecipeSynchronization.synchronizeRecipeSerializer` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `streamCodec` | `()Lnet/minecraft/network/codec/StreamCodec;` | exact | invokevirtual@79 in `ClientboundRecipeSyncPayload$Entry.read` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `streamCodec` | `()Lnet/minecraft/network/codec/StreamCodec;` | exact | invokevirtual@35 in `ClientboundRecipeSyncPayload$Entry.write` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final codec : Lcom/mojang/serialization/MapCodec;
private final streamCodec : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lcom/mojang/serialization/MapCodec;Lnet/minecraft/network/codec/StreamCodec;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public codec()Lcom/mojang/serialization/MapCodec;
public streamCodec()Lnet/minecraft/network/codec/StreamCodec;
```
