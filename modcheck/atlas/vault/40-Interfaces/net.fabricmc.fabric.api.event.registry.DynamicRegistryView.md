---
type: "interface"
fqcn: "net.fabricmc.fabric.api.event.registry.DynamicRegistryView"
module: "fabric-registry-sync-v0"
sha256: "039a5c3dee042ff156c52c4bec64a384e766e830a37cc9f6bf68f1e7dc3646b1"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.registry.DynamicRegistryView

Module: [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] -- kind: interface

```java
public abstract net.minecraft.core.RegistryAccess asRegistryAccess()
public abstract java.util.stream.Stream<net.minecraft.core.Registry<?>> stream()
public abstract <T> java.util.Optional<net.minecraft.core.Registry<T>> getOptional(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends T>>)
public abstract <T> void registerEntryAdded(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends T>>, net.fabricmc.fabric.api.event.registry.RegistryEntryAddedCallback<T>)
```
