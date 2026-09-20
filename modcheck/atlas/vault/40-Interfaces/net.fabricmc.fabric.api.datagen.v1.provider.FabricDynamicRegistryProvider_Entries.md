---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider$Entries"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider$Entries

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: class

```java
net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider$Entries(net.minecraft.core.HolderLookup$Provider, java.lang.String)
public net.minecraft.core.HolderLookup$Provider getLookups()
public <T> net.minecraft.core.HolderGetter<T> getLookup(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>)
public net.minecraft.core.HolderGetter<net.minecraft.world.level.levelgen.placement.PlacedFeature> placedFeatures()
public net.minecraft.core.HolderGetter<net.minecraft.world.level.levelgen.carver.WorldCarver> configuredCarvers()
public <T> net.minecraft.core.Holder<T> ref(net.minecraft.resources.ResourceKey<T>)
public <T> net.minecraft.core.Holder<T> add(net.minecraft.resources.ResourceKey<T>, T)
public <T> net.minecraft.core.Holder<T> add(net.minecraft.resources.ResourceKey<T>, T, net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition...)
public <T> void add(net.minecraft.core.Holder$Reference<T>)
public <T> void add(net.minecraft.core.Holder$Reference<T>, net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition...)
public <T> net.minecraft.core.Holder<T> add(net.minecraft.core.HolderLookup$RegistryLookup<T>, net.minecraft.resources.ResourceKey<T>)
public <T> net.minecraft.core.Holder<T> add(net.minecraft.core.HolderLookup$RegistryLookup<T>, net.minecraft.resources.ResourceKey<T>, net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition...)
public <T> java.util.List<net.minecraft.core.Holder<T>> addAll(net.minecraft.core.HolderLookup$RegistryLookup<T>)
<T> net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider$RegistryEntries<T> getQueuedEntries(net.minecraft.resources.ResourceKey<T>)
```
