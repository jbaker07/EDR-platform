---
type: "mechanism"
module: "fabric-resource-loader-v1"
version: "3.0.4+fcdff87f5d"
sha256: "2d2fb907728c895640c1261ff3dd115b5087913d88d1ba1d9f3a8a4cfd1ca87e"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-resource-loader-v1

**Version** `3.0.4+fcdff87f5d` -- **artifact sha256** `2d2fb907728c895640c1261ff3dd115b5087913d88d1ba1d9f3a8a4cfd1ca87e`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `null`
- mixin configs: `["fabric-resource-loader-v1.mixins.json", {"config": "fabric-resource-loader-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-resource-loader-v1.classtweaker`

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Options|Options]] | `load` | injects_into `@Inject at RETURN` | client | `OptionsMixin.onLoad` |
| [[40-Interfaces/net.minecraft.client.Options_3|Options$3]] | `process(Ljava/lang/String;Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang/Object;` | injects_into `@ModifyArg at INVOKE Ljava/util/function/Function;apply(Ljava/lang/Object;)Ljava/lang/Object;` | client | `GameOptionsWriteVisitorMixin.skipHiddenPacks` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent|ClientTooltipComponent]] | `create(Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;` | injects_into `@Inject at HEAD` | client | `ClientTooltipComponentMixin.onCreate` |
| [[40-Interfaces/net.minecraft.client.gui.screens.packs.PackSelectionModel|PackSelectionModel]] | `<init>` | injects_into `@Inject at TAIL` | client | `PackSelectionModelMixin.removeHiddenPacksInit` |
| [[40-Interfaces/net.minecraft.client.gui.screens.packs.PackSelectionModel|PackSelectionModel]] | `findNewPacks` | injects_into `@Inject at TAIL` | client | `PackSelectionModelMixin.removeHiddenPacksRefresh` |
| [[40-Interfaces/net.minecraft.client.gui.screens.packs.TransferableSelectionList_PackEntry|TransferableSelectionList$PackEntry]] | `extractContent` | injects_into `@Inject at RETURN` | client | `TransferableSelectionListPackEntryMixin.onExtractContent` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]] | `getDataPackSelectionSettings` | injects_into `@Inject at INVOKE Lnet/minecraft/server/packs/repository/PackRepository;reload()V` | client | `CreateWorldScreenMixin.onScanPacks` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]] | `openCreateWorldScreen(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Ljava/util/function/Function;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContextMapper;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;)V` | injects_into `@ModifyVariable at INVOKE Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;createDefaultLoadConfig(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/world/level/WorldDataConfiguration;)Lnet/minecraft/server/WorldLoader$InitConfig;` | client | `CreateWorldScreenMixin.onCreateResManagerInit` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]] | `openCreateWorldScreen(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Ljava/util/function/Function;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContextMapper;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;)V` | wraps `@Redirect at FIELD Lnet/minecraft/world/level/WorldDataConfiguration;DEFAULT:Lnet/minecraft/world/level/WorldDataConfiguration;` | client | `CreateWorldScreenMixin.replaceDefaultSettings` |
| [[40-Interfaces/net.minecraft.client.multiplayer.KnownPacksManager|KnownPacksManager]] | `<init>` | wraps `@Redirect at INVOKE Lnet/minecraft/server/packs/repository/ServerPacksSource;createVanillaTrustedRepository()Lnet/minecraft/server/packs/repository/PackRepository;` | client | `KnownPacksManagerMixin.createClientManager` |
| [[40-Interfaces/net.minecraft.gametest.framework.GameTestServer|GameTestServer]] | `create` | wraps `@Redirect at NEW (Ljava/util/List;Ljava/util/List;)Lnet/minecraft/world/level/DataPackConfig;` | both | `GameTestServerMixin.replaceDefaultDataPackConfig` |
| [[40-Interfaces/net.minecraft.locale.Language|Language]] | `loadDefault` | wraps `@Redirect at INVOKE Ljava/util/Map;copyOf(Ljava/util/Map;)Ljava/util/Map;` | server | `LanguageMixin.create` |
| [[40-Interfaces/net.minecraft.locale.Language|Language]] | `parseTranslations(Ljava/util/function/BiConsumer;Ljava/lang/String;)V` | wraps `@Redirect at INVOKE Ljava/lang/Class;getResourceAsStream(Ljava/lang/String;)Ljava/io/InputStream;` | server | `LanguageMixin.readCorrectVanillaResource` |
| [[40-Interfaces/net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks|ServerboundSelectKnownPacks]] | `<clinit>` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/network/codec/ByteBufCodecs;list(I)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;` | both | `ServerboundSelectKnownPacksMixin.setMaxKnownPacks` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `<init>` | injects_into `@Inject at TAIL` | both | `MinecraftServerMixin.init` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `configurePackRepository(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/world/level/WorldDataConfiguration;ZZ)Lnet/minecraft/world/level/WorldDataConfiguration;` | wraps `@Redirect at INVOKE Ljava/util/List;contains(Ljava/lang/Object;)Z` | both | `MinecraftServerMixin.onCheckDisabled` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]] | `lambda$loadResources$2` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/server/packs/resources/SimpleReloadInstance;create(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | both | `ReloadableServerResourcesMixin.onSetupDataReloaders` |
| [[40-Interfaces/net.minecraft.server.commands.DataPackCommand|DataPackCommand]] | `getPack` | injects_into `@Inject at INVOKE Ljava/util/Collection;contains(Ljava/lang/Object;)Z` | both | `DataPackCommandMixin.errorOnInternalPack` |
| [[40-Interfaces/net.minecraft.server.commands.DataPackCommand|DataPackCommand]] | `lambda$static$10` | wraps `@Redirect at INVOKE Lnet/minecraft/server/packs/repository/PackRepository;getSelectedIds()Ljava/util/Collection;` | both | `DataPackCommandMixin.filterEnabledPackSuggestions` |
| [[40-Interfaces/net.minecraft.server.dedicated.DedicatedServerProperties|DedicatedServerProperties]] | `<init>` | wraps `@Redirect at FIELD Lnet/minecraft/world/level/WorldDataConfiguration;DEFAULT:Lnet/minecraft/world/level/WorldDataConfiguration;` | both | `DedicatedServerPropertiesMixin.replaceDefaultDataConfiguration` |
| [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]] | `startConfiguration` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/server/network/config/SynchronizeRegistriesTask;<init>(Ljava/util/List;Lnet/minecraft/core/LayeredRegistryAccess;)V` | both | `ServerConfigurationPacketListenerImplMixin.filterKnownPacks` |
| [[40-Interfaces/net.minecraft.server.network.config.SynchronizeRegistriesTask|SynchronizeRegistriesTask]] | `handleResponse` | injects_into `@Inject at HEAD` | both | `SynchronizeRegistriesTaskMixin.onSelectKnownPacks` |
| [[40-Interfaces/net.minecraft.server.network.config.SynchronizeRegistriesTask|SynchronizeRegistriesTask]] | `sendRegistries` | injects_into `@Inject at HEAD` | both | `SynchronizeRegistriesTaskMixin.syncRegistryAndTags` |
| [[40-Interfaces/net.minecraft.server.network.config.SynchronizeRegistriesTask|SynchronizeRegistriesTask]] | `start` | injects_into `@Inject at HEAD` | both | `SynchronizeRegistriesTaskMixin.sendPacket` |
| [[40-Interfaces/net.minecraft.server.packs.repository.BuiltInPackSource|BuiltInPackSource]] | `loadPacks` | injects_into `@Inject at RETURN` | client | `BuiltInPackSourceMixin.addBuiltinResourcePacks` |
| [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]] | `<init>` | injects_into `@Inject at RETURN` | both | `PackRepositoryMixin.construct` |
| [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]] | `addPack` | injects_into `@Inject at INVOKE Ljava/util/List;add(Ljava/lang/Object;)Z` | both | `PackRepositoryMixin.handleAutoEnable` |
| [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]] | `rebuildSelected` | injects_into `@Inject at INVOKE Lcom/google/common/collect/ImmutableList;copyOf(Ljava/util/Collection;)Lcom/google/common/collect/ImmutableList;` | both | `PackRepositoryMixin.handleAutoEnableDisable` |
| [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]] | `removePack` | injects_into `@Inject at INVOKE Ljava/util/List;remove(Ljava/lang/Object;)Z` | both | `PackRepositoryMixin.handleAutoDisable` |
| [[40-Interfaces/net.minecraft.server.packs.resources.MultiPackResourceManager|MultiPackResourceManager]] | `<init>` | injects_into `@Inject at TAIL` | both | `MultiPackResourceManagerMixin.init` |
| [[40-Interfaces/net.minecraft.server.packs.resources.SimpleReloadInstance|SimpleReloadInstance]] | `create(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/server/packs/resources/SimpleReloadInstance;of(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;)Lnet/minecraft/server/packs/resources/ReloadInstance;` | both | `SimpleReloadInstanceMixin.sortSimple` |
| [[40-Interfaces/net.minecraft.server.packs.resources.SimpleReloadInstance|SimpleReloadInstance]] | `create(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/server/packs/resources/ProfiledReloadInstance;of(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;)Lnet/minecraft/server/packs/resources/ReloadInstance;` | both | `SimpleReloadInstanceMixin.sortProfiled` |
| [[40-Interfaces/net.minecraft.server.packs.resources.SimpleReloadInstance|SimpleReloadInstance]] | `create(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | injects_into `@ModifyVariable at LOAD` | both | `SimpleReloadInstanceMixin.adjustProfiledCheck` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.DataResourceLoader|DataResourceLoader]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.DataResourceStore|DataResourceStore]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.FabricResource|FabricResource]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.ResourceLoader|ResourceLoader]] (interface, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.pack.ModPackResources|ModPackResources]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.pack.PackActivationType|PackActivationType]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.reloader.ResourceReloaderKeys|ResourceReloaderKeys]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.reloader.SimpleReloadListener|SimpleReloadListener]] (abstract_class, 4 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
