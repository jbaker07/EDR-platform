---
type: "interface"
fqcn: "net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider$RegistryEntries"
module: "fabric-data-generation-api-v1"
sha256: "2c22049cd3a75ddd59bbdcf7f8468637d18a75b6a7917420814920d05a19193c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider$RegistryEntries

Module: [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- kind: class

```java
final net.minecraft.core.HolderOwner<T> lookup
final net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> registry
final com.mojang.serialization.Codec<T> elementCodec
java.util.Map<net.minecraft.resources.ResourceKey<T>, net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider$ConditionalEntry<T>> resources
net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider$RegistryEntries(net.minecraft.core.HolderOwner<T>, net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Codec<T>)
static <T> net.fabricmc.fabric.api.datagen.v1.provider.FabricDynamicRegistryProvider$RegistryEntries<T> create(net.minecraft.core.HolderLookup$Provider, net.minecraft.resources.RegistryDataLoader$RegistryData<T>)
net.minecraft.core.Holder<T> add(net.minecraft.resources.ResourceKey<T>, T, net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition[])
```
