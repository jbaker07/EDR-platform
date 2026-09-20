---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableMesh"
module: "fabric-renderer-api-v1"
sha256: "2e4aaeb20f8615e8176f30bc82616cee25f8727323bfef7a70d5a6555ae8e9ee"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableMesh

Module: [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] -- kind: interface

```java
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadEmitter emitter()
public abstract void forEachMutable(java.util.function.Consumer<? super net.fabricmc.fabric.api.client.renderer.v1.mesh.MutableQuadView>)
public abstract net.fabricmc.fabric.api.client.renderer.v1.mesh.Mesh immutableCopy()
public abstract void clear()
```
