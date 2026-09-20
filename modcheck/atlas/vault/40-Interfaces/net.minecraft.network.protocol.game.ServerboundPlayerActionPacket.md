---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ServerboundPlayerActionPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ServerboundPlayerActionPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/network/protocol/game/ServerboundPlayerActio` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.protocol.game.ServerboundPlayerActionPacket implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.game.ServerGamePacketListener> {
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.FriendlyByteBuf, net.minecraft.network.protocol.game.ServerboundPlayerActionPacket> STREAM_CODEC;
    private final net.minecraft.core.BlockPos pos;
    private final net.minecraft.core.Direction direction;
    private final net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action action;
    private final int sequence;
    public net.minecraft.network.protocol.game.ServerboundPlayerActionPacket(net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action, net.minecraft.core.BlockPos, net.minecraft.core.Direction, int);
    public net.minecraft.network.protocol.game.ServerboundPlayerActionPacket(net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action, net.minecraft.core.BlockPos, net.minecraft.core.Direction);
    private net.minecraft.network.protocol.game.ServerboundPlayerActionPacket(net.minecraft.network.FriendlyByteBuf);
    private void write(net.minecraft.network.FriendlyByteBuf);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.game.ServerboundPlayerActionPacket> type();
    public void handle(net.minecraft.network.protocol.game.ServerGamePacketListener);
    public net.minecraft.core.BlockPos getPos();
    public net.minecraft.core.Direction getDirection();
    public net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action getAction();
    public int getSequence();
    public void handle(net.minecraft.network.PacketListener);
    static {};
}
```
