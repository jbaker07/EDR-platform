---
type: "interface"
fqcn: "net.minecraft.server.network.ServerHandshakePacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerHandshakePacketListenerImpl

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/network/protocol/handshake/ServerHandshakePacketListener`, `net/fabricmc/fabric/api/networking/v1/context/PacketContextProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `handleIntention` | `(Lnet/minecraft/network/protocol/handshake/ClientIntentionPacket;)V` | name_only | @Inject at ['HEAD'] | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/network/Connection;` | exact | @Shadow declaration | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |
| reads | `connection` | `Lnet/minecraft/network/Connection;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| reads | `server` | `Lnet/minecraft/server/MinecraftServer;` | exact | @Shadow declaration | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |

## Declared members (3 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final IGNORE_STATUS_REASON : Lnet/minecraft/network/chat/Component;
private final server : Lnet/minecraft/server/MinecraftServer;
private final connection : Lnet/minecraft/network/Connection;
public <init>(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connection;)V
public handleIntention(Lnet/minecraft/network/protocol/handshake/ClientIntentionPacket;)V
private beginLogin(Lnet/minecraft/network/protocol/handshake/ClientIntentionPacket;Z)V
public onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V
public isAcceptingMessages()Z
static <clinit>()V
```
