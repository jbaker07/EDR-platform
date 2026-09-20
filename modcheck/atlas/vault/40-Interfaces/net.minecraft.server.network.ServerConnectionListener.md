---
type: "interface"
fqcn: "net.minecraft.server.network.ServerConnectionListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerConnectionListener

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getConnections` | `()Ljava/util/List;` | exact | invokevirtual@14 in `GlobalAttachmentsImpl.fabric_syncChange` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (7 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final server : Lnet/minecraft/server/MinecraftServer;
public running : Z
private sessionId : Ljava/util/UUID;
private final channels : Ljava/util/List;
private final connections : Ljava/util/List;
private final pendingConnections : Ljava/util/Queue;
public <init>(Lnet/minecraft/server/MinecraftServer;)V
public startTcpServerListener(Ljava/net/InetAddress;I)V
public startMemoryChannel()Ljava/net/SocketAddress;
public stop()V
public stopTcpServerListener()V
public tick()V
public getServer()Lnet/minecraft/server/MinecraftServer;
private addPendingConnections()V
public getConnections()Ljava/util/List;
public getSessionId()Ljava/util/UUID;
private static synthetic lambda$tick$0(Lnet/minecraft/network/Connection;Lnet/minecraft/network/chat/Component;)V
static <clinit>()V
```
