---
type: "interface"
fqcn: "net.minecraft.core.particles.BlockParticleOption"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.particles.BlockParticleOption

System: [[20-Systems/net.minecraft.core.particles|net.minecraft.core.particles]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/w` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getBlockPos()Lnet/minecraft/core/BlockPos;` | `` | client | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getBlockPos()Lnet/minecraft/core/BlockPos;` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.particles.BlockParticleOption implements net.minecraft.core.particles.ParticleOptions {
    private static final com.mojang.serialization.Codec<net.minecraft.world.level.block.state.BlockState> BLOCK_STATE_CODEC;
    private final net.minecraft.core.particles.ParticleType<net.minecraft.core.particles.BlockParticleOption> type;
    private final net.minecraft.world.level.block.state.BlockState state;
    public static com.mojang.serialization.MapCodec<net.minecraft.core.particles.BlockParticleOption> codec(net.minecraft.core.particles.ParticleType<net.minecraft.core.particles.BlockParticleOption>);
    public static net.minecraft.network.codec.StreamCodec<? super net.minecraft.network.RegistryFriendlyByteBuf, net.minecraft.core.particles.BlockParticleOption> streamCodec(net.minecraft.core.particles.ParticleType<net.minecraft.core.particles.BlockParticleOption>);
    public net.minecraft.core.particles.BlockParticleOption(net.minecraft.core.particles.ParticleType<net.minecraft.core.particles.BlockParticleOption>, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.core.particles.ParticleType<net.minecraft.core.particles.BlockParticleOption> getType();
    public net.minecraft.world.level.block.state.BlockState getState();
    private static net.minecraft.world.level.block.state.BlockState lambda$streamCodec$1(net.minecraft.core.particles.BlockParticleOption);
    private static net.minecraft.core.particles.BlockParticleOption lambda$streamCodec$0(net.minecraft.core.particles.ParticleType, net.minecraft.world.level.block.state.BlockState);
    private static net.minecraft.world.level.block.state.BlockState lambda$codec$1(net.minecraft.core.particles.BlockParticleOption);
    private static net.minecraft.core.particles.BlockParticleOption lambda$codec$0(net.minecraft.core.particles.ParticleType, net.minecraft.world.level.block.state.BlockState);
    static {};
}
```
