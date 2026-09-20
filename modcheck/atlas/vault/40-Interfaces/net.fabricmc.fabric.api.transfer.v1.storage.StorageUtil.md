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
public static <T> long move(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, java.util.function.Predicate<T>, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> long simulateInsert(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, T, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> long simulateExtract(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, T, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> long simulateExtract(net.fabricmc.fabric.api.transfer.v1.storage.StorageView<T>, T, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T, S extends net.fabricmc.fabric.api.transfer.v1.storage.Storage<T> & net.fabricmc.fabric.api.transfer.v1.storage.StorageView<T>> long simulateExtract(S, T, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> net.fabricmc.fabric.api.transfer.v1.storage.base.ResourceAmount<T> extractAny(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> long insertStacking(java.util.List<? extends net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage<T>>, T, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> long tryInsertStacking(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, T, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> T findStoredResource(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>)
public static <T> T findStoredResource(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, java.util.function.Predicate<T>)
public static <T> T findExtractableResource(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> T findExtractableResource(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, java.util.function.Predicate<T>, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> net.fabricmc.fabric.api.transfer.v1.storage.base.ResourceAmount<T> findExtractableContent(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> net.fabricmc.fabric.api.transfer.v1.storage.base.ResourceAmount<T> findExtractableContent(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>, java.util.function.Predicate<T>, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public static <T> int getRedstoneSignal(net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>)
```
