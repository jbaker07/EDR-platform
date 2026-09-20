---
type: "interface"
fqcn: "net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder"
module: "fabric-registry-sync-v0"
sha256: "039a5c3dee042ff156c52c4bec64a384e766e830a37cc9f6bf68f1e7dc3646b1"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder

Module: [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] -- kind: class

```java
public static <T, R extends net.minecraft.core.WritableRegistry<T>> net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder<T, R> from(R)
public static <T> net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder<T, net.minecraft.core.MappedRegistry<T>> create(net.minecraft.resources.ResourceKey<net.minecraft.core.Registry<T>>)
public static <T> net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder<T, net.minecraft.core.DefaultedMappedRegistry<T>> createDefaulted(net.minecraft.resources.ResourceKey<net.minecraft.core.Registry<T>>, net.minecraft.resources.Identifier)
public static <T> net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder<T, net.minecraft.core.MappedRegistry<T>> create(java.lang.Class<T>, net.minecraft.resources.Identifier)
public static <T> net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder<T, net.minecraft.core.DefaultedMappedRegistry<T>> createDefaulted(java.lang.Class<T>, net.minecraft.resources.Identifier, net.minecraft.resources.Identifier)
public net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder<T, R> attribute(net.fabricmc.fabric.api.event.registry.RegistryAttribute)
public R buildAndRegister()
```
