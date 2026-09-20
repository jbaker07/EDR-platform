---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricRecipeProvider$1"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricRecipeProvider$1

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: class

```java
final net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition[] val$conditions
final net.minecraft.data.recipes.RecipeOutput val$output
net.fabricmc.fabric.api.datagen.v1.provider.FabricRecipeProvider$1()
public void accept(net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>, net.minecraft.world.item.crafting.Recipe<?>, net.minecraft.advancements.AdvancementHolder)
public net.minecraft.advancements.Advancement$Builder advancement()
public void includeRootAdvancement()
public net.minecraft.resources.Identifier getRecipeIdentifier(net.minecraft.resources.Identifier)
public <S> net.minecraft.core.HolderGetter<S> lookup(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends S>>)
public <S> java.util.stream.Stream<net.minecraft.core.Holder$Reference<S>> listContextElements(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends S>>)
```
