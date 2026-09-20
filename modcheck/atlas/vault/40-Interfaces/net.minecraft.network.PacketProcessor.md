---
type: "interface"
fqcn: "net.minecraft.network.PacketProcessor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.PacketProcessor

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isSameThread` | `()Z` | exact | invokevirtual@7 in `ClientPlayNetworkAddon.isOnReceiveThread` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `isSameThread` | `()Z` | exact | invokevirtual@7 in `ServerPlayNetworkAddon.isOnReceiveThread` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `scheduleIfPossible` | `(Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/protocol` | exact | invokevirtual@73 in `ServerCommonPacketListenerImplMixin.handleCustomPayloadReceivedAsync` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `scheduleIfPossible` | `(Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/protocol` | exact | invokevirtual@31 in `ServerGamePacketListenerImplMixin.handleCustomPayloadReceivedAsync` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `scheduleIfPossible` | `(Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/protocol` | exact | invokevirtual@105 in `ClientCommonPacketListenerImplMixin.onCustomPayload` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (4 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final packetsToBeHandled : Ljava/util/Queue;
private final runningThread : Ljava/lang/Thread;
private closed : Z
public <init>(Ljava/lang/Thread;)V
public isSameThread()Z
public scheduleIfPossible(Lnet/minecraft/network/PacketListener;Lnet/minecraft/network/protocol/Packet;)V
public processQueuedPackets()V
public close()V
static <clinit>()V
```
