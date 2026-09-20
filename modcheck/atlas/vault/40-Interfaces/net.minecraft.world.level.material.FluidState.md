---
type: "interface"
fqcn: "net.minecraft.world.level.material.FluidState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.material.FluidState

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getType()Lnet/minecraft/world/level/material/Fluid;` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getType()Lnet/minecraft/world/level/material/Fluid;` | `` | client | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |
| calls | `is(Lnet/minecraft/tags/TagKey;)Z` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `is(Lnet/minecraft/tags/TagKey;)Z` | `` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (28, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.material.FluidState extends net.minecraft.world.level.block.state.StateHolder<net.minecraft.world.level.material.Fluid, net.minecraft.world.level.material.FluidState> implements net.minecraft.core.TypedInstance<net.minecraft.world.level.material.Fluid> {
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.material.FluidState> CODEC;
    public static final int AMOUNT_MAX;
    public static final int AMOUNT_FULL;
    public net.minecraft.world.level.material.FluidState(net.minecraft.world.level.material.Fluid, net.minecraft.world.level.block.state.properties.Property<?>[], java.lang.Comparable<?>[]);
    public net.minecraft.world.level.material.Fluid getType();
    public boolean isSource();
    public boolean isSourceOfType(net.minecraft.world.level.material.Fluid);
    public boolean isEmpty();
    public float getHeight(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public float getHeightForCamera(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public float getOwnHeight();
    public boolean isFull();
    public int getAmount();
    public boolean shouldRenderBackwardUpFace(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public void tick(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public void animateTick(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    public boolean isRandomlyTicking();
    public void randomTick(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    public net.minecraft.world.phys.Vec3 getFlow(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public net.minecraft.world.level.block.state.BlockState createLegacyBlock();
    public net.minecraft.core.particles.ParticleOptions getDripParticle();
    public net.minecraft.core.Holder<net.minecraft.world.level.material.Fluid> typeHolder();
    public float getExplosionResistance();
    public boolean canBeReplacedWith(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.material.Fluid, net.minecraft.core.Direction);
    public net.minecraft.world.phys.shapes.VoxelShape getShape(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public net.minecraft.world.phys.AABB getAABB(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public void entityInside(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.Entity, net.minecraft.world.entity.InsideBlockEffectApplier);
    static {};
}
```
