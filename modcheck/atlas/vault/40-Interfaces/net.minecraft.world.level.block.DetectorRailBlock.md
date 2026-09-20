---
type: "interface"
fqcn: "net.minecraft.world.level.block.DetectorRailBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.DetectorRailBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `getAnalogOutputSignal` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `POWEREDLnet/minecraft/world/level/block/state/properties/BooleanPro` | `` | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.DetectorRailBlock extends net.minecraft.world.level.block.BaseRailBlock {
    public static final net.minecraft.world.level.block.state.properties.EnumProperty<net.minecraft.world.level.block.state.properties.RailShape> SHAPE;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty POWERED;
    private static final int PRESSED_CHECK_PERIOD;
    public net.minecraft.world.level.block.DetectorRailBlock(net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    protected boolean isSignalSource(net.minecraft.world.level.block.state.BlockState);
    protected int ownSignal(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    protected void entityInside(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.Entity, net.minecraft.world.entity.InsideBlockEffectApplier, boolean);
    protected void tick(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    protected int getDirectSignal(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.core.Direction);
    private void checkPressed(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    protected void updatePowerToConnected(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, boolean);
    protected void onPlace(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, boolean);
    public net.minecraft.world.level.block.state.properties.Property<net.minecraft.world.level.block.state.properties.RailShape> getShapeProperty();
    protected boolean hasAnalogOutputSignal(net.minecraft.world.level.block.state.BlockState);
    protected int getAnalogOutputSignal(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.core.Direction);
    private <T extends net.minecraft.world.entity.vehicle.minecart.AbstractMinecart> java.util.List<T> getInteractingMinecartOfType(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, java.lang.Class<T>, java.util.function.Predicate<net.minecraft.world.entity.Entity>);
    private net.minecraft.world.phys.AABB getSearchBB(net.minecraft.core.BlockPos);
    protected net.minecraft.world.level.block.state.BlockState rotate(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Rotation);
    protected net.minecraft.world.level.block.state.BlockState mirror(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Mirror);
    protected void createBlockStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>);
    private static boolean lambda$getAnalogOutputSignal$0(net.minecraft.world.entity.Entity);
    private static boolean lambda$checkPressed$0(net.minecraft.world.entity.Entity);
    static {};
}
```
