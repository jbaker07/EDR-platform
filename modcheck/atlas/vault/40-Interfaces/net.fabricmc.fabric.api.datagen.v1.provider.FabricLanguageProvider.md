---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: abstract_class

```java
protected final net.fabricmc.fabric.api.datagen.v1.FabricPackOutput packOutput
protected net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>)
protected net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider(net.fabricmc.fabric.api.datagen.v1.FabricPackOutput, java.lang.String, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>)
public abstract void generateTranslations(net.minecraft.core.HolderLookup$Provider, net.fabricmc.fabric.api.datagen.v1.provider.FabricLanguageProvider$TranslationBuilder)
public java.util.concurrent.CompletableFuture<?> run(net.minecraft.data.CachedOutput)
protected java.nio.file.Path getLangFilePath(java.lang.String)
public java.lang.String getName()
```
