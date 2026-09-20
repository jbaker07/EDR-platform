---
type: "interface"
fqcn: "net.minecraft.core.HolderGetter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderGetter

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `getOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/H` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `getOrThrow(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet$N` | `` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `getOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/H` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `getOrThrow(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet$N` | `` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.HolderGetter<T> extends net.minecraft.core.HolderOwner<T> {
    public abstract java.util.Optional<net.minecraft.core.Holder$Reference<T>> get(net.minecraft.resources.ResourceKey<T>);
    public default net.minecraft.core.Holder$Reference<T> getOrThrow(net.minecraft.resources.ResourceKey<T>);
    public abstract java.util.Optional<net.minecraft.core.HolderSet$Named<T>> get(net.minecraft.tags.TagKey<T>);
    public default net.minecraft.core.HolderSet$Named<T> getOrThrow(net.minecraft.tags.TagKey<T>);
    public default java.util.Optional<net.minecraft.core.Holder<T>> getRandomElementOf(net.minecraft.tags.TagKey<T>, net.minecraft.util.RandomSource);
    private static java.util.Optional lambda$getRandomElementOf$0(net.minecraft.util.RandomSource, net.minecraft.core.HolderSet$Named);
    private static java.lang.IllegalStateException lambda$getOrThrow$1(net.minecraft.tags.TagKey);
    private static java.lang.IllegalStateException lambda$getOrThrow$0(net.minecraft.resources.ResourceKey);
}
```
