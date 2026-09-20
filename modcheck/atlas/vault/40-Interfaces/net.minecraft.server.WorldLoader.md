---
type: "interface"
fqcn: "net.minecraft.server.WorldLoader"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.WorldLoader

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `lambda$load$0` | `@ModifyArg at INVOKE Lnet/minecraft/resources/RegistryDataLoader;load(Lnet/minec` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.WorldLoader {
    public net.minecraft.server.WorldLoader();
    public static <D, R> java.util.concurrent.CompletableFuture<R> load(net.minecraft.server.WorldLoader$InitConfig, net.minecraft.server.WorldLoader$WorldDataSupplier<D>, net.minecraft.server.WorldLoader$ResultFactory<D, R>, java.util.concurrent.Executor, java.util.concurrent.Executor);
    private static java.util.concurrent.CompletionStage lambda$load$0(java.util.concurrent.Executor, net.minecraft.server.WorldLoader$WorldDataSupplier, net.minecraft.server.WorldLoader$InitConfig, java.util.concurrent.Executor, net.minecraft.server.WorldLoader$ResultFactory, com.mojang.datafixers.util.Pair);
    private static java.util.concurrent.CompletionStage lambda$load$1(java.util.List, net.minecraft.server.packs.resources.CloseableResourceManager, java.util.concurrent.Executor, com.mojang.datafixers.util.Pair, net.minecraft.server.WorldLoader$WorldDataSupplier, net.minecraft.core.LayeredRegistryAccess, java.util.List, net.minecraft.server.WorldLoader$InitConfig, java.util.concurrent.Executor, net.minecraft.server.WorldLoader$ResultFactory, net.minecraft.core.RegistryAccess$Frozen);
    private static java.util.concurrent.CompletionStage lambda$load$2(com.mojang.datafixers.util.Pair, java.util.List, net.minecraft.server.WorldLoader$WorldDataSupplier, net.minecraft.server.packs.resources.CloseableResourceManager, net.minecraft.core.LayeredRegistryAccess, net.minecraft.core.RegistryAccess$Frozen, java.util.List, net.minecraft.server.WorldLoader$InitConfig, java.util.concurrent.Executor, java.util.concurrent.Executor, net.minecraft.server.WorldLoader$ResultFactory, net.minecraft.core.RegistryAccess$Frozen);
    private static java.lang.Object lambda$load$4(net.minecraft.server.WorldLoader$ResultFactory, net.minecraft.server.packs.resources.CloseableResourceManager, net.minecraft.core.LayeredRegistryAccess, net.minecraft.server.WorldLoader$DataLoadOutput, net.minecraft.server.ReloadableServerResources);
    private static void lambda$load$3(net.minecraft.server.packs.resources.CloseableResourceManager, net.minecraft.server.ReloadableServerResources, java.lang.Throwable);
}
```
