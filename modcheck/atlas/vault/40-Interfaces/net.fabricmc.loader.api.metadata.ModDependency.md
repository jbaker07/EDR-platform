---
type: "interface"
fqcn: "net.fabricmc.loader.api.metadata.ModDependency"
module: "fabric-loader"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.loader.api.metadata.ModDependency

fabric-loader 0.19.5 -- kind: interface

```java
public abstract net.fabricmc.loader.api.metadata.ModDependency$Kind getKind()
public abstract java.lang.String getModId()
public abstract boolean matches(net.fabricmc.loader.api.Version)
public abstract java.util.Collection getVersionRequirements()
public abstract java.util.List getVersionIntervals()
```
