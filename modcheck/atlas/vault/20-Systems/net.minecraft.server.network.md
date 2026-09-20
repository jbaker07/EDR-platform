---
type: "system"
package: "net.minecraft.server.network"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network

Analyst note: [[_authored/systems/net.minecraft.server.network|Server-side connection handling]]

53 classes (29 top-level) across 2 packages in the processed jar; 4 changed by Loom processing; 11 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.server.network.CommonListenerCookie|CommonListenerCookie]] -- calls:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.server.network.ConfigurationTask|ConfigurationTask]] -- calls:4 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.network.ConfigurationTask_Type|ConfigurationTask$Type]] -- calls:8 -- by fabric-data-attachment-api-v1, fabric-networking-api-v1, fabric-recipe-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.server.network.PlayerChunkSender|PlayerChunkSender]] -- wraps:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.server.network.ServerCommonPacketListenerImpl|ServerCommonPacketListenerImpl]] -- calls:3, injects_into:2, reads:2 -- by fabric-networking-api-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]] -- calls:26, injects_into:3, reads:2, wraps:1 -- by fabric-data-attachment-api-v1, fabric-networking-api-v1, fabric-particles-v1, fabric-recipe-api-v1, fabric-registry-sync-v0, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.network.ServerConnectionListener|ServerConnectionListener]] -- calls:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]] -- calls:14, injects_into:3, reads:5, wraps:3 -- by fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-menu-api-v1, fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.network.ServerHandshakePacketListenerImpl|ServerHandshakePacketListenerImpl]] -- injects_into:1, reads:3 -- by fabric-lifecycle-events-v1, fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]] -- calls:2, injects_into:3, reads:1, wraps:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.network.config.SynchronizeRegistriesTask|SynchronizeRegistriesTask]] -- calls:1, injects_into:3, reads:1 -- by fabric-resource-loader-v1

## Declared inventory

### `net.minecraft.server.network` (23 top-level)

[[40-Interfaces/net.minecraft.server.network.CommonListenerCookie|CommonListenerCookie]], [[40-Interfaces/net.minecraft.server.network.ConfigurationTask|ConfigurationTask]], `EventLoopGroupHolder`, `Filterable`, `FilteredText`, `LegacyProtocolUtils`, `LegacyQueryHandler`, `LegacyTextFilter`, `MemoryServerHandshakePacketListenerImpl`, [[40-Interfaces/net.minecraft.server.network.PlayerChunkSender|PlayerChunkSender]], `PlayerSafetyServiceTextFilter`, `ServerCommandSuggestionsProvider`, [[40-Interfaces/net.minecraft.server.network.ServerCommonPacketListenerImpl|ServerCommonPacketListenerImpl]], [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]], [[40-Interfaces/net.minecraft.server.network.ServerConnectionListener|ServerConnectionListener]], [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]], [[40-Interfaces/net.minecraft.server.network.ServerHandshakePacketListenerImpl|ServerHandshakePacketListenerImpl]], [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]], `ServerPlayerConnection`, `ServerStatusPacketListenerImpl`, `ServerTextFilter`, `TextFilter`, `package-info`

### `net.minecraft.server.network.config` (6 top-level)

`JoinWorldTask`, `PrepareSpawnTask`, `ServerCodeOfConductConfigurationTask`, `ServerResourcePackConfigurationTask`, [[40-Interfaces/net.minecraft.server.network.config.SynchronizeRegistriesTask|SynchronizeRegistriesTask]], `package-info`

