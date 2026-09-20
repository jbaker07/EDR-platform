---
type: "interface"
fqcn: "net.minecraft.world.item.component.BundleContents$Mutable"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.component.BundleContents$Mutable

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `toImmutable()Lnet/minecraft/world/item/component/BundleContents;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `tryInsert(Lnet/minecraft/world/item/ItemStack;)I` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.item.component.BundleContents$Mutable extends net.minecraft.world.item.component.GrowableMutableContainer<net.minecraft.world.item.component.BundleContents> {
    private org.apache.commons.lang3.math.Fraction weight;
    private int selectedItem;
    private boolean needsFlattening;
    private net.minecraft.world.item.component.BundleContents$Mutable(java.util.List<net.minecraft.world.item.ItemStack>, org.apache.commons.lang3.math.Fraction, int);
    public net.minecraft.world.item.component.BundleContents$Mutable();
    public net.minecraft.world.item.component.BundleContents$Mutable clearItems();
    private int findStackIndexWithinRange(net.minecraft.world.item.ItemStack, int, int);
    private int findStackIndex(net.minecraft.world.item.ItemStack);
    private int getMaxAmountToAdd(org.apache.commons.lang3.math.Fraction);
    public int tryInsert(net.minecraft.world.item.ItemStack);
    public int tryTransfer(net.minecraft.world.inventory.Slot, net.minecraft.world.entity.player.Player);
    public void toggleSelectedItem(int);
    private boolean indexIsOutsideAllowedBounds(int);
    public net.minecraft.world.item.ItemStack removeOne();
    private static org.apache.commons.lang3.math.Fraction getStackedWeight(org.apache.commons.lang3.math.Fraction, int);
    private static org.apache.commons.lang3.math.Fraction getStackedWeight(net.minecraft.world.item.ItemStack);
    public org.apache.commons.lang3.math.Fraction weight();
    public int replaceSlotItems(net.minecraft.world.item.ItemProvider, net.minecraft.world.item.slot.SlotSelector);
    public void modifySlots(java.util.function.Consumer<? super net.minecraft.world.entity.SlotAccess>, net.minecraft.world.item.slot.SlotSelector);
    protected boolean setItem(int, net.minecraft.world.item.ItemStack);
    protected boolean addSlotWithItem(net.minecraft.world.item.ItemProvider);
    private static org.apache.commons.lang3.math.Fraction getWeightWithAddedItems(org.apache.commons.lang3.math.Fraction, net.minecraft.world.item.ItemStack);
    public boolean canInsertNewSlots();
    private void mergeIdenticalStacks();
    public net.minecraft.world.item.component.BundleContents toImmutable();
    public java.lang.Object toImmutable();
}
```
