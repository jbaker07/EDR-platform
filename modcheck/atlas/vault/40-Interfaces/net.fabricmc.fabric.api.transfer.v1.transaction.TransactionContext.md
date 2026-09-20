---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: interface

```java
public abstract net.fabricmc.fabric.api.transfer.v1.transaction.Transaction openNested()
public abstract int nestingDepth()
public abstract net.fabricmc.fabric.api.transfer.v1.transaction.Transaction getOpenTransaction(int)
public abstract void addCloseCallback(net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext$CloseCallback)
public abstract void addOuterCloseCallback(net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext$OuterCloseCallback)
```
