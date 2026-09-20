---
type: "interface"
fqcn: "net.fabricmc.loader.api.SemanticVersion"
module: "fabric-loader"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.loader.api.SemanticVersion

fabric-loader 0.19.5 -- kind: interface

```java
public static final int COMPONENT_WILDCARD
public abstract int getVersionComponentCount()
public abstract int getVersionComponent(int)
public abstract java.util.Optional getPrereleaseKey()
public abstract java.util.Optional getBuildKey()
public abstract boolean hasWildcard()
public int compareTo(net.fabricmc.loader.api.SemanticVersion)
public static net.fabricmc.loader.api.SemanticVersion parse(java.lang.String)
```
