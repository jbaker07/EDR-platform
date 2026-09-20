---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.LevelChunkSection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.LevelChunkSection

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `setBlockState(IIILnet/minecraft/world/level/block/state/BlockState;Z)Lnet/minecraft/world/level/block/state/BlockState;` | `@Redirect at INVOKE Lnet/minecraft/world/level/block/state/BlockState;isAir()Z` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (32, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.chunk.LevelChunkSection {
    public static final int BIOME_CONTAINER_BITS;
    private short nonEmptyBlockCount;
    private short fluidCount;
    private short tickingBlockCount;
    private short tickingFluidCount;
    private final net.minecraft.world.level.chunk.PalettedContainer<net.minecraft.world.level.block.state.BlockState> states;
    private net.minecraft.world.level.chunk.PalettedContainerRO<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> biomes;
    private net.minecraft.world.level.chunk.LevelChunkSection(net.minecraft.world.level.chunk.LevelChunkSection);
    public net.minecraft.world.level.chunk.LevelChunkSection(net.minecraft.world.level.chunk.PalettedContainer<net.minecraft.world.level.block.state.BlockState>, net.minecraft.world.level.chunk.PalettedContainerRO<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>>);
    public net.minecraft.world.level.chunk.LevelChunkSection(net.minecraft.world.level.chunk.PalettedContainerFactory);
    public net.minecraft.world.level.block.state.BlockState getBlockState(int, int, int);
    public net.minecraft.world.level.material.FluidState getFluidState(int, int, int);
    public void acquire();
    public void release();
    public net.minecraft.world.level.block.state.BlockState setBlockState(int, int, int, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.level.block.state.BlockState setBlockState(int, int, int, net.minecraft.world.level.block.state.BlockState, boolean);
    public boolean hasOnlyAir();
    public boolean hasFluid();
    public boolean isRandomlyTicking();
    public boolean isRandomlyTickingBlocks();
    public boolean isRandomlyTickingFluids();
    public void recalcBlockCounts();
    public net.minecraft.world.level.chunk.PalettedContainer<net.minecraft.world.level.block.state.BlockState> getStates();
    public net.minecraft.world.level.chunk.PalettedContainerRO<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> getBiomes();
    public void read(net.minecraft.network.FriendlyByteBuf);
    public void readBiomes(net.minecraft.network.FriendlyByteBuf);
    public void write(net.minecraft.network.FriendlyByteBuf);
    public int getSerializedSize();
    public boolean maybeHas(java.util.function.Predicate<net.minecraft.world.level.block.state.BlockState>);
    public net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome> getNoiseBiome(int, int, int);
    public void fillBiomesFromNoise(net.minecraft.world.level.biome.BiomeResolver, int, int, int);
    public net.minecraft.world.level.chunk.LevelChunkSection copy();
}
```
