---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ServerboundPickItemFromEntityPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ServerboundPickItemFromEntityPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `includeData()Z` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.game.ServerboundPickItemFromEntityPacket extends java.lang.Record implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.game.ServerGamePacketListener> {
    private final int id;
    private final boolean includeData;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.network.protocol.game.ServerboundPickItemFromEntityPacket> STREAM_CODEC;
    public net.minecraft.network.protocol.game.ServerboundPickItemFromEntityPacket(int, boolean);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.game.ServerboundPickItemFromEntityPacket> type();
    public void handle(net.minecraft.network.protocol.game.ServerGamePacketListener);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public int id();
    public boolean includeData();
    public void handle(net.minecraft.network.PacketListener);
    static {};
}
```
