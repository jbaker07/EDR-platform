---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/` | `` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.game.ClientGamePacketListener> {
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket> STREAM_CODEC;
    private final net.minecraft.core.BlockPos pos;
    private final net.minecraft.world.level.block.state.BlockState blockState;
    public net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket> type();
    public void handle(net.minecraft.network.protocol.game.ClientGamePacketListener);
    public net.minecraft.world.level.block.state.BlockState getBlockState();
    public net.minecraft.core.BlockPos getPos();
    public void handle(net.minecraft.network.PacketListener);
    static {};
}
```
