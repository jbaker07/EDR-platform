---
type: "interface"
fqcn: "net.minecraft.data.recipes.RecipeOutput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.recipes.RecipeOutput

System: [[20-Systems/net.minecraft.data.recipes|net.minecraft.data.recipes]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/data/worldgen/BootstrapContextAccess`, `net/fabricmc/fabric/api/datagen/v1/recipe/FabricRecipeOutput`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `accept` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/item/crafti` | exact | invokeinterface@30 in `FabricRecipeProvider$1.accept` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `advancement` | `()Lnet/minecraft/advancements/Advancement$Builder;` | exact | invokeinterface@4 in `FabricRecipeProvider$1.advancement` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRecipeIdentifier` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identif` | inherited_exact | invokeinterface@5 in `FabricRecipeProvider$1.getRecipeIdentifier` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRecipeIdentifier` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identif` | inherited_exact | invokeinterface@9 in `AllCraftingRecipeJsonBuildersMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRecipeIdentifier` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identif` | inherited_exact | invokeinterface@9 in `SmithingTransformRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRecipeIdentifier` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identif` | inherited_exact | invokeinterface@9 in `SmithingTrimRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRecipeIdentifier` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identif` | inherited_exact | invokeinterface@9 in `SpecialRecipeBuilderMixin.modifyRecipeKey` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `listContextElements` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@5 in `FabricRecipeProvider$1.listContextElements` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderGette` | inherited_exact | invokeinterface@5 in `FabricRecipeProvider$1.lookup` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (0 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract accept(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/item/crafting/Recipe;Lnet/minecraft/advancements/AdvancementHolder;)V
public abstract advancement()Lnet/minecraft/advancements/Advancement$Builder;
```
