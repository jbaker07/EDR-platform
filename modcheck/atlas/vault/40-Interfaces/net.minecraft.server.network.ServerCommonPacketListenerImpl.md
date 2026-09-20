---
type: "interface"
fqcn: "net.minecraft.server.network.ServerCommonPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerCommonPacketListenerImpl

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/network/protocol/common/ServerCommonPacketListener`, `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connecti` | exact | invokespecial@4 in `ServerConfigurationPacketListenerImplMixin.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connecti` | exact | invokespecial@4 in `ServerGamePacketListenerImplMixin.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connecti` | exact | invokespecial@4 in `ServerConfigurationPacketListenerImplMixin.<init>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `handleCustomPayload` | `(Lnet/minecraft/network/protocol/common/ServerboundCustomPayloadPacket` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handlePong` | `(Lnet/minecraft/network/protocol/common/ServerboundPongPacket;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/network/Connection;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| reads | `server` | `Lnet/minecraft/server/MinecraftServer;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |

## Declared members (15 fields, 26 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final LATENCY_CHECK_INTERVAL : I
private static final CLOSED_LISTENER_TIMEOUT : I
private static final TIMEOUT_DISCONNECTION_MESSAGE : Lnet/minecraft/network/chat/Component;
static final DISCONNECT_UNEXPECTED_QUERY : Lnet/minecraft/network/chat/Component;
protected final server : Lnet/minecraft/server/MinecraftServer;
protected final connection : Lnet/minecraft/network/Connection;
private final transferred : Z
private keepAliveTime : J
private keepAlivePending : Z
private keepAliveChallenge : J
private closedListenerTime : J
private closed : Z
private latency : I
private suspendFlushingOnServerThread : Z
public <init>(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connection;Lnet/minecraft/server/network/CommonListenerCookie;)V
private close()V
public onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V
public onPacketError(Lnet/minecraft/network/protocol/Packet;Ljava/lang/Exception;)V
public handleKeepAlive(Lnet/minecraft/network/protocol/common/ServerboundKeepAlivePacket;)V
public handlePong(Lnet/minecraft/network/protocol/common/ServerboundPongPacket;)V
public handleCustomPayload(Lnet/minecraft/network/protocol/common/ServerboundCustomPayloadPacket;)V
public handleCustomClickAction(Lnet/minecraft/network/protocol/common/ServerboundCustomClickActionPacket;)V
public handleResourcePackResponse(Lnet/minecraft/network/protocol/common/ServerboundResourcePackPacket;)V
public handleCookieResponse(Lnet/minecraft/network/protocol/cookie/ServerboundCookieResponsePacket;)V
protected keepConnectionAlive()V
private checkIfClosed(J)Z
public suspendFlushing()V
public resumeFlushing()V
public send(Lnet/minecraft/network/protocol/Packet;)V
public send(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutureListener;)V
public disconnect(Lnet/minecraft/network/chat/Component;)V
public disconnect(Lnet/minecraft/network/DisconnectionDetails;)V
protected isSingleplayerOwner()Z
protected abstract playerProfile()Lcom/mojang/authlib/GameProfile;
public getOwner()Lcom/mojang/authlib/GameProfile;
public latency()I
protected createCookie(Lnet/minecraft/server/level/ClientInformation;)Lnet/minecraft/server/network/CommonListenerCookie;
private synthetic lambda$disconnect$0(Lnet/minecraft/network/DisconnectionDetails;)V
private static synthetic lambda$send$0(Lnet/minecraft/network/protocol/Packet;)Ljava/lang/String;
static <clinit>()V
```
