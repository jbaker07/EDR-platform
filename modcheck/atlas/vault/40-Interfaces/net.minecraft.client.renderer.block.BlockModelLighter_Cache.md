---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockModelLighter$Cache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockModelLighter$Cache

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getLightCoords(Lnet/minecraft/world/level/block/state/BlockState;Lnet/mine` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords(Lnet/minecraft/world/level/block/state/BlockState;Lnet/mine` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness(Lnet/minecraft/world/level/block/state/BlockState;Lnet/mine` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.block.BlockModelLighter$Cache {
    private boolean enabled;
    private final it.unimi.dsi.fastutil.longs.Long2IntLinkedOpenHashMap colorCache;
    private final it.unimi.dsi.fastutil.longs.Long2FloatLinkedOpenHashMap brightnessCache;
    private final net.minecraft.util.LightCoordsUtil$BrightnessGetter cachedBrightnessGetter;
    public net.minecraft.client.renderer.block.BlockModelLighter$Cache();
    public void enable();
    public void disable();
    public int getLightCoords(net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos);
    public float getShadeBrightness(net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos);
    private int lambda$new$2(net.minecraft.world.level.BlockAndLightGetter, net.minecraft.core.BlockPos);
    private it.unimi.dsi.fastutil.longs.Long2FloatLinkedOpenHashMap lambda$new$1();
    private it.unimi.dsi.fastutil.longs.Long2IntLinkedOpenHashMap lambda$new$0();
}
```
