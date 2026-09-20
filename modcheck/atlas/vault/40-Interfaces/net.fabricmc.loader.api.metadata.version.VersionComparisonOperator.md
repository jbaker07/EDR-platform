---
type: "interface"
fqcn: "net.fabricmc.loader.api.metadata.version.VersionComparisonOperator"
module: "fabric-loader"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.loader.api.metadata.version.VersionComparisonOperator

fabric-loader 0.19.5 -- kind: enum

```java
public static final net.fabricmc.loader.api.metadata.version.VersionComparisonOperator GREATER_EQUAL
public static final net.fabricmc.loader.api.metadata.version.VersionComparisonOperator LESS_EQUAL
public static final net.fabricmc.loader.api.metadata.version.VersionComparisonOperator GREATER
public static final net.fabricmc.loader.api.metadata.version.VersionComparisonOperator LESS
public static final net.fabricmc.loader.api.metadata.version.VersionComparisonOperator EQUAL
public static final net.fabricmc.loader.api.metadata.version.VersionComparisonOperator SAME_TO_NEXT_MINOR
public static final net.fabricmc.loader.api.metadata.version.VersionComparisonOperator SAME_TO_NEXT_MAJOR
public static net.fabricmc.loader.api.metadata.version.VersionComparisonOperator[] values()
public static net.fabricmc.loader.api.metadata.version.VersionComparisonOperator valueOf(java.lang.String)
public final java.lang.String getSerialized()
public final boolean isMinInclusive()
public final boolean isMaxInclusive()
public final boolean test(net.fabricmc.loader.api.Version, net.fabricmc.loader.api.Version)
public abstract boolean test(net.fabricmc.loader.api.SemanticVersion, net.fabricmc.loader.api.SemanticVersion)
public net.fabricmc.loader.api.SemanticVersion minVersion(net.fabricmc.loader.api.SemanticVersion)
public net.fabricmc.loader.api.SemanticVersion maxVersion(net.fabricmc.loader.api.SemanticVersion)
```
