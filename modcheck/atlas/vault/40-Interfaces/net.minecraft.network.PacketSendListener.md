---
type: "interface"
fqcn: "net.minecraft.network.PacketSendListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.PacketSendListener

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `thenRun` | `(Ljava/lang/Runnable;)Lio/netty/channel/ChannelFutureListener;` | exact | invokestatic@44 in `ServerLoginNetworkAddon.sendCompressionPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (1 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public <init>()V
public static thenRun(Ljava/lang/Runnable;)Lio/netty/channel/ChannelFutureListener;
public static exceptionallySend(Ljava/util/function/Supplier;)Lio/netty/channel/ChannelFutureListener;
private static synthetic lambda$exceptionallySend$0(Ljava/util/function/Supplier;Lio/netty/channel/ChannelFuture;)V
private static synthetic lambda$thenRun$0(Ljava/lang/Runnable;Lio/netty/channel/ChannelFuture;)V
static <clinit>()V
```
