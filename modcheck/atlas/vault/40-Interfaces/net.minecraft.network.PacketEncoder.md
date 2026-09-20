---
type: "interface"
fqcn: "net.minecraft.network.PacketEncoder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.PacketEncoder

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`class` public; extends `io/netty/handler/codec/MessageToByteEncoder`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `encode` | `(Lio/netty/channel/ChannelHandlerContext;Lnet/minecraft/network/protoc` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `encode` | `(Lio/netty/channel/ChannelHandlerContext;Lnet/minecraft/network/protoc` | exact | @WrapMethod | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (2 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final protocolInfo : Lnet/minecraft/network/ProtocolInfo;
public <init>(Lnet/minecraft/network/ProtocolInfo;)V
protected encode(Lio/netty/channel/ChannelHandlerContext;Lnet/minecraft/network/protocol/Packet;Lio/netty/buffer/ByteBuf;)V
protected synthetic encode(Lio/netty/channel/ChannelHandlerContext;Ljava/lang/Object;Lio/netty/buffer/ByteBuf;)V
static <clinit>()V
```
