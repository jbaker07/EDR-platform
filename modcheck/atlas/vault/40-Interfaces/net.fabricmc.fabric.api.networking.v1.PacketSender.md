---
type: "interface"
fqcn: "net.fabricmc.fabric.api.networking.v1.PacketSender"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.PacketSender

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: interface

```java
public abstract net.minecraft.network.protocol.Packet<?> createPacket(net.minecraft.network.protocol.common.custom.CustomPacketPayload)
public default void sendPacket(net.minecraft.network.protocol.Packet<?>)
public default void sendPacket(net.minecraft.network.protocol.common.custom.CustomPacketPayload)
public abstract void sendPacket(net.minecraft.network.protocol.Packet<?>, io.netty.channel.ChannelFutureListener)
public default void sendPacket(net.minecraft.network.protocol.common.custom.CustomPacketPayload, io.netty.channel.ChannelFutureListener)
public abstract void disconnect(net.minecraft.network.chat.Component)
```
