---
type: "interface"
fqcn: "net.fabricmc.fabric.api.resource.ModResourcePack"
module: "fabric-resource-loader-v0"
sha256: "18afa6466d69ff68e1eeb74088d210a55004d11380ec14aad6ea66faa7da1922"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.resource.ModResourcePack

Module: [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] -- kind: interface

```java
public abstract net.fabricmc.loader.api.metadata.ModMetadata getFabricModMetadata()
public abstract net.fabricmc.fabric.api.resource.ModResourcePack createOverlay(java.lang.String)
public default net.fabricmc.fabric.api.resource.v1.pack.ModPackResources createOverlay(java.lang.String)
```
