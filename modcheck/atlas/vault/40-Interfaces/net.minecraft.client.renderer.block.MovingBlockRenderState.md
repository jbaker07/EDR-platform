---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.MovingBlockRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.MovingBlockRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `blockPosLnet/minecraft/core/BlockPos;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `blockStateLnet/minecraft/world/level/block/state/BlockState;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `randomSeedPosLnet/minecraft/core/BlockPos;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.block.MovingBlockRenderState implements net.minecraft.client.renderer.block.BlockAndTintGetter {
    public net.minecraft.core.BlockPos randomSeedPos;
    public net.minecraft.core.BlockPos blockPos;
    public net.minecraft.world.level.block.state.BlockState blockState;
    public net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> biome;
    public net.minecraft.world.level.CardinalLighting cardinalLighting;
    public net.minecraft.world.level.lighting.LevelLightEngine lightEngine;
    public net.minecraft.client.renderer.block.MovingBlockRenderState();
    public net.minecraft.world.level.CardinalLighting cardinalLighting();
    public net.minecraft.world.level.lighting.LevelLightEngine getLightEngine();
    public int getBlockTint(net.minecraft.core.BlockPos, net.minecraft.world.level.ColorResolver);
    public net.minecraft.world.level.block.entity.BlockEntity getBlockEntity(net.minecraft.core.BlockPos);
    public net.minecraft.world.level.block.state.BlockState getBlockState(net.minecraft.core.BlockPos);
    public net.minecraft.world.level.material.FluidState getFluidState(net.minecraft.core.BlockPos);
    public int getHeight();
    public int getMinY();
}
```
