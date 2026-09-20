---
type: "interface"
fqcn: "net.minecraft.network.Connection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.Connection

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `disconnect(Lnet/minecraft/network/chat/Component;)V` | `` | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `disconnect(Lnet/minecraft/network/chat/Component;)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `disconnect(Lnet/minecraft/network/chat/Component;)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext()Lnet/fabricmc/fabric/api/networking/v1/context/PacketConte` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext()Lnet/fabricmc/fabric/api/networking/v1/context/PacketConte` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext()Lnet/fabricmc/fabric/api/networking/v1/context/PacketConte` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext()Lnet/fabricmc/fabric/api/networking/v1/context/PacketConte` | `` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext()Lnet/fabricmc/fabric/api/networking/v1/context/PacketConte` | `` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext()Lnet/fabricmc/fabric/api/networking/v1/context/PacketConte` | `` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketListener()Lnet/minecraft/network/PacketListener;` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `isMemoryConnection()Z` | `` | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `isMemoryConnection()Z` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `send(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/Ch` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `send(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/Ch` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `send(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/Ch` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `setupCompression(IZ)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `channelInactive` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleDisconnection` | `@Inject at INVOKE Lnet/minecraft/network/PacketListener;onDisconnect(Lnet/minecr` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `sendPacket` | `@Inject at FIELD Lnet/minecraft/network/Connection;sentPackets:I` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `setupInboundProtocol` | `@ModifyArg at INVOKE Lio/netty/channel/Channel;writeAndFlush(Ljava/lang/Object;)` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `setupOutboundProtocol` | `@ModifyArg at INVOKE Lio/netty/channel/Channel;writeAndFlush(Ljava/lang/Object;)` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `validateListener` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (93, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.Connection extends io.netty.channel.SimpleChannelInboundHandler<net.minecraft.network.protocol.Packet<?>> {
    private static final float AVERAGE_PACKETS_SMOOTHING;
    private static final org.slf4j.Logger LOGGER;
    public static final org.slf4j.Marker ROOT_MARKER;
    public static final org.slf4j.Marker PACKET_MARKER;
    public static final org.slf4j.Marker PACKET_RECEIVED_MARKER;
    public static final org.slf4j.Marker PACKET_SENT_MARKER;
    private static final net.minecraft.network.ProtocolInfo<net.minecraft.network.protocol.handshake.ServerHandshakePacketListener> INITIAL_PROTOCOL;
    private final net.minecraft.network.protocol.PacketFlow receiving;
    private volatile boolean sendLoginDisconnect;
    private final java.util.Queue<java.util.function.Consumer<net.minecraft.network.Connection>> pendingActions;
    private io.netty.channel.Channel channel;
    private java.net.SocketAddress address;
    private volatile net.minecraft.network.PacketListener disconnectListener;
    private volatile net.minecraft.network.PacketListener packetListener;
    private net.minecraft.network.DisconnectionDetails disconnectionDetails;
    private boolean disconnectionHandled;
    private int receivedPackets;
    private int sentPackets;
    private float averageReceivedPackets;
    private float averageSentPackets;
    private int tickCount;
    private boolean handlingFault;
    private volatile net.minecraft.network.DisconnectionDetails delayedDisconnect;
    private net.minecraft.network.BandwidthDebugMonitor bandwidthDebugMonitor;
    public net.minecraft.network.Connection(net.minecraft.network.protocol.PacketFlow);
    public void channelActive(io.netty.channel.ChannelHandlerContext) throws java.lang.Exception;
    public void channelInactive(io.netty.channel.ChannelHandlerContext);
    public void exceptionCaught(io.netty.channel.ChannelHandlerContext, java.lang.Throwable);
    protected void channelRead0(io.netty.channel.ChannelHandlerContext, net.minecraft.network.protocol.Packet<?>);
    private static <T extends net.minecraft.network.PacketListener> void genericsFtw(net.minecraft.network.protocol.Packet<T>, net.minecraft.network.PacketListener);
    private void validateListener(net.minecraft.network.ProtocolInfo<?>, net.minecraft.network.PacketListener);
    private static void syncAfterConfigurationChange(io.netty.channel.ChannelFuture);
    public <T extends net.minecraft.network.PacketListener> void setupInboundProtocol(net.minecraft.network.ProtocolInfo<T>, T);
    public void setupOutboundProtocol(net.minecraft.network.ProtocolInfo<?>);
    public void setListenerForServerboundHandshake(net.minecraft.network.PacketListener);
    public void initiateServerboundStatusConnection(java.lang.String, int, net.minecraft.network.protocol.status.ClientStatusPacketListener);
    public void initiateServerboundPlayConnection(java.lang.String, int, net.minecraft.network.protocol.login.ClientLoginPacketListener);
    public <S extends net.minecraft.network.ServerboundPacketListener, C extends net.minecraft.network.ClientboundPacketListener> void initiateServerboundPlayConnection(java.lang.String, int, net.minecraft.network.ProtocolInfo<S>, net.minecraft.network.ProtocolInfo<C>, C, boolean);
    private <S extends net.minecraft.network.ServerboundPacketListener, C extends net.minecraft.network.ClientboundPacketListener> void initiateServerboundConnection(java.lang.String, int, net.minecraft.network.ProtocolInfo<S>, net.minecraft.network.ProtocolInfo<C>, C, net.minecraft.network.protocol.handshake.ClientIntent);
    public void send(net.minecraft.network.protocol.Packet<?>);
    public void send(net.minecraft.network.protocol.Packet<?>, io.netty.channel.ChannelFutureListener);
    public void send(net.minecraft.network.protocol.Packet<?>, io.netty.channel.ChannelFutureListener, boolean);
    public void runOnceConnected(java.util.function.Consumer<net.minecraft.network.Connection>);
    private void sendPacket(net.minecraft.network.protocol.Packet<?>, io.netty.channel.ChannelFutureListener, boolean);
    private void doSendPacket(net.minecraft.network.protocol.Packet<?>, io.netty.channel.ChannelFutureListener, boolean);
    public void flushChannel();
    private void flush();
    private void flushQueue();
    public void tick();
    protected void tickSecond();
    public java.net.SocketAddress getRemoteAddress();
    public java.lang.String getLoggableAddress(boolean);
    public void disconnect(net.minecraft.network.chat.Component);
    public void disconnect(net.minecraft.network.DisconnectionDetails);
    public boolean isMemoryConnection();
    public net.minecraft.network.protocol.PacketFlow getReceiving();
    public net.minecraft.network.protocol.PacketFlow getSending();
    public static net.minecraft.network.Connection connectToServer(java.net.InetSocketAddress, net.minecraft.server.network.EventLoopGroupHolder, net.minecraft.util.debugchart.LocalSampleLogger);
    public static io.netty.channel.ChannelFuture connect(java.net.InetSocketAddress, net.minecraft.server.network.EventLoopGroupHolder, net.minecraft.network.Connection);
    private static java.lang.String outboundHandlerName(boolean);
    private static java.lang.String inboundHandlerName(boolean);
    public void configurePacketHandler(io.netty.channel.ChannelPipeline);
    public static void configureSerialization(io.netty.channel.ChannelPipeline, net.minecraft.network.protocol.PacketFlow, boolean, net.minecraft.network.BandwidthDebugMonitor);
    private static io.netty.channel.ChannelOutboundHandler createFrameEncoder(boolean);
    private static io.netty.channel.ChannelInboundHandler createFrameDecoder(net.minecraft.network.BandwidthDebugMonitor, boolean);
    public static void configureInMemoryPipeline(io.netty.channel.ChannelPipeline, net.minecraft.network.protocol.PacketFlow);
    public static net.minecraft.network.Connection connectToLocalServer(java.net.SocketAddress);
    public static net.minecraft.network.Connection fromChannel(io.netty.channel.Channel, net.minecraft.network.protocol.PacketFlow, net.minecraft.util.debugchart.LocalSampleLogger);
    public void setEncryptionKey(javax.crypto.Cipher, javax.crypto.Cipher);
    public boolean isConnected();
    public boolean isConnecting();
    public net.minecraft.network.PacketListener getPacketListener();
    public net.minecraft.network.DisconnectionDetails getDisconnectionDetails();
    public void setReadOnly();
    public void setupCompression(int, boolean);
    public void handleDisconnection();
    public float getAverageReceivedPackets();
    public float getAverageSentPackets();
    public void setBandwidthLogger(net.minecraft.util.debugchart.LocalSampleLogger);
    protected void channelRead0(io.netty.channel.ChannelHandlerContext, java.lang.Object) throws java.lang.Exception;
    private static net.minecraft.network.DisconnectionDetails lambda$handleDisconnection$0();
    private void lambda$flush$0();
    private void lambda$sendPacket$0(net.minecraft.network.protocol.Packet, io.netty.channel.ChannelFutureListener, boolean);
    private static void lambda$send$0(net.minecraft.network.protocol.Packet, io.netty.channel.ChannelFutureListener, boolean, net.minecraft.network.Connection);
    private void lambda$initiateServerboundConnection$0(net.minecraft.network.ProtocolInfo, net.minecraft.network.ClientboundPacketListener, java.lang.String, int, net.minecraft.network.protocol.handshake.ClientIntent, net.minecraft.network.ProtocolInfo, net.minecraft.network.Connection);
    private void lambda$setupOutboundProtocol$1(boolean, io.netty.channel.ChannelHandlerContext);
    private static void lambda$setupOutboundProtocol$0(net.minecraft.network.PacketBundleUnpacker, io.netty.channel.ChannelHandlerContext);
    private static void lambda$setupInboundProtocol$0(net.minecraft.network.PacketBundlePacker, io.netty.channel.ChannelHandlerContext);
    private void lambda$exceptionCaught$0(net.minecraft.network.DisconnectionDetails);
    private static void lambda$static$2(org.slf4j.Marker);
    private static void lambda$static$1(org.slf4j.Marker);
    private static void lambda$static$0(org.slf4j.Marker);
    static {};
}
```
