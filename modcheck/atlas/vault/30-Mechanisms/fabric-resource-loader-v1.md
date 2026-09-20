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
- mixin classes: 25 found by annotation, 25 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Options|Options]].`load` | `()V` | name_only | @Inject | RETURN | client | 1000 (default) | `OptionsMixin.onLoad` |
| [[40-Interfaces/net.minecraft.client.Options|Options]].`updateResourcePacks` | `(Lnet/minecraft/server/packs/repository/PackRepository;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/server/packs/repository/Pack;isFixedPosition()Z` (exact) | client | 1000 (default) | `OptionsMixin.excludeInternalResourcePacksFromRefreshCheck` |
| [[40-Interfaces/net.minecraft.client.Options_3|Options$3]].`process` | `(Ljava/lang/String;Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang/Object;` | exact | @ModifyArg | INVOKE `Ljava/util/function/Function;apply(Ljava/lang/Object;)Ljava/lang/Object;` (exact) | client | 1000 (default) | `GameOptionsWriteVisitorMixin.skipHiddenPacks` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent|ClientTooltipComponent]].`create` | `(Lnet/minecraft/world/inventory/tooltip/TooltipComponent;)Lnet/minecraft/client/gui/screens/inventory/tooltip/ClientTooltipComponent;` | exact | @Inject | HEAD | client | 1000 (default) | `ClientTooltipComponentMixin.onCreate` |
| [[40-Interfaces/net.minecraft.client.gui.screens.packs.PackSelectionModel|PackSelectionModel]].`<init>` | `(Ljava/util/function/Consumer;Ljava/util/function/Function;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/util/function/Consumer;)V` | name_only | @Inject | TAIL | client | 1000 (default) | `PackSelectionModelMixin.removeHiddenPacksInit` |
| [[40-Interfaces/net.minecraft.client.gui.screens.packs.PackSelectionModel|PackSelectionModel]].`findNewPacks` | `()V` | name_only | @Inject | TAIL | client | 1000 (default) | `PackSelectionModelMixin.removeHiddenPacksRefresh` |
| [[40-Interfaces/net.minecraft.client.gui.screens.packs.TransferableSelectionList_PackEntry|TransferableSelectionList$PackEntry]].`extractContent` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIZF)V` | name_only | @Inject | RETURN | client | 1000 (default) | `TransferableSelectionListPackEntryMixin.onExtractContent` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]].`getDataPackSelectionSettings` | `(Lnet/minecraft/world/level/WorldDataConfiguration;)Lcom/mojang/datafixers/util/Pair;` | name_only | @Inject | INVOKE `Lnet/minecraft/server/packs/repository/PackRepository;reload()V` (exact) | client | 1000 (default) | `CreateWorldScreenMixin.onScanPacks` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]].`openCreateWorldScreen` | `(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Ljava/util/function/Function;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContextMapper;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;)V` | exact | @ModifyVariable | INVOKE `Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;createDefaultLoadConfig(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/world/level/WorldDataConfiguration;)Lnet/minecraft/server/WorldLoader$InitConfig;` (exact) | client | 1000 (default) | `CreateWorldScreenMixin.onCreateResManagerInit` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]].`openCreateWorldScreen` | `(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Ljava/util/function/Function;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContextMapper;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;)V` | exact | @Redirect | FIELD `Lnet/minecraft/world/level/WorldDataConfiguration;DEFAULT:Lnet/minecraft/world/level/WorldDataConfiguration;` (exact) | client | 1000 (default) | `CreateWorldScreenMixin.replaceDefaultSettings` |
| [[40-Interfaces/net.minecraft.client.multiplayer.KnownPacksManager|KnownPacksManager]].`<init>` | `()V` | name_only | @Redirect | INVOKE `Lnet/minecraft/server/packs/repository/ServerPacksSource;createVanillaTrustedRepository()Lnet/minecraft/server/packs/repository/PackRepository;` (exact) | client | 1000 (default) | `KnownPacksManagerMixin.createClientManager` |
| [[40-Interfaces/net.minecraft.client.multiplayer.KnownPacksManager|KnownPacksManager]].`trySelectingPacks` | `(Ljava/util/List;)Ljava/util/List;` | name_only | @ModifyReturnValue | RETURN | client | 1000 (default) | `KnownPacksManagerMixin.getCommonKnownPacksReturn` |
| [[40-Interfaces/net.minecraft.client.resources.language.ClientLanguage|ClientLanguage]].`loadFrom` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Z)Lnet/minecraft/client/resources/language/ClientLanguage;` | name_only | @ModifyExpressionValue | INVOKE `Ljava/util/List;isEmpty()Z` (exact) | client | 1000 (default) | `ClientLanguageMixin.allowMissingLanguageFiles` |
| [[40-Interfaces/net.minecraft.gametest.framework.GameTestServer|GameTestServer]].`create` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/util/Optional;ZI)Lnet/minecraft/gametest/framework/GameTestServer;` | name_only | @Redirect | NEW `(Ljava/util/List;Ljava/util/List;)Lnet/minecraft/world/level/DataPackConfig;` (exact) | both | 1000 (default) | `GameTestServerMixin.replaceDefaultDataPackConfig` |
| [[40-Interfaces/net.minecraft.locale.Language|Language]].`loadDefault` | `()Lnet/minecraft/locale/Language;` | name_only | @Redirect | INVOKE `Ljava/util/Map;copyOf(Ljava/util/Map;)Ljava/util/Map;` (exact) | server | 1000 (default) | `LanguageMixin.create` |
| [[40-Interfaces/net.minecraft.locale.Language|Language]].`parseTranslations` | `(Ljava/util/function/BiConsumer;Ljava/lang/String;)V` | exact | @Redirect | INVOKE `Ljava/lang/Class;getResourceAsStream(Ljava/lang/String;)Ljava/io/InputStream;` (exact) | server | 1000 (default) | `LanguageMixin.readCorrectVanillaResource` |
| [[40-Interfaces/net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks|ServerboundSelectKnownPacks]].`<clinit>` | `()V` | exact | @ModifyArg | INVOKE `Lnet/minecraft/network/codec/ByteBufCodecs;list(I)Lnet/minecraft/network/codec/StreamCodec$CodecOperation;` (exact) | both | 1000 (default) | `ServerboundSelectKnownPacksMixin.setMaxKnownPacks` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`<init>` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Ljava/net/Proxy;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/server/Services;Lnet/minecraft/server/level/progress/LevelLoadListener;ZLnet/minecraft/server/notifications/NotificationManager;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `MinecraftServerMixin.init` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`configurePackRepository` | `(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/world/level/WorldDataConfiguration;ZZ)Lnet/minecraft/world/level/WorldDataConfiguration;` | exact | @Redirect | INVOKE `Ljava/util/List;contains(Ljava/lang/Object;)Z` (exact) | both | 1000 (default) | `MinecraftServerMixin.onCheckDisabled` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]].`lambda$loadResources$2` | `(Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/commands/Commands$CommandSelection;Ljava/util/List;Lnet/minecraft/server/permissions/PermissionSet;Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/List;)Ljava/util/concurrent/CompletionStage;` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/server/packs/resources/SimpleReloadInstance;create(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` (exact) | both | 1000 (default) | `ReloadableServerResourcesMixin.onSetupDataReloaders` |
| [[40-Interfaces/net.minecraft.server.commands.DataPackCommand|DataPackCommand]].`getPack` | `(Lcom/mojang/brigadier/context/CommandContext;Ljava/lang/String;Z)Lnet/minecraft/server/packs/repository/Pack;` | name_only | @Inject | INVOKE `Ljava/util/Collection;contains(Ljava/lang/Object;)Z` (exact) | both | 1000 (default) | `DataPackCommandMixin.errorOnInternalPack` |
| [[40-Interfaces/net.minecraft.server.commands.DataPackCommand|DataPackCommand]].`lambda$static$10` | `(Lcom/mojang/brigadier/context/CommandContext;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Redirect | INVOKE `Lnet/minecraft/server/packs/repository/PackRepository;getSelectedIds()Ljava/util/Collection;` (exact) | both | 1000 (default) | `DataPackCommandMixin.filterEnabledPackSuggestions` |
| [[40-Interfaces/net.minecraft.server.commands.DataPackCommand|DataPackCommand]].`lambda$static$11` | `(Lcom/mojang/brigadier/context/CommandContext;Lcom/mojang/brigadier/suggestion/SuggestionsBuilder;)Ljava/util/concurrent/CompletableFuture;` | name_only | @WrapOperation | INVOKE `Ljava/util/stream/Stream;filter(Ljava/util/function/Predicate;)Ljava/util/stream/Stream;` (exact) | both | 1000 (default) | `DataPackCommandMixin.filterDisabledPackSuggestions` |
| [[40-Interfaces/net.minecraft.server.dedicated.DedicatedServerProperties|DedicatedServerProperties]].`<init>` | `(Ljava/util/Properties;)V` | name_only | @Redirect | FIELD `Lnet/minecraft/world/level/WorldDataConfiguration;DEFAULT:Lnet/minecraft/world/level/WorldDataConfiguration;` (exact) | both | 1000 (default) | `DedicatedServerPropertiesMixin.replaceDefaultDataConfiguration` |
| [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]].`startConfiguration` | `()V` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/server/network/config/SynchronizeRegistriesTask;<init>(Ljava/util/List;Lnet/minecraft/core/LayeredRegistryAccess;)V` (exact) | both | 1000 (default) | `ServerConfigurationPacketListenerImplMixin.filterKnownPacks` |
| [[40-Interfaces/net.minecraft.server.network.config.SynchronizeRegistriesTask|SynchronizeRegistriesTask]].`handleResponse` | `(Ljava/util/List;Ljava/util/function/Consumer;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SynchronizeRegistriesTaskMixin.onSelectKnownPacks` |
| [[40-Interfaces/net.minecraft.server.network.config.SynchronizeRegistriesTask|SynchronizeRegistriesTask]].`sendRegistries` | `(Ljava/util/function/Consumer;Ljava/util/Set;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SynchronizeRegistriesTaskMixin.syncRegistryAndTags` |
| [[40-Interfaces/net.minecraft.server.network.config.SynchronizeRegistriesTask|SynchronizeRegistriesTask]].`start` | `(Ljava/util/function/Consumer;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SynchronizeRegistriesTaskMixin.sendPacket` |
| [[40-Interfaces/net.minecraft.server.packs.repository.BuiltInPackSource|BuiltInPackSource]].`loadPacks` | `(Ljava/util/function/Consumer;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `BuiltInPackSourceMixin.addBuiltinResourcePacks` |
| [[40-Interfaces/net.minecraft.server.packs.repository.Pack|Pack]].`open` | `()Ljava/util/stream/Stream;` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `PackMixin.onCreateResourcePack` |
| [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]].`<init>` | `([Lnet/minecraft/server/packs/repository/RepositorySource;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `PackRepositoryMixin.construct` |
| [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]].`addPack` | `(Ljava/lang/String;)Z` | name_only | @Inject | INVOKE `Ljava/util/List;add(Ljava/lang/Object;)Z` (exact) | both | 1000 (default) | `PackRepositoryMixin.handleAutoEnable` |
| [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]].`rebuildSelected` | `(Ljava/util/Collection;)Ljava/util/List;` | name_only | @Inject | INVOKE `Lcom/google/common/collect/ImmutableList;copyOf(Ljava/util/Collection;)Lcom/google/common/collect/ImmutableList;` (exact) | both | 1000 (default) | `PackRepositoryMixin.handleAutoEnableDisable` |
| [[40-Interfaces/net.minecraft.server.packs.repository.PackRepository|PackRepository]].`removePack` | `(Ljava/lang/String;)Z` | name_only | @Inject | INVOKE `Ljava/util/List;remove(Ljava/lang/Object;)Z` (exact) | both | 1000 (default) | `PackRepositoryMixin.handleAutoDisable` |
| [[40-Interfaces/net.minecraft.server.packs.resources.MultiPackResourceManager|MultiPackResourceManager]].`<init>` | `(Lnet/minecraft/server/packs/PackType;Ljava/util/List;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `MultiPackResourceManagerMixin.init` |
| [[40-Interfaces/net.minecraft.server.packs.resources.SimpleReloadInstance|SimpleReloadInstance]].`create` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | exact | @ModifyArg | INVOKE `Lnet/minecraft/server/packs/resources/SimpleReloadInstance;of(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;)Lnet/minecraft/server/packs/resources/ReloadInstance;` (exact) | both | 1000 (default) | `SimpleReloadInstanceMixin.sortSimple` |
| [[40-Interfaces/net.minecraft.server.packs.resources.SimpleReloadInstance|SimpleReloadInstance]].`create` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | exact | @ModifyArg | INVOKE `Lnet/minecraft/server/packs/resources/ProfiledReloadInstance;of(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;)Lnet/minecraft/server/packs/resources/ReloadInstance;` (exact) | both | 1000 (default) | `SimpleReloadInstanceMixin.sortProfiled` |
| [[40-Interfaces/net.minecraft.server.packs.resources.SimpleReloadInstance|SimpleReloadInstance]].`create` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;Ljava/util/concurrent/CompletableFuture;Z)Lnet/minecraft/server/packs/resources/ReloadInstance;` | exact | @ModifyVariable | LOAD | both | 1000 (default) | `SimpleReloadInstanceMixin.adjustProfiledCheck` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.DataResourceLoader|DataResourceLoader]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.DataResourceStore|DataResourceStore]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.FabricResource|FabricResource]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.ResourceLoader|ResourceLoader]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.pack.ModPackResources|ModPackResources]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.pack.PackActivationType|PackActivationType]] (enum, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.reloader.ResourceReloaderKeys|ResourceReloaderKeys]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.v1.reloader.SimpleReloadListener|SimpleReloadListener]] (abstract_class, 4 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
