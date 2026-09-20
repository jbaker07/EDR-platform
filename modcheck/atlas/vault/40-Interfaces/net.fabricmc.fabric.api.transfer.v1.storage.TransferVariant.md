---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: interface

```java
public abstract boolean isBlank()
public abstract O getObject()
public abstract net.minecraft.core.component.DataComponentPatch getComponentsPatch()
public abstract net.minecraft.core.component.DataComponentMap getComponents()
public default boolean hasComponents()
public default boolean componentsMatch(net.minecraft.core.component.DataComponentPatch)
public default boolean isOf(O)
public default net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant<O> withComponents(net.minecraft.core.component.DataComponentPatch)
```
