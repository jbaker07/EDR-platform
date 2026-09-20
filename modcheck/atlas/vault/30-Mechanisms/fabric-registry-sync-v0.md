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

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback.EVENT|DynamicRegistrySetupCallback.EVENT]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `<init>` | injects_into `@Inject at INVOKE Ljava/lang/Thread;currentThread()Ljava/lang/Thread;` | client | `MinecraftMixin.afterModInit` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `disconnect(Lnet/minecraft/client/gui/screens/Screen;ZZ)V` | injects_into `@Inject at RETURN` | client | `MinecraftMixin.disconnectAfter` |
| [[40-Interfaces/net.minecraft.client.particle.ParticleResources|ParticleResources]] | `<init>` | injects_into `@Inject at RETURN` | client | `ParticleResourcesMixin.onInit` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `<init>(Lnet/minecraft/resources/ResourceKey;Lcom/mojang/serialization/Lifecycle;Z)V` | injects_into `@Inject at RETURN` | both | `MappedRegistryMixin.init` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `containsKey(Lnet/minecraft/resources/Identifier;)Z` | injects_into `@ModifyVariable at HEAD` | both | `MappedRegistryMixin.aliasIdentifierParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `containsKey(Lnet/minecraft/resources/ResourceKey;)Z` | injects_into `@ModifyVariable at HEAD` | both | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `get(Lnet/minecraft/resources/Identifier;)Ljava/util/Optional;` | injects_into `@ModifyVariable at HEAD` | both | `MappedRegistryMixin.aliasIdentifierParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | injects_into `@ModifyVariable at HEAD` | both | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `getOrCreateHolderOrThrow` | injects_into `@ModifyVariable at HEAD` | both | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `getValue(Lnet/minecraft/resources/Identifier;)Ljava/lang/Object;` | injects_into `@ModifyVariable at HEAD` | both | `MappedRegistryMixin.aliasIdentifierParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `getValue(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;` | injects_into `@ModifyVariable at HEAD` | both | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `register` | injects_into `@Inject at RETURN` | both | `MappedRegistryMixin.set` |
| [[40-Interfaces/net.minecraft.core.MappedRegistry|MappedRegistry]] | `registrationInfo` | injects_into `@ModifyVariable at HEAD` | both | `MappedRegistryMixin.aliasResourceKeyParameter` |
| [[40-Interfaces/net.minecraft.core.RegistrySynchronization|RegistrySynchronization]] | `lambda$ownedNetworkableRegistries$0` | injects_into `@Inject at HEAD` | both | `RegistrySynchronizationMixin.filterNonSyncedEntries` |
| [[40-Interfaces/net.minecraft.core.RegistrySynchronization|RegistrySynchronization]] | `lambda$packRegistry$0` | injects_into `@Inject at HEAD` | both | `RegistrySynchronizationMixin.filterNonSyncedEntriesAgain` |
| [[40-Interfaces/net.minecraft.core.registries.BuiltInRegistries|BuiltInRegistries]] | `createContents` | injects_into `@Inject at HEAD` | both | `BuiltInRegistriesMixin.init` |
| [[40-Interfaces/net.minecraft.data.registries.RegistriesDatapackGenerator|RegistriesDatapackGenerator]] | `forReloadableLayer` | wraps `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;RELOADABLE_REGISTRIES:Ljava/util/List;` | both | `RegistriesDatapackGeneratorMixin.getReloadableRegistries` |
| [[40-Interfaces/net.minecraft.data.registries.RegistriesDatapackGenerator|RegistriesDatapackGenerator]] | `forWorldLayer` | wraps `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;WORLD_REGISTRIES:Ljava/util/List;` | both | `RegistriesDatapackGeneratorMixin.getWorldRegistries` |
| [[40-Interfaces/net.minecraft.data.registries.RegistryPatchGenerator|RegistryPatchGenerator]] | `lambda$createReloadableLookup$0` | wraps `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;RELOADABLE_REGISTRIES:Ljava/util/List;` | both | `RegistryPatchGeneratorMixin.getReloadableRegistries` |
| [[40-Interfaces/net.minecraft.data.registries.RegistryPatchGenerator|RegistryPatchGenerator]] | `lambda$createWorldLookup$0` | wraps `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;WORLD_REGISTRIES:Ljava/util/List;` | both | `RegistryPatchGeneratorMixin.getWorldRegistries` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]] | `lambda$load$0` | injects_into `@ModifyArg at INVOKE Ljava/util/concurrent/CompletableFuture;thenApplyAsync(Ljava/util/function/Function;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | both | `RegistryDataLoaderMixin.thenApplyAsync` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]] | `load(Lnet/minecraft/resources/RegistryDataLoader$LoaderFactory;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | injects_into `@ModifyArg at INVOKE Ljava/util/concurrent/CompletableFuture;supplyAsync(Ljava/util/function/Supplier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | both | `RegistryDataLoaderMixin.supplyAsync` |
| [[40-Interfaces/net.minecraft.server.Bootstrap|Bootstrap]] | `bootStrap` | injects_into `@Inject at INVOKE Lnet/minecraft/server/Bootstrap;wrapStreams()V` | both | `BootstrapMixin.afterInitialize` |
| [[40-Interfaces/net.minecraft.server.Bootstrap|Bootstrap]] | `bootStrap` | wraps `@Redirect at INVOKE Lnet/minecraft/core/registries/BuiltInRegistries;bootStrap()V` | both | `BootstrapMixin.delayRegistryFreeze` |
| [[40-Interfaces/net.minecraft.server.Main|Main]] | `main` | injects_into `@Inject at INVOKE Lnet/minecraft/util/Util;startTimerHackThread()V` | both | `MainMixin.afterModInit` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `<init>` | injects_into `@Inject at INVOKE Lnet/minecraft/world/level/storage/SavedDataStorage;set(Lnet/minecraft/world/level/saveddata/SavedDataType;Lnet/minecraft/world/level/saveddata/SavedData;)V` | both | `MinecraftServerMixin.saveRegistryEntryInfo` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerRegistries|ReloadableServerRegistries]] | `reload` | wraps `@Redirect at FIELD Lnet/minecraft/resources/RegistryDataLoader;RELOADABLE_REGISTRIES:Ljava/util/List;` | both | `ReloadableServerRegistriesMixin.getReloadableRegistries` |
| [[40-Interfaces/net.minecraft.server.WorldLoader|WorldLoader]] | `lambda$load$0` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/resources/RegistryDataLoader;load(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | both | `WorldLoaderMixin.modifyLoadedEntries` |
| [[40-Interfaces/net.minecraft.world.level.block.Blocks|Blocks]] | `<clinit>` | injects_into `@Inject at TAIL` | both | `BlocksMixin.initShapeCache` |
| [[40-Interfaces/net.minecraft.world.level.chunk.storage.SerializableChunkData|SerializableChunkData]] | `lambda$unpackStructureReferences$0` | wraps `@Redirect at INVOKE Lorg/slf4j/Logger;warn(Ljava/lang/String;Ljava/lang/Object;Ljava/lang/Object;)V` | both | `SerializableChunkDataMixin.log` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.DynamicRegistries|DynamicRegistries]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback|DynamicRegistrySetupCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.DynamicRegistryView|DynamicRegistryView]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.FabricRegistry|FabricRegistry]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.FabricRegistryBuilder|FabricRegistryBuilder]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.RegistryAttribute|RegistryAttribute]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.RegistryAttributeHolder|RegistryAttributeHolder]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.RegistryEntryAddedCallback|RegistryEntryAddedCallback]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.event.registry.RegistryIdRemapCallback|RegistryIdRemapCallback]] (interface, 2 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
