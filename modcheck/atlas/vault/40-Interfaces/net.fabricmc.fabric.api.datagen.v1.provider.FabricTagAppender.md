---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricTagAppender"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricTagAppender

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: interface

```java
public net.minecraft.data.tags.TagAppender setReplace(boolean)
public net.minecraft.data.tags.TagAppender forceAddTag(net.minecraft.tags.TagKey)
public net.minecraft.data.tags.TagAppender remove(net.minecraft.resources.ResourceKey)
public net.minecraft.data.tags.TagAppender remove(net.minecraft.resources.ResourceKey[])
public net.minecraft.data.tags.TagAppender removeAll(java.util.Collection)
public net.minecraft.data.tags.TagAppender removeAll(java.util.stream.Stream)
public net.minecraft.data.tags.TagAppender removeTag(net.minecraft.tags.TagKey)
public net.minecraft.tags.TagBuilder getBuilder()
```
