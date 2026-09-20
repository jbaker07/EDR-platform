---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockModelLighter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockModelLighter

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `prepareQuadAmbientOcclusion(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.block.BlockModelLighter {
    private static final int CACHE_SIZE;
    private static final java.lang.ThreadLocal<net.minecraft.client.renderer.block.BlockModelLighter$Cache> CACHE;
    public static final int CHECK_LIGHT;
    private final net.minecraft.client.renderer.block.BlockModelLighter$Cache cache;
    private final net.minecraft.core.BlockPos$MutableBlockPos scratchPos;
    private boolean faceCubic;
    private boolean facePartial;
    private final float[] faceShape;
    public net.minecraft.client.renderer.block.BlockModelLighter();
    public int getLightCoords(net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos);
    public void prepareQuadAmbientOcclusion(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, net.minecraft.client.resources.model.geometry.BakedQuad, com.mojang.blaze3d.vertex.QuadInstance);
    public void prepareQuadFlat(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, int, net.minecraft.client.resources.model.geometry.BakedQuad, com.mojang.blaze3d.vertex.QuadInstance);
    private void prepareQuadShape(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, net.minecraft.client.resources.model.geometry.BakedQuad, boolean);
    private static float getDirectionalBrightness(net.minecraft.world.level.CardinalLighting, net.minecraft.client.resources.model.geometry.BakedQuad, net.minecraft.core.Direction);
    public static void enableCaching();
    public static void clearCache();
    static {};
}
```
