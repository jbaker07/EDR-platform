---
type: "interface"
fqcn: "net.fabricmc.loader.api.ObjectShare"
module: "fabric-loader"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.loader.api.ObjectShare

fabric-loader 0.19.5 -- kind: interface

```java
public abstract java.lang.Object get(java.lang.String)
public abstract void whenAvailable(java.lang.String, java.util.function.BiConsumer)
public abstract java.lang.Object put(java.lang.String, java.lang.Object)
public abstract java.lang.Object putIfAbsent(java.lang.String, java.lang.Object)
public abstract java.lang.Object remove(java.lang.String)
```
