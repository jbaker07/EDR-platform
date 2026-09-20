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
public static java.util.List getAllDynamicRegistries()
public static java.util.List getWorldRegistries()
public static java.util.List getReloadableRegistries()
public static void register(net.minecraft.resources.ResourceKey, com.mojang.serialization.Codec)
public static void registerSynced(net.minecraft.resources.ResourceKey, com.mojang.serialization.Codec, net.fabricmc.fabric.api.event.registry.DynamicRegistries$SyncOption[])
public static void registerSynced(net.minecraft.resources.ResourceKey, com.mojang.serialization.Codec, com.mojang.serialization.Codec, net.fabricmc.fabric.api.event.registry.DynamicRegistries$SyncOption[])
public static void registerReloadable(net.minecraft.resources.ResourceKey, com.mojang.serialization.Codec)
```
