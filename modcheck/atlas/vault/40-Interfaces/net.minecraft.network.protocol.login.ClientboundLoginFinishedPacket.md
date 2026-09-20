---
type: "interface"
fqcn: "net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `gameProfile()Lcom/mojang/authlib/GameProfile;` | `` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket extends java.lang.Record implements net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.login.ClientLoginPacketListener> {
    private final com.mojang.authlib.GameProfile gameProfile;
    private final java.util.UUID sessionId;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket> STREAM_CODEC;
    public net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket(com.mojang.authlib.GameProfile, java.util.UUID);
    public net.minecraft.network.protocol.PacketType<net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket> type();
    public void handle(net.minecraft.network.protocol.login.ClientLoginPacketListener);
    public boolean isTerminal();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public com.mojang.authlib.GameProfile gameProfile();
    public java.util.UUID sessionId();
    public void handle(net.minecraft.network.PacketListener);
    static {};
}
```
