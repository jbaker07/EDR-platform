---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricRecipeProvider"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricRecipeProvider

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: abstract_class

```java
protected final net.fabricmc.fabric.api.datagen.v1.FabricPackOutput output
public <init>(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.util.concurrent.CompletableFuture)
protected abstract net.minecraft.data.recipes.RecipeProvider createRecipeProvider(net.minecraft.core.HolderLookup$Provider, net.minecraft.data.worldgen.BootstrapContext, net.minecraft.data.worldgen.BootstrapContext)
protected net.minecraft.data.recipes.RecipeOutput withConditions(net.minecraft.data.recipes.RecipeOutput, net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition[])
public java.util.concurrent.CompletableFuture run(net.minecraft.data.CachedOutput)
protected net.minecraft.resources.Identifier getRecipeIdentifier(net.minecraft.resources.Identifier)
public java.lang.String getName()
```
