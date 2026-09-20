---
type: "index"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Contested vanilla methods

Methods that more than one Fabric API module modifies. Each is a place where injection ORDER matters and where a third mod's mixin joins an existing crowd. Multiple injections coexisting is the normal case; the list exists because it is where a cancelling injection or an @Overwrite would break others.

| vanilla method | modules | relations |
|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`<init>` | fabric-data-generation-api-v1, fabric-registry-sync-v0, fabric-screen-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`disconnect(Lnet/minecraft/client/gui/screens/Screen;ZZ)V` | fabric-client-gametest-api-v1, fabric-registry-sync-v0 | injects_into |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`doWorldLoad` | fabric-client-gametest-api-v1, fabric-screen-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`exitWorldAndClose` | fabric-lifecycle-events-v1, fabric-screen-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`tick` | fabric-client-gametest-api-v1, fabric-lifecycle-events-v1 | injects_into |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent|ClientTooltipComponent]].`create(Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;` | fabric-rendering-v1, fabric-resource-loader-v1 | injects_into |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]].`handleConfigurationFinished` | fabric-lifecycle-events-v1, fabric-networking-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`<init>` | fabric-command-api-v2, fabric-data-attachment-api-v1, fabric-networking-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleLogin` | fabric-command-api-v2, fabric-lifecycle-events-v1, fabric-networking-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`sendCommand` | fabric-command-api-v2, fabric-message-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.client.renderer.extract.LevelExtractor|LevelExtractor]].`extractBlockOutline` | fabric-renderer-api-v1, fabric-rendering-v1 | injects_into, wraps |
| [[40-Interfaces/net.minecraft.commands.Commands|Commands]].`<init>` | fabric-command-api-v2, fabric-networking-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.core.registries.BuiltInRegistries|BuiltInRegistries]].`freeze` | fabric-item-api-v1, fabric-object-builder-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.data.DataProvider|DataProvider]].`lambda$static$0` | fabric-data-generation-api-v1, fabric-resource-conditions-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.server.Main|Main]].`main` | fabric-data-generation-api-v1, fabric-gametest-api-v1, fabric-registry-sync-v0 | injects_into |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`<init>` | fabric-biome-api-v1, fabric-data-attachment-api-v1, fabric-registry-sync-v0, fabric-resource-loader-v1 | injects_into |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`runServer` | fabric-client-gametest-api-v1, fabric-lifecycle-events-v1 | injects_into |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]].`<init>` | fabric-lifecycle-events-v1, fabric-tag-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]].`updateComponentsAndStaticRegistryTags` | fabric-lifecycle-events-v1, fabric-tag-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.server.dedicated.DedicatedServer|DedicatedServer]].`initServer` | fabric-client-gametest-api-v1, fabric-lifecycle-events-v1 | injects_into, wraps |
| [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]].`startConfiguration` | fabric-networking-api-v1, fabric-resource-loader-v1 | injects_into |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`placeNewPlayer` | fabric-entity-events-v1, fabric-lifecycle-events-v1, fabric-networking-api-v1 | injects_into |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`<init>` | fabric-content-registries-v0, fabric-permission-api-v1 | injects_into |
