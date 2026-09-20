---
type: "interface"
fqcn: "net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityDataRegistry"
module: "fabric-object-builder-api-v1"
sha256: "3a5f0ccef440552828d9469420547dc3cdbe3e206af73fa98d0bfcfb5b75c1ba"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.object.builder.v1.entity.FabricEntityDataRegistry

Module: [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] -- kind: class

```java
public static void register(net.minecraft.resources.Identifier, net.minecraft.network.syncher.EntityDataSerializer<?>)
public static net.minecraft.network.syncher.EntityDataSerializer<?> get(net.minecraft.resources.Identifier)
public static net.minecraft.resources.Identifier getId(net.minecraft.network.syncher.EntityDataSerializer<?>)
```
