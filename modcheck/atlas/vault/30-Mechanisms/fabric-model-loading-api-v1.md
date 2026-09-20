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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel_Unbaked|BlockStateModel$Unbaked]] | `<clinit>()V` | wraps `@Redirect at INVOKE Lcom/mojang/serialization/Codec;flatComapMap(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | client | `BlockStateModelUnbakedMixin.replaceWeightedCodec` |
| [[40-Interfaces/net.minecraft.client.renderer.block.dispatch.BlockStateModel_Unbaked|BlockStateModel$Unbaked]] | `<clinit>()V` | wraps `@Redirect at INVOKE Lcom/mojang/serialization/Codec;flatComapMap(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` | client | `BlockStateModelUnbakedMixin.replaceCodec` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelBakery|ModelBakery]] | `<init>` | injects_into `@Inject at RETURN` | client | `ModelBakeryMixin.onReturnInit` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelBakery|ModelBakery]] | `bakeModels` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/util/thread/ParallelMapTransform;schedule(Ljava/util/Map;Ljava/util/function/BiFunction;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | client | `ModelBakeryMixin.hookBlockModelBake` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]] | `apply` | injects_into `@Inject at RETURN` | client | `ModelManagerMixin.onReturnUpload` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]] | `discoverModelDependencies` | injects_into `@Inject at INVOKE Lnet/minecraft/client/resources/model/ModelDiscovery;resolve()Ljava/util/Map;` | client | `ModelManagerMixin.resolveExtraModels` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]] | `lambda$loadBlockModels$2(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;` | wraps `@Redirect at INVOKE Lnet/minecraft/client/resources/model/cuboid/CuboidModel;fromStream(Ljava/io/Reader;)Lnet/minecraft/client/resources/model/cuboid/CuboidModel;` | client | `ModelManagerMixin.cancelVanillaDeserialize` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]] | `lambda$loadBlockModels$2(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;` | injects_into `@ModifyArg at INVOKE Lcom/mojang/datafixers/util/Pair;of(Ljava/lang/Object;Ljava/lang/Object;)Lcom/mojang/datafixers/util/Pair;` | client | `ModelManagerMixin.actuallyDeserializeModel` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]] | `reload` | injects_into `@Inject at HEAD` | client | `ModelManagerMixin.onHeadReload` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]] | `reload` | injects_into `@ModifyArg at INVOKE Ljava/util/concurrent/CompletableFuture;thenApplyAsync(Ljava/util/function/Function;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | client | `ModelManagerMixin.hookModelCollect` |
| [[40-Interfaces/net.minecraft.client.resources.model.ModelManager|ModelManager]] | `reload` | injects_into `@ModifyArg at INVOKE Ljava/util/concurrent/CompletableFuture;thenComposeAsync(Ljava/util/function/Function;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | client | `ModelManagerMixin.hookModelBaking` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.BlockStateResolver|BlockStateResolver]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.CompositeBlockStateModel|CompositeBlockStateModel]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.CustomUnbakedBlockStateModel|CustomUnbakedBlockStateModel]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.ExtraModelKey|ExtraModelKey]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.FabricModelManager|FabricModelManager]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.ModelLoadingPlugin|ModelLoadingPlugin]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.ModelModifier|ModelModifier]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.PreparableModelLoadingPlugin|PreparableModelLoadingPlugin]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.SimpleUnbakedExtraModel|SimpleUnbakedExtraModel]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.UnbakedExtraModel|UnbakedExtraModel]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.UnbakedModelDeserializer|UnbakedModelDeserializer]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperBakedItemModel|WrapperBakedItemModel]] (abstract_class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperBlockStateModel|WrapperBlockStateModel]] (abstract_class, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperUnbakedItemModel|WrapperUnbakedItemModel]] (abstract_class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperUnbakedModel|WrapperUnbakedModel]] (abstract_class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperUnbakedRootBlockStateModel|WrapperUnbakedRootBlockStateModel]] (abstract_class, 6 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
