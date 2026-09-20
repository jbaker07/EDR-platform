---
type: "interface"
fqcn: "net.minecraft.server.network.ServerLoginPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerLoginPacketListenerImpl

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/network/protocol/login/ServerLoginPacketListener`, `net/minecraft/network/TickablePacketListener`, `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getUserName` | `()Ljava/lang/String;` | exact | invokevirtual@5 in `ServerLoginNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `verifyLoginAndFinishConnectionSetup` | `(Lcom/mojang/authlib/GameProfile;)V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| injects_into | `<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connecti` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `finishLoginAndWaitForClient` | `(Lcom/mojang/authlib/GameProfile;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleCustomQueryPacket` | `(Lnet/minecraft/network/protocol/login/ServerboundCustomQueryAnswerPac` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/network/Connection;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| wraps | `tick` | `()V` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `verifyLoginAndFinishConnectionSetup` | `(Lcom/mojang/authlib/GameProfile;)V` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (13 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final UNIQUE_THREAD_ID : Ljava/util/concurrent/atomic/AtomicInteger;
private static final LOGGER : Lorg/slf4j/Logger;
private static final MAX_TICKS_BEFORE_LOGIN : I
private final challenge : [B
private final server : Lnet/minecraft/server/MinecraftServer;
private final connection : Lnet/minecraft/network/Connection;
private final serverActivityMonitor : Lnet/minecraft/server/notifications/ServerActivityMonitor;
private state : Lnet/minecraft/server/network/ServerLoginPacketListenerImpl$State;
private tick : I
private requestedUsername : Ljava/lang/String;
private authenticatedProfile : Lcom/mojang/authlib/GameProfile;
private final serverId : Ljava/lang/String;
private final transferred : Z
public <init>(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connection;Z)V
public tick()V
public isAcceptingMessages()Z
public disconnect(Lnet/minecraft/network/chat/Component;)V
private isPlayerAlreadyInWorld(Lcom/mojang/authlib/GameProfile;)Z
public onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V
public getUserName()Ljava/lang/String;
public handleHello(Lnet/minecraft/network/protocol/login/ServerboundHelloPacket;)V
private startClientVerification(Lcom/mojang/authlib/GameProfile;)V
private verifyLoginAndFinishConnectionSetup(Lcom/mojang/authlib/GameProfile;)V
private finishLoginAndWaitForClient(Lcom/mojang/authlib/GameProfile;)V
public handleKey(Lnet/minecraft/network/protocol/login/ServerboundKeyPacket;)V
public handleCustomQueryPacket(Lnet/minecraft/network/protocol/login/ServerboundCustomQueryAnswerPacket;)V
public handleLoginAcknowledgement(Lnet/minecraft/network/protocol/login/ServerboundLoginAcknowledgedPacket;)V
public fillListenerSpecificCrashDetails(Lnet/minecraft/CrashReport;Lnet/minecraft/CrashReportCategory;)V
public handleCookieResponse(Lnet/minecraft/network/protocol/cookie/ServerboundCookieResponsePacket;)V
private synthetic lambda$fillListenerSpecificCrashDetails$0()Ljava/lang/String;
private synthetic lambda$verifyLoginAndFinishConnectionSetup$0()V
static <clinit>()V
```
