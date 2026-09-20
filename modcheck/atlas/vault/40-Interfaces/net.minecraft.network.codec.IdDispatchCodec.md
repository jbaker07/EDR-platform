---
type: "interface"
fqcn: "net.minecraft.network.codec.IdDispatchCodec"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.codec.IdDispatchCodec

System: [[20-Systems/net.minecraft.network.codec|net.minecraft.network.codec]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `decode(Lio/netty/buffer/ByteBuf;)Ljava/lang/Object;` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `decode(Lio/netty/buffer/ByteBuf;)Ljava/lang/Object;` | `@Inject at NEW (Ljava/lang/String;Ljava/lang/Throwable;)Lio/netty/handler/codec/` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `encode(Lio/netty/buffer/ByteBuf;Ljava/lang/Object;)V` | `@Inject at NEW (Ljava/lang/String;Ljava/lang/Throwable;)Lio/netty/handler/codec/` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.codec.IdDispatchCodec<B extends io.netty.buffer.ByteBuf, V, T> implements net.minecraft.network.codec.StreamCodec<B, V> {
    private static final int UNKNOWN_TYPE;
    private final java.util.function.Function<V, ? extends T> typeGetter;
    private final java.util.List<net.minecraft.network.codec.IdDispatchCodec$Entry<B, V, T>> byId;
    private final it.unimi.dsi.fastutil.objects.Object2IntMap<T> toId;
    private net.minecraft.network.codec.IdDispatchCodec(java.util.function.Function<V, ? extends T>, java.util.List<net.minecraft.network.codec.IdDispatchCodec$Entry<B, V, T>>, it.unimi.dsi.fastutil.objects.Object2IntMap<T>);
    public V decode(B);
    public void encode(B, V);
    public static <B extends io.netty.buffer.ByteBuf, V, T> net.minecraft.network.codec.IdDispatchCodec$Builder<B, V, T> builder(java.util.function.Function<V, ? extends T>);
    public void encode(java.lang.Object, java.lang.Object);
    public java.lang.Object decode(java.lang.Object);
}
```
