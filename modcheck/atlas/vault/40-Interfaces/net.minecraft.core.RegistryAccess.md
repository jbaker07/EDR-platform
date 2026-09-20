---
type: "interface"
fqcn: "net.minecraft.core.RegistryAccess"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistryAccess

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `fromRegistryOfRegistries(Lnet/minecraft/core/Registry;)Lnet/minecraft/core/RegistryA` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `listRegistries()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/R` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/R` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/R` | `` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.RegistryAccess extends net.minecraft.core.HolderLookup$Provider {
    public static final org.slf4j.Logger LOGGER;
    public static final net.minecraft.core.RegistryAccess$Frozen EMPTY;
    public abstract <E> java.util.Optional<net.minecraft.core.Registry<E>> lookup(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends E>>);
    public default <E> net.minecraft.core.Registry<E> lookupOrThrow(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends E>>);
    public abstract java.util.stream.Stream<net.minecraft.core.RegistryAccess$RegistryEntry<?>> registries();
    public default java.util.stream.Stream<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>> listRegistryKeys();
    public static net.minecraft.core.RegistryAccess$Frozen fromRegistryOfRegistries(net.minecraft.core.Registry<? extends net.minecraft.core.Registry<?>>);
    public default net.minecraft.core.RegistryAccess$Frozen freeze();
    public default net.minecraft.core.HolderLookup$RegistryLookup lookupOrThrow(net.minecraft.resources.ResourceKey);
    public default net.minecraft.core.HolderGetter lookupOrThrow(net.minecraft.resources.ResourceKey);
    private static net.minecraft.resources.ResourceKey lambda$listRegistryKeys$0(net.minecraft.core.RegistryAccess$RegistryEntry);
    private static java.lang.IllegalStateException lambda$lookupOrThrow$0(net.minecraft.resources.ResourceKey);
    static {};
}
```
