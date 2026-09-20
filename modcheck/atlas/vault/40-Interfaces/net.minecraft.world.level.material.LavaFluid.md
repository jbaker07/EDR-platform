---
type: "interface"
fqcn: "net.minecraft.world.level.material.LavaFluid"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.material.LavaFluid

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `spreadTo` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.level.material.LavaFluid extends net.minecraft.world.level.material.FlowingFluid {
    public static final int LIGHT_EMISSION;
    public static final float MIN_LEVEL_CUTOFF;
    public net.minecraft.world.level.material.LavaFluid();
    public net.minecraft.world.level.material.Fluid getFlowing();
    public net.minecraft.world.level.material.Fluid getSource();
    public net.minecraft.world.item.Item getBucket();
    public void animateTick(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.material.FluidState, net.minecraft.util.RandomSource);
    public void randomTick(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.material.FluidState, net.minecraft.util.RandomSource);
    protected void entityInside(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.Entity, net.minecraft.world.entity.InsideBlockEffectApplier);
    private boolean hasFlammableNeighbours(net.minecraft.world.level.LevelReader, net.minecraft.core.BlockPos);
    private boolean isFlammable(net.minecraft.world.level.LevelReader, net.minecraft.core.BlockPos);
    public net.minecraft.core.particles.ParticleOptions getDripParticle();
    protected void beforeDestroyingBlock(net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public int getSlopeFindDistance(net.minecraft.world.level.LevelReader);
    public net.minecraft.world.level.block.state.BlockState createLegacyBlock(net.minecraft.world.level.material.FluidState);
    public boolean isSame(net.minecraft.world.level.material.Fluid);
    public int getDropOff(net.minecraft.world.level.LevelReader);
    public boolean canBeReplacedWith(net.minecraft.world.level.material.FluidState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.material.Fluid, net.minecraft.core.Direction);
    public int getTickDelay(net.minecraft.world.level.LevelReader);
    public int getSpreadDelay(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.material.FluidState, net.minecraft.world.level.material.FluidState);
    private void fizz(net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos);
    protected boolean canConvertToSource(net.minecraft.server.level.ServerLevel);
    protected void spreadTo(net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.Direction, net.minecraft.world.level.material.FluidState);
    protected boolean isRandomlyTicking();
    protected float getExplosionResistance();
    public java.util.Optional<net.minecraft.sounds.SoundEvent> getPickupSound();
    private static boolean isFastLava(net.minecraft.world.level.LevelReader);
}
```
