---
type: "interface"
fqcn: "net.minecraft.resources.RegistryLoadTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryLoadTask

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `registry` | `Lnet/minecraft/core/WritableRegistry;` | exact | getfield@42 in `FabricGameTestModInitializer.registerDynamicEntries` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `registry` | `Lnet/minecraft/core/WritableRegistry;` | exact | getfield@51 in `FabricGameTestModInitializer.registerDynamicEntries` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| reads | `registry` | `Lnet/minecraft/core/WritableRegistry;` | exact | getfield@62 in `RegistryDataLoaderMixin.beforeLoad` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `registry` | `Lnet/minecraft/core/WritableRegistry;` | exact | getfield@72 in `RegistryDataLoaderMixin.beforeLoad` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (6 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final registryWriteLock : Ljava/lang/Object;
protected final data : Lnet/minecraft/resources/RegistryDataLoader$RegistryData;
private final registry : Lnet/minecraft/core/WritableRegistry;
protected final concurrentRegistrationGetter : Lnet/minecraft/core/registries/ConcurrentHolderGetter;
protected final loadingErrors : Ljava/util/Map;
private elementsRegistered : Z
protected <init>(Lnet/minecraft/resources/RegistryDataLoader$RegistryData;Lcom/mojang/serialization/Lifecycle;Ljava/util/Map;)V
protected registryKey()Lnet/minecraft/resources/ResourceKey;
protected readOnlyRegistry()Lnet/minecraft/core/Registry;
public abstract load(Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
protected registerElements(Ljava/util/stream/Stream;)V
protected registerTags(Ljava/util/Map;)V
public freezeRegistry(Ljava/util/Map;)Z
public validateRegistry(Ljava/util/Map;)Ljava/util/Optional;
private synthetic lambda$registerElements$0(Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;)V
private synthetic lambda$registerElements$2(Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;Ljava/lang/Exception;)V
private synthetic lambda$registerElements$1(Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;Ljava/lang/Object;)V
```
