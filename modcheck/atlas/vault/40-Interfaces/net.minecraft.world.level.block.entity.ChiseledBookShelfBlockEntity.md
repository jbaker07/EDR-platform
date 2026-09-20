---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `setItem` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity extends net.minecraft.world.level.block.entity.BlockEntity implements net.minecraft.world.level.block.entity.ListBackedContainer {
    public static final int MAX_BOOKS_IN_STORAGE;
    private static final org.slf4j.Logger LOGGER;
    private static final int DEFAULT_LAST_INTERACTED_SLOT;
    private final net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> items;
    private int lastInteractedSlot;
    public net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    private void updateState(int);
    protected void loadAdditional(net.minecraft.world.level.storage.ValueInput);
    protected void saveAdditional(net.minecraft.world.level.storage.ValueOutput);
    public int getMaxStackSize();
    public boolean acceptsItemType(net.minecraft.world.item.ItemStack);
    public net.minecraft.world.item.ItemStack removeItem(int, int);
    public void setItem(int, net.minecraft.world.item.ItemStack);
    public boolean canTakeItem(net.minecraft.world.Container, int, net.minecraft.world.item.ItemStack);
    public net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getItems();
    public boolean stillValid(net.minecraft.world.entity.player.Player);
    public int getLastInteractedSlot();
    protected void applyImplicitComponents(net.minecraft.core.component.DataComponentGetter);
    protected void collectImplicitComponents(net.minecraft.core.component.DataComponentMap$Builder);
    public void removeComponentsFromTag(net.minecraft.world.level.storage.ValueOutput);
    private static boolean lambda$canTakeItem$0(net.minecraft.world.item.ItemStack, net.minecraft.world.Container, net.minecraft.world.item.ItemStack);
    static {};
}
```
