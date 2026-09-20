---
type: "interface"
fqcn: "net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(ILnet/minecraft/network/protocol/login/custom/CustomQueryAn` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload()Lnet/minecraft/network/protocol/login/custom/CustomQueryAn` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `payload()Lnet/minecraft/network/protocol/login/custom/CustomQueryAn` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `transactionId()I` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `readPayload` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket extends java.lang.Record implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.login.ServerLoginPacketListener> {
    private final int transactionId;
    private final net.minecraft.network.protocol.login.custom.CustomQueryAnswerPayload payload;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.FriendlyByteBuf, net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket> STREAM_CODEC;
    private static final int MAX_PAYLOAD_SIZE;
    public net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket(int, net.minecraft.network.protocol.login.custom.CustomQueryAnswerPayload);
    private static net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket read(net.minecraft.network.FriendlyByteBuf);
    private static net.minecraft.network.protocol.login.custom.CustomQueryAnswerPayload readPayload(int, net.minecraft.network.FriendlyByteBuf);
    private static net.minecraft.network.protocol.login.custom.CustomQueryAnswerPayload readUnknownPayload(net.minecraft.network.FriendlyByteBuf);
    private void write(net.minecraft.network.FriendlyByteBuf);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket> type();
    public void handle(net.minecraft.network.protocol.login.ServerLoginPacketListener);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public int transactionId();
    public net.minecraft.network.protocol.login.custom.CustomQueryAnswerPayload payload();
    public void handle(net.minecraft.network.PacketListener);
    private static void lambda$write$0(net.minecraft.network.FriendlyByteBuf, net.minecraft.network.protocol.login.custom.CustomQueryAnswerPayload);
    static {};
}
```
