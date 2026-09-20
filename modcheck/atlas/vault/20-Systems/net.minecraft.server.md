---
type: "system"
package: "net.minecraft.server"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server

Analyst note: [[_authored/systems/net.minecraft.server|MinecraftServer -- the server run loop]]

52 classes (27 top-level) across 1 packages in the processed jar; 2 changed by Loom processing; 13 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.server.Bootstrap|Bootstrap]] -- injects_into:1, wraps:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.server.Main|Main]] -- calls:1, injects_into:5, reads:1, wraps:1 -- by fabric-client-gametest-api-v1, fabric-data-generation-api-v1, fabric-gametest-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] -- calls:41, injects_into:19, reads:3, wraps:3 -- by fabric-api-lookup-api-v1, fabric-biome-api-v1, fabric-client-gametest-api-v1, fabric-convention-tags-v2, fabric-data-attachment-api-v1, fabric-dimensions-v1, fabric-game-rule-api-v1, fabric-gametest-api-v1, fabric-lifecycle-events-v1, fabric-loot-api-v3, fabric-message-api-v1, fabric-networking-api-v1, fabric-permission-api-v1, fabric-registry-sync-v0, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.MinecraftServer_ReloadableResources|MinecraftServer$ReloadableResources]] -- calls:3 -- by fabric-lifecycle-events-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.PlayerAdvancements|PlayerAdvancements]] -- injects_into:2, reads:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.server.RegistryLayer|RegistryLayer]] -- reads:2 -- by fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.server.ReloadableServerRegistries|ReloadableServerRegistries]] -- wraps:3 -- by fabric-advancement-api-v1, fabric-loot-api-v3, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.server.ReloadableServerRegistries_Holder|ReloadableServerRegistries$Holder]] -- calls:1 -- by fabric-loot-api-v3
- [[40-Interfaces/net.minecraft.server.ReloadableServerRegistries_LoadResult|ReloadableServerRegistries$LoadResult]] -- calls:3 -- by fabric-lifecycle-events-v1, fabric-resource-loader-v1, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]] -- injects_into:6 -- by fabric-lifecycle-events-v1, fabric-resource-conditions-api-v1, fabric-resource-loader-v1, fabric-tag-api-v1
- [[40-Interfaces/net.minecraft.server.RunningOnDifferentThreadException|RunningOnDifferentThreadException]] -- reads:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.WorldLoader|WorldLoader]] -- injects_into:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.server.WorldStem|WorldStem]] -- calls:3 -- by fabric-registry-sync-v0, fabric-resource-loader-v1

## Declared inventory

### `net.minecraft.server` (27 top-level)

[[40-Interfaces/net.minecraft.server.Bootstrap|Bootstrap]], `ChainedJsonException`, `ConsoleInput`, `DebugLoggedPrintStream`, `Eula`, `LoggedPrintStream`, [[40-Interfaces/net.minecraft.server.Main|Main]], [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]], [[40-Interfaces/net.minecraft.server.PlayerAdvancements|PlayerAdvancements]], [[40-Interfaces/net.minecraft.server.RegistryLayer|RegistryLayer]], [[40-Interfaces/net.minecraft.server.ReloadableServerRegistries|ReloadableServerRegistries]], [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]], [[40-Interfaces/net.minecraft.server.RunningOnDifferentThreadException|RunningOnDifferentThreadException]], `ServerAdvancementManager`, `ServerFunctionLibrary`, `ServerFunctionManager`, `ServerInfo`, `ServerInterface`, `ServerLinks`, `ServerScoreboard`, `ServerTickRateManager`, `Services`, `SuppressedExceptionCollector`, `TickTask`, [[40-Interfaces/net.minecraft.server.WorldLoader|WorldLoader]], [[40-Interfaces/net.minecraft.server.WorldStem|WorldStem]], `package-info`

