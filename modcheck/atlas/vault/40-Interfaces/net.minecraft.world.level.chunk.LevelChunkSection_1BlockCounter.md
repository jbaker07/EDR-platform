---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.LevelChunkSection$1BlockCounter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.LevelChunkSection$1BlockCounter

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `accept(Lnet/minecraft/world/level/block/state/BlockState;I)V` | `@Redirect at INVOKE Lnet/minecraft/world/level/block/state/BlockState;isAir()Z` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
class net.minecraft.world.level.chunk.LevelChunkSection$1BlockCounter implements net.minecraft.world.level.chunk.PalettedContainer$CountConsumer<net.minecraft.world.level.block.state.BlockState> {
    public int nonEmptyBlockCount;
    public int fluidCount;
    public int tickingBlockCount;
    public int tickingFluidCount;
    net.minecraft.world.level.chunk.LevelChunkSection$1BlockCounter(net.minecraft.world.level.chunk.LevelChunkSection);
    public void accept(net.minecraft.world.level.block.state.BlockState, int);
    public void accept(java.lang.Object, int);
}
```
