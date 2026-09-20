---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: abstract_class

```java
public net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>)
protected abstract void addTags(net.minecraft.core.HolderLookup$Provider)
protected net.minecraft.data.tags.TagAppender<T> builder(net.minecraft.tags.TagKey<T>)
protected net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider<T>.AliasGroupBuilder aliasGroup(net.minecraft.resources.Identifier)
protected net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider<T>.AliasGroupBuilder aliasGroup(java.lang.String)
public java.util.Map<net.minecraft.resources.Identifier, net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider<T>.AliasGroupBuilder> getAliasGroupBuilders()
static net.minecraft.tags.TagBuilder access$000(net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider, net.minecraft.tags.TagKey)
static net.minecraft.resources.ResourceKey access$100(net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider)
static net.minecraft.resources.ResourceKey access$200(net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider)
static net.minecraft.resources.ResourceKey access$300(net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider)
static net.minecraft.resources.ResourceKey access$400(net.fabricmc.fabric.api.datagen.v1.provider.FabricTagsProvider)
```
