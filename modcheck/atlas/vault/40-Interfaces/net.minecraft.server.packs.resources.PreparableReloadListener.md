---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.PreparableReloadListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.PreparableReloadListener

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@20 in `ResourceLoaderImpl.getResourceReloaderIdForSorting` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$Shared` | exact | invokeinterface@35 in `ResourceManagerHelperImpl$1.reload` | unknown | [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract reload(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
public prepareSharedState(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;)V
public getName()Ljava/lang/String;
```
