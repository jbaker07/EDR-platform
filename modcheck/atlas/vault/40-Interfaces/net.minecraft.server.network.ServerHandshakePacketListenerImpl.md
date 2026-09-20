---
type: "interface"
fqcn: "net.minecraft.server.network.ServerHandshakePacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerHandshakePacketListenerImpl

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `handleIntention` | `@Inject at HEAD` | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.network.ServerHandshakePacketListenerImpl implements net.minecraft.network.protocol.handshake.ServerHandshakePacketListener {
    private static final net.minecraft.network.chat.Component IGNORE_STATUS_REASON;
    private final net.minecraft.server.MinecraftServer server;
    private final net.minecraft.network.Connection connection;
    public net.minecraft.server.network.ServerHandshakePacketListenerImpl(net.minecraft.server.MinecraftServer, net.minecraft.network.Connection);
    public void handleIntention(net.minecraft.network.protocol.handshake.ClientIntentionPacket);
    private void beginLogin(net.minecraft.network.protocol.handshake.ClientIntentionPacket, boolean);
    public void onDisconnect(net.minecraft.network.DisconnectionDetails);
    public boolean isAcceptingMessages();
    static {};
}
```
