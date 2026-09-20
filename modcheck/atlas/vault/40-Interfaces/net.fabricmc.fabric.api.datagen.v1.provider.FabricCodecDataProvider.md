---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricCodecDataProvider"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricCodecDataProvider

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: abstract_class

```java
protected <init>(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.util.concurrent.CompletableFuture, net.minecraft.data.PackOutput$Target, java.lang.String, com.mojang.serialization.Codec)
protected <init>(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.util.concurrent.CompletableFuture, net.minecraft.data.PackOutput$Target, java.lang.String, com.mojang.serialization.Codec, java.lang.String)
protected <init>(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.util.concurrent.CompletableFuture, net.minecraft.resources.ResourceKey, com.mojang.serialization.Codec)
protected <init>(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.util.concurrent.CompletableFuture, net.minecraft.resources.ResourceKey, com.mojang.serialization.Codec, java.lang.String)
public java.util.concurrent.CompletableFuture run(net.minecraft.data.CachedOutput)
protected abstract void configure(java.util.function.BiConsumer, net.minecraft.core.HolderLookup$Provider)
```
