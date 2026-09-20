---
type: "interface"
fqcn: "net.fabricmc.fabric.api.resource.v1.reloader.SimpleReloadListener"
module: "fabric-resource-loader-v1"
sha256: "2d2fb907728c895640c1261ff3dd115b5087913d88d1ba1d9f3a8a4cfd1ca87e"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.resource.v1.reloader.SimpleReloadListener

Module: [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] -- kind: abstract_class

```java
public <init>()
public final java.util.concurrent.CompletableFuture reload(net.minecraft.server.packs.resources.PreparableReloadListener$SharedState, java.util.concurrent.Executor, net.minecraft.server.packs.resources.PreparableReloadListener$PreparationBarrier, java.util.concurrent.Executor)
protected abstract java.lang.Object prepare(net.minecraft.server.packs.resources.PreparableReloadListener$SharedState)
protected abstract void apply(java.lang.Object, net.minecraft.server.packs.resources.PreparableReloadListener$SharedState)
```
