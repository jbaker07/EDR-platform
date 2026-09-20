---
type: "interface"
fqcn: "net.minecraft.network.PacketDecoder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.PacketDecoder

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`class` public; extends `io/netty/handler/codec/ByteToMessageDecoder`; implements `net/minecraft/network/ProtocolSwapHandler`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `decode` | `(Lio/netty/channel/ChannelHandlerContext;Lio/netty/buffer/ByteBuf;Ljav` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (2 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final protocolInfo : Lnet/minecraft/network/ProtocolInfo;
public <init>(Lnet/minecraft/network/ProtocolInfo;)V
protected decode(Lio/netty/channel/ChannelHandlerContext;Lio/netty/buffer/ByteBuf;Ljava/util/List;)V
static <clinit>()V
```
