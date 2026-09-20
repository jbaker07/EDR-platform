---
type: "interface"
fqcn: "net.minecraft.world.level.material.Fluid"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.material.Fluid

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `"<init>"()V` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `defaultFluidState()Lnet/minecraft/world/level/material/FluidState;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBucket()Lnet/minecraft/world/item/Item;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getStateDefinition()Lnet/minecraft/world/level/block/state/StateDefinition;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `isSource(Lnet/minecraft/world/level/material/FluidState;)Z` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| injects_into | `getPickupSound` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `FLUID_STATE_REGISTRYLnet/minecraft/core/IdMapper;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (33, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.level.material.Fluid {
    public static final net.minecraft.core.IdMapper<net.minecraft.world.level.material.FluidState> FLUID_STATE_REGISTRY;
    protected final net.minecraft.world.level.block.state.StateDefinition<net.minecraft.world.level.material.Fluid, net.minecraft.world.level.material.FluidState> stateDefinition;
    private net.minecraft.world.level.material.FluidState defaultFluidState;
    private final net.minecraft.core.Holder$Reference<net.minecraft.world.level.material.Fluid> builtInRegistryHolder;
    protected net.minecraft.world.level.material.Fluid();
    protected void createFluidStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.material.Fluid, net.minecraft.world.level.material.FluidState>);
    public net.minecraft.world.level.block.state.StateDefinition<net.minecraft.world.level.material.Fluid, net.minecraft.world.level.material.FluidState> getStateDefinition();
    protected final void registerDefaultState(net.minecraft.world.level.material.FluidState);
    public final net.minecraft.world.level.material.FluidState defaultFluidState();
    public abstract net.minecraft.world.item.Item getBucket();
    protected void animateTick(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.material.FluidState, net.minecraft.util.RandomSource);
    protected void tick(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.material.FluidState);
    protected void randomTick(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.material.FluidState, net.minecraft.util.RandomSource);
    protected void entityInside(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.Entity, net.minecraft.world.entity.InsideBlockEffectApplier);
    protected net.minecraft.core.particles.ParticleOptions getDripParticle();
    protected abstract boolean canBeReplacedWith(net.minecraft.world.level.material.FluidState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.material.Fluid, net.minecraft.core.Direction);
    protected abstract net.minecraft.world.phys.Vec3 getFlow(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.material.FluidState);
    public abstract int getTickDelay(net.minecraft.world.level.LevelReader);
    protected boolean isRandomlyTicking();
    protected boolean isEmpty();
    protected abstract float getExplosionResistance();
    public abstract float getHeight(net.minecraft.world.level.material.FluidState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public abstract float getOwnHeight(net.minecraft.world.level.material.FluidState);
    protected abstract net.minecraft.world.level.block.state.BlockState createLegacyBlock(net.minecraft.world.level.material.FluidState);
    public abstract boolean isSource(net.minecraft.world.level.material.FluidState);
    public abstract int getAmount(net.minecraft.world.level.material.FluidState);
    public boolean isSame(net.minecraft.world.level.material.Fluid);
    public boolean is(net.minecraft.tags.TagKey<net.minecraft.world.level.material.Fluid>);
    public abstract net.minecraft.world.phys.shapes.VoxelShape getShape(net.minecraft.world.level.material.FluidState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public net.minecraft.world.phys.AABB getAABB(net.minecraft.world.level.material.FluidState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public java.util.Optional<net.minecraft.sounds.SoundEvent> getPickupSound();
    public net.minecraft.core.Holder$Reference<net.minecraft.world.level.material.Fluid> builtInRegistryHolder();
    static {};
}
```
