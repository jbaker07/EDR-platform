---
type: "interface"
fqcn: "net.minecraft.network.protocol.Packet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.Packet

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `isTerminal()Z` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `isTerminal()Z` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `type()Lnet/minecraft/network/protocol/PacketType;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.network.protocol.Packet<T extends net.minecraft.network.PacketListener> {
    public abstract net.minecraft.network.protocol.PacketType<? extends net.minecraft.network.protocol.Packet<T>> type();
    public abstract void handle(T);
    public default boolean isSkippable();
    public default boolean isTerminal();
    public static <B extends io.netty.buffer.ByteBuf, T extends net.minecraft.network.protocol.Packet<?>> net.minecraft.network.codec.StreamCodec<B, T> codec(net.minecraft.network.codec.StreamMemberEncoder<B, T>, net.minecraft.network.codec.StreamDecoder<B, T>);
}
```
