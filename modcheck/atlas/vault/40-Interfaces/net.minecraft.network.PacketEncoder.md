---
type: "interface"
fqcn: "net.minecraft.network.PacketEncoder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.PacketEncoder

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `encode(Lio/netty/channel/ChannelHandlerContext;Lnet/minecraft/network/protocol/Packet;Lio/netty/buffer/ByteBuf;)V` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.PacketEncoder<T extends net.minecraft.network.PacketListener> extends io.netty.handler.codec.MessageToByteEncoder<net.minecraft.network.protocol.Packet<T>> {
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.network.ProtocolInfo<T> protocolInfo;
    public net.minecraft.network.PacketEncoder(net.minecraft.network.ProtocolInfo<T>);
    protected void encode(io.netty.channel.ChannelHandlerContext, net.minecraft.network.protocol.Packet<T>, io.netty.buffer.ByteBuf) throws java.lang.Exception;
    protected void encode(io.netty.channel.ChannelHandlerContext, java.lang.Object, io.netty.buffer.ByteBuf) throws java.lang.Exception;
    static {};
}
```
