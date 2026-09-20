---
type: "interface"
fqcn: "net.minecraft.network.PacketSendListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.PacketSendListener

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `thenRun(Ljava/lang/Runnable;)Lio/netty/channel/ChannelFutureListene` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.PacketSendListener {
    private static final org.slf4j.Logger LOGGER;
    public net.minecraft.network.PacketSendListener();
    public static io.netty.channel.ChannelFutureListener thenRun(java.lang.Runnable);
    public static io.netty.channel.ChannelFutureListener exceptionallySend(java.util.function.Supplier<net.minecraft.network.protocol.Packet<?>>);
    private static void lambda$exceptionallySend$0(java.util.function.Supplier, io.netty.channel.ChannelFuture) throws java.lang.Exception;
    private static void lambda$thenRun$0(java.lang.Runnable, io.netty.channel.ChannelFuture) throws java.lang.Exception;
    static {};
}
```
