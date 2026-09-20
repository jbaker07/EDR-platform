---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.item.ContainerStorage"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.item.ContainerStorage

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: interface

```java
public static net.fabricmc.fabric.api.transfer.v1.item.ContainerStorage of(net.minecraft.world.Container, net.minecraft.core.Direction)
public abstract java.util.List<net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage<net.fabricmc.fabric.api.transfer.v1.item.ItemVariant>> getSlots()
public default int getSlotCount()
public default net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage<net.fabricmc.fabric.api.transfer.v1.item.ItemVariant> getSlot(int)
```
