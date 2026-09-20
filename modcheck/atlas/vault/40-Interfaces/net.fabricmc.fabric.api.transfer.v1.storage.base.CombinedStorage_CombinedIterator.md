---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.storage.base.CombinedStorage$CombinedIterator"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.storage.base.CombinedStorage$CombinedIterator

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: class

```java
final java.util.Iterator<S> partIterator
java.util.Iterator<? extends net.fabricmc.fabric.api.transfer.v1.storage.StorageView<T>> currentPartIterator
final net.fabricmc.fabric.api.transfer.v1.storage.base.CombinedStorage this$0
net.fabricmc.fabric.api.transfer.v1.storage.base.CombinedStorage$CombinedIterator(net.fabricmc.fabric.api.transfer.v1.storage.base.CombinedStorage)
public boolean hasNext()
public net.fabricmc.fabric.api.transfer.v1.storage.StorageView<T> next()
public java.lang.Object next()
```
