---
type: "interface"
fqcn: "net.minecraft.core.HolderSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderSet

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `contains(Lnet/minecraft/core/Holder;)Z` | `` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `direct(Ljava/util/List;)Lnet/minecraft/core/HolderSet$Direct;` | `` | both | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `direct([Lnet/minecraft/core/Holder;)Lnet/minecraft/core/HolderSet$` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.core.HolderSet<T> extends java.lang.Iterable<net.minecraft.core.Holder<T>> {
    public abstract java.util.stream.Stream<net.minecraft.core.Holder<T>> stream();
    public abstract int size();
    public abstract boolean isBound();
    public abstract com.mojang.datafixers.util.Either<net.minecraft.tags.TagKey<T>, java.util.List<net.minecraft.core.Holder<T>>> unwrap();
    public abstract java.util.Optional<net.minecraft.core.Holder<T>> getRandomElement(net.minecraft.util.RandomSource);
    public abstract net.minecraft.core.Holder<T> get(int);
    public abstract boolean contains(net.minecraft.core.Holder<T>);
    public abstract boolean canSerializeIn(net.minecraft.core.HolderOwner<T>);
    public abstract java.util.Optional<net.minecraft.tags.TagKey<T>> unwrapKey();
    public static <T> net.minecraft.core.HolderSet$Named<T> emptyNamed(net.minecraft.core.HolderOwner<T>, net.minecraft.tags.TagKey<T>);
    public static <T> net.minecraft.core.HolderSet<T> empty();
    public static <T> net.minecraft.core.HolderSet$Direct<T> direct(net.minecraft.core.Holder<T>...);
    public static <T> net.minecraft.core.HolderSet$Direct<T> direct(java.util.List<? extends net.minecraft.core.Holder<T>>);
    public static <E, T> net.minecraft.core.HolderSet$Direct<T> direct(java.util.function.Function<E, net.minecraft.core.Holder<T>>, E...);
    public static <E, T> net.minecraft.core.HolderSet$Direct<T> direct(java.util.function.Function<E, net.minecraft.core.Holder<T>>, java.util.Collection<E>);
}
```
