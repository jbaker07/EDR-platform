---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.ServerboundPongPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.ServerboundPongPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getId()I` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.protocol.common.ServerboundPongPacket implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.common.ServerCommonPacketListener> {
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.FriendlyByteBuf, net.minecraft.network.protocol.common.ServerboundPongPacket> STREAM_CODEC;
    private final int id;
    public net.minecraft.network.protocol.common.ServerboundPongPacket(int);
    private net.minecraft.network.protocol.common.ServerboundPongPacket(net.minecraft.network.FriendlyByteBuf);
    private void write(net.minecraft.network.FriendlyByteBuf);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ServerboundPongPacket> type();
    public void handle(net.minecraft.network.protocol.common.ServerCommonPacketListener);
    public int getId();
    public void handle(net.minecraft.network.PacketListener);
    static {};
}
```
