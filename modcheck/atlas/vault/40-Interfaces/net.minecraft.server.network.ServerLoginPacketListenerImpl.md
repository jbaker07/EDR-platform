---
type: "interface"
fqcn: "net.minecraft.server.network.ServerLoginPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerLoginPacketListenerImpl

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getUserName()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `finishLoginAndWaitForClient` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleCustomQueryPacket` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `tick` | `@Redirect at INVOKE Lnet/minecraft/server/network/ServerLoginPacketListenerImpl;` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| wraps | `verifyLoginAndFinishConnectionSetup` | `@Redirect at INVOKE Lnet/minecraft/server/MinecraftServer;getCompressionThreshol` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (32, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.network.ServerLoginPacketListenerImpl implements net.minecraft.network.protocol.login.ServerLoginPacketListener,net.minecraft.network.TickablePacketListener {
    private static final java.util.concurrent.atomic.AtomicInteger UNIQUE_THREAD_ID;
    private static final org.slf4j.Logger LOGGER;
    private static final int MAX_TICKS_BEFORE_LOGIN;
    private final byte[] challenge;
    private final net.minecraft.server.MinecraftServer server;
    private final net.minecraft.network.Connection connection;
    private final net.minecraft.server.notifications.ServerActivityMonitor serverActivityMonitor;
    private volatile net.minecraft.server.network.ServerLoginPacketListenerImpl$State state;
    private int tick;
    private java.lang.String requestedUsername;
    private com.mojang.authlib.GameProfile authenticatedProfile;
    private final java.lang.String serverId;
    private final boolean transferred;
    public net.minecraft.server.network.ServerLoginPacketListenerImpl(net.minecraft.server.MinecraftServer, net.minecraft.network.Connection, boolean);
    public void tick();
    public boolean isAcceptingMessages();
    public void disconnect(net.minecraft.network.chat.Component);
    private boolean isPlayerAlreadyInWorld(com.mojang.authlib.GameProfile);
    public void onDisconnect(net.minecraft.network.DisconnectionDetails);
    public java.lang.String getUserName();
    public void handleHello(net.minecraft.network.protocol.login.ServerboundHelloPacket);
    private void startClientVerification(com.mojang.authlib.GameProfile);
    private void verifyLoginAndFinishConnectionSetup(com.mojang.authlib.GameProfile);
    private void finishLoginAndWaitForClient(com.mojang.authlib.GameProfile);
    public void handleKey(net.minecraft.network.protocol.login.ServerboundKeyPacket);
    public void handleCustomQueryPacket(net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket);
    public void handleLoginAcknowledgement(net.minecraft.network.protocol.login.ServerboundLoginAcknowledgedPacket);
    public void fillListenerSpecificCrashDetails(net.minecraft.CrashReport, net.minecraft.CrashReportCategory);
    public void handleCookieResponse(net.minecraft.network.protocol.cookie.ServerboundCookieResponsePacket);
    private java.lang.String lambda$fillListenerSpecificCrashDetails$0() throws java.lang.Exception;
    private void lambda$verifyLoginAndFinishConnectionSetup$0();
    static {};
}
```
