---
type: "mechanism"
module: "fabric-model-loading-api-v1"
version: "8.0.35+fcdff87f5d"
sha256: "4889e5947bb2f9d899f72c17676b27aaef7d2594ecaaf75e89626e1eea169711"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-model-loading-api-v1

**Version** `8.0.35+fcdff87f5d` -- **artifact sha256** `4889e5947bb2f9d899f72c17676b27aaef7d2594ecaaf75e89626e1eea169711`

## Declared (fabric.mod.json)

- environment: `client`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-renderer-api-v1": "*"}`
- entrypoints: `{"client": ["net.fabricmc.fabric.impl.client.model.loading.CustomUnbakedBlockStateModelInit"]}`
- mixin configs: `[{"environment": "client", "config": "fabric-model-loading-api-v1.mixins.json"}]`
- access widener: `fabric-model-loading-api-v1.classtweaker`
- mixin classes: 6 found by annotation, 6 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel_Unbaked|BlockStateModel$Unbaked]].`<clinit>` | `()V` | exact | @Redirect | INVOKE `Lcom/mojang/serialization/Codec;flatComapMap(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` (exact) | client | 1000 (default) | `BlockStateModelUnbakedMixin.replaceWeightedCodec` |
| [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel_Unbaked|BlockStateModel$Unbaked]].`<clinit>` | `()V` | exact | @Redirect | INVOKE `Lcom/mojang/serialization/Codec;flatComapMap(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` (exact) | client | 1000 (default) | `BlockStateModelUnbakedMixin.replaceCodec` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelBakery|ModelBakery]].`<init>` | `(Lnet/minecraft/client/model/geom/EntityModelSet;Lnet/minecraft/client/resources/model/sprite/SpriteGetter;Lnet/minecraft/client/renderer/PlayerSkinRenderCache;Ljava/util/Map;Ljava/util/Map;Ljava/util/Map;Lnet/minecraft/client/resources/model/ResolvedModel;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `ModelBakeryMixin.onReturnInit` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelBakery|ModelBakery]].`bakeModels` | `(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/util/thread/ParallelMapTransform;schedule(Ljava/util/Map;Ljava/util/function/BiFunction;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | client | 1000 (default) | `ModelBakeryMixin.hookBlockModelBake` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelBakery|ModelBakery]].`bakeModels` | `(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyReturnValue | RETURN | client | 1000 (default) | `ModelBakeryMixin.withExtraModels` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelBakery|ModelBakery]].`lambda$bakeModels$0` | `(Lnet/minecraft/client/resources/model/ModelBakery$ModelBakerImpl;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel$UnbakedRoot;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel$UnbakedRoot;bake(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/resources/model/ModelBaker;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;` (exact) | client | 1000 (default) | `ModelBakeryMixin.wrapBlockModelBake` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelBakery|ModelBakery]].`lambda$bakeModels$1` | `(Lnet/minecraft/client/resources/model/ModelBakery$ModelBakerImpl;Lnet/minecraft/client/resources/model/ModelBakery$MissingModels;Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/renderer/item/ClientItem;)Lnet/minecraft/client/renderer/item/ItemModel;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/renderer/item/ItemModel$Unbaked;bake(Lnet/minecraft/client/renderer/item/ItemModel$BakingContext;Lorg/joml/Matrix4fc;)Lnet/minecraft/client/renderer/item/ItemModel;` (exact) | client | 1000 (default) | `ModelBakeryMixin.wrapItemModelBake` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`apply` | `(Lnet/minecraft/client/resources/model/ModelManager$ReloadState;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `ModelManagerMixin.onReturnUpload` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`discoverModelDependencies` | `(Ljava/util/Map;Lnet/minecraft/client/resources/model/BlockStateModelLoader$LoadedModels;Lnet/minecraft/client/resources/model/ClientItemInfoLoader$LoadedClientInfos;)Lnet/minecraft/client/resources/model/ModelManager$ResolvedModels;` | name_only | @Inject | INVOKE `Lnet/minecraft/client/resources/model/ModelDiscovery;resolve()Ljava/util/Map;` (exact) | client | 1000 (default) | `ModelManagerMixin.resolveExtraModels` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`lambda$loadBlockModels$2` | `(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;` | exact | @Redirect | INVOKE `Lnet/minecraft/client/resources/model/cuboid/CuboidModel;fromStream(Ljava/io/Reader;)Lnet/minecraft/client/resources/model/cuboid/CuboidModel;` (exact) | client | 1000 (default) | `ModelManagerMixin.cancelVanillaDeserialize` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`lambda$loadBlockModels$2` | `(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;` | exact | @ModifyArg | INVOKE `Lcom/mojang/datafixers/util/Pair;of(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair;` (exact) | client | 1000 (default) | `ModelManagerMixin.actuallyDeserializeModel` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Inject | HEAD | client | 1000 (default) | `ModelManagerMixin.onHeadReload` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyReturnValue | RETURN | client | 1000 (default) | `ModelManagerMixin.resetEventDispatcherFuture` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/client/resources/model/ModelManager;loadBlockModels(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | client | 1000 (default) | `ModelManagerMixin.hookModels` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/client/resources/model/BlockStateModelLoader;loadBlockStates(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | client | 1000 (default) | `ModelManagerMixin.hookBlockStateModels` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyArg | INVOKE `Ljava/util/concurrent/CompletableFuture;thenApplyAsync(Ljava/util/function/Function;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | client | 1000 (default) | `ModelManagerMixin.hookModelCollect` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]].`reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @ModifyArg | INVOKE `Ljava/util/concurrent/CompletableFuture;thenComposeAsync(Ljava/util/function/Function;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | client | 1000 (default) | `ModelManagerMixin.hookModelBaking` |
| [[40-Interfaces/net.minecraft.client.resources.model.cuboid.CuboidModel|CuboidModel]].`<clinit>` | `()V` | exact | @ModifyExpressionValue | NEW `com/google/gson/GsonBuilder` (exact) | client | 1000 (default) | `CuboidModelMixin.addUnbakedModelAdapter` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.BlockStateResolver|BlockStateResolver]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.CompositeBlockStateModel|CompositeBlockStateModel]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.CustomUnbakedBlockStateModel|CustomUnbakedBlockStateModel]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.ExtraModelKey|ExtraModelKey]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.FabricModelManager|FabricModelManager]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.ModelLoadingPlugin|ModelLoadingPlugin]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.ModelModifier|ModelModifier]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.PreparableModelLoadingPlugin|PreparableModelLoadingPlugin]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.SimpleUnbakedExtraModel|SimpleUnbakedExtraModel]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.UnbakedExtraModel|UnbakedExtraModel]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.UnbakedModelDeserializer|UnbakedModelDeserializer]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperBakedItemModel|WrapperBakedItemModel]] (abstract_class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperBlockStateModel|WrapperBlockStateModel]] (abstract_class, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperUnbakedItemModel|WrapperUnbakedItemModel]] (abstract_class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperUnbakedModel|WrapperUnbakedModel]] (abstract_class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperUnbakedRootBlockStateModel|WrapperUnbakedRootBlockStateModel]] (abstract_class, 6 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
