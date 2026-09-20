---
type: "interface"
fqcn: "net.fabricmc.loader.api.MappingResolver"
module: "fabric-loader"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.loader.api.MappingResolver

fabric-loader 0.19.5 -- kind: interface

```java
public abstract java.util.Collection getNamespaces()
public abstract java.lang.String getCurrentRuntimeNamespace()
public abstract java.lang.String mapClassName(java.lang.String, java.lang.String)
public abstract java.lang.String unmapClassName(java.lang.String, java.lang.String)
public abstract java.lang.String mapFieldName(java.lang.String, java.lang.String, java.lang.String, java.lang.String)
public abstract java.lang.String mapMethodName(java.lang.String, java.lang.String, java.lang.String, java.lang.String)
```
