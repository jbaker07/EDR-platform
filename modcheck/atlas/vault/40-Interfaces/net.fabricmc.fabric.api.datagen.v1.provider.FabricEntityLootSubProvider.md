---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricEntityLootSubProvider"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricEntityLootSubProvider

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: abstract_class

```java
protected net.fabricmc.fabric.api.datagen.v1.provider.FabricEntityLootSubProvider(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>)
public abstract void generate()
public void excludeFromStrictValidation(net.minecraft.world.entity.EntityType<?>)
public void generate(java.util.function.BiConsumer<net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>, net.minecraft.world.level.storage.loot.LootTable$Builder>)
public java.util.concurrent.CompletableFuture<?> run(net.minecraft.data.CachedOutput)
public void run()
public java.lang.String getName()
```
