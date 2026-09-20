---
type: "interface"
fqcn: "net.minecraft.world.inventory.AnvilMenu"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.inventory.AnvilMenu

System: [[20-Systems/net.minecraft.world.inventory|net.minecraft.world.inventory]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `createResult` | `@Redirect at INVOKE Lnet/minecraft/world/item/enchantment/Enchantment;canEnchant` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (36, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.inventory.AnvilMenu extends net.minecraft.world.inventory.ItemCombinerMenu {
    public static final int INPUT_SLOT;
    public static final int ADDITIONAL_SLOT;
    public static final int RESULT_SLOT;
    private static final org.slf4j.Logger LOGGER;
    private static final boolean DEBUG_COST;
    public static final int MAX_NAME_LENGTH;
    private int repairItemCountCost;
    private java.lang.String itemName;
    private final net.minecraft.world.inventory.DataSlot cost;
    private boolean onlyRenaming;
    private static final int COST_FAIL;
    private static final int COST_BASE;
    private static final int COST_ADDED_BASE;
    private static final int COST_REPAIR_MATERIAL;
    private static final int COST_REPAIR_SACRIFICE;
    private static final int COST_INCOMPATIBLE_PENALTY;
    private static final int COST_RENAME;
    private static final int INPUT_SLOT_X_PLACEMENT;
    private static final int ADDITIONAL_SLOT_X_PLACEMENT;
    private static final int RESULT_SLOT_X_PLACEMENT;
    private static final int SLOT_Y_PLACEMENT;
    public net.minecraft.world.inventory.AnvilMenu(int, net.minecraft.world.entity.player.Inventory);
    public net.minecraft.world.inventory.AnvilMenu(int, net.minecraft.world.entity.player.Inventory, net.minecraft.world.inventory.ContainerLevelAccess);
    private static net.minecraft.world.inventory.ItemCombinerMenuSlotDefinition createInputSlotDefinitions();
    protected boolean isValidBlock(net.minecraft.world.level.block.state.BlockState);
    protected boolean mayPickup(net.minecraft.world.entity.player.Player, boolean);
    protected void onTake(net.minecraft.world.entity.player.Player, net.minecraft.world.item.ItemStack);
    public void createResult();
    public static int calculateIncreasedRepairCost(int);
    public boolean setItemName(java.lang.String);
    private static java.lang.String validateName(java.lang.String);
    public int getCost();
    private static void lambda$onTake$0(net.minecraft.world.entity.player.Player, net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    private static boolean lambda$createInputSlotDefinitions$1(net.minecraft.world.item.ItemStack);
    private static boolean lambda$createInputSlotDefinitions$0(net.minecraft.world.item.ItemStack);
    static {};
}
```
