---
type: "interface"
fqcn: "net.minecraft.core.HolderLookup$Provider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderLookup$Provider

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `createSerializationContext(Lcom/mojang/serialization/DynamicOps;)Lnet/minecraft/resour` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `listRegistryKeys()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `listRegistryKeys()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `listRegistryKeys()Ljava/util/stream/Stream;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/H` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/H` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/H` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.HolderLookup$Provider extends net.minecraft.core.HolderGetter$Provider {
    public abstract java.util.stream.Stream<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>> listRegistryKeys();
    public default java.util.stream.Stream<net.minecraft.core.HolderLookup$RegistryLookup<?>> listRegistries();
    public abstract <T> java.util.Optional<? extends net.minecraft.core.HolderLookup$RegistryLookup<T>> lookup(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends T>>);
    public default <T> net.minecraft.core.HolderLookup$RegistryLookup<T> lookupOrThrow(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends T>>);
    public default <V> net.minecraft.resources.RegistryOps<V> createSerializationContext(com.mojang.serialization.DynamicOps<V>);
    public static net.minecraft.core.HolderLookup$Provider create(java.util.stream.Stream<net.minecraft.core.HolderLookup$RegistryLookup<?>>);
    public default com.mojang.serialization.Lifecycle allRegistriesLifecycle();
    public default net.minecraft.core.HolderGetter lookupOrThrow(net.minecraft.resources.ResourceKey);
    private static net.minecraft.core.HolderLookup$RegistryLookup lambda$create$0(net.minecraft.core.HolderLookup$RegistryLookup);
    private static java.lang.IllegalStateException lambda$lookupOrThrow$0(net.minecraft.resources.ResourceKey);
}
```
