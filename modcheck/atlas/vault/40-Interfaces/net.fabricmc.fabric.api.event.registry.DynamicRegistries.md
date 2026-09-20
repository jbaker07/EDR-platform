---
type: "interface"
fqcn: "net.fabricmc.fabric.api.event.registry.DynamicRegistries"
module: "fabric-registry-sync-v0"
sha256: "039a5c3dee042ff156c52c4bec64a384e766e830a37cc9f6bf68f1e7dc3646b1"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.registry.DynamicRegistries

Module: [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] -- kind: class

```java
public static java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>> getAllDynamicRegistries()
public static java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>> getWorldRegistries()
public static java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>> getReloadableRegistries()
public static <T> void register(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Codec<T>)
public static <T> void registerSynced(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Codec<T>, net.fabricmc.fabric.api.event.registry.DynamicRegistries$SyncOption...)
public static <T> void registerSynced(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Codec<T>, com.mojang.serialization.Codec<T>, net.fabricmc.fabric.api.event.registry.DynamicRegistries$SyncOption...)
public static <T> void registerReloadable(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>>, com.mojang.serialization.Codec<T>)
```
