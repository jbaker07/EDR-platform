---
type: "mechanism"
module: "fabric-registry-sync-v0"
version: "8.0.1+fcdff87f5d"
sha256: "039a5c3dee042ff156c52c4bec64a384e766e830a37cc9f6bf68f1e7dc3646b1"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-registry-sync-v0

**Version** `8.0.1+fcdff87f5d` -- **artifact sha256** `039a5c3dee042ff156c52c4bec64a384e766e830a37cc9f6bf68f1e7dc3646b1`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-networking-api-v1": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.registry.sync.FabricRegistryInit"], "client": ["net.fabricmc.fabric.impl.client.registry.sync.FabricRegistryClientInit"]}`
- mixin configs: `["fabric-registry-sync-v0.mixins.json", {"config": "fabric-registry-sync-v0.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-registry-sync-v0.classtweaker`
- mixin classes: 25 found by annotation, 25 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback.EVENT|DynamicRegistrySetupCallback.EVENT]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`<init>` | `(Lnet/minecraft/client/main/GameConfig;)V` | name_only | @Inject | INVOKE `Ljava/lang/Thread;currentThread()Ljava/lang/Thread;` (exact) | client | 1000 (default) | `MinecraftMixin.afterModInit` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`disconnect` | `(Lnet/minecraft/client/gui/screens/Screen;ZZ)V` | exact | @Inject | RETURN | client | 1000 (default) | `MinecraftMixin.disconnectAfter` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldOpenFlows|WorldOpenFlows]].`askForBackup` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;ZLjava/lang/Runnable;Ljava/lang/Runnable;)V` | name_only | @WrapOperation | NEW `(Ljava/lang/Runnable;Lnet/minecraft/client/gui/screens/BackupConfirmScreen$Listener;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Z)Lnet/minecraft/client/gui/screens/BackupConfirmScreen;` (exact) | client | 1000 (default) | `WorldOpenFlowsMixin.replaceBackupScreen` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldOpenFlows|WorldOpenFlows]].`openWorldCheckWorldStemCompatibility` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/WorldStem;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/lang/Runnable;)V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/world/level/storage/WorldData;worldGenSettingsLifecycle()Lcom/mojang/serialization/Lifecycle;` (exact) | client | 1000 (default) | `WorldOpenFlowsMixin.injectHereForCustomScreen` |
| [[40-Interfaces/net.minecraft.client.multiplayer.RegistryDataCollector|RegistryDataCollector]].`loadNewElementsAndTags` | `(Lnet/minecraft/server/packs/resources/ResourceProvider;Lnet/minecraft/client/multiplayer/RegistryDataCollector$ContentsCollector;Z)Lnet/minecraft/core/RegistryAccess;` | name_only | @WrapOperation | FIELD `Lnet/minecraft/resources/RegistryDataLoader;SYNCHRONIZED_REGISTRIES:Ljava/util/List;` (exact) | client | 1000 (default) | `RegistryDataCollectorMixin.skipEmptyRegistries` |
| [[40-Interfaces/net.minecraft.client.particle.ParticleResources|ParticleResources]].`<init>` | `()V` | name_only | @Inject | RETURN | client | 1000 (default) | `ParticleResourcesMixin.onInit` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`<init>` | `(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecycle;Z)V` | exact | @Inject | RETURN | both | 1000 (default) | `MappedRegistryMixin.init` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`containsKey` | `(Lnet/minecraft/resources/Identifier;)Z` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `MappedRegistryMixin.aliasIdentifierParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`containsKey` | `(Lnet/minecraft/resources/ResourceKey;)Z` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`get` | `(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `MappedRegistryMixin.aliasIdentifierParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`get` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`getOrCreateHolderOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Reference;` | name_only | @ModifyVariable | HEAD | both | 1000 (default) | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`getValue` | `(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `MappedRegistryMixin.aliasIdentifierParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`getValue` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | exact | @ModifyVariable | HEAD | both | 1000 (default) | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`register` | `?` | ambiguous | @Inject | RETURN | both | 1000 (default) | `MappedRegistryMixin.set` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]].`registrationInfo` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | name_only | @ModifyVariable | HEAD | both | 1000 (default) | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.RegistrySynchronization|RegistrySynchronization]].`lambda$ownedNetworkableRegistries$0` | `(Lnet/minecraft/core/RegistryAccess$RegistryEntry;)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `RegistrySynchronizationMixin.filterNonSyncedEntries` |
| [[40-Interfaces/net.minecraft.core.RegistrySynchronization|RegistrySynchronization]].`lambda$packRegistry$0` | `(Ljava/util/Set;Lnet/minecraft/resources/RegistryDataLoader$RegistryData;Lcom/mojang/serialization/DynamicOps;Ljava/util/function/BiConsumer;Lnet/minecraft/core/Registry;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `RegistrySynchronizationMixin.filterNonSyncedEntriesAgain` |
| [[40-Interfaces/net.minecraft.core.registries.BuiltInRegistries|BuiltInRegistries]].`createContents` | `()V` | name_only | @Inject | HEAD | both | 1000 (default) | `BuiltInRegistriesMixin.init` |
| [[40-Interfaces/net.minecraft.core.registries.Registries|Registries]].`elementsDirPath` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `RegistriesMixin.prependDirectoryWithNamespace` |
| [[40-Interfaces/net.minecraft.core.registries.Registries|Registries]].`tagsDirPath` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `RegistriesMixin.prependTagDirectoryWithNamespace` |
| [[40-Interfaces/net.minecraft.data.registries.RegistriesDatapackGenerator|RegistriesDatapackGenerator]].`forReloadableLayer` | `(Lnet/minecraft/data/PackOutput;Ljava/util/concurrent/CompletableFuture;)Lnet/minecraft/data/DataProvider;` | name_only | @Redirect | FIELD `Lnet/minecraft/resources/RegistryDataLoader;RELOADABLE_REGISTRIES:Ljava/util/List;` (exact) | both | 1000 (default) | `RegistriesDatapackGeneratorMixin.getReloadableRegistries` |
| [[40-Interfaces/net.minecraft.data.registries.RegistriesDatapackGenerator|RegistriesDatapackGenerator]].`forWorldLayer` | `(Lnet/minecraft/data/PackOutput;Ljava/util/concurrent/CompletableFuture;)Lnet/minecraft/data/registries/RegistriesDatapackGenerator;` | name_only | @Redirect | FIELD `Lnet/minecraft/resources/RegistryDataLoader;WORLD_REGISTRIES:Ljava/util/List;` (exact) | both | 1000 (default) | `RegistriesDatapackGeneratorMixin.getWorldRegistries` |
| [[40-Interfaces/net.minecraft.data.registries.RegistryPatchGenerator|RegistryPatchGenerator]].`lambda$createReloadableLookup$0` | `(Lnet/minecraft/core/RegistrySetBuilder;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/RegistrySetBuilder$PatchedRegistries;` | name_only | @Redirect | FIELD `Lnet/minecraft/resources/RegistryDataLoader;RELOADABLE_REGISTRIES:Ljava/util/List;` (exact) | both | 1000 (default) | `RegistryPatchGeneratorMixin.getReloadableRegistries` |
| [[40-Interfaces/net.minecraft.data.registries.RegistryPatchGenerator|RegistryPatchGenerator]].`lambda$createWorldLookup$0` | `(Lnet/minecraft/core/RegistrySetBuilder;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/RegistrySetBuilder$PatchedRegistries;` | name_only | @Redirect | FIELD `Lnet/minecraft/resources/RegistryDataLoader;WORLD_REGISTRIES:Ljava/util/List;` (exact) | both | 1000 (default) | `RegistryPatchGeneratorMixin.getWorldRegistries` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]].`lambda$load$0` | `(Ljava/util/List;Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyArg | INVOKE `Ljava/util/concurrent/CompletableFuture;thenApplyAsync(Ljava/util/function/Function;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | both | 1000 (default) | `RegistryDataLoaderMixin.thenApplyAsync` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]].`lambda$load$0` | `(Ljava/util/List;Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/resources/RegistryDataLoader;createContext(Ljava/util/List;Ljava/util/List;)Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;` (exact) | both | 1000 (default) | `RegistryDataLoaderMixin.beforeLoad` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]].`load` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | exact | @WrapOperation | INVOKE `Lnet/minecraft/resources/RegistryDataLoader;load(Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | both | 1000 (default) | `RegistryDataLoaderMixin.wrapIsServerCall` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]].`load` | `(Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | exact | @ModifyArg | INVOKE `Ljava/util/concurrent/CompletableFuture;supplyAsync(Ljava/util/function/Supplier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | both | 1000 (default) | `RegistryDataLoaderMixin.supplyAsync` |
| [[40-Interfaces/net.minecraft.server.Bootstrap|Bootstrap]].`bootStrap` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/Bootstrap;wrapStreams()V` (exact) | both | 1000 (default) | `BootstrapMixin.afterInitialize` |
| [[40-Interfaces/net.minecraft.server.Bootstrap|Bootstrap]].`bootStrap` | `()V` | name_only | @Redirect | INVOKE `Lnet/minecraft/core/registries/BuiltInRegistries;bootStrap()V` (exact) | both | 1000 (default) | `BootstrapMixin.delayRegistryFreeze` |
| [[40-Interfaces/net.minecraft.server.Main|Main]].`main` | `([Ljava/lang/String;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/util/Util;startTimerHackThread()V` (exact) | both | 1000 (default) | `MainMixin.afterModInit` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`<init>` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Ljava/net/Proxy;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/server/Services;Lnet/minecraft/server/level/progress/LevelLoadListener;ZLnet/minecraft/server/notifications/NotificationManager;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/world/level/storage/SavedDataStorage;set(Lnet/minecraft/world/level/saveddata/SavedDataType;Lnet/minecraft/world/level/saveddata/SavedData;)V` (exact) | both | 1000 (default) | `MinecraftServerMixin.saveRegistryEntryInfo` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerRegistries|ReloadableServerRegistries]].`reload` | `(Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Redirect | FIELD `Lnet/minecraft/resources/RegistryDataLoader;RELOADABLE_REGISTRIES:Ljava/util/List;` (exact) | both | 1000 (default) | `ReloadableServerRegistriesMixin.getReloadableRegistries` |
| [[40-Interfaces/net.minecraft.server.WorldLoader|WorldLoader]].`lambda$load$0` | `(Ljava/util/concurrent/Executor;Lnet/minecraft/server/WorldLoader$WorldDataSupplier;Lnet/minecraft/server/WorldLoader$InitConfig;Ljava/util/concurrent/Executor;Lnet/minecraft/server/WorldLoader$ResultFactory;Lcom/mojang/datafixers/util/Pair;)Ljava/util/concurrent/CompletionStage;` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/resources/RegistryDataLoader;load(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | both | 1000 (default) | `WorldLoaderMixin.modifyLoadedEntries` |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]].`<clinit>` | `()V` | exact | @Inject | TAIL | both | 1000 (default) | `BlocksMixin.initShapeCache` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]].`lambda$unpackStructureReferences$0` | `(Lnet/minecraft/core/Registry;Lnet/minecraft/world/level/ChunkPos;Ljava/util/Map;Ljava/lang/String;Lnet/minecraft/nbt/Tag;)V` | name_only | @Redirect | INVOKE `Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V` (exact) | both | 1000 (default) | `SerializableChunkDataMixin.log` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.DynamicRegistries|DynamicRegistries]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback|DynamicRegistrySetupCallback]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.DynamicRegistryView|DynamicRegistryView]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.FabricRegistry|FabricRegistry]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder|FabricRegistryBuilder]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.RegistryAttribute|RegistryAttribute]] (enum, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.RegistryAttributeHolder|RegistryAttributeHolder]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.RegistryEntryAddedCallback|RegistryEntryAddedCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.RegistryIdRemapCallback|RegistryIdRemapCallback]] (interface, 2 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
