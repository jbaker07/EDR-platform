---
type: "interface"
fqcn: "net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/network/protocol/common/custom/CustomPacketP` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload()Lnet/minecraft/network/protocol/common/custom/CustomPacket` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket extends java.lang.Record implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.common.ServerCommonPacketListener> {
    private final net.minecraft.network.protocol.common.custom.CustomPacketPayload payload;
    private static final int MAX_PAYLOAD_SIZE;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.FriendlyByteBuf, net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket> STREAM_CODEC;
    public net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket(net.minecraft.network.protocol.common.custom.CustomPacketPayload);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket> type();
    public void handle(net.minecraft.network.protocol.common.ServerCommonPacketListener);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.protocol.common.custom.CustomPacketPayload payload();
    public void handle(net.minecraft.network.PacketListener);
    private static void lambda$static$1(java.util.ArrayList);
    private static net.minecraft.network.codec.StreamCodec lambda$static$0(net.minecraft.resources.Identifier);
    static {};
}
```
