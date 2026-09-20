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
public abstract java.util.List<java.nio.file.Path> getRootPaths()
public default java.util.Optional<java.nio.file.Path> findPath(java.lang.String)
public abstract net.fabricmc.loader.api.metadata.ModOrigin getOrigin()
public abstract java.util.Optional<net.fabricmc.loader.api.ModContainer> getContainingMod()
public abstract java.util.Collection<net.fabricmc.loader.api.ModContainer> getContainedMods()
public default java.nio.file.Path getRoot()
public abstract java.nio.file.Path getRootPath()
public abstract java.nio.file.Path getPath(java.lang.String)
```
