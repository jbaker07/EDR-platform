---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.storage.base.SingleVariantStorage"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.storage.base.SingleVariantStorage

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: abstract_class

```java
public net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant variant
public long amount
public <init>()
protected abstract net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant getBlankVariant()
protected abstract long getCapacity(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant)
protected boolean canInsert(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant)
protected boolean canExtract(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant)
public long insert(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public long extract(net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public boolean isResourceBlank()
public net.fabricmc.fabric.api.transfer.v1.storage.TransferVariant getResource()
public long getAmount()
public long getCapacity()
protected net.fabricmc.fabric.api.transfer.v1.storage.base.ResourceAmount createSnapshot()
protected void readSnapshot(net.fabricmc.fabric.api.transfer.v1.storage.base.ResourceAmount)
public java.lang.String toString()
public static void readValue(net.fabricmc.fabric.api.transfer.v1.storage.base.SingleVariantStorage, com.mojang.serialization.Codec, java.util.function.Supplier, net.minecraft.world.level.storage.ValueInput)
public static void writeValue(net.fabricmc.fabric.api.transfer.v1.storage.base.SingleVariantStorage, com.mojang.serialization.Codec, net.minecraft.world.level.storage.ValueOutput)
protected void readSnapshot(java.lang.Object)
protected java.lang.Object createSnapshot()
public long extract(java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public long insert(java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public java.lang.Object getResource()
```
