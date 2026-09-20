---
type: "interface"
fqcn: "net.minecraft.world.level.block.FireBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.FireBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `getBurnOdds` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| injects_into | `getIgniteOdds(Lnet/minecraft/world/level/block/state/BlockState;)I` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (47, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.FireBlock extends net.minecraft.world.level.block.BaseFireBlock {
    public static final int MAX_AGE;
    public static final net.minecraft.world.level.block.state.properties.IntegerProperty AGE;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty NORTH;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty EAST;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty SOUTH;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty WEST;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty UP;
    public static final java.util.Map<net.minecraft.core.Direction, net.minecraft.world.level.block.state.properties.BooleanProperty> PROPERTY_BY_DIRECTION;
    private final java.util.function.Function<net.minecraft.world.level.block.state.BlockState, net.minecraft.world.phys.shapes.VoxelShape> shapes;
    private static final int IGNITE_INSTANT;
    private static final int IGNITE_EASY;
    private static final int IGNITE_MEDIUM;
    private static final int IGNITE_HARD;
    private static final int BURN_INSTANT;
    private static final int BURN_EASY;
    private static final int BURN_MEDIUM;
    private static final int BURN_HARD;
    private final it.unimi.dsi.fastutil.objects.Object2IntMap<net.minecraft.world.level.block.Block> igniteOdds;
    private final it.unimi.dsi.fastutil.objects.Object2IntMap<net.minecraft.world.level.block.Block> burnOdds;
    public net.minecraft.world.level.block.FireBlock(net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    private java.util.function.Function<net.minecraft.world.level.block.state.BlockState, net.minecraft.world.phys.shapes.VoxelShape> makeShapes();
    protected net.minecraft.world.level.block.state.BlockState updateShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelReader, net.minecraft.world.level.ScheduledTickAccess, net.minecraft.core.BlockPos, net.minecraft.core.Direction, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource);
    protected net.minecraft.world.phys.shapes.VoxelShape getShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.phys.shapes.CollisionContext);
    public net.minecraft.world.level.block.state.BlockState getStateForPlacement(net.minecraft.world.item.context.BlockPlaceContext);
    protected net.minecraft.world.level.block.state.BlockState getStateForPlacement(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    protected boolean canSurvive(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelReader, net.minecraft.core.BlockPos);
    protected void tick(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    protected boolean isNearRain(net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    private int getBurnOdds(net.minecraft.world.level.block.state.BlockState);
    private int getIgniteOdds(net.minecraft.world.level.block.state.BlockState);
    private void checkBurnOut(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, int, net.minecraft.util.RandomSource, int);
    private net.minecraft.world.level.block.state.BlockState getStateWithAge(net.minecraft.world.level.LevelReader, net.minecraft.core.BlockPos, int);
    private boolean isValidFireLocation(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    private int getIgniteOdds(net.minecraft.world.level.LevelReader, net.minecraft.core.BlockPos);
    protected boolean canBurn(net.minecraft.world.level.block.state.BlockState);
    protected void onPlace(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, boolean);
    private static int getFireTickDelay(net.minecraft.util.RandomSource);
    protected void createBlockStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>);
    private void setFlammable(net.minecraft.world.level.block.Block, int, int);
    public static void bootStrap();
    private static void lambda$bootStrap$3(net.minecraft.world.level.block.FireBlock, net.minecraft.world.level.block.Block);
    private static void lambda$bootStrap$2(net.minecraft.world.level.block.FireBlock, net.minecraft.world.level.block.Block);
    private static void lambda$bootStrap$1(net.minecraft.world.level.block.FireBlock, net.minecraft.world.level.block.Block);
    private static void lambda$bootStrap$0(net.minecraft.world.level.block.FireBlock, net.minecraft.world.level.block.Block);
    private static net.minecraft.world.phys.shapes.VoxelShape lambda$makeShapes$0(java.util.Map, net.minecraft.world.level.block.state.BlockState);
    private static boolean lambda$static$0(java.util.Map$Entry);
    static {};
}
```
