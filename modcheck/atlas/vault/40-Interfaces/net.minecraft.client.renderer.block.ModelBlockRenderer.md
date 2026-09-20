---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.ModelBlockRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.ModelBlockRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `forceOpaque(ZLnet/minecraft/world/level/block/state/BlockState;)Z` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `computeTintColor(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;I)I` | `@Inject at FIELD Lnet/minecraft/client/renderer/block/ModelBlockRenderer;tintSou` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.block.ModelBlockRenderer {
    private static final net.minecraft.core.Direction[] DIRECTIONS;
    private final net.minecraft.client.renderer.block.BlockModelLighter lighter;
    private final boolean ambientOcclusion;
    private final boolean cull;
    private final net.minecraft.client.color.block.BlockColors blockColors;
    private final net.minecraft.util.RandomSource random;
    private final java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart> parts;
    private final net.minecraft.core.BlockPos$MutableBlockPos scratchPos;
    private final com.mojang.blaze3d.vertex.QuadInstance quadInstance;
    private int tintCacheIndex;
    private int tintCacheValue;
    private boolean tintSourcesInitialized;
    private final java.util.List<net.minecraft.client.color.block.BlockTintSource> tintSources;
    private final it.unimi.dsi.fastutil.ints.IntList computedTintValues;
    public net.minecraft.client.renderer.block.ModelBlockRenderer(boolean, boolean, net.minecraft.client.color.block.BlockColors);
    public static boolean forceOpaque(boolean, net.minecraft.world.level.block.state.BlockState);
    public void tesselateBlock(net.minecraft.client.renderer.block.BlockQuadOutput, float, float, float, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel, long);
    private void configureTintCache(net.minecraft.world.level.block.state.BlockState);
    private void resetTintCache();
    private void tesselateAmbientOcclusion(net.minecraft.client.renderer.block.BlockQuadOutput, float, float, float, java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart>, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos);
    private void tesselateFlat(net.minecraft.client.renderer.block.BlockQuadOutput, float, float, float, java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart>, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos);
    private boolean shouldRenderFace(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.Direction, net.minecraft.core.BlockPos);
    private void putQuadWithTint(net.minecraft.client.renderer.block.BlockQuadOutput, float, float, float, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, net.minecraft.client.resources.model.geometry.BakedQuad);
    private int getTintColor(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, int);
    private int computeTintColor(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, int);
    static {};
}
```
