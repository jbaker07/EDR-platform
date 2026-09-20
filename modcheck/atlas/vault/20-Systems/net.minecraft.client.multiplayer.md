---
type: "system"
package: "net.minecraft.client.multiplayer"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer

Analyst note: [[_authored/systems/net.minecraft.client.multiplayer|Client-side world and connection]]

130 classes (63 top-level) across 5 packages in the processed jar; 4 changed by Loom processing; 18 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]] -- injects_into:4, reads:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache_Storage|ClientChunkCache$Storage]] -- calls:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl|ClientCommonPacketListenerImpl]] -- calls:2, injects_into:1, reads:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]] -- calls:1, injects_into:4 -- by fabric-lifecycle-events-v1, fabric-networking-api-v1, fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientDebugSubscriber|ClientDebugSubscriber]] -- injects_into:1 -- by fabric-debug-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]] -- injects_into:3, reads:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel|ClientLevel]] -- calls:13, injects_into:5, reads:1 -- by fabric-block-getter-api-v2, fabric-client-gametest-api-v1, fabric-data-attachment-api-v1, fabric-lifecycle-events-v1, fabric-rendering-v1, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel_EntityCallbacks|ClientLevel$EntityCallbacks]] -- injects_into:2, reads:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] -- calls:10, injects_into:15, reads:8, wraps:2 -- by fabric-command-api-v2, fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-lifecycle-events-v1, fabric-message-api-v1, fabric-networking-api-v1, fabric-recipe-api-v1, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientRecipeContainer|ClientRecipeContainer]] -- calls:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ClientSuggestionProvider|ClientSuggestionProvider]] -- reads:1 -- by fabric-command-api-v2
- [[40-Interfaces/net.minecraft.client.multiplayer.KnownPacksManager|KnownPacksManager]] -- injects_into:1, wraps:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]] -- calls:2, injects_into:6, reads:5, wraps:1 -- by fabric-events-interaction-v0, fabric-item-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.RegistryDataCollector|RegistryDataCollector]] -- wraps:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.multiplayer.ServerData|ServerData]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.ServerData_Type|ServerData$Type]] -- reads:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]] -- injects_into:5 -- by fabric-message-api-v1
- [[40-Interfaces/net.minecraft.client.multiplayer.resolver.ServerAddress|ServerAddress]] -- calls:1 -- by fabric-client-gametest-api-v1

## Declared inventory

### `net.minecraft.client.multiplayer` (30 top-level)

`AccountProfileKeyPairManager`, `CacheSlot`, `ChunkBatchSizeCalculator`, `ClientAdvancements`, [[40-Interfaces/net.minecraft.client.multiplayer.ClientChunkCache|ClientChunkCache]], [[40-Interfaces/net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl|ClientCommonPacketListenerImpl]], [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]], [[40-Interfaces/net.minecraft.client.multiplayer.ClientDebugSubscriber|ClientDebugSubscriber]], `ClientExplosionTracker`, [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]], [[40-Interfaces/net.minecraft.client.multiplayer.ClientLevel|ClientLevel]], [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]], [[40-Interfaces/net.minecraft.client.multiplayer.ClientRecipeContainer|ClientRecipeContainer]], `ClientRegistryLayer`, [[40-Interfaces/net.minecraft.client.multiplayer.ClientSuggestionProvider|ClientSuggestionProvider]], `CommonListenerCookie`, [[40-Interfaces/net.minecraft.client.multiplayer.KnownPacksManager|KnownPacksManager]], `LegacyServerPinger`, `LevelLoadTracker`, [[40-Interfaces/net.minecraft.client.multiplayer.MultiPlayerGameMode|MultiPlayerGameMode]], `PingDebugMonitor`, `PlayerInfo`, `ProfileKeyPairManager`, [[40-Interfaces/net.minecraft.client.multiplayer.RegistryDataCollector|RegistryDataCollector]], [[40-Interfaces/net.minecraft.client.multiplayer.ServerData|ServerData]], `ServerList`, `ServerStatusPinger`, `SessionSearchTrees`, `TransferState`, `package-info`

### `net.minecraft.client.multiplayer.chat` (11 top-level)

`ChatAbilities`, [[40-Interfaces/net.minecraft.client.multiplayer.chat.ChatListener|ChatListener]], `ChatLog`, `ChatRestriction`, `ChatTrustLevel`, `GuiMessage`, `GuiMessageSource`, `GuiMessageTag`, `LoggedChatEvent`, `LoggedChatMessage`, `package-info`

### `net.minecraft.client.multiplayer.chat.report` (12 top-level)

`AbuseReportSender`, `BanReason`, `ChatReport`, `ChatReportContextBuilder`, `NameReport`, `Report`, `ReportEnvironment`, `ReportReason`, `ReportType`, `ReportingContext`, `SkinReport`, `package-info`

### `net.minecraft.client.multiplayer.prediction` (3 top-level)

`BlockStatePredictionHandler`, `PredictiveAction`, `package-info`

### `net.minecraft.client.multiplayer.resolver` (7 top-level)

`AddressCheck`, `ResolvedServerAddress`, [[40-Interfaces/net.minecraft.client.multiplayer.resolver.ServerAddress|ServerAddress]], `ServerAddressResolver`, `ServerNameResolver`, `ServerRedirectHandler`, `package-info`

