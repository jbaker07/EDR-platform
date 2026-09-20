---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.storage.Storage"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.storage.Storage

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: interface

```java
public static <T> net.fabricmc.fabric.api.transfer.v1.storage.Storage<T> empty()
public default boolean supportsInsertion()
public abstract long insert(T, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public default boolean supportsExtraction()
public abstract long extract(T, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public abstract java.util.Iterator<net.fabricmc.fabric.api.transfer.v1.storage.StorageView<T>> iterator()
public default java.util.Iterator<net.fabricmc.fabric.api.transfer.v1.storage.StorageView<T>> nonEmptyIterator()
public default java.lang.Iterable<net.fabricmc.fabric.api.transfer.v1.storage.StorageView<T>> nonEmptyViews()
public default long getVersion()
public static <T> java.lang.Class<net.fabricmc.fabric.api.transfer.v1.storage.Storage<T>> asClass()
```
