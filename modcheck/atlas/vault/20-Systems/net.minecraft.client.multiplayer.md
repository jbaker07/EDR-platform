---
type: "system"
package: "net.minecraft.client.multiplayer"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer

Analyst note: [[_authored/systems/net.minecraft.client.multiplayer|Client-side world and connection]]

130 classes in the jar. Hooked types: 15

- [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]] -- injects_into:4 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache_Storage|ClientChunkCache$Storage]] -- calls:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl|ClientCommonPacketListenerImpl]] -- calls:2, injects_into:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]] -- injects_into:4 -- by fabric-lifecycle-events-v1, fabric-networking-api-v1, fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientDebugSubscriber|ClientDebugSubscriber]] -- injects_into:1 -- by fabric-debug-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]] -- injects_into:3 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel|ClientLevel]] -- calls:9, injects_into:4 -- by fabric-block-getter-api-v2, fabric-client-gametest-api-v1, fabric-lifecycle-events-v1, fabric-rendering-v1, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel_EntityCallbacks|ClientLevel$EntityCallbacks]] -- injects_into:2 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] -- calls:7, injects_into:15 -- by fabric-command-api-v2, fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-lifecycle-events-v1, fabric-message-api-v1, fabric-networking-api-v1, fabric-recipe-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientRecipeContainer|ClientRecipeContainer]] -- calls:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.KnownPacksManager|KnownPacksManager]] -- wraps:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]] -- calls:1, injects_into:6, wraps:1 -- by fabric-events-interaction-v0, fabric-item-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ServerData|ServerData]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]] -- injects_into:5 -- by fabric-message-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.resolver.ServerAddress|ServerAddress]] -- calls:1 -- by fabric-client-gametest-api-v1
