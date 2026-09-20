---
type: "interface"
fqcn: "net.minecraft.client.resources.model.ModelManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.ModelManager

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/packs/resources/PreparableReloadListener`, `net/fabricmc/fabric/api/client/model/loading/v1/FabricModelManager`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getFluidStateModelSet` | `()Lnet/minecraft/client/renderer/block/FluidStateModelSet;` | exact | invokevirtual@17 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| injects_into | `apply` | `(Lnet/minecraft/client/resources/model/ModelManager$ReloadState;)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `discoverModelDependencies` | `(Ljava/util/Map;Lnet/minecraft/client/resources/model/BlockStateModelL` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `lambda$loadBlockModels$2` | `(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;` | exact | @ModifyArg at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$Shared` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$Shared` | name_only | @ModifyReturnValue at ['RETURN'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$Shared` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$Shared` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$Shared` | name_only | @ModifyArg at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `reload` | `(Lnet/minecraft/server/packs/resources/PreparableReloadListener$Shared` | name_only | @ModifyArg at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| wraps | `lambda$loadBlockModels$2` | `(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;` | exact | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (13 fields, 31 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final MODEL_LISTER : Lnet/minecraft/resources/FileToIdConverter;
private bakedItemStackModels : Ljava/util/Map;
private itemProperties : Ljava/util/Map;
private final atlasManager : Lnet/minecraft/client/resources/model/sprite/AtlasManager;
private final playerSkinRenderCache : Lnet/minecraft/client/renderer/PlayerSkinRenderCache;
private final blockColors : Lnet/minecraft/client/color/block/BlockColors;
private entityModelSet : Lnet/minecraft/client/model/geom/EntityModelSet;
private missingModels : Lnet/minecraft/client/resources/model/ModelBakery$MissingModels;
private blockStateModelSet : Lnet/minecraft/client/renderer/block/BlockStateModelSet;
private blockModelSet : Lnet/minecraft/client/renderer/block/BlockModelSet;
private fluidStateModelSet : Lnet/minecraft/client/renderer/block/FluidStateModelSet;
private modelGroups : Lit/unimi/dsi/fastutil/objects/Object2IntMap;
public <init>(Lnet/minecraft/client/color/block/BlockColors;Lnet/minecraft/client/resources/model/sprite/AtlasManager;Lnet/minecraft/client/renderer/PlayerSkinRenderCache;)V
public getItemModel(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/item/ItemModel;
public getItemProperties(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/item/ClientItem$Properties;
public getBlockStateModelSet()Lnet/minecraft/client/renderer/block/BlockStateModelSet;
public getBlockModelSet()Lnet/minecraft/client/renderer/block/BlockModelSet;
public getFluidStateModelSet()Lnet/minecraft/client/renderer/block/FluidStateModelSet;
public final reload(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static loadBlockModels(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static discoverModelDependencies(Ljava/util/Map;Lnet/minecraft/client/resources/model/BlockStateModelLoader$LoadedModels;Lnet/minecraft/client/resources/model/ClientItemInfoLoader$LoadedClientInfos;)Lnet/minecraft/client/resources/model/ModelManager$ResolvedModels;
private static loadModels(Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;Lnet/minecraft/client/resources/model/ModelBakery;Lnet/minecraft/client/renderer/block/LoadedBlockModels;Lit/unimi/dsi/fastutil/objects/Object2IntMap;Lnet/minecraft/client/model/geom/EntityModelSet;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static createBlockStateToModelDispatch(Ljava/util/Map;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;)Ljava/util/Map;
private static buildModelGroups(Lnet/minecraft/client/color/block/BlockColors;Lnet/minecraft/client/resources/model/BlockStateModelLoader$LoadedModels;)Lit/unimi/dsi/fastutil/objects/Object2IntMap;
private apply(Lnet/minecraft/client/resources/model/ModelManager$ReloadState;)V
public requiresRender(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)Z
public entityModels()Ljava/util/function/Supplier;
private synthetic lambda$entityModels$0()Lnet/minecraft/client/model/geom/EntityModelSet;
private static synthetic lambda$createBlockStateToModelDispatch$0(Ljava/util/Map;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;Lnet/minecraft/world/level/block/state/BlockState;)V
private static synthetic lambda$loadModels$1(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;Lit/unimi/dsi/fastutil/objects/Object2IntMap;Lnet/minecraft/client/model/geom/EntityModelSet;Lnet/minecraft/client/resources/model/ModelBakery$BakingResult;Ljava/util/Map;)Lnet/minecraft/client/resources/model/ModelManager$ReloadState;
private static synthetic lambda$loadModels$0(Lnet/minecraft/client/renderer/block/LoadedBlockModels;Ljava/util/concurrent/Executor;Lnet/minecraft/client/resources/model/ModelBakery$BakingResult;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$discoverModelDependencies$0(Lnet/minecraft/client/resources/model/ModelDiscovery;Lnet/minecraft/client/renderer/item/ClientItem;)V
private static synthetic lambda$loadBlockModels$1(Ljava/util/concurrent/Executor;Ljava/util/Map;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$loadBlockModels$3(Ljava/util/List;)Ljava/util/Map;
private static synthetic lambda$loadBlockModels$2(Ljava/util/Map$Entry;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$loadBlockModels$0(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map;
private synthetic lambda$reload$4(Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/Executor;Ljava/lang/Void;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$reload$5(Lnet/minecraft/resources/Identifier;)Ljava/lang/String;
private synthetic lambda$reload$3(Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/lang/Void;)Lnet/minecraft/client/renderer/block/LoadedBlockModels;
private synthetic lambda$reload$2(Lnet/minecraft/client/resources/model/BlockStateModelLoader$LoadedModels;)Lit/unimi/dsi/fastutil/objects/Object2IntMap;
private static synthetic lambda$reload$1(Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/util/concurrent/CompletableFuture;Ljava/lang/Void;)Lnet/minecraft/client/resources/model/ModelManager$ResolvedModels;
private synthetic lambda$reload$0()Ljava/util/Map;
static <clinit>()V
```
