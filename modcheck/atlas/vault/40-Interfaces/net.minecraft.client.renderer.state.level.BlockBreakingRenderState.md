---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.level.BlockBreakingRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.level.BlockBreakingRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `blockPos()Lnet/minecraft/core/BlockPos;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `blockState()Lnet/minecraft/world/level/block/state/BlockState;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.state.level.BlockBreakingRenderState extends java.lang.Record {
    private final net.minecraft.core.BlockPos blockPos;
    private final net.minecraft.world.level.block.state.BlockState blockState;
    private final int progress;
    public net.minecraft.client.renderer.state.level.BlockBreakingRenderState(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, int);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.BlockPos blockPos();
    public net.minecraft.world.level.block.state.BlockState blockState();
    public int progress();
}
```
