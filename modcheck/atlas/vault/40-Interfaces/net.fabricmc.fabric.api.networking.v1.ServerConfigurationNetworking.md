---
type: "interface"
fqcn: "net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: class

```java
public static <T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> boolean registerGlobalReceiver(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T>, net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking$ConfigurationPacketHandler<T>)
public static net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking$ConfigurationPacketHandler<?> unregisterGlobalReceiver(net.minecraft.resources.Identifier)
public static java.util.Set<net.minecraft.resources.Identifier> getGlobalReceivers()
public static <T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> boolean registerReceiver(net.minecraft.server.network.ServerConfigurationPacketListenerImpl, net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T>, net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking$ConfigurationPacketHandler<T>)
public static net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking$ConfigurationPacketHandler<?> unregisterReceiver(net.minecraft.server.network.ServerConfigurationPacketListenerImpl, net.minecraft.resources.Identifier)
public static java.util.Set<net.minecraft.resources.Identifier> getReceived(net.minecraft.server.network.ServerConfigurationPacketListenerImpl)
public static java.util.Set<net.minecraft.resources.Identifier> getSendable(net.minecraft.server.network.ServerConfigurationPacketListenerImpl)
public static boolean canSend(net.minecraft.server.network.ServerConfigurationPacketListenerImpl, net.minecraft.resources.Identifier)
public static boolean canSend(net.minecraft.server.network.ServerConfigurationPacketListenerImpl, net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<?>)
public static net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.common.ClientCommonPacketListener> createClientboundPacket(net.minecraft.network.protocol.common.custom.CustomPacketPayload)
public static net.fabricmc.fabric.api.networking.v1.PacketSender getSender(net.minecraft.server.network.ServerConfigurationPacketListenerImpl)
public static void send(net.minecraft.server.network.ServerConfigurationPacketListenerImpl, net.minecraft.network.protocol.common.custom.CustomPacketPayload)
public static net.minecraft.server.MinecraftServer getServer(net.minecraft.server.network.ServerConfigurationPacketListenerImpl)
public static boolean isReconfiguring(net.minecraft.server.network.ServerConfigurationPacketListenerImpl)
```
