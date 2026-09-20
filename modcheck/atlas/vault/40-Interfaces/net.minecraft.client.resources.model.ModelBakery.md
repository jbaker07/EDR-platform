---
type: "interface"
fqcn: "net.minecraft.client.resources.model.ModelBakery"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.ModelBakery

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Lnet/minecraft/client/model/geom/EntityModelSet;Lnet/minecraft/client` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `bakeModels` | `(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;Ljava/util` | name_only | @ModifyArg at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `bakeModels` | `(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;Ljava/util` | name_only | @ModifyReturnValue at ['RETURN'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| reads | `DESTROY_TYPES` | `Ljava/util/List;` | exact | getstatic@32 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `DESTROY_TYPES_OIT` | `Ljava/util/List;` | exact | getstatic@26 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `LOGGER` | `Lorg/slf4j/Logger;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | declared |
| reads | `resolvedModels` | `Ljava/util/Map;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | declared |
| wraps | `lambda$bakeModels$0` | `(Lnet/minecraft/client/resources/model/ModelBakery$ModelBakerImpl;Lnet` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| wraps | `lambda$bakeModels$1` | `(Lnet/minecraft/client/resources/model/ModelBakery$ModelBakerImpl;Lnet` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (18 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final FIRE_0 : Lnet/minecraft/client/resources/model/sprite/SpriteId;
public static final FIRE_1 : Lnet/minecraft/client/resources/model/sprite/SpriteId;
private static final DESTROY_STAGE_COUNT : I
private static final DESTROY_STAGES : Ljava/util/List;
private static final DESTROY_STAGES_OIT : Ljava/util/List;
private static final BREAKING_LOCATIONS : Ljava/util/List;
private static final BREAKING_LOCATIONS_OIT : Ljava/util/List;
public static final DESTROY_TYPES : Ljava/util/List;
public static final DESTROY_TYPES_OIT : Ljava/util/List;
private static final IDENTITY : Lorg/joml/Matrix4fc;
private final entityModelSet : Lnet/minecraft/client/model/geom/EntityModelSet;
private final sprites : Lnet/minecraft/client/resources/model/sprite/SpriteGetter;
private final playerSkinRenderCache : Lnet/minecraft/client/renderer/PlayerSkinRenderCache;
private final unbakedBlockStateModels : Ljava/util/Map;
private final clientInfos : Ljava/util/Map;
private final resolvedModels : Ljava/util/Map;
private final missingModel : Lnet/minecraft/client/resources/model/ResolvedModel;
public <init>(Lnet/minecraft/client/model/geom/EntityModelSet;Lnet/minecraft/client/resources/model/sprite/SpriteGetter;Lnet/minecraft/client/renderer/PlayerSkinRenderCache;Ljava/util/Map;Ljava/util/Map;Ljava/util/Map;Lnet/minecraft/client/resources/model/ResolvedModel;)V
public bakeModels(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$bakeModels$3(Lnet/minecraft/client/resources/model/ModelBakery$MissingModels;Ljava/util/Map;Ljava/util/Map;Ljava/util/Map;)Lnet/minecraft/client/resources/model/ModelBakery$BakingResult;
private static synthetic lambda$bakeModels$2(Ljava/util/Map;Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/renderer/item/ClientItem;)V
private synthetic lambda$bakeModels$1(Lnet/minecraft/client/resources/model/ModelBakery$ModelBakerImpl;Lnet/minecraft/client/resources/model/ModelBakery$MissingModels;Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/renderer/item/ClientItem;)Lnet/minecraft/client/renderer/item/ItemModel;
private static synthetic lambda$bakeModels$0(Lnet/minecraft/client/resources/model/ModelBakery$ModelBakerImpl;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel$UnbakedRoot;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;
private static synthetic lambda$static$4(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$static$5(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$static$2(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$static$3(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$static$1(I)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$static$0(I)Lnet/minecraft/resources/Identifier;
static <clinit>()V
```
