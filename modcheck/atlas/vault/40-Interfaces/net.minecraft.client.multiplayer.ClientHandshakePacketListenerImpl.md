---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/network/protocol/login/ClientLoginPacketListener`, `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Lnet/minecraft/network/Connection;Lnet/minecraft/client/Minecraft;Lne` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleCustomQuery` | `(Lnet/minecraft/network/protocol/login/ClientboundCustomQueryPacket;)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleLoginFinished` | `(Lnet/minecraft/network/protocol/login/ClientboundLoginFinishedPacket;` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/network/Connection;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |

## Declared members (15 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final minecraft : Lnet/minecraft/client/Minecraft;
private final serverData : Lnet/minecraft/client/multiplayer/ServerData;
private final parent : Lnet/minecraft/client/gui/screens/Screen;
private final updateStatus : Ljava/util/function/Consumer;
private final connection : Lnet/minecraft/network/Connection;
private final newWorld : Z
private final worldLoadDuration : Ljava/time/Duration;
private minigameName : Ljava/lang/String;
private final levelLoadTracker : Lnet/minecraft/client/multiplayer/LevelLoadTracker;
private final cookies : Ljava/util/Map;
private final wasTransferredTo : Z
private final seenPlayers : Ljava/util/Map;
private final seenInsecureChatWarning : Z
private final state : Ljava/util/concurrent/atomic/AtomicReference;
public <init>(Lnet/minecraft/network/Connection;Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/multiplayer/ServerData;Lnet/minecraft/client/gui/screens/Screen;ZLjava/time/Duration;Ljava/util/function/Consumer;Lnet/minecraft/client/multiplayer/LevelLoadTracker;Lnet/minecraft/client/multiplayer/TransferState;)V
private switchState(Lnet/minecraft/client/multiplayer/ClientHandshakePacketListenerImpl$State;)V
public handleHello(Lnet/minecraft/network/protocol/login/ClientboundHelloPacket;)V
private setEncryption(Lnet/minecraft/network/protocol/login/ServerboundKeyPacket;Ljavax/crypto/Cipher;Ljavax/crypto/Cipher;)V
private authenticateServer(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
public handleLoginFinished(Lnet/minecraft/network/protocol/login/ClientboundLoginFinishedPacket;)V
public onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V
public isAcceptingMessages()Z
public handleDisconnect(Lnet/minecraft/network/protocol/login/ClientboundLoginDisconnectPacket;)V
public handleCompression(Lnet/minecraft/network/protocol/login/ClientboundLoginCompressionPacket;)V
public handleCustomQuery(Lnet/minecraft/network/protocol/login/ClientboundCustomQueryPacket;)V
public setMinigameName(Ljava/lang/String;)V
public handleRequestCookie(Lnet/minecraft/network/protocol/cookie/ClientboundCookieRequestPacket;)V
public fillListenerSpecificCrashDetails(Lnet/minecraft/CrashReport;Lnet/minecraft/CrashReportCategory;)V
private synthetic lambda$fillListenerSpecificCrashDetails$2()Ljava/lang/String;
private synthetic lambda$fillListenerSpecificCrashDetails$1()Ljava/lang/String;
private synthetic lambda$fillListenerSpecificCrashDetails$0()Ljava/lang/String;
private synthetic lambda$setEncryption$0(Ljavax/crypto/Cipher;Ljavax/crypto/Cipher;)V
private synthetic lambda$handleHello$0(Ljava/lang/String;Lnet/minecraft/network/protocol/login/ServerboundKeyPacket;Ljavax/crypto/Cipher;Ljavax/crypto/Cipher;)V
private static synthetic lambda$switchState$0(Lnet/minecraft/client/multiplayer/ClientHandshakePacketListenerImpl$State;Lnet/minecraft/client/multiplayer/ClientHandshakePacketListenerImpl$State;)Lnet/minecraft/client/multiplayer/ClientHandshakePacketListenerImpl$State;
static <clinit>()V
```
