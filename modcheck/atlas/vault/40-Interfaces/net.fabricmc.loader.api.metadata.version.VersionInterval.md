---
type: "interface"
fqcn: "net.fabricmc.loader.api.metadata.version.VersionInterval"
module: "fabric-loader"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.loader.api.metadata.version.VersionInterval

fabric-loader 0.19.5 -- kind: interface

```java
public static final net.fabricmc.loader.api.metadata.version.VersionInterval INFINITE
public abstract boolean isSemantic()
public abstract net.fabricmc.loader.api.Version getMin()
public abstract boolean isMinInclusive()
public abstract net.fabricmc.loader.api.Version getMax()
public abstract boolean isMaxInclusive()
public default net.fabricmc.loader.api.metadata.version.VersionInterval and(net.fabricmc.loader.api.metadata.version.VersionInterval)
public default java.util.List<net.fabricmc.loader.api.metadata.version.VersionInterval> or(java.util.Collection<net.fabricmc.loader.api.metadata.version.VersionInterval>)
public default java.util.List<net.fabricmc.loader.api.metadata.version.VersionInterval> not()
public static net.fabricmc.loader.api.metadata.version.VersionInterval and(net.fabricmc.loader.api.metadata.version.VersionInterval, net.fabricmc.loader.api.metadata.version.VersionInterval)
public static java.util.List<net.fabricmc.loader.api.metadata.version.VersionInterval> and(java.util.Collection<net.fabricmc.loader.api.metadata.version.VersionInterval>, java.util.Collection<net.fabricmc.loader.api.metadata.version.VersionInterval>)
public static java.util.List<net.fabricmc.loader.api.metadata.version.VersionInterval> or(java.util.Collection<net.fabricmc.loader.api.metadata.version.VersionInterval>, net.fabricmc.loader.api.metadata.version.VersionInterval)
public static java.util.List<net.fabricmc.loader.api.metadata.version.VersionInterval> not(net.fabricmc.loader.api.metadata.version.VersionInterval)
public static java.util.List<net.fabricmc.loader.api.metadata.version.VersionInterval> not(java.util.Collection<net.fabricmc.loader.api.metadata.version.VersionInterval>)
static {}
```
