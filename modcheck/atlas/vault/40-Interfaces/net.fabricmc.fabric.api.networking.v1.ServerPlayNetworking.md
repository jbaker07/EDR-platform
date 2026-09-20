---
type: "interface"
fqcn: "net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: class

```java
public static boolean registerGlobalReceiver(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type, net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking$PlayPayloadHandler)
public static net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking$PlayPayloadHandler unregisterGlobalReceiver(net.minecraft.resources.Identifier)
public static java.util.Set getGlobalReceivers()
public static boolean registerReceiver(net.minecraft.server.network.ServerGamePacketListenerImpl, net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type, net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking$PlayPayloadHandler)
public static net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking$PlayPayloadHandler unregisterReceiver(net.minecraft.server.network.ServerGamePacketListenerImpl, net.minecraft.resources.Identifier)
public static java.util.Set getReceived(net.minecraft.server.level.ServerPlayer)
public static java.util.Set getReceived(net.minecraft.server.network.ServerGamePacketListenerImpl)
public static java.util.Set getSendable(net.minecraft.server.level.ServerPlayer)
public static java.util.Set getSendable(net.minecraft.server.network.ServerGamePacketListenerImpl)
public static boolean canSend(net.minecraft.server.level.ServerPlayer, net.minecraft.resources.Identifier)
public static boolean canSend(net.minecraft.server.level.ServerPlayer, net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type)
public static boolean canSend(net.minecraft.server.network.ServerGamePacketListenerImpl, net.minecraft.resources.Identifier)
public static boolean canSend(net.minecraft.server.network.ServerGamePacketListenerImpl, net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type)
public static net.minecraft.network.protocol.Packet createClientboundPacket(net.minecraft.network.protocol.common.custom.CustomPacketPayload)
public static net.fabricmc.fabric.api.networking.v1.PacketSender getSender(net.minecraft.server.level.ServerPlayer)
public static net.fabricmc.fabric.api.networking.v1.PacketSender getSender(net.minecraft.server.network.ServerGamePacketListenerImpl)
public static void send(net.minecraft.server.level.ServerPlayer, net.minecraft.network.protocol.common.custom.CustomPacketPayload)
public static void reconfigure(net.minecraft.server.level.ServerPlayer)
public static void reconfigure(net.minecraft.server.network.ServerGamePacketListenerImpl)
```
