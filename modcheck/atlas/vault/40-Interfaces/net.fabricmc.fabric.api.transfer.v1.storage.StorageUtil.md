---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.storage.StorageUtil"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.storage.StorageUtil

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: class

```java
public static long move(net.fabricmc.fabric.api.transfer.v1.storage.Storage, net.fabricmc.fabric.api.transfer.v1.storage.Storage, java.util.function.Predicate, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static long simulateInsert(net.fabricmc.fabric.api.transfer.v1.storage.Storage, java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static long simulateExtract(net.fabricmc.fabric.api.transfer.v1.storage.Storage, java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static long simulateExtract(net.fabricmc.fabric.api.transfer.v1.storage.StorageView, java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static long simulateExtract(java.lang.Object, java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static net.fabricmc.fabric.api.transfer.v1.storage.base.ResourceAmount extractAny(net.fabricmc.fabric.api.transfer.v1.storage.Storage, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static long insertStacking(java.util.List, java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static long tryInsertStacking(net.fabricmc.fabric.api.transfer.v1.storage.Storage, java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static java.lang.Object findStoredResource(net.fabricmc.fabric.api.transfer.v1.storage.Storage)
public static java.lang.Object findStoredResource(net.fabricmc.fabric.api.transfer.v1.storage.Storage, java.util.function.Predicate)
public static java.lang.Object findExtractableResource(net.fabricmc.fabric.api.transfer.v1.storage.Storage, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static java.lang.Object findExtractableResource(net.fabricmc.fabric.api.transfer.v1.storage.Storage, java.util.function.Predicate, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static net.fabricmc.fabric.api.transfer.v1.storage.base.ResourceAmount findExtractableContent(net.fabricmc.fabric.api.transfer.v1.storage.Storage, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static net.fabricmc.fabric.api.transfer.v1.storage.base.ResourceAmount findExtractableContent(net.fabricmc.fabric.api.transfer.v1.storage.Storage, java.util.function.Predicate, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static int getRedstoneSignal(net.fabricmc.fabric.api.transfer.v1.storage.Storage)
```
