---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.storage.base.FilteringStorage"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.storage.base.FilteringStorage

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: abstract_class

```java
protected final java.util.function.Supplier backingStorage
public static net.fabricmc.fabric.api.transfer.v1.storage.Storage insertOnlyOf(net.fabricmc.fabric.api.transfer.v1.storage.Storage)
public static net.fabricmc.fabric.api.transfer.v1.storage.Storage extractOnlyOf(net.fabricmc.fabric.api.transfer.v1.storage.Storage)
public static net.fabricmc.fabric.api.transfer.v1.storage.Storage readOnlyOf(net.fabricmc.fabric.api.transfer.v1.storage.Storage)
public static net.fabricmc.fabric.api.transfer.v1.storage.Storage of(net.fabricmc.fabric.api.transfer.v1.storage.Storage, boolean, boolean)
public <init>(net.fabricmc.fabric.api.transfer.v1.storage.Storage)
public <init>(java.util.function.Supplier)
protected boolean canInsert(java.lang.Object)
protected boolean canExtract(java.lang.Object)
public boolean supportsInsertion()
public long insert(java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public boolean supportsExtraction()
public long extract(java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public java.util.Iterator iterator()
public long getVersion()
public java.lang.String toString()
```
