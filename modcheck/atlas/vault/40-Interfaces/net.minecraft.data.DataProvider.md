---
type: "interface"
fqcn: "net.minecraft.data.DataProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.DataProvider

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `saveStable(Lnet/minecraft/data/CachedOutput;Lcom/mojang/serialization/` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `saveStable(Lnet/minecraft/data/CachedOutput;Lcom/google/gson/JsonEleme` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$static$0` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `lambda$static$0` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.data.DataProvider {
    public static final java.util.function.ToIntFunction<java.lang.String> FIXED_ORDER_FIELDS;
    public static final java.util.Comparator<java.lang.String> KEY_COMPARATOR;
    public static final org.slf4j.Logger LOGGER;
    public abstract java.util.concurrent.CompletableFuture<?> run(net.minecraft.data.CachedOutput);
    public abstract java.lang.String getName();
    public static <T> java.util.concurrent.CompletableFuture<?> saveAll(net.minecraft.data.CachedOutput, com.mojang.serialization.Codec<T>, net.minecraft.data.PackOutput$PathProvider, java.util.Map<net.minecraft.resources.Identifier, T>);
    public static <T, E> java.util.concurrent.CompletableFuture<?> saveAll(net.minecraft.data.CachedOutput, com.mojang.serialization.Codec<E>, java.util.function.Function<T, java.nio.file.Path>, java.util.Map<T, E>);
    public static <T, E> java.util.concurrent.CompletableFuture<?> saveAll(net.minecraft.data.CachedOutput, java.util.function.Function<E, com.google.gson.JsonElement>, java.util.function.Function<T, java.nio.file.Path>, java.util.Map<T, E>);
    public static <T> java.util.concurrent.CompletableFuture<?> saveStable(net.minecraft.data.CachedOutput, net.minecraft.core.HolderLookup$Provider, com.mojang.serialization.Codec<T>, T, java.nio.file.Path);
    public static <T> java.util.concurrent.CompletableFuture<?> saveStable(net.minecraft.data.CachedOutput, com.mojang.serialization.Codec<T>, T, java.nio.file.Path);
    private static <T> java.util.concurrent.CompletableFuture<?> saveStable(net.minecraft.data.CachedOutput, com.mojang.serialization.DynamicOps<com.google.gson.JsonElement>, com.mojang.serialization.Codec<T>, T, java.nio.file.Path);
    public static java.util.concurrent.CompletableFuture<?> saveStable(net.minecraft.data.CachedOutput, com.google.gson.JsonElement, java.nio.file.Path);
    private static void lambda$saveStable$0(com.google.gson.JsonElement, net.minecraft.data.CachedOutput, java.nio.file.Path);
    private static java.util.concurrent.CompletableFuture[] lambda$saveAll$2(int);
    private static java.util.concurrent.CompletableFuture lambda$saveAll$1(java.util.function.Function, java.util.function.Function, net.minecraft.data.CachedOutput, java.util.Map$Entry);
    private static com.google.gson.JsonElement lambda$saveAll$0(com.mojang.serialization.Codec, java.lang.Object);
    private static java.lang.String lambda$static$1(java.lang.String);
    private static void lambda$static$0(it.unimi.dsi.fastutil.objects.Object2IntOpenHashMap);
    static {};
}
```
