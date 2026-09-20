---
type: "interface"
fqcn: "net.minecraft.util.random.Weighted"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.random.Weighted

System: [[20-Systems/net.minecraft.util.random|net.minecraft.util.random]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/Object;I)V` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `map(Ljava/util/function/Function;)Lnet/minecraft/util/random/We` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `value()Ljava/lang/Object;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `weight()I` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.util.random.Weighted<T> extends java.lang.Record {
    private final T value;
    private final int weight;
    private static final org.slf4j.Logger LOGGER;
    public net.minecraft.util.random.Weighted(T, int);
    public static <E> com.mojang.serialization.Codec<net.minecraft.util.random.Weighted<E>> codec(com.mojang.serialization.Codec<E>);
    public static <E> com.mojang.serialization.Codec<net.minecraft.util.random.Weighted<E>> codec(com.mojang.serialization.MapCodec<E>);
    public static <B extends io.netty.buffer.ByteBuf, T> net.minecraft.network.codec.StreamCodec<B, net.minecraft.util.random.Weighted<T>> streamCodec(net.minecraft.network.codec.StreamCodec<B, T>);
    public <U> net.minecraft.util.random.Weighted<U> map(java.util.function.Function<T, U>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public T value();
    public int weight();
    private static com.mojang.datafixers.kinds.App lambda$codec$0(com.mojang.serialization.MapCodec, com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
