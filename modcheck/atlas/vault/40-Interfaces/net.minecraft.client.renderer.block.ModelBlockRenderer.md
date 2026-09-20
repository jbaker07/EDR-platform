---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.ModelBlockRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.ModelBlockRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `forceOpaque` | `(ZLnet/minecraft/world/level/block/state/BlockState;)Z` | exact | invokestatic@24 in `FabricBlockStateModel.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `forceOpaque` | `(ZLnet/minecraft/world/level/block/state/BlockState;)Z` | exact | invokestatic@24 in `SingleVariantMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `computeTintColor` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | exact | @Inject at ['FIELD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `computedTintValues` | `Lit/unimi/dsi/fastutil/ints/IntList;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| reads | `tintSources` | `Ljava/util/List;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |

## Declared members (14 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DIRECTIONS : [Lnet/minecraft/core/Direction;
private final lighter : Lnet/minecraft/client/renderer/block/BlockModelLighter;
private final ambientOcclusion : Z
private final cull : Z
private final blockColors : Lnet/minecraft/client/color/block/BlockColors;
private final random : Lnet/minecraft/util/RandomSource;
private final parts : Ljava/util/List;
private final scratchPos : Lnet/minecraft/core/BlockPos$MutableBlockPos;
private final quadInstance : Lcom/mojang/blaze3d/vertex/QuadInstance;
private tintCacheIndex : I
private tintCacheValue : I
private tintSourcesInitialized : Z
private final tintSources : Ljava/util/List;
private final computedTintValues : Lit/unimi/dsi/fastutil/ints/IntList;
public <init>(ZZLnet/minecraft/client/color/block/BlockColors;)V
public static forceOpaque(ZLnet/minecraft/world/level/block/state/BlockState;)Z
public tesselateBlock(Lnet/minecraft/client/renderer/block/BlockQuadOutput;FFFLnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;J)V
private configureTintCache(Lnet/minecraft/world/level/block/state/BlockState;)V
private resetTintCache()V
private tesselateAmbientOcclusion(Lnet/minecraft/client/renderer/block/BlockQuadOutput;FFFLjava/util/List;Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V
private tesselateFlat(Lnet/minecraft/client/renderer/block/BlockQuadOutput;FFFLjava/util/List;Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V
private shouldRenderFace(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;)Z
private putQuadWithTint(Lnet/minecraft/client/renderer/block/BlockQuadOutput;FFFLnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/client/resources/model/geometry/BakedQuad;)V
private getTintColor(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;I)I
private computeTintColor(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;I)I
static <clinit>()V
```
