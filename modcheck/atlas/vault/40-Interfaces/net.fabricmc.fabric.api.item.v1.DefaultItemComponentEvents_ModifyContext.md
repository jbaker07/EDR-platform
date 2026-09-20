---
type: "interface"
fqcn: "net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents$ModifyContext"
module: "fabric-item-api-v1"
sha256: "aff8cffc3d6da5475f21060e45cd28307676974674c4e10504c0fcc89c7746c4"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents$ModifyContext

Module: [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] -- kind: interface

```java
public abstract void modify(java.util.function.Predicate, net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents$ModifyConsumer)
public void modify(net.minecraft.world.item.Item, net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents$ModifyConsumer)
public void modify(java.util.Collection, net.fabricmc.fabric.api.item.v1.DefaultItemComponentEvents$ModifyConsumer)
public void modify(java.util.function.Predicate, java.util.function.BiConsumer)
public void modify(net.minecraft.world.item.Item, java.util.function.Consumer)
public void modify(java.util.Collection, java.util.function.BiConsumer)
```
