---
type: "interface"
fqcn: "net.minecraft.client.resources.model.ModelBakery"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.ModelBakery

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| injects_into | `bakeModels` | `@ModifyArg at INVOKE Lnet/minecraft/util/thread/ParallelMapTransform;schedule(Lj` | client | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| reads | `DESTROY_TYPESLjava/util/List;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `DESTROY_TYPES_OITLjava/util/List;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (31, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.resources.model.ModelBakery {
    private static final org.slf4j.Logger LOGGER;
    public static final net.minecraft.client.resources.model.sprite.SpriteId FIRE_0;
    public static final net.minecraft.client.resources.model.sprite.SpriteId FIRE_1;
    private static final int DESTROY_STAGE_COUNT;
    private static final java.util.List<net.minecraft.resources.Identifier> DESTROY_STAGES;
    private static final java.util.List<net.minecraft.resources.Identifier> DESTROY_STAGES_OIT;
    private static final java.util.List<net.minecraft.resources.Identifier> BREAKING_LOCATIONS;
    private static final java.util.List<net.minecraft.resources.Identifier> BREAKING_LOCATIONS_OIT;
    public static final java.util.List<net.minecraft.client.renderer.rendertype.RenderType> DESTROY_TYPES;
    public static final java.util.List<net.minecraft.client.renderer.rendertype.RenderType> DESTROY_TYPES_OIT;
    private static final org.joml.Matrix4fc IDENTITY;
    private final net.minecraft.client.model.geom.EntityModelSet entityModelSet;
    private final net.minecraft.client.resources.model.sprite.SpriteGetter sprites;
    private final net.minecraft.client.renderer.PlayerSkinRenderCache playerSkinRenderCache;
    private final java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel$UnbakedRoot> unbakedBlockStateModels;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.renderer.item.ClientItem> clientInfos;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.resources.model.ResolvedModel> resolvedModels;
    private final net.minecraft.client.resources.model.ResolvedModel missingModel;
    public net.minecraft.client.resources.model.ModelBakery(net.minecraft.client.model.geom.EntityModelSet, net.minecraft.client.resources.model.sprite.SpriteGetter, net.minecraft.client.renderer.PlayerSkinRenderCache, java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel$UnbakedRoot>, java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.renderer.item.ClientItem>, java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.resources.model.ResolvedModel>, net.minecraft.client.resources.model.ResolvedModel);
    public java.util.concurrent.CompletableFuture<net.minecraft.client.resources.model.ModelBakery$BakingResult> bakeModels(net.minecraft.client.resources.model.sprite.MaterialBaker, java.util.concurrent.Executor);
    private static net.minecraft.client.resources.model.ModelBakery$BakingResult lambda$bakeModels$3(net.minecraft.client.resources.model.ModelBakery$MissingModels, java.util.Map, java.util.Map, java.util.Map);
    private static void lambda$bakeModels$2(java.util.Map, net.minecraft.resources.Identifier, net.minecraft.client.renderer.item.ClientItem);
    private net.minecraft.client.renderer.item.ItemModel lambda$bakeModels$1(net.minecraft.client.resources.model.ModelBakery$ModelBakerImpl, net.minecraft.client.resources.model.ModelBakery$MissingModels, net.minecraft.resources.Identifier, net.minecraft.client.renderer.item.ClientItem);
    private static net.minecraft.client.renderer.block.dispatch.BlockStateModel lambda$bakeModels$0(net.minecraft.client.resources.model.ModelBakery$ModelBakerImpl, net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel$UnbakedRoot);
    private static net.minecraft.resources.Identifier lambda$static$4(net.minecraft.resources.Identifier);
    private static java.lang.String lambda$static$5(java.lang.String);
    private static net.minecraft.resources.Identifier lambda$static$2(net.minecraft.resources.Identifier);
    private static java.lang.String lambda$static$3(java.lang.String);
    private static net.minecraft.resources.Identifier lambda$static$1(int);
    private static net.minecraft.resources.Identifier lambda$static$0(int);
    static {};
}
```
