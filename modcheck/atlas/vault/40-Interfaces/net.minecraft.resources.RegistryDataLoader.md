---
type: "interface"
fqcn: "net.minecraft.resources.RegistryDataLoader"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryDataLoader

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `lambda$load$0` | `@ModifyArg at INVOKE Ljava/util/concurrent/CompletableFuture;thenApplyAsync(Ljav` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `lambda$load$2(Ljava/util/List;Ljava/util/Map;Ljava/lang/Void;)Lnet/minecraft/core/RegistryAccess$Frozen;` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| injects_into | `load(Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | `@ModifyArg at INVOKE Ljava/util/concurrent/CompletableFuture;supplyAsync(Ljava/u` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `load(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (29, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.resources.RegistryDataLoader {
    private static final org.slf4j.Logger LOGGER;
    private static final java.util.Comparator<net.minecraft.resources.ResourceKey<?>> ERROR_KEY_COMPARATOR;
    public static final java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>> WORLD_REGISTRIES;
    public static final java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>> DIMENSION_REGISTRIES;
    public static final java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>> RELOADABLE_REGISTRIES;
    public static final java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>> SYNCHRONIZED_REGISTRIES;
    public net.minecraft.resources.RegistryDataLoader();
    public static java.util.concurrent.CompletableFuture<net.minecraft.core.RegistryAccess$Frozen> load(net.minecraft.server.packs.resources.ResourceManager, java.util.List<net.minecraft.core.HolderLookup$RegistryLookup<?>>, java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>>, java.util.concurrent.Executor);
    public static java.util.concurrent.CompletableFuture<net.minecraft.core.RegistryAccess$Frozen> load(java.util.Map<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>, net.minecraft.resources.RegistryDataLoader$NetworkedRegistryData>, net.minecraft.server.packs.resources.ResourceProvider, java.util.List<net.minecraft.core.HolderLookup$RegistryLookup<?>>, java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>>, java.util.concurrent.Executor);
    private static java.util.concurrent.CompletableFuture<net.minecraft.core.RegistryAccess$Frozen> load(net.minecraft.resources.RegistryDataLoader$LoaderFactory, java.util.List<net.minecraft.core.HolderLookup$RegistryLookup<?>>, java.util.List<net.minecraft.resources.RegistryDataLoader$RegistryData<?>>, java.util.concurrent.Executor);
    private static net.minecraft.resources.RegistryOps$RegistryInfoLookup createContext(java.util.List<net.minecraft.core.HolderLookup$RegistryLookup<?>>, java.util.List<net.minecraft.resources.RegistryLoadTask<?>>);
    private static net.minecraft.ReportedException logErrors(java.util.Map<net.minecraft.resources.ResourceKey<?>, java.lang.Exception>);
    private static void printFullDetailsToLog(java.util.Map<net.minecraft.resources.ResourceKey<?>, java.lang.Exception>);
    private static net.minecraft.ReportedException createReportWithBriefInfo(java.util.Map<net.minecraft.resources.ResourceKey<?>, java.lang.Exception>);
    private static java.lang.String lambda$createReportWithBriefInfo$0(java.util.Map) throws java.lang.Exception;
    private static void lambda$createReportWithBriefInfo$1(java.lang.StringBuilder, java.util.Map$Entry);
    private static void lambda$printFullDetailsToLog$2(java.io.PrintWriter, java.util.Map$Entry);
    private static void lambda$printFullDetailsToLog$3(java.io.PrintWriter, java.util.Map$Entry);
    private static net.minecraft.resources.Identifier lambda$printFullDetailsToLog$1(java.util.Map$Entry);
    private static net.minecraft.resources.Identifier lambda$printFullDetailsToLog$0(java.util.Map$Entry);
    private static void lambda$createContext$1(java.util.Map, net.minecraft.resources.RegistryLoadTask);
    private static void lambda$createContext$0(java.util.Map, net.minecraft.core.HolderLookup$RegistryLookup);
    private static java.util.concurrent.CompletionStage lambda$load$5(java.util.concurrent.CompletableFuture);
    private static java.util.concurrent.CompletableFuture lambda$load$0(java.util.List, net.minecraft.resources.RegistryDataLoader$LoaderFactory, java.util.List, java.util.concurrent.Executor);
    private static net.minecraft.core.RegistryAccess$Frozen lambda$load$2(java.util.List, java.util.Map, java.lang.Void);
    private static java.util.stream.Stream lambda$load$4(java.util.Map, net.minecraft.resources.RegistryLoadTask);
    private static boolean lambda$load$3(java.util.Map, net.minecraft.resources.RegistryLoadTask);
    private static net.minecraft.resources.RegistryLoadTask lambda$load$1(net.minecraft.resources.RegistryDataLoader$LoaderFactory, java.util.Map, net.minecraft.resources.RegistryDataLoader$RegistryData);
    static {};
}
```
