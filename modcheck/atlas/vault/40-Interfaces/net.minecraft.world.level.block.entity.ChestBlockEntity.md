---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.ChestBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.ChestBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getBlockPos()Lnet/minecraft/core/BlockPos;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlockState()Lnet/minecraft/world/level/block/state/BlockState;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLevel()Lnet/minecraft/world/level/Level;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.entity.ChestBlockEntity extends net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity implements net.minecraft.world.level.block.entity.LidBlockEntity {
    private static final int EVENT_SET_OPEN_COUNT;
    private static final net.minecraft.network.chat.Component DEFAULT_NAME;
    private net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> items;
    private final net.minecraft.world.level.block.entity.ContainerOpenersCounter openersCounter;
    private final net.minecraft.world.level.block.entity.ChestLidController chestLidController;
    protected net.minecraft.world.level.block.entity.ChestBlockEntity(net.minecraft.world.level.block.entity.BlockEntityType<?>, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.level.block.entity.ChestBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public int getContainerSize();
    protected net.minecraft.network.chat.Component getDefaultName();
    protected void loadAdditional(net.minecraft.world.level.storage.ValueInput);
    protected void saveAdditional(net.minecraft.world.level.storage.ValueOutput);
    public static void lidAnimateTick(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.ChestBlockEntity);
    private static void playSound(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.sounds.SoundEvent);
    public boolean triggerEvent(int, int);
    public void startOpen(net.minecraft.world.entity.ContainerUser);
    public void stopOpen(net.minecraft.world.entity.ContainerUser);
    public java.util.List<net.minecraft.world.entity.ContainerUser> getEntitiesWithContainerOpen();
    protected net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getItems();
    protected void setItems(net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>);
    public float getOpenNess(float);
    public static int getOpenCount(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public static void swapContents(net.minecraft.world.level.block.entity.ChestBlockEntity, net.minecraft.world.level.block.entity.ChestBlockEntity);
    protected net.minecraft.world.inventory.AbstractContainerMenu createMenu(int, net.minecraft.world.entity.player.Inventory);
    public void recheckOpen();
    protected void signalOpenCount(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, int, int);
    static {};
}
```
