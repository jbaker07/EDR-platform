---
type: "interface"
fqcn: "net.minecraft.data.registries.RegistriesDatapackGenerator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.registries.RegistriesDatapackGenerator

System: [[20-Systems/net.minecraft.data.registries|net.minecraft.data.registries]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/data/DataProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `forReloadableLayer` | `(Lnet/minecraft/data/PackOutput;Ljava/util/concurrent/CompletableFutur` | name_only | @Redirect at ['FIELD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| wraps | `forWorldLayer` | `(Lnet/minecraft/data/PackOutput;Ljava/util/concurrent/CompletableFutur` | name_only | @Redirect at ['FIELD'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (4 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final output : Lnet/minecraft/data/PackOutput;
private final name : Ljava/lang/String;
private final registryData : Ljava/util/Collection;
private final registries : Ljava/util/concurrent/CompletableFuture;
public <init>(Lnet/minecraft/data/PackOutput;Ljava/lang/String;Ljava/util/Collection;Ljava/util/concurrent/CompletableFuture;)V
public static forWorldLayer(Lnet/minecraft/data/PackOutput;Ljava/util/concurrent/CompletableFuture;)Lnet/minecraft/data/registries/RegistriesDatapackGenerator;
public static forReloadableLayer(Lnet/minecraft/data/PackOutput;Ljava/util/concurrent/CompletableFuture;)Lnet/minecraft/data/DataProvider;
public run(Lnet/minecraft/data/CachedOutput;)Ljava/util/concurrent/CompletableFuture;
private dumpRegistryCap(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/core/HolderLookup$Provider;Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;)Ljava/util/Optional;
private static dumpValue(Ljava/nio/file/Path;Lnet/minecraft/data/CachedOutput;Lcom/mojang/serialization/DynamicOps;Lcom/mojang/serialization/Encoder;Ljava/lang/Object;)Ljava/util/concurrent/CompletableFuture;
public getName()Ljava/lang/String;
private static synthetic lambda$dumpValue$1(Ljava/nio/file/Path;Lcom/mojang/serialization/DataResult$Error;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$dumpValue$0(Lnet/minecraft/data/CachedOutput;Ljava/nio/file/Path;Lcom/google/gson/JsonElement;)Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$dumpRegistryCap$0(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/data/CachedOutput;Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;Lnet/minecraft/core/HolderLookup$RegistryLookup;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$dumpRegistryCap$2(I)[Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$dumpRegistryCap$1(Lnet/minecraft/data/PackOutput$PathProvider;Lnet/minecraft/data/CachedOutput;Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;Lnet/minecraft/core/Holder$Reference;)Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$run$0(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/core/HolderLookup$Provider;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$run$2(I)[Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$run$1(Lnet/minecraft/data/CachedOutput;Lnet/minecraft/core/HolderLookup$Provider;Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;)Ljava/util/stream/Stream;
```
