---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.LevelChunkSection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.LevelChunkSection

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `setBlockState` | `(IIILnet/minecraft/world/level/block/state/BlockState;Z)Lnet/minecraft` | exact | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (7 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final BIOME_CONTAINER_BITS : I
private nonEmptyBlockCount : S
private fluidCount : S
private tickingBlockCount : S
private tickingFluidCount : S
private final states : Lnet/minecraft/world/level/chunk/PalettedContainer;
private biomes : Lnet/minecraft/world/level/chunk/PalettedContainerRO;
private <init>(Lnet/minecraft/world/level/chunk/LevelChunkSection;)V
public <init>(Lnet/minecraft/world/level/chunk/PalettedContainer;Lnet/minecraft/world/level/chunk/PalettedContainerRO;)V
public <init>(Lnet/minecraft/world/level/chunk/PalettedContainerFactory;)V
public getBlockState(III)Lnet/minecraft/world/level/block/state/BlockState;
public getFluidState(III)Lnet/minecraft/world/level/material/FluidState;
public acquire()V
public release()V
public setBlockState(IIILnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;
public setBlockState(IIILnet/minecraft/world/level/block/state/BlockState;Z)Lnet/minecraft/world/level/block/state/BlockState;
public hasOnlyAir()Z
public hasFluid()Z
public isRandomlyTicking()Z
public isRandomlyTickingBlocks()Z
public isRandomlyTickingFluids()Z
public recalcBlockCounts()V
public getStates()Lnet/minecraft/world/level/chunk/PalettedContainer;
public getBiomes()Lnet/minecraft/world/level/chunk/PalettedContainerRO;
public read(Lnet/minecraft/network/FriendlyByteBuf;)V
public readBiomes(Lnet/minecraft/network/FriendlyByteBuf;)V
public write(Lnet/minecraft/network/FriendlyByteBuf;)V
public getSerializedSize()I
public maybeHas(Ljava/util/function/Predicate;)Z
public getNoiseBiome(III)Lnet/minecraft/core/Holder;
public fillBiomesFromNoise(Lnet/minecraft/world/level/biome/BiomeResolver;III)V
public copy()Lnet/minecraft/world/level/chunk/LevelChunkSection;
```
