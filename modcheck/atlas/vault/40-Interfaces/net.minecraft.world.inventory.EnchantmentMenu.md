---
type: "interface"
fqcn: "net.minecraft.world.inventory.EnchantmentMenu"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.inventory.EnchantmentMenu

System: [[20-Systems/net.minecraft.world.inventory|net.minecraft.world.inventory]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `lambda$slotsChanged$0` | `@ModifyArg at INVOKE Lnet/minecraft/world/item/enchantment/EnchantmentHelper;get` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.inventory.EnchantmentMenu extends net.minecraft.world.inventory.AbstractContainerMenu {
    private static final net.minecraft.resources.Identifier EMPTY_SLOT_LAPIS_LAZULI;
    private final net.minecraft.world.Container enchantSlots;
    private final net.minecraft.world.inventory.ContainerLevelAccess access;
    private final net.minecraft.util.RandomSource random;
    private final net.minecraft.world.inventory.DataSlot enchantmentSeed;
    public final int[] costs;
    public final int[] enchantClue;
    public final int[] levelClue;
    public net.minecraft.world.inventory.EnchantmentMenu(int, net.minecraft.world.entity.player.Inventory);
    public net.minecraft.world.inventory.EnchantmentMenu(int, net.minecraft.world.entity.player.Inventory, net.minecraft.world.inventory.ContainerLevelAccess);
    public void slotsChanged(net.minecraft.world.Container);
    public boolean clickMenuButton(net.minecraft.world.entity.player.Player, int);
    private java.util.List<net.minecraft.world.item.enchantment.EnchantmentInstance> getEnchantmentList(net.minecraft.core.RegistryAccess, net.minecraft.world.item.ItemStack, int, int);
    public int getGoldCount();
    public int getEnchantmentSeed();
    public void removed(net.minecraft.world.entity.player.Player);
    public boolean stillValid(net.minecraft.world.entity.player.Player);
    public net.minecraft.world.item.ItemStack quickMoveStack(net.minecraft.world.entity.player.Player, int);
    private void lambda$removed$0(net.minecraft.world.entity.player.Player, net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    private void lambda$clickMenuButton$0(net.minecraft.world.item.ItemStack, int, net.minecraft.world.entity.player.Player, int, net.minecraft.world.item.ItemStack, net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    private void lambda$slotsChanged$0(net.minecraft.world.item.ItemStack, net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    static {};
}
```
