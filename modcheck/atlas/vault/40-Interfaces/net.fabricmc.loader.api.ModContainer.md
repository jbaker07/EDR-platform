---
type: "interface"
fqcn: "net.fabricmc.loader.api.ModContainer"
module: "fabric-loader"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.loader.api.ModContainer

fabric-loader 0.19.5 -- kind: interface

```java
public abstract net.fabricmc.loader.api.metadata.ModMetadata getMetadata()
public abstract java.util.List getRootPaths()
public java.util.Optional findPath(java.lang.String)
public abstract net.fabricmc.loader.api.metadata.ModOrigin getOrigin()
public abstract java.util.Optional getContainingMod()
public abstract java.util.Collection getContainedMods()
public java.nio.file.Path getRoot()
public abstract java.nio.file.Path getRootPath()
public abstract java.nio.file.Path getPath(java.lang.String)
```
