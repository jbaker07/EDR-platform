---
type: "system"
package: "net.minecraft.server.network"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network

Analyst note: [[_authored/systems/net.minecraft.server.network|Server-side connection handling]]

53 classes in the jar. Hooked types: 10

- [[40-Interfaces/net.minecraft.server.network.CommonListenerCookie|CommonListenerCookie]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.server.network.ConfigurationTask|ConfigurationTask]] -- calls:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.network.ConfigurationTask_Type|ConfigurationTask$Type]] -- calls:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.network.ServerCommonPacketListenerImpl|ServerCommonPacketListenerImpl]] -- calls:3, injects_into:2 -- by fabric-networking-api-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]] -- calls:18, injects_into:3 -- by fabric-data-attachment-api-v1, fabric-networking-api-v1, fabric-particles-v1, fabric-recipe-api-v1, fabric-registry-sync-v0, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.network.ServerConnectionListener|ServerConnectionListener]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]] -- calls:7, injects_into:3 -- by fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-menu-api-v1, fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.network.ServerHandshakePacketListenerImpl|ServerHandshakePacketListenerImpl]] -- injects_into:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]] -- calls:1, injects_into:3, wraps:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.network.config.SynchronizeRegistriesTask|SynchronizeRegistriesTask]] -- injects_into:3 -- by fabric-resource-loader-v1
