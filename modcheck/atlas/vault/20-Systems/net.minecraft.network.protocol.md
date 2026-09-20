---
type: "system"
package: "net.minecraft.network.protocol"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol

Analyst note: [[_authored/systems/net.minecraft.network.protocol|Packets and protocol phases]]

368 classes in the jar. Hooked types: 30

- [[40-Interfaces/net.minecraft.network.protocol.BundlePacket|BundlePacket]] -- calls:1, injects_into:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.Packet|Packet]] -- calls:3 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.PacketType|PacketType]] -- calls:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket|ClientboundCustomPayloadPacket]] -- calls:3 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.ClientboundPingPacket|ClientboundPingPacket]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.CommonPacketTypes|CommonPacketTypes]] -- reads:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket|ServerboundCustomPayloadPacket]] -- calls:4 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.ServerboundPongPacket|ServerboundPongPacket]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.custom.BrandPayload|BrandPayload]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload|CustomPacketPayload]] -- calls:13 -- by fabric-networking-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload_Type|CustomPacketPayload$Type]] -- calls:20 -- by fabric-client-gametest-api-v1, fabric-data-attachment-api-v1, fabric-networking-api-v1, fabric-recipe-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload_TypeAndCodec|CustomPacketPayload$TypeAndCodec]] -- calls:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks|ClientboundSelectKnownPacks]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks|ServerboundSelectKnownPacks]] -- injects_into:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.network.protocol.game.ClientboundBlockUpdatePacket|ClientboundBlockUpdatePacket]] -- calls:2 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ClientboundBundlePacket|ClientboundBundlePacket]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.game.ClientboundRespawnPacket|ClientboundRespawnPacket]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundAttackPacket|ServerboundAttackPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundInteractPacket|ServerboundInteractPacket]] -- calls:3 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPickItemFromBlockPacket|ServerboundPickItemFromBlockPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPickItemFromEntityPacket|ServerboundPickItemFromEntityPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPlayerActionPacket|ServerboundPlayerActionPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundPlayerActionPacket_Action|ServerboundPlayerActionPacket$Action]] -- reads:2 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundUseItemOnPacket|ServerboundUseItemOnPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.game.ServerboundUseItemPacket|ServerboundUseItemPacket]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundCustomQueryPacket|ClientboundCustomQueryPacket]] -- calls:6, injects_into:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundLoginCompressionPacket|ClientboundLoginCompressionPacket]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundLoginFinishedPacket|ClientboundLoginFinishedPacket]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket|ServerboundCustomQueryAnswerPacket]] -- calls:4, injects_into:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.protocol.login.custom.CustomQueryPayload|CustomQueryPayload]] -- calls:2 -- by fabric-networking-api-v1
