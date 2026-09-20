---
type: "interface"
fqcn: "net.minecraft.network.Connection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.Connection

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`class` public; extends `io/netty/channel/SimpleChannelInboundHandler`; implements `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/network/protocol/PacketFlow;)V` | exact | invokespecial@4 in `FakePlayerPacketListener$FakeConnection.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `disconnect` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokevirtual@56 in `ServerHandshakePacketListenerImplMixin.rejectConnectionsDuringStartup | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `disconnect` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokevirtual@12 in `AbstractChanneledNetworkAddon.disconnect` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `disconnect` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokevirtual@13 in `ServerLoginNetworkAddon.disconnect` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `ServerCommonPacketListenerImplMixin.getPacketContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `ServerHandshakePacketListenerImplMixin.getPacketContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@1 in `ServerLoginPacketListenerImplMixin.initAddon` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@14 in `ServerLoginPacketListenerImplMixin.initAddon` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `ServerLoginPacketListenerImplMixin.getPacketContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `ClientCommonPacketListenerImplMixin.getPacketContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `ClientConfigurationPacketListenerImplMixin.handleComplete` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `ClientHandshakePacketListenerImplMixin.setGameProfileContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@4 in `ClientHandshakePacketListenerImplMixin.getPacketContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketListener` | `()Lnet/minecraft/network/PacketListener;` | exact | invokevirtual@1 in `GlobalAttachmentsImpl.lambda$fabric_syncChange$0` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getPacketListener` | `()Lnet/minecraft/network/PacketListener;` | exact | invokevirtual@9 in `ClientLoginNetworking.registerReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketListener` | `()Lnet/minecraft/network/PacketListener;` | exact | invokevirtual@9 in `ClientLoginNetworking.unregisterReceiver` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `isMemoryConnection` | `()Z` | exact | invokevirtual@22 in `ServerHandshakePacketListenerImplMixin.rejectConnectionsDuringStartup | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `isMemoryConnection` | `()Z` | exact | invokevirtual@14 in `ServerLoginNetworkAddon.sendCompressionPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutur` | exact | invokevirtual@13 in `AbstractChanneledNetworkAddon.sendPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutur` | exact | invokevirtual@46 in `ClientLoginNetworkAddon.lambda$handlePacket$0` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutur` | exact | invokevirtual@47 in `ServerLoginNetworkAddon.sendCompressionPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutur` | exact | invokevirtual@14 in `ServerLoginNetworkAddon.sendPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `setupCompression` | `(IZ)V` | exact | invokevirtual@12 in `ServerLoginNetworkAddon.lambda$sendCompressionPacket$0` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/network/protocol/PacketFlow;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `channelInactive` | `(Lio/netty/channel/ChannelHandlerContext;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleDisconnection` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `sendPacket` | `(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutur` | name_only | @Inject at ['FIELD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `setupInboundProtocol` | `(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketList` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `setupOutboundProtocol` | `(Lnet/minecraft/network/ProtocolInfo;)V` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `validateListener` | `(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketList` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `packetListener` | `Lnet/minecraft/network/PacketListener;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |

## Declared members (24 fields, 69 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final AVERAGE_PACKETS_SMOOTHING : F
private static final LOGGER : Lorg/slf4j/Logger;
public static final ROOT_MARKER : Lorg/slf4j/Marker;
public static final PACKET_MARKER : Lorg/slf4j/Marker;
public static final PACKET_RECEIVED_MARKER : Lorg/slf4j/Marker;
public static final PACKET_SENT_MARKER : Lorg/slf4j/Marker;
private static final INITIAL_PROTOCOL : Lnet/minecraft/network/ProtocolInfo;
private final receiving : Lnet/minecraft/network/protocol/PacketFlow;
private sendLoginDisconnect : Z
private final pendingActions : Ljava/util/Queue;
private channel : Lio/netty/channel/Channel;
private address : Ljava/net/SocketAddress;
private disconnectListener : Lnet/minecraft/network/PacketListener;
private packetListener : Lnet/minecraft/network/PacketListener;
private disconnectionDetails : Lnet/minecraft/network/DisconnectionDetails;
private disconnectionHandled : Z
private receivedPackets : I
private sentPackets : I
private averageReceivedPackets : F
private averageSentPackets : F
private tickCount : I
private handlingFault : Z
private delayedDisconnect : Lnet/minecraft/network/DisconnectionDetails;
private bandwidthDebugMonitor : Lnet/minecraft/network/BandwidthDebugMonitor;
public <init>(Lnet/minecraft/network/protocol/PacketFlow;)V
public channelActive(Lio/netty/channel/ChannelHandlerContext;)V
public channelInactive(Lio/netty/channel/ChannelHandlerContext;)V
public exceptionCaught(Lio/netty/channel/ChannelHandlerContext;Ljava/lang/Throwable;)V
protected channelRead0(Lio/netty/channel/ChannelHandlerContext;Lnet/minecraft/network/protocol/Packet;)V
private static genericsFtw(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/network/PacketListener;)V
private validateListener(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V
private static syncAfterConfigurationChange(Lio/netty/channel/ChannelFuture;)V
public setupInboundProtocol(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V
public setupOutboundProtocol(Lnet/minecraft/network/ProtocolInfo;)V
public setListenerForServerboundHandshake(Lnet/minecraft/network/PacketListener;)V
public initiateServerboundStatusConnection(Ljava/lang/String;ILnet/minecraft/network/protocol/status/ClientStatusPacketListener;)V
public initiateServerboundPlayConnection(Ljava/lang/String;ILnet/minecraft/network/protocol/login/ClientLoginPacketListener;)V
public initiateServerboundPlayConnection(Ljava/lang/String;ILnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/ClientboundPacketListener;Z)V
private initiateServerboundConnection(Ljava/lang/String;ILnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/ClientboundPacketListener;Lnet/minecraft/network/protocol/handshake/ClientIntent;)V
public send(Lnet/minecraft/network/protocol/Packet;)V
public send(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutureListener;)V
public send(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutureListener;Z)V
public runOnceConnected(Ljava/util/function/Consumer;)V
private sendPacket(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutureListener;Z)V
private doSendPacket(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutureListener;Z)V
public flushChannel()V
private flush()V
private flushQueue()V
public tick()V
protected tickSecond()V
public getRemoteAddress()Ljava/net/SocketAddress;
public getLoggableAddress(Z)Ljava/lang/String;
public disconnect(Lnet/minecraft/network/chat/Component;)V
public disconnect(Lnet/minecraft/network/DisconnectionDetails;)V
public isMemoryConnection()Z
public getReceiving()Lnet/minecraft/network/protocol/PacketFlow;
public getSending()Lnet/minecraft/network/protocol/PacketFlow;
public static connectToServer(Ljava/net/InetSocketAddress;Lnet/minecraft/server/network/EventLoopGroupHolder;Lnet/minecraft/util/debugchart/LocalSampleLogger;)Lnet/minecraft/network/Connection;
public static connect(Ljava/net/InetSocketAddress;Lnet/minecraft/server/network/EventLoopGroupHolder;Lnet/minecraft/network/Connection;)Lio/netty/channel/ChannelFuture;
private static outboundHandlerName(Z)Ljava/lang/String;
private static inboundHandlerName(Z)Ljava/lang/String;
public configurePacketHandler(Lio/netty/channel/ChannelPipeline;)V
public static configureSerialization(Lio/netty/channel/ChannelPipeline;Lnet/minecraft/network/protocol/PacketFlow;ZLnet/minecraft/network/BandwidthDebugMonitor;)V
private static createFrameEncoder(Z)Lio/netty/channel/ChannelOutboundHandler;
private static createFrameDecoder(Lnet/minecraft/network/BandwidthDebugMonitor;Z)Lio/netty/channel/ChannelInboundHandler;
public static configureInMemoryPipeline(Lio/netty/channel/ChannelPipeline;Lnet/minecraft/network/protocol/PacketFlow;)V
public static connectToLocalServer(Ljava/net/SocketAddress;)Lnet/minecraft/network/Connection;
public static fromChannel(Lio/netty/channel/Channel;Lnet/minecraft/network/protocol/PacketFlow;Lnet/minecraft/util/debugchart/LocalSampleLogger;)Lnet/minecraft/network/Connection;
public setEncryptionKey(Ljavax/crypto/Cipher;Ljavax/crypto/Cipher;)V
public isConnected()Z
public isConnecting()Z
public getPacketListener()Lnet/minecraft/network/PacketListener;
public getDisconnectionDetails()Lnet/minecraft/network/DisconnectionDetails;
public setReadOnly()V
public setupCompression(IZ)V
public handleDisconnection()V
public getAverageReceivedPackets()F
public getAverageSentPackets()F
public setBandwidthLogger(Lnet/minecraft/util/debugchart/LocalSampleLogger;)V
protected synthetic channelRead0(Lio/netty/channel/ChannelHandlerContext;Ljava/lang/Object;)V
private static synthetic lambda$handleDisconnection$0()Lnet/minecraft/network/DisconnectionDetails;
private synthetic lambda$flush$0()V
private synthetic lambda$sendPacket$0(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutureListener;Z)V
private static synthetic lambda$send$0(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutureListener;ZLnet/minecraft/network/Connection;)V
private synthetic lambda$initiateServerboundConnection$0(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/ClientboundPacketListener;Ljava/lang/String;ILnet/minecraft/network/protocol/handshake/ClientIntent;Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/Connection;)V
private synthetic lambda$setupOutboundProtocol$1(ZLio/netty/channel/ChannelHandlerContext;)V
private static synthetic lambda$setupOutboundProtocol$0(Lnet/minecraft/network/PacketBundleUnpacker;Lio/netty/channel/ChannelHandlerContext;)V
private static synthetic lambda$setupInboundProtocol$0(Lnet/minecraft/network/PacketBundlePacker;Lio/netty/channel/ChannelHandlerContext;)V
private synthetic lambda$exceptionCaught$0(Lnet/minecraft/network/DisconnectionDetails;)V
private static synthetic lambda$static$2(Lorg/slf4j/Marker;)V
private static synthetic lambda$static$1(Lorg/slf4j/Marker;)V
private static synthetic lambda$static$0(Lorg/slf4j/Marker;)V
static <clinit>()V
```
