---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.model.loading.v1.PreparableModelLoadingPlugin"
module: "fabric-model-loading-api-v1"
sha256: "4889e5947bb2f9d899f72c17676b27aaef7d2594ecaaf75e89626e1eea169711"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.model.loading.v1.PreparableModelLoadingPlugin

Module: [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] -- kind: interface

```java
public static <T> void register(net.fabricmc.fabric.api.client.model.loading.v1.PreparableModelLoadingPlugin$DataLoader<T>, net.fabricmc.fabric.api.client.model.loading.v1.PreparableModelLoadingPlugin<T>)
public static java.util.List<net.fabricmc.fabric.api.client.model.loading.v1.PreparableModelLoadingPlugin$Holder<?>> getAll()
public abstract void initialize(T, net.fabricmc.fabric.api.client.model.loading.v1.ModelLoadingPlugin$Context)
```
