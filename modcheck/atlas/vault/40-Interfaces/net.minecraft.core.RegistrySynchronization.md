---
type: "interface"
fqcn: "net.minecraft.core.RegistrySynchronization"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistrySynchronization

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `lambda$ownedNetworkableRegistries$0` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `lambda$packRegistry$0` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.RegistrySynchronization {
    private static final java.util.Set<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>> NETWORKABLE_REGISTRIES;
    public net.minecraft.core.RegistrySynchronization();
    public static void packRegistries(com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag>, net.minecraft.core.RegistryAccess, java.util.Set<net.minecraft.server.packs.repository.KnownPack>, java.util.function.BiConsumer<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>, java.util.List<net.minecraft.core.RegistrySynchronization$PackedRegistryEntry>>);
    private static <T> void packRegistry(com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag>, net.minecraft.resources.RegistryDataLoader$RegistryData<T>, net.minecraft.core.RegistryAccess, java.util.Set<net.minecraft.server.packs.repository.KnownPack>, java.util.function.BiConsumer<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>, java.util.List<net.minecraft.core.RegistrySynchronization$PackedRegistryEntry>>);
    private static java.util.stream.Stream<net.minecraft.core.RegistryAccess$RegistryEntry<?>> ownedNetworkableRegistries(net.minecraft.core.RegistryAccess);
    public static java.util.stream.Stream<net.minecraft.core.RegistryAccess$RegistryEntry<?>> networkedRegistries(net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>);
    public static java.util.stream.Stream<net.minecraft.core.RegistryAccess$RegistryEntry<?>> networkSafeRegistries(net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>);
    public static boolean isNetworkable(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>);
    private static boolean lambda$ownedNetworkableRegistries$0(net.minecraft.core.RegistryAccess$RegistryEntry);
    private static void lambda$packRegistry$0(java.util.Set, net.minecraft.resources.RegistryDataLoader$RegistryData, com.mojang.serialization.DynamicOps, java.util.function.BiConsumer, net.minecraft.core.Registry);
    private static void lambda$packRegistry$1(net.minecraft.core.Registry, java.util.Set, net.minecraft.resources.RegistryDataLoader$RegistryData, com.mojang.serialization.DynamicOps, java.util.List, net.minecraft.core.Holder$Reference);
    private static java.lang.IllegalArgumentException lambda$packRegistry$2(net.minecraft.core.Holder$Reference, java.lang.String);
    private static void lambda$packRegistries$0(com.mojang.serialization.DynamicOps, net.minecraft.core.RegistryAccess, java.util.Set, java.util.function.BiConsumer, net.minecraft.resources.RegistryDataLoader$RegistryData);
    static {};
}
```
