---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/network/protocol/common/ClientCommonPacketListener`, `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lne` | exact | invokespecial@4 in `ClientConfigurationPacketListenerImplMixin.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lne` | exact | invokespecial@4 in `ClientPacketListenerMixin.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleCustomPayload` | `(Lnet/minecraft/network/protocol/common/ClientboundCustomPayloadPacket` | exact | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/network/Connection;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |

## Declared members (15 fields, 44 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final GENERIC_DISCONNECT_MESSAGE : Lnet/minecraft/network/chat/Component;
private static final LOGGER : Lorg/slf4j/Logger;
protected final minecraft : Lnet/minecraft/client/Minecraft;
protected final connection : Lnet/minecraft/network/Connection;
protected final serverData : Lnet/minecraft/client/multiplayer/ServerData;
protected serverBrand : Ljava/lang/String;
protected final telemetryManager : Lnet/minecraft/client/telemetry/WorldSessionTelemetryManager;
protected final postDisconnectScreen : Lnet/minecraft/client/gui/screens/Screen;
protected isTransferring : Z
private final deferredPackets : Ljava/util/List;
protected final serverCookies : Ljava/util/Map;
protected customReportDetails : Ljava/util/Map;
private serverLinks : Lnet/minecraft/server/ServerLinks;
protected final seenPlayers : Ljava/util/Map;
protected seenInsecureChatWarning : Z
protected <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lnet/minecraft/client/multiplayer/CommonListenerCookie;)V
public serverLinks()Lnet/minecraft/server/ServerLinks;
public onPacketError(Lnet/minecraft/network/protocol/Packet;Ljava/lang/Exception;)V
public createDisconnectionInfo(Lnet/minecraft/network/chat/Component;Ljava/lang/Throwable;)Lnet/minecraft/network/DisconnectionDetails;
private storeDisconnectionReport(Lnet/minecraft/network/protocol/Packet;Ljava/lang/Throwable;)Ljava/util/Optional;
public shouldHandleMessage(Lnet/minecraft/network/protocol/Packet;)Z
public handleKeepAlive(Lnet/minecraft/network/protocol/common/ClientboundKeepAlivePacket;)V
public handlePing(Lnet/minecraft/network/protocol/common/ClientboundPingPacket;)V
public handleCustomPayload(Lnet/minecraft/network/protocol/common/ClientboundCustomPayloadPacket;)V
protected abstract handleCustomPayload(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V
public handleResourcePackPush(Lnet/minecraft/network/protocol/common/ClientboundResourcePackPushPacket;)V
public handleResourcePackPop(Lnet/minecraft/network/protocol/common/ClientboundResourcePackPopPacket;)V
public handlePostEffects(Lnet/minecraft/network/protocol/common/ClientboundPostEffectsPacket;)V
private static preparePackPrompt(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
private static parseResourcePackUrl(Ljava/lang/String;)Ljava/net/URL;
public handleRequestCookie(Lnet/minecraft/network/protocol/cookie/ClientboundCookieRequestPacket;)V
public handleStoreCookie(Lnet/minecraft/network/protocol/common/ClientboundStoreCookiePacket;)V
public handleCustomReportDetails(Lnet/minecraft/network/protocol/common/ClientboundCustomReportDetailsPacket;)V
public handleServerLinks(Lnet/minecraft/network/protocol/common/ClientboundServerLinksPacket;)V
public handleShowDialog(Lnet/minecraft/network/protocol/common/ClientboundShowDialogPacket;)V
protected abstract createDialogAccess()Lnet/minecraft/client/gui/screens/dialog/DialogConnectionAccess;
public showDialog(Lnet/minecraft/core/Holder;Lnet/minecraft/client/gui/screens/Screen;)V
protected showDialog(Lnet/minecraft/core/Holder;Lnet/minecraft/client/gui/screens/dialog/DialogConnectionAccess;Lnet/minecraft/client/gui/screens/Screen;)V
public handleClearDialog(Lnet/minecraft/network/protocol/common/ClientboundClearDialogPacket;)V
public clearDialog()V
public handleTransfer(Lnet/minecraft/network/protocol/common/ClientboundTransferPacket;)V
public handleDisconnect(Lnet/minecraft/network/protocol/common/ClientboundDisconnectPacket;)V
protected sendDeferredPackets()V
public send(Lnet/minecraft/network/protocol/Packet;)V
public onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V
public fillListenerSpecificCrashDetails(Lnet/minecraft/CrashReport;Lnet/minecraft/CrashReportCategory;)V
protected createDisconnectScreen(Lnet/minecraft/network/DisconnectionDetails;)Lnet/minecraft/client/gui/screens/Screen;
public serverBrand()Ljava/lang/String;
private sendWhen(Lnet/minecraft/network/protocol/Packet;Ljava/util/function/BooleanSupplier;Ljava/time/Duration;)V
private addOrUpdatePackPrompt(Ljava/util/UUID;Ljava/net/URL;Ljava/lang/String;ZLnet/minecraft/network/chat/Component;)Lnet/minecraft/client/gui/screens/Screen;
private synthetic lambda$createDisconnectScreen$0()Lnet/minecraft/client/gui/screens/Screen;
private synthetic lambda$fillListenerSpecificCrashDetails$2()Ljava/lang/String;
private synthetic lambda$fillListenerSpecificCrashDetails$1()Ljava/lang/String;
private synthetic lambda$fillListenerSpecificCrashDetails$0()Ljava/lang/String;
private synthetic lambda$handleResourcePackPop$1()V
private synthetic lambda$handleResourcePackPop$0(Ljava/util/UUID;)V
private static synthetic lambda$handleKeepAlive$0()Z
private static synthetic lambda$storeDisconnectionReport$0(Lnet/minecraft/server/ServerLinks$Entry;)Ljava/util/List;
static <clinit>()V
```
