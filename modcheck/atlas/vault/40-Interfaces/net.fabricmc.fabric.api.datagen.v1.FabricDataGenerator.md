---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.FabricDataGenerator"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.FabricDataGenerator

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: class

```java
public <init>(java.nio.file.Path, net.fabricmc.loader.api.ModContainer, boolean, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture)
public net.fabricmc.fabric.api.datagen.v1.FabricDataGenerator$Pack createPack()
public net.fabricmc.fabric.api.datagen.v1.FabricDataGenerator$Pack createBuiltinResourcePack(net.minecraft.resources.Identifier)
public net.fabricmc.loader.api.ModContainer getModContainer()
public java.lang.String getModId()
public boolean isStrictValidationEnabled()
public java.util.concurrent.CompletableFuture getRegistries()
public java.util.concurrent.CompletableFuture getWorldRegistries()
public net.minecraft.data.DataGenerator$PackGenerator getVanillaPack(boolean)
public net.minecraft.data.DataGenerator$PackGenerator getBuiltinDatapack(boolean, java.lang.String)
```
