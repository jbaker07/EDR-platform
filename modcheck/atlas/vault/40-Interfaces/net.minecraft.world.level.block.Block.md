---
type: "interface"
fqcn: "net.minecraft.world.level.block.Block"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.Block

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/block/state/BlockBehaviour$Prope` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `defaultBlockState()Lnet/minecraft/world/level/block/state/BlockState;` | `` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `defaultBlockState()Lnet/minecraft/world/level/block/state/BlockState;` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `defaultBlockState()Lnet/minecraft/world/level/block/state/BlockState;` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `defaultBlockState()Lnet/minecraft/world/level/block/state/BlockState;` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `defaultBlockState()Lnet/minecraft/world/level/block/state/BlockState;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getStateDefinition()Lnet/minecraft/world/level/block/state/StateDefinition;` | `` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getStateDefinition()Lnet/minecraft/world/level/block/state/StateDefinition;` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getStateDefinition()Lnet/minecraft/world/level/block/state/StateDefinition;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getStateDefinition()Lnet/minecraft/world/level/block/state/StateDefinition;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getStateDefinition()Lnet/minecraft/world/level/block/state/StateDefinition;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `shouldRenderFace(Lnet/minecraft/world/level/block/state/BlockState;Lnet/mine` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `BLOCK_STATE_REGISTRYLnet/minecraft/core/IdMapper;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (111, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.Block extends net.minecraft.world.level.block.state.BlockBehaviour implements net.minecraft.world.level.ItemLike {
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.core.Holder$Reference<net.minecraft.world.level.block.Block> builtInRegistryHolder;
    public static final net.minecraft.core.IdMapper<net.minecraft.world.level.block.state.BlockState> BLOCK_STATE_REGISTRY;
    private static final com.google.common.cache.LoadingCache<net.minecraft.world.phys.shapes.VoxelShape, java.lang.Boolean> SHAPE_FULL_BLOCK_CACHE;
    public static final int UPDATE_NEIGHBORS;
    public static final int UPDATE_CLIENTS;
    public static final int UPDATE_INVISIBLE;
    public static final int UPDATE_IMMEDIATE;
    public static final int UPDATE_KNOWN_SHAPE;
    public static final int UPDATE_SUPPRESS_DROPS;
    public static final int UPDATE_MOVE_BY_PISTON;
    public static final int UPDATE_SKIP_SHAPE_UPDATE_ON_WIRE;
    public static final int UPDATE_SKIP_BLOCK_ENTITY_SIDEEFFECTS;
    public static final int UPDATE_SKIP_ON_PLACE;
    public static final int UPDATE_NONE;
    public static final int UPDATE_ALL;
    public static final int UPDATE_ALL_IMMEDIATE;
    public static final int UPDATE_SKIP_ALL_SIDEEFFECTS;
    public static final float INDESTRUCTIBLE;
    public static final float INSTANT;
    public static final int UPDATE_LIMIT;
    protected final net.minecraft.world.level.block.state.StateDefinition<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState> stateDefinition;
    private net.minecraft.world.level.block.state.BlockState defaultBlockState;
    private net.minecraft.world.item.Item item;
    private static final int CACHE_SIZE;
    private static final java.lang.ThreadLocal<it.unimi.dsi.fastutil.objects.Object2ByteLinkedOpenHashMap<net.minecraft.world.level.block.Block$ShapePairKey>> OCCLUSION_CACHE;
    public static int getId(net.minecraft.world.level.block.state.BlockState);
    public static net.minecraft.world.level.block.state.BlockState stateById(int);
    public static net.minecraft.world.level.block.Block byItem(net.minecraft.world.item.Item);
    public static net.minecraft.world.level.block.state.BlockState pushEntitiesUp(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos);
    public static net.minecraft.world.phys.shapes.VoxelShape box(double, double, double, double, double, double);
    public static net.minecraft.world.phys.shapes.VoxelShape[] boxes(int, java.util.function.IntFunction<net.minecraft.world.phys.shapes.VoxelShape>);
    public static net.minecraft.world.phys.shapes.VoxelShape cube(double);
    public static net.minecraft.world.phys.shapes.VoxelShape cube(double, double, double);
    public static net.minecraft.world.phys.shapes.VoxelShape column(double, double, double);
    public static net.minecraft.world.phys.shapes.VoxelShape column(double, double, double, double);
    public static net.minecraft.world.phys.shapes.VoxelShape boxZ(double, double, double);
    public static net.minecraft.world.phys.shapes.VoxelShape boxZ(double, double, double, double);
    public static net.minecraft.world.phys.shapes.VoxelShape boxZ(double, double, double, double, double);
    public static net.minecraft.world.level.block.state.BlockState updateFromNeighbourShapes(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos);
    public static void updateOrDestroy(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos, int);
    public static void updateOrDestroy(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos, int, int);
    public net.minecraft.world.level.block.Block(net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    public static boolean isExceptionForConnection(net.minecraft.world.level.block.state.BlockState);
    public static boolean dropFromBlockInteractLootTable(net.minecraft.server.level.ServerLevel, net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.BlockEntity, net.minecraft.world.item.ItemInstance, net.minecraft.world.entity.Entity, java.util.function.BiConsumer<net.minecraft.server.level.ServerLevel, net.minecraft.world.item.ItemStack>);
    protected static boolean dropFromLootTable(net.minecraft.server.level.ServerLevel, net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>, java.util.function.Function<net.minecraft.world.level.storage.loot.LootParams$Builder, net.minecraft.world.level.storage.loot.LootParams>, java.util.function.BiConsumer<net.minecraft.server.level.ServerLevel, net.minecraft.world.item.ItemStack>);
    public static boolean shouldRenderFace(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.Direction);
    public static boolean canSupportRigidBlock(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public static boolean canSupportCenter(net.minecraft.world.level.LevelReader, net.minecraft.core.BlockPos, net.minecraft.core.Direction);
    public static boolean isFaceFull(net.minecraft.world.phys.shapes.VoxelShape, net.minecraft.core.Direction);
    public static boolean isShapeFullBlock(net.minecraft.world.phys.shapes.VoxelShape);
    public void animateTick(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    public void destroy(net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public static java.util.List<net.minecraft.world.item.ItemStack> getDrops(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.BlockEntity);
    public static java.util.List<net.minecraft.world.item.ItemStack> getDrops(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.BlockEntity, net.minecraft.world.entity.Entity, net.minecraft.world.item.ItemInstance);
    public static void dropResources(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    public static void dropResources(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.BlockEntity);
    public static void dropResources(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.BlockEntity, net.minecraft.world.entity.Entity, net.minecraft.world.item.ItemStack);
    public static void popResource(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.item.ItemStack);
    public static void popResourceFromFace(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.core.Direction, net.minecraft.world.item.ItemStack);
    private static void popResource(net.minecraft.world.level.Level, java.util.function.Supplier<net.minecraft.world.entity.item.ItemEntity>, net.minecraft.world.item.ItemStack);
    protected void popExperience(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, int);
    public float getExplosionResistance();
    public void wasExploded(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.Explosion);
    public void stepOn(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.entity.Entity);
    public net.minecraft.world.level.block.state.BlockState getStateForPlacement(net.minecraft.world.item.context.BlockPlaceContext);
    public void playerDestroy(net.minecraft.server.level.ServerLevel, net.minecraft.server.level.ServerPlayer, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.BlockEntity, net.minecraft.world.item.ItemStack);
    public void setPlacedBy(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.entity.LivingEntity, net.minecraft.world.item.ItemStack);
    public boolean isPossibleToRespawnInThis(net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.network.chat.MutableComponent getName();
    public void fallOn(net.minecraft.world.level.Level, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, net.minecraft.world.entity.Entity, double);
    public void bounceOn(net.minecraft.world.level.Level, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, net.minecraft.world.entity.Entity, double);
    public float getBounceRestitution();
    public float getFallDistanceReduction();
    public float getFriction();
    public float getSpeedFactor();
    public float getJumpFactor();
    public void spawnDestroyByEntityParticles(net.minecraft.world.level.Level, net.minecraft.world.entity.Entity, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public void spawnDestroyParticles(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.level.block.state.BlockState playerWillDestroy(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.entity.player.Player);
    public void handlePrecipitation(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.biome.Biome$Precipitation);
    public boolean dropFromExplosion(net.minecraft.world.level.Explosion);
    protected void createBlockStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>);
    public net.minecraft.world.level.block.state.StateDefinition<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState> getStateDefinition();
    protected final void registerDefaultState(net.minecraft.world.level.block.state.BlockState);
    public final net.minecraft.world.level.block.state.BlockState defaultBlockState();
    public final net.minecraft.world.level.block.state.BlockState withPropertiesOf(net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.item.Item asItem();
    public boolean hasDynamicShape();
    public java.lang.String toString();
    protected net.minecraft.world.level.block.Block asBlock();
    protected java.util.function.Function<net.minecraft.world.level.block.state.BlockState, net.minecraft.world.phys.shapes.VoxelShape> getShapeForEachState(java.util.function.Function<net.minecraft.world.level.block.state.BlockState, net.minecraft.world.phys.shapes.VoxelShape>);
    protected java.util.function.Function<net.minecraft.world.level.block.state.BlockState, net.minecraft.world.phys.shapes.VoxelShape> getShapeForEachState(java.util.function.Function<net.minecraft.world.level.block.state.BlockState, net.minecraft.world.phys.shapes.VoxelShape>, net.minecraft.world.level.block.state.properties.Property<?>...);
    private static <S extends net.minecraft.world.level.block.state.StateHolder<?, S>, T extends java.lang.Comparable<T>> S setValueHelper(S, net.minecraft.world.level.block.state.properties.Property<T>, java.lang.Object);
    public net.minecraft.core.Holder$Reference<net.minecraft.world.level.block.Block> builtInRegistryHolder();
    protected void tryDropExperience(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.item.ItemStack, net.minecraft.util.valueproviders.IntProvider);
    private static net.minecraft.world.phys.shapes.VoxelShape lambda$getShapeForEachState$4(java.util.Map, com.google.common.collect.ImmutableMap, net.minecraft.world.level.block.state.BlockState);
    private static boolean lambda$getShapeForEachState$2(java.util.Map, net.minecraft.world.level.block.state.BlockState);
    private static boolean lambda$getShapeForEachState$3(net.minecraft.world.level.block.state.BlockState, java.util.Map$Entry);
    private static java.lang.Object lambda$getShapeForEachState$1(net.minecraft.world.level.block.state.properties.Property);
    private static net.minecraft.world.level.block.state.properties.Property lambda$getShapeForEachState$0(net.minecraft.world.level.block.state.properties.Property);
    private static net.minecraft.world.entity.item.ItemEntity lambda$popResourceFromFace$0(net.minecraft.world.level.Level, double, double, double, net.minecraft.world.item.ItemStack, double, double, double);
    private static net.minecraft.world.entity.item.ItemEntity lambda$popResource$0(net.minecraft.world.level.Level, double, double, double, net.minecraft.world.item.ItemStack);
    private static void lambda$dropResources$2(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.item.ItemStack);
    private static void lambda$dropResources$1(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.item.ItemStack);
    private static void lambda$dropResources$0(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.item.ItemStack);
    private static it.unimi.dsi.fastutil.objects.Object2ByteLinkedOpenHashMap lambda$static$0();
    private static void lambda$dropFromLootTable$0(java.util.function.BiConsumer, net.minecraft.server.level.ServerLevel, net.minecraft.world.item.ItemStack);
    private static net.minecraft.world.level.storage.loot.LootParams lambda$dropFromBlockInteractLootTable$0(net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.BlockEntity, net.minecraft.world.entity.Entity, net.minecraft.world.item.ItemInstance, net.minecraft.world.level.storage.loot.LootParams$Builder);
    private static net.minecraft.world.phys.shapes.VoxelShape[] lambda$boxes$0(int);
    static {};
}
```
