---
type: "interface"
fqcn: "net.minecraft.world.inventory.ItemCombinerMenu"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.inventory.ItemCombinerMenu

System: [[20-Systems/net.minecraft.world.inventory|net.minecraft.world.inventory]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/inventory/MenuType;ILnet/minecraft/wor` | `` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (28, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.inventory.ItemCombinerMenu extends net.minecraft.world.inventory.AbstractContainerMenu {
    private static final int INVENTORY_SLOTS_PER_ROW;
    private static final int INVENTORY_ROWS;
    private static final int INPUT_SLOT_START;
    protected final net.minecraft.world.inventory.ContainerLevelAccess access;
    protected final net.minecraft.world.entity.player.Player player;
    protected final net.minecraft.world.Container inputSlots;
    protected final net.minecraft.world.inventory.ResultContainer resultSlots;
    private final int resultSlotIndex;
    protected boolean mayPickup(net.minecraft.world.entity.player.Player, boolean);
    protected abstract void onTake(net.minecraft.world.entity.player.Player, net.minecraft.world.item.ItemStack);
    protected abstract boolean isValidBlock(net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.inventory.ItemCombinerMenu(net.minecraft.world.inventory.MenuType<?>, int, net.minecraft.world.entity.player.Inventory, net.minecraft.world.inventory.ContainerLevelAccess, net.minecraft.world.inventory.ItemCombinerMenuSlotDefinition);
    private void createInputSlots(net.minecraft.world.inventory.ItemCombinerMenuSlotDefinition);
    private void createResultSlot(net.minecraft.world.inventory.ItemCombinerMenuSlotDefinition);
    public abstract void createResult();
    private net.minecraft.world.SimpleContainer createContainer(int);
    public void slotsChanged(net.minecraft.world.Container);
    public void removed(net.minecraft.world.entity.player.Player);
    public boolean stillValid(net.minecraft.world.entity.player.Player);
    public net.minecraft.world.item.ItemStack quickMoveStack(net.minecraft.world.entity.player.Player, int);
    protected boolean canMoveIntoInputSlots(net.minecraft.world.item.ItemStack);
    public int getResultSlot();
    private int getInventorySlotStart();
    private int getInventorySlotEnd();
    private int getUseRowStart();
    private int getUseRowEnd();
    private java.lang.Boolean lambda$stillValid$0(net.minecraft.world.entity.player.Player, net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    private void lambda$removed$0(net.minecraft.world.entity.player.Player, net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
}
```
