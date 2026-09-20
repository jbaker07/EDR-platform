---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ClientboundBundlePacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ClientboundBundlePacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/Iterable;)V` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.protocol.game.ClientboundBundlePacket extends net.minecraft.network.protocol.BundlePacket<net.minecraft.network.protocol.game.ClientGamePacketListener> {
    public net.minecraft.network.protocol.game.ClientboundBundlePacket(java.lang.Iterable<net.minecraft.network.protocol.Packet<? super net.minecraft.network.protocol.game.ClientGamePacketListener>>);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.game.ClientboundBundlePacket> type();
    public void handle(net.minecraft.network.protocol.game.ClientGamePacketListener);
    public void handle(net.minecraft.network.PacketListener);
}
```
