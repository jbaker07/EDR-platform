---
type: "interface"
fqcn: "net.minecraft.server.network.ServerConnectionListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerConnectionListener

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getConnections()Ljava/util/List;` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.network.ServerConnectionListener {
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.server.MinecraftServer server;
    public volatile boolean running;
    private volatile java.util.UUID sessionId;
    private final java.util.List<io.netty.channel.ChannelFuture> channels;
    private final java.util.List<net.minecraft.network.Connection> connections;
    private final java.util.Queue<net.minecraft.network.Connection> pendingConnections;
    public net.minecraft.server.network.ServerConnectionListener(net.minecraft.server.MinecraftServer);
    public void startTcpServerListener(java.net.InetAddress, int) throws java.io.IOException;
    public java.net.SocketAddress startMemoryChannel();
    public void stop();
    public void stopTcpServerListener();
    public void tick();
    public net.minecraft.server.MinecraftServer getServer();
    private void addPendingConnections();
    public java.util.List<net.minecraft.network.Connection> getConnections();
    public java.util.UUID getSessionId();
    private static void lambda$tick$0(net.minecraft.network.Connection, net.minecraft.network.chat.Component);
    static {};
}
```
