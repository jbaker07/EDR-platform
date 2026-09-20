---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking"
module: "fabric-networking-api-v1"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- kind: class

```java
public static <T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> boolean registerGlobalReceiver(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T>, net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking$ConfigurationPayloadHandler<T>)
public static net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking$ConfigurationPayloadHandler<?> unregisterGlobalReceiver(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<?>)
public static java.util.Set<net.minecraft.resources.Identifier> getGlobalReceivers()
public static <T extends net.minecraft.network.protocol.common.custom.CustomPacketPayload> boolean registerReceiver(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<T>, net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking$ConfigurationPayloadHandler<T>)
public static net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking$ConfigurationPayloadHandler<?> unregisterReceiver(net.minecraft.resources.Identifier)
public static java.util.Set<net.minecraft.resources.Identifier> getReceived() throws java.lang.IllegalStateException
public static java.util.Set<net.minecraft.resources.Identifier> getSendable() throws java.lang.IllegalStateException
public static boolean canSend(net.minecraft.resources.Identifier) throws java.lang.IllegalArgumentException
public static boolean canSend(net.minecraft.network.protocol.common.custom.CustomPacketPayload$Type<?>)
public static net.fabricmc.fabric.api.networking.v1.PacketSender getSender() throws java.lang.IllegalStateException
public static void send(net.minecraft.network.protocol.common.custom.CustomPacketPayload)
```
