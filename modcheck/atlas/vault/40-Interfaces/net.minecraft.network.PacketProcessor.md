---
type: "interface"
fqcn: "net.minecraft.network.PacketProcessor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.PacketProcessor

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `isSameThread()Z` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `isSameThread()Z` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `scheduleIfPossible(Lnet/minecraft/network/PacketListener;Lnet/minecraft/networ` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `scheduleIfPossible(Lnet/minecraft/network/PacketListener;Lnet/minecraft/networ` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `scheduleIfPossible(Lnet/minecraft/network/PacketListener;Lnet/minecraft/networ` | `` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.PacketProcessor implements java.lang.AutoCloseable {
    private static final org.slf4j.Logger LOGGER;
    private final java.util.Queue<net.minecraft.network.PacketProcessor$ListenerAndPacket<?>> packetsToBeHandled;
    private final java.lang.Thread runningThread;
    private boolean closed;
    public net.minecraft.network.PacketProcessor(java.lang.Thread);
    public boolean isSameThread();
    public <T extends net.minecraft.network.PacketListener> void scheduleIfPossible(T, net.minecraft.network.protocol.Packet<T>);
    public void processQueuedPackets();
    public void close();
    static {};
}
```
