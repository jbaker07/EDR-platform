---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.item.base.SingleStackStorage"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.item.base.SingleStackStorage

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: abstract_class

```java
public net.fabricmc.fabric.api.transfer.v1.item.base.SingleStackStorage()
protected abstract net.minecraft.world.item.ItemStack getStack()
protected abstract void setStack(net.minecraft.world.item.ItemStack)
protected boolean canInsert(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant)
protected boolean canExtract(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant)
protected int getCapacity(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant)
public boolean isResourceBlank()
public net.fabricmc.fabric.api.transfer.v1.item.ItemVariant getResource()
public long getAmount()
public long getCapacity()
public long insert(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public long extract(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
protected net.minecraft.world.item.ItemStack createSnapshot()
protected void readSnapshot(net.minecraft.world.item.ItemStack)
public java.lang.String toString()
protected void readSnapshot(java.lang.Object)
protected java.lang.Object createSnapshot()
public long extract(java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public long insert(java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public java.lang.Object getResource()
```
