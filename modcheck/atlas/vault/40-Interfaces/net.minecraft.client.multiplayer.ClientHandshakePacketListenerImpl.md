---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleCustomQuery` | `@Inject at INVOKE Ljava/util/function/Consumer;accept(Ljava/lang/Object;)V` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleLoginFinished` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (36, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl implements net.minecraft.network.protocol.login.ClientLoginPacketListener {
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.client.Minecraft minecraft;
    private final net.minecraft.client.multiplayer.ServerData serverData;
    private final net.minecraft.client.gui.screens.Screen parent;
    private final java.util.function.Consumer<net.minecraft.network.chat.Component> updateStatus;
    private final net.minecraft.network.Connection connection;
    private final boolean newWorld;
    private final java.time.Duration worldLoadDuration;
    private java.lang.String minigameName;
    private final net.minecraft.client.multiplayer.LevelLoadTracker levelLoadTracker;
    private final java.util.Map<net.minecraft.resources.Identifier, byte[]> cookies;
    private final boolean wasTransferredTo;
    private final java.util.Map<java.util.UUID, net.minecraft.client.multiplayer.PlayerInfo> seenPlayers;
    private final boolean seenInsecureChatWarning;
    private final java.util.concurrent.atomic.AtomicReference<net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl$State> state;
    public net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl(net.minecraft.network.Connection, net.minecraft.client.Minecraft, net.minecraft.client.multiplayer.ServerData, net.minecraft.client.gui.screens.Screen, boolean, java.time.Duration, java.util.function.Consumer<net.minecraft.network.chat.Component>, net.minecraft.client.multiplayer.LevelLoadTracker, net.minecraft.client.multiplayer.TransferState);
    private void switchState(net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl$State);
    public void handleHello(net.minecraft.network.protocol.login.ClientboundHelloPacket);
    private void setEncryption(net.minecraft.network.protocol.login.ServerboundKeyPacket, javax.crypto.Cipher, javax.crypto.Cipher);
    private net.minecraft.network.chat.Component authenticateServer(java.lang.String);
    public void handleLoginFinished(net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket);
    public void onDisconnect(net.minecraft.network.DisconnectionDetails);
    public boolean isAcceptingMessages();
    public void handleDisconnect(net.minecraft.network.protocol.login.ClientboundLoginDisconnectPacket);
    public void handleCompression(net.minecraft.network.protocol.login.ClientboundLoginCompressionPacket);
    public void handleCustomQuery(net.minecraft.network.protocol.login.ClientboundCustomQueryPacket);
    public void setMinigameName(java.lang.String);
    public void handleRequestCookie(net.minecraft.network.protocol.cookie.ClientboundCookieRequestPacket);
    public void fillListenerSpecificCrashDetails(net.minecraft.CrashReport, net.minecraft.CrashReportCategory);
    private java.lang.String lambda$fillListenerSpecificCrashDetails$2() throws java.lang.Exception;
    private java.lang.String lambda$fillListenerSpecificCrashDetails$1() throws java.lang.Exception;
    private java.lang.String lambda$fillListenerSpecificCrashDetails$0() throws java.lang.Exception;
    private void lambda$setEncryption$0(javax.crypto.Cipher, javax.crypto.Cipher);
    private void lambda$handleHello$0(java.lang.String, net.minecraft.network.protocol.login.ServerboundKeyPacket, javax.crypto.Cipher, javax.crypto.Cipher);
    private static net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl$State lambda$switchState$0(net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl$State, net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl$State);
    static {};
}
```
