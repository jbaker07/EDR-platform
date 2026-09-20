---
type: "interface"
fqcn: "net.minecraft.resources.RegistryLoadTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.RegistryLoadTask

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `registryLnet/minecraft/core/WritableRegistry;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.resources.RegistryLoadTask<T> {
    private final java.lang.Object registryWriteLock;
    protected final net.minecraft.resources.RegistryDataLoader$RegistryData<T> data;
    private final net.minecraft.core.WritableRegistry<T> registry;
    protected final net.minecraft.core.registries.ConcurrentHolderGetter<T> concurrentRegistrationGetter;
    protected final java.util.Map<net.minecraft.resources.ResourceKey<?>, java.lang.Exception> loadingErrors;
    private volatile boolean elementsRegistered;
    protected net.minecraft.resources.RegistryLoadTask(net.minecraft.resources.RegistryDataLoader$RegistryData<T>, com.mojang.serialization.Lifecycle, java.util.Map<net.minecraft.resources.ResourceKey<?>, java.lang.Exception>);
    protected net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<T>> registryKey();
    protected net.minecraft.core.Registry<T> readOnlyRegistry();
    public abstract java.util.concurrent.CompletableFuture<?> load(net.minecraft.resources.RegistryOps$RegistryInfoLookup, java.util.concurrent.Executor);
    protected void registerElements(java.util.stream.Stream<net.minecraft.resources.RegistryLoadTask$PendingRegistration<T>>);
    protected void registerTags(java.util.Map<net.minecraft.tags.TagKey<T>, java.util.List<net.minecraft.core.Holder<T>>>);
    public boolean freezeRegistry(java.util.Map<net.minecraft.resources.ResourceKey<?>, java.lang.Exception>);
    public java.util.Optional<net.minecraft.core.Registry<T>> validateRegistry(java.util.Map<net.minecraft.resources.ResourceKey<?>, java.lang.Exception>);
    private void lambda$registerElements$0(net.minecraft.resources.RegistryLoadTask$PendingRegistration);
    private void lambda$registerElements$2(net.minecraft.resources.RegistryLoadTask$PendingRegistration, java.lang.Exception);
    private void lambda$registerElements$1(net.minecraft.resources.RegistryLoadTask$PendingRegistration, java.lang.Object);
}
```
