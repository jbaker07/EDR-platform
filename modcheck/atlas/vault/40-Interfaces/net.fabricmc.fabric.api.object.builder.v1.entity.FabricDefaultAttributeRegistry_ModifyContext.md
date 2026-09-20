---
type: "interface"
fqcn: "net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry$ModifyContext"
module: "fabric-object-builder-api-v1"
sha256: "3a5f0ccef440552828d9469420547dc3cdbe3e206af73fa98d0bfcfb5b75c1ba"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry$ModifyContext

Module: [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] -- kind: interface

```java
public abstract void modify(java.util.function.Predicate, net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry$ModifyConsumer)
public void modify(net.minecraft.world.entity.EntityType, net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry$ModifyConsumer)
public void modify(java.util.Collection, net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry$ModifyConsumer)
public void modifyAll(net.fabricmc.fabric.api.object.builder.v1.entity.FabricDefaultAttributeRegistry$ModifyConsumer)
```
