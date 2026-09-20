---
type: "interface"
fqcn: "net.minecraft.network.codec.IdDispatchCodec$Entry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.codec.IdDispatchCodec$Entry

System: [[20-Systems/net.minecraft.network.codec|net.minecraft.network.codec]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `type()Ljava/lang/Object;` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
final class net.minecraft.network.codec.IdDispatchCodec$Entry<B, V, T> extends java.lang.Record {
    private final net.minecraft.network.codec.StreamCodec<? super B, ? extends V> serializer;
    private final T type;
    private net.minecraft.network.codec.IdDispatchCodec$Entry(net.minecraft.network.codec.StreamCodec<? super B, ? extends V>, T);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.codec.StreamCodec<? super B, ? extends V> serializer();
    public T type();
}
```
