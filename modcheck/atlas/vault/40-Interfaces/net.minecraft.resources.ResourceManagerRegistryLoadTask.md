---
type: "interface"
fqcn: "net.minecraft.resources.ResourceManagerRegistryLoadTask"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.ResourceManagerRegistryLoadTask

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `load` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.resources.ResourceManagerRegistryLoadTask<T> extends net.minecraft.resources.RegistryLoadTask<T> {
    private static final java.util.function.Function<java.util.Optional<net.minecraft.server.packs.repository.KnownPack>, net.minecraft.core.RegistrationInfo> REGISTRATION_INFO_CACHE;
    private final net.minecraft.server.packs.resources.ResourceManager resourceManager;
    public net.minecraft.resources.ResourceManagerRegistryLoadTask(net.minecraft.resources.RegistryDataLoader$RegistryData<T>, com.mojang.serialization.Lifecycle, java.util.Map<net.minecraft.resources.ResourceKey<?>, java.lang.Exception>, net.minecraft.server.packs.resources.ResourceManager);
    public java.util.concurrent.CompletableFuture<?> load(net.minecraft.resources.RegistryOps$RegistryInfoLookup, java.util.concurrent.Executor);
    private void lambda$load$3(java.util.Map);
    private java.util.concurrent.CompletionStage lambda$load$1(net.minecraft.resources.RegistryOps$RegistryInfoLookup, net.minecraft.resources.FileToIdConverter, java.util.concurrent.Executor, java.util.Map);
    private net.minecraft.resources.RegistryLoadTask$PendingRegistration lambda$load$2(net.minecraft.resources.FileToIdConverter, net.minecraft.resources.RegistryOps, net.minecraft.resources.Identifier, net.minecraft.server.packs.resources.Resource);
    private java.util.Map lambda$load$0(net.minecraft.resources.FileToIdConverter);
    private static net.minecraft.core.RegistrationInfo lambda$static$0(java.util.Optional);
    private static com.mojang.serialization.Lifecycle lambda$static$1(java.lang.Boolean);
    static {};
}
```
