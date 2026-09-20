---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider$ItemTagsProvider"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider$ItemTagsProvider

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: abstract_class

```java
public <init>(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.util.concurrent.CompletableFuture, net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider$BlockTagsProvider)
public <init>(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.util.concurrent.CompletableFuture)
public void copy(net.minecraft.tags.TagKey, net.minecraft.tags.TagKey)
public void copy(net.minecraft.tags.BlockItemTagId)
protected net.minecraft.data.tags.BlockItemTagAppender builder(net.minecraft.tags.TagKey)
protected net.minecraft.data.tags.TagAppender builder(net.minecraft.tags.TagKey)
```
