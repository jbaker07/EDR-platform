---
type: "interface"
fqcn: "net.minecraft.world.entity.vehicle.minecart.AbstractMinecart"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.vehicle.minecart.AbstractMinecart

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getType()Lnet/minecraft/world/entity/EntityType;` | `` | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (75, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.entity.vehicle.minecart.AbstractMinecart extends net.minecraft.world.entity.vehicle.VehicleEntity {
    private static final net.minecraft.world.phys.Vec3 LOWERED_PASSENGER_ATTACHMENT;
    private static final net.minecraft.network.syncher.EntityDataAccessor<java.util.Optional<net.minecraft.world.level.block.state.BlockState>> DATA_ID_CUSTOM_DISPLAY_BLOCK;
    private static final net.minecraft.network.syncher.EntityDataAccessor<java.lang.Integer> DATA_ID_DISPLAY_OFFSET;
    private static final com.google.common.collect.ImmutableMap<net.minecraft.world.entity.Pose, com.google.common.collect.ImmutableList<java.lang.Integer>> POSE_DISMOUNT_HEIGHTS;
    protected static final float WATER_SLOWDOWN_FACTOR;
    private static final boolean DEFAULT_FLIPPED_ROTATION;
    private boolean onRails;
    private boolean flipped;
    private final net.minecraft.world.entity.vehicle.minecart.MinecartBehavior behavior;
    private static final java.util.Map<net.minecraft.world.level.block.state.properties.RailShape, com.mojang.datafixers.util.Pair<net.minecraft.core.Vec3i, net.minecraft.core.Vec3i>> EXITS;
    protected net.minecraft.world.entity.vehicle.minecart.AbstractMinecart(net.minecraft.world.entity.EntityType<?>, net.minecraft.world.level.Level);
    protected net.minecraft.world.entity.vehicle.minecart.AbstractMinecart(net.minecraft.world.entity.EntityType<?>, net.minecraft.world.level.Level, double, double, double);
    public void setInitialPos(double, double, double);
    public static <T extends net.minecraft.world.entity.vehicle.minecart.AbstractMinecart> T createMinecart(net.minecraft.world.level.Level, double, double, double, net.minecraft.world.entity.EntityType<T>, net.minecraft.world.entity.EntitySpawnReason, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.player.Player);
    public net.minecraft.world.entity.vehicle.minecart.MinecartBehavior getBehavior();
    protected net.minecraft.world.entity.Entity$MovementEmission getMovementEmission();
    protected void defineSynchedData(net.minecraft.network.syncher.SynchedEntityData$Builder);
    public boolean canCollideWith(net.minecraft.world.entity.Entity);
    public boolean isPushable();
    public net.minecraft.world.phys.Vec3 getRelativePortalPosition(net.minecraft.core.Direction$Axis, net.minecraft.util.BlockUtil$FoundRectangle);
    protected net.minecraft.world.phys.Vec3 getPassengerAttachmentPoint(net.minecraft.world.entity.Entity, net.minecraft.world.entity.EntityDimensions, float);
    public net.minecraft.world.phys.Vec3 getDismountLocationForPassenger(net.minecraft.world.entity.LivingEntity);
    protected float getBlockSpeedFactor();
    public void animateHurt(float);
    public boolean isPickable();
    public static com.mojang.datafixers.util.Pair<net.minecraft.core.Vec3i, net.minecraft.core.Vec3i> exits(net.minecraft.world.level.block.state.properties.RailShape);
    public net.minecraft.core.Direction getMotionDirection();
    protected double getDefaultGravity();
    public void tick();
    public boolean isFirstTick();
    public net.minecraft.core.BlockPos getCurrentBlockPosOrRailBelow();
    protected double getMaxSpeed(net.minecraft.server.level.ServerLevel);
    public void activateMinecart(net.minecraft.server.level.ServerLevel, int, int, int, boolean);
    public void lerpPositionAndRotationStep(int, double, double, double, double, double);
    public void applyGravity();
    public void reapplyPosition();
    public boolean updateFluidInteraction();
    public net.minecraft.world.phys.Vec3 getKnownMovement();
    protected net.minecraft.world.entity.InterpolationHandler createInterpolationHandler();
    public void onInterpolationStart(net.minecraft.world.entity.InterpolationHandler);
    public void recreateFromPacket(net.minecraft.network.protocol.game.ClientboundAddEntityPacket);
    public void lerpMotion(net.minecraft.world.phys.Vec3);
    protected void moveAlongTrack(net.minecraft.server.level.ServerLevel);
    protected void comeOffTrack(net.minecraft.server.level.ServerLevel);
    protected float getAirDrag();
    protected double makeStepAlongTrack(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.properties.RailShape, double);
    public void move(net.minecraft.world.entity.MoverType, net.minecraft.world.phys.Vec3);
    public void applyEffectsFromBlocks();
    public boolean isOnRails();
    public void setOnRails(boolean);
    public boolean isFlipped();
    public void setFlipped(boolean);
    public net.minecraft.world.phys.Vec3 getRedstoneDirection(net.minecraft.core.BlockPos);
    public boolean isRedstoneConductor(net.minecraft.core.BlockPos);
    protected net.minecraft.world.phys.Vec3 applyNaturalSlowdown(net.minecraft.world.phys.Vec3);
    protected void readAdditionalSaveData(net.minecraft.world.level.storage.ValueInput);
    protected void addAdditionalSaveData(net.minecraft.world.level.storage.ValueOutput);
    public void push(net.minecraft.world.entity.Entity);
    private void pushOtherMinecart(net.minecraft.world.entity.vehicle.minecart.AbstractMinecart, double, double);
    public net.minecraft.world.level.block.state.BlockState getDisplayBlockState();
    private java.util.Optional<net.minecraft.world.level.block.state.BlockState> getCustomDisplayBlockState();
    public net.minecraft.world.level.block.state.BlockState getDefaultDisplayBlockState();
    public int getDisplayOffset();
    public int getDefaultDisplayOffset();
    public void setCustomDisplayBlockState(java.util.Optional<net.minecraft.world.level.block.state.BlockState>);
    public void setDisplayOffset(int);
    public static boolean useExperimentalMovement(net.minecraft.world.level.Level);
    public abstract net.minecraft.world.item.ItemStack getPickResult();
    public boolean isRideable();
    public boolean isFurnace();
    private static void lambda$addAdditionalSaveData$0(net.minecraft.world.level.storage.ValueOutput, net.minecraft.world.level.block.state.BlockState);
    private static com.google.common.collect.ImmutableMap lambda$static$0();
    private net.minecraft.world.phys.shapes.VoxelShape lambda$getDismountLocationForPassenger$1(net.minecraft.core.BlockPos);
    private net.minecraft.world.phys.shapes.VoxelShape lambda$getDismountLocationForPassenger$0(net.minecraft.core.BlockPos$MutableBlockPos);
    static {};
}
```
