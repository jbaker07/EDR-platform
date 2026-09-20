---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.model.loading.v1.ModelLoadingPlugin$Context"
module: "fabric-model-loading-api-v1"
sha256: "4889e5947bb2f9d899f72c17676b27aaef7d2594ecaaf75e89626e1eea169711"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.model.loading.v1.ModelLoadingPlugin$Context

Module: [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] -- kind: interface

```java
public abstract void registerBlockStateResolver(net.minecraft.world.level.block.Block, net.fabricmc.fabric.api.client.model.loading.v1.BlockStateResolver)
public abstract void addModel(net.fabricmc.fabric.api.client.model.loading.v1.ExtraModelKey, net.fabricmc.fabric.api.client.model.loading.v1.UnbakedExtraModel)
public abstract net.fabricmc.fabric.api.event.Event modifyModelOnLoad()
public abstract net.fabricmc.fabric.api.event.Event modifyBlockModelOnLoad()
public abstract net.fabricmc.fabric.api.event.Event modifyBlockModelBeforeBake()
public abstract net.fabricmc.fabric.api.event.Event modifyBlockModelAfterBake()
public abstract net.fabricmc.fabric.api.event.Event modifyItemModelBeforeBake()
public abstract net.fabricmc.fabric.api.event.Event modifyItemModelAfterBake()
```
