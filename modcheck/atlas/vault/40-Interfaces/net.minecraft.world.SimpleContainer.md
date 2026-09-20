---
type: "interface"
fqcn: "net.minecraft.world.SimpleContainer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.SimpleContainer

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `setChanged()V` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| wraps | `setItem(ILnet/minecraft/world/item/ItemStack;)V` | `@Redirect at INVOKE Lnet/minecraft/world/SimpleContainer;setChanged()V` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.SimpleContainer implements net.minecraft.world.Container,net.minecraft.world.inventory.StackedContentsCompatible {
    private final int size;
    private final net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> items;
    public net.minecraft.world.SimpleContainer(int);
    public net.minecraft.world.SimpleContainer(net.minecraft.world.item.ItemStack...);
    public net.minecraft.world.item.ItemStack getItem(int);
    public java.util.List<net.minecraft.world.item.ItemStack> removeAllItems();
    public net.minecraft.world.item.ItemStack removeItem(int, int);
    public net.minecraft.world.item.ItemStack removeItemType(net.minecraft.world.item.Item, int);
    public net.minecraft.world.item.ItemStack addItem(net.minecraft.world.item.ItemStack);
    public boolean canAddItem(net.minecraft.world.item.ItemStack);
    public net.minecraft.world.item.ItemStack removeItemNoUpdate(int);
    public void setItem(int, net.minecraft.world.item.ItemStack);
    public void setChanged();
    public int getContainerSize();
    public boolean isEmpty();
    public boolean stillValid(net.minecraft.world.entity.player.Player);
    public void clearContent();
    public void fillStackedContents(net.minecraft.world.entity.player.StackedItemContents);
    public java.lang.String toString();
    private void moveItemToEmptySlots(net.minecraft.world.item.ItemStack);
    private void moveItemToOccupiedSlotsWithSameType(net.minecraft.world.item.ItemStack);
    private void moveItemsBetweenStacks(net.minecraft.world.item.ItemStack, net.minecraft.world.item.ItemStack);
    public void fromItemList(net.minecraft.world.level.storage.ValueInput$TypedInputList<net.minecraft.world.item.ItemStack>);
    public void storeAsItemList(net.minecraft.world.level.storage.ValueOutput$TypedOutputList<net.minecraft.world.item.ItemStack>);
    public net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getItems();
    private static boolean lambda$toString$0(net.minecraft.world.item.ItemStack);
    private static boolean lambda$removeAllItems$0(net.minecraft.world.item.ItemStack);
}
```
