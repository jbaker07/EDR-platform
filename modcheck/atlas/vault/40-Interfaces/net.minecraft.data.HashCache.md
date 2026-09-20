---
type: "interface"
fqcn: "net.minecraft.data.HashCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.HashCache

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `lambda$purgeStaleAndWrite$0` | `(Ljava/util/Set;Ljava/lang/String;Lnet/minecraft/data/HashCache$Provid` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (10 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final HEADER_MARKER : Ljava/lang/String;
private final rootDir : Ljava/nio/file/Path;
private final cacheDir : Ljava/nio/file/Path;
private final versionId : Ljava/lang/String;
private final caches : Ljava/util/Map;
private final cachesToWrite : Ljava/util/Set;
private final cachePaths : Ljava/util/Set;
private final initialCount : I
private writes : I
private getProviderCachePath(Ljava/lang/String;)Ljava/nio/file/Path;
public <init>(Ljava/nio/file/Path;Ljava/util/Collection;Lnet/minecraft/WorldVersion;)V
private static readCache(Ljava/nio/file/Path;Ljava/nio/file/Path;)Lnet/minecraft/data/HashCache$ProviderCache;
public shouldRunInThisVersion(Ljava/lang/String;)Z
public generateUpdate(Ljava/lang/String;Lnet/minecraft/data/HashCache$UpdateFunction;)Ljava/util/concurrent/CompletableFuture;
public applyUpdate(Lnet/minecraft/data/HashCache$UpdateResult;)V
public purgeStaleAndWrite()V
private synthetic lambda$purgeStaleAndWrite$0(Ljava/util/Set;Ljava/lang/String;Lnet/minecraft/data/HashCache$ProviderCache;)V
private static synthetic lambda$generateUpdate$0(Lnet/minecraft/data/HashCache$CacheUpdater;Ljava/lang/Object;)Lnet/minecraft/data/HashCache$UpdateResult;
static <clinit>()V
```
