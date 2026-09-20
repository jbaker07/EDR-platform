---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockModelLighter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockModelLighter

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@74 in `AoCalculator.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `prepareQuadAmbientOcclusion` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | exact | invokevirtual@79 in `AoCalculator.calcVanilla` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (8 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final CACHE_SIZE : I
private static final CACHE : Ljava/lang/ThreadLocal;
public static final CHECK_LIGHT : I
private final cache : Lnet/minecraft/client/renderer/block/BlockModelLighter$Cache;
private final scratchPos : Lnet/minecraft/core/BlockPos$MutableBlockPos;
private faceCubic : Z
private facePartial : Z
private final faceShape : [F
public <init>()V
public getLightCoords(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)I
public prepareQuadAmbientOcclusion(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;)V
public prepareQuadFlat(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;ILnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;)V
private prepareQuadShape(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/client/resources/model/geometry/BakedQuad;Z)V
private static getDirectionalBrightness(Lnet/minecraft/world/level/CardinalLighting;Lnet/minecraft/client/resources/model/geometry/BakedQuad;Lnet/minecraft/core/Direction;)F
public static enableCaching()V
public static clearCache()V
static <clinit>()V
```
