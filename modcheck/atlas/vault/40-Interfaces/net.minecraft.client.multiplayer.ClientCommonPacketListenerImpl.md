---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Conn` | `` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Conn` | `` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleCustomPayload(Lnet/minecraft/network/protocol/common/ClientboundCustomPayloadPacket;)V` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (59, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl implements net.minecraft.network.protocol.common.ClientCommonPacketListener {
    private static final net.minecraft.network.chat.Component GENERIC_DISCONNECT_MESSAGE;
    private static final org.slf4j.Logger LOGGER;
    protected final net.minecraft.client.Minecraft minecraft;
    protected final net.minecraft.network.Connection connection;
    protected final net.minecraft.client.multiplayer.ServerData serverData;
    protected java.lang.String serverBrand;
    protected final net.minecraft.client.telemetry.WorldSessionTelemetryManager telemetryManager;
    protected final net.minecraft.client.gui.screens.Screen postDisconnectScreen;
    protected boolean isTransferring;
    private final java.util.List<net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl$DeferredPacket> deferredPackets;
    protected final java.util.Map<net.minecraft.resources.Identifier, byte[]> serverCookies;
    protected java.util.Map<java.lang.String, java.lang.String> customReportDetails;
    private net.minecraft.server.ServerLinks serverLinks;
    protected final java.util.Map<java.util.UUID, net.minecraft.client.multiplayer.PlayerInfo> seenPlayers;
    protected boolean seenInsecureChatWarning;
    protected net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl(net.minecraft.client.Minecraft, net.minecraft.network.Connection, net.minecraft.client.multiplayer.CommonListenerCookie);
    public net.minecraft.server.ServerLinks serverLinks();
    public void onPacketError(net.minecraft.network.protocol.Packet, java.lang.Exception);
    public net.minecraft.network.DisconnectionDetails createDisconnectionInfo(net.minecraft.network.chat.Component, java.lang.Throwable);
    private java.util.Optional<java.nio.file.Path> storeDisconnectionReport(net.minecraft.network.protocol.Packet, java.lang.Throwable);
    public boolean shouldHandleMessage(net.minecraft.network.protocol.Packet<?>);
    public void handleKeepAlive(net.minecraft.network.protocol.common.ClientboundKeepAlivePacket);
    public void handlePing(net.minecraft.network.protocol.common.ClientboundPingPacket);
    public void handleCustomPayload(net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket);
    protected abstract void handleCustomPayload(net.minecraft.network.protocol.common.custom.CustomPacketPayload);
    public void handleResourcePackPush(net.minecraft.network.protocol.common.ClientboundResourcePackPushPacket);
    public void handleResourcePackPop(net.minecraft.network.protocol.common.ClientboundResourcePackPopPacket);
    public void handlePostEffects(net.minecraft.network.protocol.common.ClientboundPostEffectsPacket);
    private static net.minecraft.network.chat.Component preparePackPrompt(net.minecraft.network.chat.Component, net.minecraft.network.chat.Component);
    private static java.net.URL parseResourcePackUrl(java.lang.String);
    public void handleRequestCookie(net.minecraft.network.protocol.cookie.ClientboundCookieRequestPacket);
    public void handleStoreCookie(net.minecraft.network.protocol.common.ClientboundStoreCookiePacket);
    public void handleCustomReportDetails(net.minecraft.network.protocol.common.ClientboundCustomReportDetailsPacket);
    public void handleServerLinks(net.minecraft.network.protocol.common.ClientboundServerLinksPacket);
    public void handleShowDialog(net.minecraft.network.protocol.common.ClientboundShowDialogPacket);
    protected abstract net.minecraft.client.gui.screens.dialog.DialogConnectionAccess createDialogAccess();
    public void showDialog(net.minecraft.core.Holder<net.minecraft.server.dialog.Dialog>, net.minecraft.client.gui.screens.Screen);
    protected void showDialog(net.minecraft.core.Holder<net.minecraft.server.dialog.Dialog>, net.minecraft.client.gui.screens.dialog.DialogConnectionAccess, net.minecraft.client.gui.screens.Screen);
    public void handleClearDialog(net.minecraft.network.protocol.common.ClientboundClearDialogPacket);
    public void clearDialog();
    public void handleTransfer(net.minecraft.network.protocol.common.ClientboundTransferPacket);
    public void handleDisconnect(net.minecraft.network.protocol.common.ClientboundDisconnectPacket);
    protected void sendDeferredPackets();
    public void send(net.minecraft.network.protocol.Packet<?>);
    public void onDisconnect(net.minecraft.network.DisconnectionDetails);
    public void fillListenerSpecificCrashDetails(net.minecraft.CrashReport, net.minecraft.CrashReportCategory);
    protected net.minecraft.client.gui.screens.Screen createDisconnectScreen(net.minecraft.network.DisconnectionDetails);
    public java.lang.String serverBrand();
    private void sendWhen(net.minecraft.network.protocol.Packet<? extends net.minecraft.network.ServerboundPacketListener>, java.util.function.BooleanSupplier, java.time.Duration);
    private net.minecraft.client.gui.screens.Screen addOrUpdatePackPrompt(java.util.UUID, java.net.URL, java.lang.String, boolean, net.minecraft.network.chat.Component);
    private net.minecraft.client.gui.screens.Screen lambda$createDisconnectScreen$0();
    private java.lang.String lambda$fillListenerSpecificCrashDetails$2() throws java.lang.Exception;
    private java.lang.String lambda$fillListenerSpecificCrashDetails$1() throws java.lang.Exception;
    private java.lang.String lambda$fillListenerSpecificCrashDetails$0() throws java.lang.Exception;
    private void lambda$handleResourcePackPop$1();
    private void lambda$handleResourcePackPop$0(java.util.UUID);
    private static boolean lambda$handleKeepAlive$0();
    private static java.util.List lambda$storeDisconnectionReport$0(net.minecraft.server.ServerLinks$Entry);
    static {};
}
```
