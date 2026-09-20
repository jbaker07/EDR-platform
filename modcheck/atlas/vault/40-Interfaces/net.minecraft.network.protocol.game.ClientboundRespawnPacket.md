---
type: "interface"
fqcn: "net.minecraft.network.protocol.game.ClientboundRespawnPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.game.ClientboundRespawnPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `shouldKeep(B)Z` | `` | client | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.game.ClientboundRespawnPacket extends java.lang.Record implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.game.ClientGamePacketListener> {
    private final net.minecraft.network.protocol.game.CommonPlayerSpawnInfo commonPlayerSpawnInfo;
    private final byte dataToKeep;
    public static final net.minecraft.network.codec.StreamCodec<net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.network.protocol.game.ClientboundRespawnPacket> STREAM_CODEC;
    public static final byte KEEP_ATTRIBUTE_MODIFIERS;
    public static final byte KEEP_ENTITY_DATA;
    public static final byte KEEP_ALL_DATA;
    public net.minecraft.network.protocol.game.ClientboundRespawnPacket(net.minecraft.network.protocol.game.CommonPlayerSpawnInfo, byte);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.game.ClientboundRespawnPacket> type();
    public void handle(net.minecraft.network.protocol.game.ClientGamePacketListener);
    public boolean shouldKeep(byte);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.network.protocol.game.CommonPlayerSpawnInfo commonPlayerSpawnInfo();
    public byte dataToKeep();
    public void handle(net.minecraft.network.PacketListener);
    static {};
}
```
