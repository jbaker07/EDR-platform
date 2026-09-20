---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `canPlaceItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Di` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (42, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity extends net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity implements net.minecraft.world.WorldlyContainer {
    public static final int COLUMNS;
    public static final int ROWS;
    public static final int CONTAINER_SIZE;
    public static final int EVENT_SET_OPEN_COUNT;
    public static final int OPENING_TICK_LENGTH;
    public static final float MAX_LID_HEIGHT;
    public static final float MAX_LID_ROTATION;
    private static final int[] SLOTS;
    private static final net.minecraft.network.chat.Component DEFAULT_NAME;
    private net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> itemStacks;
    private int openCount;
    private net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity$AnimationStatus animationStatus;
    private float progress;
    private float progressOld;
    private final net.minecraft.world.item.DyeColor color;
    public net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity(net.minecraft.world.item.DyeColor, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public static void tick(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity);
    private void updateAnimation(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity$AnimationStatus getAnimationStatus();
    public net.minecraft.world.phys.AABB getBoundingBox(net.minecraft.world.level.block.state.BlockState);
    private void moveCollidedEntities(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public int getContainerSize();
    public boolean triggerEvent(int, int);
    private static void doNeighborUpdates(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public void preRemoveSideEffects(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public void startOpen(net.minecraft.world.entity.ContainerUser);
    public void stopOpen(net.minecraft.world.entity.ContainerUser);
    protected net.minecraft.network.chat.Component getDefaultName();
    protected void loadAdditional(net.minecraft.world.level.storage.ValueInput);
    protected void saveAdditional(net.minecraft.world.level.storage.ValueOutput);
    public void loadFromTag(net.minecraft.world.level.storage.ValueInput);
    protected net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getItems();
    protected void setItems(net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>);
    public int[] getSlotsForFace(net.minecraft.core.Direction);
    public boolean canPlaceItemThroughFace(int, net.minecraft.world.item.ItemStack, net.minecraft.core.Direction);
    public boolean canTakeItemThroughFace(int, net.minecraft.world.item.ItemStack, net.minecraft.core.Direction);
    public float getProgress(float);
    public net.minecraft.world.item.DyeColor getColor();
    protected net.minecraft.world.inventory.AbstractContainerMenu createMenu(int, net.minecraft.world.entity.player.Inventory);
    public boolean isClosed();
    static {};
}
```
