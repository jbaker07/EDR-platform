---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.FluidRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.FluidRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `tesselate(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lne` | `` | unknown | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |
| injects_into | `tesselate` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.block.FluidRenderer {
    private static final float MAX_FLUID_HEIGHT;
    private final net.minecraft.client.renderer.block.FluidStateModelSet fluidModels;
    public net.minecraft.client.renderer.block.FluidRenderer(net.minecraft.client.renderer.block.FluidStateModelSet);
    private static boolean isNeighborSameFluid(net.minecraft.world.level.material.FluidState, net.minecraft.world.level.material.FluidState);
    private static boolean isFaceOccludedByState(net.minecraft.core.Direction, float, net.minecraft.world.level.block.state.BlockState);
    private static boolean isFaceOccludedByNeighbor(net.minecraft.core.Direction, float, net.minecraft.world.level.block.state.BlockState);
    private static boolean isFaceOccludedBySelf(net.minecraft.world.level.block.state.BlockState, net.minecraft.core.Direction);
    public static boolean shouldRenderFace(net.minecraft.world.level.material.FluidState, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.Direction, net.minecraft.world.level.material.FluidState);
    public void tesselate(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.client.renderer.block.FluidRenderer$Output, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.material.FluidState);
    private void addFace(com.mojang.blaze3d.vertex.VertexConsumer, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, float, int, int, boolean);
    private float calculateAverageHeight(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.material.Fluid, float, float, float, net.minecraft.core.BlockPos);
    private void addWeightedHeight(float[], float);
    private float getHeight(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.material.Fluid, net.minecraft.core.BlockPos);
    private float getHeight(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.world.level.material.Fluid, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.material.FluidState);
    private void vertex(com.mojang.blaze3d.vertex.VertexConsumer, float, float, float, int, float, float, int);
    private int getLightCoords(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos);
}
```
