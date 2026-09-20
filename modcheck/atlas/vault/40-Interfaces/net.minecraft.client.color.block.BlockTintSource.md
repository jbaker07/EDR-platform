---
type: "interface"
fqcn: "net.minecraft.client.color.block.BlockTintSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.color.block.BlockTintSource

System: [[20-Systems/net.minecraft.client.color|net.minecraft.client.color]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `colorInWorld(Lnet/minecraft/world/level/block/state/BlockState;Lnet/mine` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (4, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.color.block.BlockTintSource {
    public abstract int color(net.minecraft.world.level.block.state.BlockState);
    public default int colorInWorld(net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos);
    public default int colorAsTerrainParticle(net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos);
    public default java.util.Set<net.minecraft.world.level.block.state.properties.Property<?>> relevantProperties();
}
```
