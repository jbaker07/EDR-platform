---
type: "interface"
fqcn: "net.minecraft.data.registries.RegistriesDatapackGenerator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.registries.RegistriesDatapackGenerator

System: [[20-Systems/net.minecraft.data.registries|net.minecraft.data.registries]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `forReloadableLayer` | `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;RELOADABLE_REGIST` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| wraps | `forWorldLayer` | `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;WORLD_REGISTRIES:` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.registries.RegistriesDatapackGenerator implements net.minecraft.data.DataProvider {
    private final net.minecraft.data.PackOutput output;
    private final java.lang.String name;
    private final java.util.Collection<net.minecraft.resources.RegistryDataLoader$RegistryData<?>> registryData;
    private final java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider> registries;
    public net.minecraft.data.registries.RegistriesDatapackGenerator(net.minecraft.data.PackOutput, java.lang.String, java.util.Collection<net.minecraft.resources.RegistryDataLoader$RegistryData<?>>, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>);
    public static net.minecraft.data.registries.RegistriesDatapackGenerator forWorldLayer(net.minecraft.data.PackOutput, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>);
    public static net.minecraft.data.DataProvider forReloadableLayer(net.minecraft.data.PackOutput, java.util.concurrent.CompletableFuture<net.minecraft.core.HolderLookup$Provider>);
    public java.util.concurrent.CompletableFuture<?> run(net.minecraft.data.CachedOutput);
    private <T> java.util.Optional<java.util.concurrent.CompletableFuture<?>> dumpRegistryCap(net.minecraft.data.CachedOutput, net.minecraft.core.HolderLookup$Provider, com.mojang.serialization.DynamicOps<com.google.gson.JsonElement>, net.minecraft.resources.RegistryDataLoader$RegistryData<T>);
    private static <E> java.util.concurrent.CompletableFuture<?> dumpValue(java.nio.file.Path, net.minecraft.data.CachedOutput, com.mojang.serialization.DynamicOps<com.google.gson.JsonElement>, com.mojang.serialization.Encoder<E>, E);
    public final java.lang.String getName();
    private static java.util.concurrent.CompletableFuture lambda$dumpValue$1(java.nio.file.Path, com.mojang.serialization.DataResult$Error);
    private static java.util.concurrent.CompletableFuture lambda$dumpValue$0(net.minecraft.data.CachedOutput, java.nio.file.Path, com.google.gson.JsonElement);
    private java.util.concurrent.CompletableFuture lambda$dumpRegistryCap$0(net.minecraft.resources.ResourceKey, net.minecraft.data.CachedOutput, com.mojang.serialization.DynamicOps, net.minecraft.resources.RegistryDataLoader$RegistryData, net.minecraft.core.HolderLookup$RegistryLookup);
    private static java.util.concurrent.CompletableFuture[] lambda$dumpRegistryCap$2(int);
    private static java.util.concurrent.CompletableFuture lambda$dumpRegistryCap$1(net.minecraft.data.PackOutput$PathProvider, net.minecraft.data.CachedOutput, com.mojang.serialization.DynamicOps, net.minecraft.resources.RegistryDataLoader$RegistryData, net.minecraft.core.Holder$Reference);
    private java.util.concurrent.CompletionStage lambda$run$0(net.minecraft.data.CachedOutput, net.minecraft.core.HolderLookup$Provider);
    private static java.util.concurrent.CompletableFuture[] lambda$run$2(int);
    private java.util.stream.Stream lambda$run$1(net.minecraft.data.CachedOutput, net.minecraft.core.HolderLookup$Provider, com.mojang.serialization.DynamicOps, net.minecraft.resources.RegistryDataLoader$RegistryData);
}
```
