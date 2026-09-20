---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ServerboundUseItemOnPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ServerboundUseItemOnPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/p` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.game.ServerboundUseItemOnPacket extends java.lang.Record implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.game.ServerGamePacketListener> {
    private final net.minecraft.world.InteractionHand hand;
    private final net.minecraft.world.phys.BlockHitResult hitResult;
    private final int sequence;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.FriendlyByteBuf, net.minecraft.network.protocol.game.ServerboundUseItemOnPacket> STREAM_CODEC;
    public net.minecraft.network.protocol.game.ServerboundUseItemOnPacket(net.minecraft.world.InteractionHand, net.minecraft.world.phys.BlockHitResult, int);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.game.ServerboundUseItemOnPacket> type();
    public void handle(net.minecraft.network.protocol.game.ServerGamePacketListener);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.InteractionHand hand();
    public net.minecraft.world.phys.BlockHitResult hitResult();
    public int sequence();
    public void handle(net.minecraft.network.PacketListener);
    static {};
}
```
