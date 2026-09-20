---
type: "interface"
fqcn: "net.fabricmc.fabric.api.resource.SimpleResourceReloadListener"
module: "fabric-resource-loader-v0"
sha256: "18afa6466d69ff68e1eeb74088d210a55004d11380ec14aad6ea66faa7da1922"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.resource.SimpleResourceReloadListener

Module: [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] -- kind: interface

```java
public default java.util.concurrent.CompletableFuture<java.lang.Void> reload(net.minecraft.server.packs.resources.PreparableReloadListener$SharedState, java.util.concurrent.Executor, net.minecraft.server.packs.resources.PreparableReloadListener$PreparationBarrier, java.util.concurrent.Executor)
public abstract java.util.concurrent.CompletableFuture<T> load(net.minecraft.server.packs.resources.ResourceManager, java.util.concurrent.Executor)
public abstract java.util.concurrent.CompletableFuture<java.lang.Void> apply(T, net.minecraft.server.packs.resources.ResourceManager, java.util.concurrent.Executor)
```
