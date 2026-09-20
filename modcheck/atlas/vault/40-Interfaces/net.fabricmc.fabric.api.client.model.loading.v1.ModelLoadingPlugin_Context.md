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
public abstract <T> void addModel(net.fabricmc.fabric.api.client.model.loading.v1.ExtraModelKey<T>, net.fabricmc.fabric.api.client.model.loading.v1.UnbakedExtraModel<T>)
public abstract net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.model.loading.v1.ModelModifier$OnLoad> modifyModelOnLoad()
public abstract net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.model.loading.v1.ModelModifier$OnLoadBlock> modifyBlockModelOnLoad()
public abstract net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.model.loading.v1.ModelModifier$BeforeBakeBlock> modifyBlockModelBeforeBake()
public abstract net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.model.loading.v1.ModelModifier$AfterBakeBlock> modifyBlockModelAfterBake()
public abstract net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.model.loading.v1.ModelModifier$BeforeBakeItem> modifyItemModelBeforeBake()
public abstract net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.model.loading.v1.ModelModifier$AfterBakeItem> modifyItemModelAfterBake()
```
