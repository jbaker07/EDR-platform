---
type: "interface"
fqcn: "net.minecraft.data.recipes.RecipeProvider$1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.RecipeProvider$1

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/data/recipes/RecipeOutput`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `val$recipeOutput` | `Lnet/minecraft/data/worldgen/BootstrapContext;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | declared |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic val$recipeOutput : Lnet/minecraft/data/worldgen/BootstrapContext;
final synthetic val$advancementOutput : Lnet/minecraft/data/worldgen/BootstrapContext;
 <init>(Lnet/minecraft/data/recipes/RecipeProvider;Lnet/minecraft/data/worldgen/BootstrapContext;Lnet/minecraft/data/worldgen/BootstrapContext;)V
public accept(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/item/crafting/Recipe;Lnet/minecraft/advancements/AdvancementHolder;)V
private acceptAdvancement(Lnet/minecraft/advancements/AdvancementHolder;)V
public advancement()Lnet/minecraft/advancements/Advancement$Builder;
public lookup(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderGetter;
public listContextElements(Lnet/minecraft/resources/ResourceKey;)Ljava/util/stream/Stream;
```
