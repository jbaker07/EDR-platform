---
type: "interface"
fqcn: "net.fabricmc.fabric.api.event.registry.RegistryAttributeHolder"
module: "fabric-registry-sync-v0"
sha256: "039a5c3dee042ff156c52c4bec64a384e766e830a37cc9f6bf68f1e7dc3646b1"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.registry.RegistryAttributeHolder

Module: [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] -- kind: interface

```java
public static net.fabricmc.fabric.api.event.registry.RegistryAttributeHolder get(net.minecraft.resources.ResourceKey<?>)
public static net.fabricmc.fabric.api.event.registry.RegistryAttributeHolder get(net.minecraft.core.Registry<?>)
public abstract net.fabricmc.fabric.api.event.registry.RegistryAttributeHolder addAttribute(net.fabricmc.fabric.api.event.registry.RegistryAttribute)
public abstract boolean hasAttribute(net.fabricmc.fabric.api.event.registry.RegistryAttribute)
```
