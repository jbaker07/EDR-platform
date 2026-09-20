---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.ConnectScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.ConnectScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/screens/Screen`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `startConnecting` | `(Lnet/minecraft/client/gui/screens/Screen;Lnet/minecraft/client/Minecr` | exact | invokestatic@35 in `TestDedicatedServerContextImpl.lambda$connect$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (12 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final UNIQUE_THREAD_ID : Ljava/util/concurrent/atomic/AtomicInteger;
private static final LOGGER : Lorg/slf4j/Logger;
private static final NARRATION_DELAY_MS : J
public static final ABORT_CONNECTION : Lnet/minecraft/network/chat/Component;
public static final UNKNOWN_HOST_MESSAGE : Lnet/minecraft/network/chat/Component;
private connection : Lnet/minecraft/network/Connection;
private channelFuture : Lio/netty/channel/ChannelFuture;
private aborted : Z
private final parent : Lnet/minecraft/client/gui/screens/Screen;
private status : Lnet/minecraft/network/chat/Component;
private lastNarration : J
private final connectFailedTitle : Lnet/minecraft/network/chat/Component;
private <init>(Lnet/minecraft/client/gui/screens/Screen;Lnet/minecraft/network/chat/Component;)V
public static startConnecting(Lnet/minecraft/client/gui/screens/Screen;Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/multiplayer/resolver/ServerAddress;Lnet/minecraft/client/multiplayer/ServerData;ZLnet/minecraft/client/multiplayer/TransferState;)V
private connect(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/multiplayer/resolver/ServerAddress;Lnet/minecraft/client/multiplayer/ServerData;Lnet/minecraft/client/multiplayer/TransferState;)V
private updateStatus(Lnet/minecraft/network/chat/Component;)V
public tick()V
public shouldCloseOnEsc()Z
protected init()V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
private synthetic lambda$init$0(Lnet/minecraft/client/gui/components/Button;)V
static <clinit>()V
```
