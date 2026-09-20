---
type: "interface"
fqcn: "net.minecraft.util.random.WeightedList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.random.WeightedList

System: [[20-Systems/net.minecraft.util.random|net.minecraft.util.random]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getRandomOrThrow(Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `of(Ljava/util/List;)Lnet/minecraft/util/random/WeightedList;` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `unwrap()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `unwrap()Ljava/util/List;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (29, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.util.random.WeightedList<E> {
    private static final int FLAT_THRESHOLD;
    private final int totalWeight;
    private final java.util.List<net.minecraft.util.random.Weighted<E>> items;
    private final net.minecraft.util.random.WeightedList$Selector<E> selector;
    private net.minecraft.util.random.WeightedList(java.util.List<? extends net.minecraft.util.random.Weighted<E>>);
    public static <E> net.minecraft.util.random.WeightedList<E> of();
    public static <E> net.minecraft.util.random.WeightedList<E> of(E);
    public static <E> net.minecraft.util.random.WeightedList<E> of(E...);
    public static <E> net.minecraft.util.random.WeightedList<E> of(net.minecraft.util.random.Weighted<E>...);
    public static <E> net.minecraft.util.random.WeightedList<E> of(java.util.List<net.minecraft.util.random.Weighted<E>>);
    public static <E> net.minecraft.util.random.WeightedList$Builder<E> builder();
    public boolean isEmpty();
    public <T> net.minecraft.util.random.WeightedList<T> map(java.util.function.Function<E, T>);
    public java.util.Optional<E> getRandom(net.minecraft.util.RandomSource);
    public E getRandomOrThrow(net.minecraft.util.RandomSource);
    public java.util.List<net.minecraft.util.random.Weighted<E>> unwrap();
    private static <E> com.mojang.serialization.Codec<net.minecraft.util.random.WeightedList<E>> entryToListCodec(com.mojang.serialization.Codec<net.minecraft.util.random.Weighted<E>>);
    public static <E> com.mojang.serialization.Codec<net.minecraft.util.random.WeightedList<E>> codec(com.mojang.serialization.Codec<E>);
    public static <E> com.mojang.serialization.Codec<net.minecraft.util.random.WeightedList<E>> codec(com.mojang.serialization.MapCodec<E>);
    private static <E> com.mojang.serialization.Codec<net.minecraft.util.random.WeightedList<E>> entryToNonEmptyListCodec(com.mojang.serialization.Codec<net.minecraft.util.random.Weighted<E>>);
    public static <E> com.mojang.serialization.Codec<net.minecraft.util.random.WeightedList<E>> nonEmptyCodec(com.mojang.serialization.Codec<E>);
    public static <E> com.mojang.serialization.Codec<net.minecraft.util.random.WeightedList<E>> nonEmptyCodec(com.mojang.serialization.MapCodec<E>);
    public static <E, B extends io.netty.buffer.ByteBuf> net.minecraft.network.codec.StreamCodec<B, net.minecraft.util.random.WeightedList<E>> streamCodec(net.minecraft.network.codec.StreamCodec<B, E>);
    public boolean contains(E);
    public boolean equals(java.lang.Object);
    public int hashCode();
    private static com.mojang.serialization.DataResult lambda$entryToNonEmptyListCodec$0(net.minecraft.util.random.WeightedList);
    private static java.lang.String lambda$entryToNonEmptyListCodec$1();
    private static net.minecraft.util.random.Weighted lambda$map$0(java.util.function.Function, net.minecraft.util.random.Weighted);
}
```
