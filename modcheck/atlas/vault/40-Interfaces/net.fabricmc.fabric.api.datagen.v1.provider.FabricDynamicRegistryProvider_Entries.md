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
public net.minecraft.core.HolderLookup$Provider getLookups()
public net.minecraft.core.HolderGetter getLookup(net.minecraft.resources.ResourceKey)
public net.minecraft.core.HolderGetter placedFeatures()
public net.minecraft.core.HolderGetter configuredCarvers()
public net.minecraft.core.Holder ref(net.minecraft.resources.ResourceKey)
public net.minecraft.core.Holder add(net.minecraft.resources.ResourceKey, java.lang.Object)
public net.minecraft.core.Holder add(net.minecraft.resources.ResourceKey, java.lang.Object, net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition[])
public void add(net.minecraft.core.Holder$Reference)
public void add(net.minecraft.core.Holder$Reference, net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition[])
public net.minecraft.core.Holder add(net.minecraft.core.HolderLookup$RegistryLookup, net.minecraft.resources.ResourceKey)
public net.minecraft.core.Holder add(net.minecraft.core.HolderLookup$RegistryLookup, net.minecraft.resources.ResourceKey, net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition[])
public java.util.List addAll(net.minecraft.core.HolderLookup$RegistryLookup)
```
