---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.storage.base.SingleVariantItemStorage"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.storage.base.SingleVariantItemStorage

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: abstract_class

```java
public <init>(net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext)
protected abstract net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant getBlankResource()
protected abstract net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant getResource(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant)
protected abstract long getAmount(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant)
protected abstract long getCapacity(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant)
protected abstract net.fabricmc.fabric.api.transfer.v1.item.ItemVariant getUpdatedVariant(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant, long)
protected boolean canInsert(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant)
protected boolean canExtract(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant)
public boolean supportsInsertion()
public long insert(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public boolean supportsExtraction()
public long extract(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public boolean isResourceBlank()
public net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant getResource()
public long getAmount()
public long getCapacity()
public java.lang.String toString()
public long extract(java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public long insert(java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public java.lang.Object getResource()
```
