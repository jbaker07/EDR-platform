---
type: "interface"
fqcn: "net.minecraft.client.resources.model.ModelManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.ModelManager

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `apply` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `discoverModelDependencies` | `@Inject at INVOKE Lnet/minecraft/client/resources/model/ModelDiscovery;resolve()` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `lambda$loadBlockModels$2(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;` | `@ModifyArg at INVOKE Lcom/mojang/datafixers/util/Pair;of(Ljava/lang/Object;Ljava` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `reload` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `reload` | `@ModifyArg at INVOKE Ljava/util/concurrent/CompletableFuture;thenApplyAsync(Ljav` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `reload` | `@ModifyArg at INVOKE Ljava/util/concurrent/CompletableFuture;thenComposeAsync(Lj` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| wraps | `lambda$loadBlockModels$2(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;` | `@Redirect at INVOKE Lnet/minecraft/client/resources/model/cuboid/CuboidModel;fro` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (44, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.resources.model.ModelManager implements net.minecraft.server.packs.resources.PreparableReloadListener {
    private static final org.slf4j.Logger LOGGER;
    private static final net.minecraft.resources.FileToIdConverter MODEL_LISTER;
    private java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.renderer.item.ItemModel> bakedItemStackModels;
    private java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.renderer.item.ClientItem$Properties> itemProperties;
    private final net.minecraft.client.resources.model.sprite.AtlasManager atlasManager;
    private final net.minecraft.client.renderer.PlayerSkinRenderCache playerSkinRenderCache;
    private final net.minecraft.client.color.block.BlockColors blockColors;
    private net.minecraft.client.model.geom.EntityModelSet entityModelSet;
    private net.minecraft.client.resources.model.ModelBakery$MissingModels missingModels;
    private net.minecraft.client.renderer.block.BlockStateModelSet blockStateModelSet;
    private net.minecraft.client.renderer.block.BlockModelSet blockModelSet;
    private net.minecraft.client.renderer.block.FluidStateModelSet fluidStateModelSet;
    private it.unimi.dsi.fastutil.objects.Object2IntMap<net.minecraft.world.level.block.state.BlockState> modelGroups;
    public net.minecraft.client.resources.model.ModelManager(net.minecraft.client.color.block.BlockColors, net.minecraft.client.resources.model.sprite.AtlasManager, net.minecraft.client.renderer.PlayerSkinRenderCache);
    public net.minecraft.client.renderer.item.ItemModel getItemModel(net.minecraft.resources.Identifier);
    public net.minecraft.client.renderer.item.ClientItem$Properties getItemProperties(net.minecraft.resources.Identifier);
    public net.minecraft.client.renderer.block.BlockStateModelSet getBlockStateModelSet();
    public net.minecraft.client.renderer.block.BlockModelSet getBlockModelSet();
    public net.minecraft.client.renderer.block.FluidStateModelSet getFluidStateModelSet();
    public final java.util.concurrent.CompletableFuture<java.lang.Void> reload(net.minecraft.server.packs.resources.PreparableReloadListener$SharedState, java.util.concurrent.Executor, net.minecraft.server.packs.resources.PreparableReloadListener$PreparationBarrier, java.util.concurrent.Executor);
    private static java.util.concurrent.CompletableFuture<java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.resources.model.UnbakedModel>> loadBlockModels(net.minecraft.server.packs.resources.ResourceManager, java.util.concurrent.Executor);
    private static net.minecraft.client.resources.model.ModelManager$ResolvedModels discoverModelDependencies(java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.resources.model.UnbakedModel>, net.minecraft.client.resources.model.BlockStateModelLoader$LoadedModels, net.minecraft.client.resources.model.ClientItemInfoLoader$LoadedClientInfos);
    private static java.util.concurrent.CompletableFuture<net.minecraft.client.resources.model.ModelManager$ReloadState> loadModels(net.minecraft.client.renderer.texture.SpriteLoader$Preparations, net.minecraft.client.renderer.texture.SpriteLoader$Preparations, net.minecraft.client.resources.model.ModelBakery, net.minecraft.client.renderer.block.LoadedBlockModels, it.unimi.dsi.fastutil.objects.Object2IntMap<net.minecraft.world.level.block.state.BlockState>, net.minecraft.client.model.geom.EntityModelSet, java.util.concurrent.Executor);
    private static java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel> createBlockStateToModelDispatch(java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel>, net.minecraft.client.renderer.block.dispatch.BlockStateModel);
    private static it.unimi.dsi.fastutil.objects.Object2IntMap<net.minecraft.world.level.block.state.BlockState> buildModelGroups(net.minecraft.client.color.block.BlockColors, net.minecraft.client.resources.model.BlockStateModelLoader$LoadedModels);
    private void apply(net.minecraft.client.resources.model.ModelManager$ReloadState);
    public boolean requiresRender(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState);
    public java.util.function.Supplier<net.minecraft.client.model.geom.EntityModelSet> entityModels();
    private net.minecraft.client.model.geom.EntityModelSet lambda$entityModels$0();
    private static void lambda$createBlockStateToModelDispatch$0(java.util.Map, net.minecraft.client.renderer.block.dispatch.BlockStateModel, net.minecraft.world.level.block.state.BlockState);
    private static net.minecraft.client.resources.model.ModelManager$ReloadState lambda$loadModels$1(net.minecraft.client.resources.model.sprite.MaterialBaker, it.unimi.dsi.fastutil.objects.Object2IntMap, net.minecraft.client.model.geom.EntityModelSet, net.minecraft.client.resources.model.ModelBakery$BakingResult, java.util.Map);
    private static java.util.concurrent.CompletionStage lambda$loadModels$0(net.minecraft.client.renderer.block.LoadedBlockModels, java.util.concurrent.Executor, net.minecraft.client.resources.model.ModelBakery$BakingResult);
    private static void lambda$discoverModelDependencies$0(net.minecraft.client.resources.model.ModelDiscovery, net.minecraft.client.renderer.item.ClientItem);
    private static java.util.concurrent.CompletionStage lambda$loadBlockModels$1(java.util.concurrent.Executor, java.util.Map);
    private static java.util.Map lambda$loadBlockModels$3(java.util.List);
    private static com.mojang.datafixers.util.Pair lambda$loadBlockModels$2(java.util.Map$Entry);
    private static java.util.Map lambda$loadBlockModels$0(net.minecraft.server.packs.resources.ResourceManager);
    private java.util.concurrent.CompletionStage lambda$reload$4(java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.util.concurrent.Executor, java.lang.Void);
    private static java.lang.String lambda$reload$5(net.minecraft.resources.Identifier);
    private net.minecraft.client.renderer.block.LoadedBlockModels lambda$reload$3(java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.lang.Void);
    private it.unimi.dsi.fastutil.objects.Object2IntMap lambda$reload$2(net.minecraft.client.resources.model.BlockStateModelLoader$LoadedModels);
    private static net.minecraft.client.resources.model.ModelManager$ResolvedModels lambda$reload$1(java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture, java.lang.Void);
    private java.util.Map lambda$reload$0();
    static {};
}
```
