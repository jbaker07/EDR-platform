---
type: "interface"
fqcn: "net.minecraft.server.network.ServerCommonPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerCommonPacketListenerImpl

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/networ` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/networ` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/networ` | `` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `handleCustomPayload` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handlePong` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (41, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.server.network.ServerCommonPacketListenerImpl implements net.minecraft.network.protocol.common.ServerCommonPacketListener {
    private static final org.slf4j.Logger LOGGER;
    public static final int LATENCY_CHECK_INTERVAL;
    private static final int CLOSED_LISTENER_TIMEOUT;
    private static final net.minecraft.network.chat.Component TIMEOUT_DISCONNECTION_MESSAGE;
    static final net.minecraft.network.chat.Component DISCONNECT_UNEXPECTED_QUERY;
    protected final net.minecraft.server.MinecraftServer server;
    protected final net.minecraft.network.Connection connection;
    private final boolean transferred;
    private long keepAliveTime;
    private boolean keepAlivePending;
    private long keepAliveChallenge;
    private long closedListenerTime;
    private boolean closed;
    private int latency;
    private volatile boolean suspendFlushingOnServerThread;
    public net.minecraft.server.network.ServerCommonPacketListenerImpl(net.minecraft.server.MinecraftServer, net.minecraft.network.Connection, net.minecraft.server.network.CommonListenerCookie);
    private void close();
    public void onDisconnect(net.minecraft.network.DisconnectionDetails);
    public void onPacketError(net.minecraft.network.protocol.Packet, java.lang.Exception) throws net.minecraft.ReportedException;
    public void handleKeepAlive(net.minecraft.network.protocol.common.ServerboundKeepAlivePacket);
    public void handlePong(net.minecraft.network.protocol.common.ServerboundPongPacket);
    public void handleCustomPayload(net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket);
    public void handleCustomClickAction(net.minecraft.network.protocol.common.ServerboundCustomClickActionPacket);
    public void handleResourcePackResponse(net.minecraft.network.protocol.common.ServerboundResourcePackPacket);
    public void handleCookieResponse(net.minecraft.network.protocol.cookie.ServerboundCookieResponsePacket);
    protected void keepConnectionAlive();
    private boolean checkIfClosed(long);
    public void suspendFlushing();
    public void resumeFlushing();
    public void send(net.minecraft.network.protocol.Packet<?>);
    public void send(net.minecraft.network.protocol.Packet<?>, io.netty.channel.ChannelFutureListener);
    public void disconnect(net.minecraft.network.chat.Component);
    public void disconnect(net.minecraft.network.DisconnectionDetails);
    protected boolean isSingleplayerOwner();
    protected abstract com.mojang.authlib.GameProfile playerProfile();
    public com.mojang.authlib.GameProfile getOwner();
    public int latency();
    protected net.minecraft.server.network.CommonListenerCookie createCookie(net.minecraft.server.level.ClientInformation);
    private void lambda$disconnect$0(net.minecraft.network.DisconnectionDetails);
    private static java.lang.String lambda$send$0(net.minecraft.network.protocol.Packet) throws java.lang.Exception;
    static {};
}
```
