---
type: "interface"
fqcn: "net.minecraft.data.HashCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.HashCache

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `lambda$purgeStaleAndWrite$0` | `@Redirect at INVOKE Ljava/time/ZonedDateTime;now()Ljava/time/ZonedDateTime;` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.HashCache {
    private static final org.slf4j.Logger LOGGER;
    private static final java.lang.String HEADER_MARKER;
    private final java.nio.file.Path rootDir;
    private final java.nio.file.Path cacheDir;
    private final java.lang.String versionId;
    private final java.util.Map<java.lang.String, net.minecraft.data.HashCache$ProviderCache> caches;
    private final java.util.Set<java.lang.String> cachesToWrite;
    private final java.util.Set<java.nio.file.Path> cachePaths;
    private final int initialCount;
    private int writes;
    private java.nio.file.Path getProviderCachePath(java.lang.String);
    public net.minecraft.data.HashCache(java.nio.file.Path, java.util.Collection<java.lang.String>, net.minecraft.WorldVersion) throws java.io.IOException;
    private static net.minecraft.data.HashCache$ProviderCache readCache(java.nio.file.Path, java.nio.file.Path);
    public boolean shouldRunInThisVersion(java.lang.String);
    public java.util.concurrent.CompletableFuture<net.minecraft.data.HashCache$UpdateResult> generateUpdate(java.lang.String, net.minecraft.data.HashCache$UpdateFunction);
    public void applyUpdate(net.minecraft.data.HashCache$UpdateResult);
    public void purgeStaleAndWrite() throws java.io.IOException;
    private void lambda$purgeStaleAndWrite$0(java.util.Set, java.lang.String, net.minecraft.data.HashCache$ProviderCache);
    private static net.minecraft.data.HashCache$UpdateResult lambda$generateUpdate$0(net.minecraft.data.HashCache$CacheUpdater, java.lang.Object);
    static {};
}
```
