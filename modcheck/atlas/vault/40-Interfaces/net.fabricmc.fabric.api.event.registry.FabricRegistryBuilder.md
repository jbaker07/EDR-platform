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
public static net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder from(net.minecraft.core.WritableRegistry)
public static net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder create(net.minecraft.resources.ResourceKey)
public static net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder createDefaulted(net.minecraft.resources.ResourceKey, net.minecraft.resources.Identifier)
public static net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder create(java.lang.Class, net.minecraft.resources.Identifier)
public static net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder createDefaulted(java.lang.Class, net.minecraft.resources.Identifier, net.minecraft.resources.Identifier)
public net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder attribute(net.fabricmc.fabric.api.event.registry.RegistryAttribute)
public net.minecraft.core.WritableRegistry buildAndRegister()
```
