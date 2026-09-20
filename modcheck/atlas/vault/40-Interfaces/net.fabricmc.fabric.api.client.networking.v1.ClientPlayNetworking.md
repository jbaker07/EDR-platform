---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: class

```java
public static boolean registerGlobalReceiver(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type, net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking$PlayPayloadHandler)
public static net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking$PlayPayloadHandler unregisterGlobalReceiver(net.minecraft.resources.Identifier)
public static java.util.Set getGlobalReceivers()
public static boolean registerReceiver(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type, net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking$PlayPayloadHandler)
public static net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking$PlayPayloadHandler unregisterReceiver(net.minecraft.resources.Identifier)
public static java.util.Set getReceived()
public static java.util.Set getSendable()
public static boolean canSend(net.minecraft.resources.Identifier)
public static boolean canSend(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type)
public static net.minecraft.network.protocol.Packet createServerboundPacket(net.minecraft.network.protocol.common.custom.CustomPacketPayload)
public static net.fabricmc.fabric.api.networking.v1.PacketSender getSender()
public static void send(net.minecraft.network.protocol.common.custom.CustomPacketPayload)
```
