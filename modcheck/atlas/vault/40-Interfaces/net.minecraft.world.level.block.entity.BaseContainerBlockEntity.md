---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.BaseContainerBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.BaseContainerBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lne` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.level.block.entity.BaseContainerBlockEntity extends net.minecraft.world.level.block.entity.BlockEntity implements net.minecraft.world.Container,net.minecraft.world.MenuProvider,net.minecraft.world.Nameable {
    private net.minecraft.world.LockCode lockKey;
    private net.minecraft.network.chat.Component name;
    protected net.minecraft.world.level.block.entity.BaseContainerBlockEntity(net.minecraft.world.level.block.entity.BlockEntityType<?>, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    protected void loadAdditional(net.minecraft.world.level.storage.ValueInput);
    protected void saveAdditional(net.minecraft.world.level.storage.ValueOutput);
    public net.minecraft.network.chat.Component getName();
    public net.minecraft.network.chat.Component getDisplayName();
    public net.minecraft.network.chat.Component getCustomName();
    protected abstract net.minecraft.network.chat.Component getDefaultName();
    public boolean canOpen(net.minecraft.world.entity.player.Player);
    public static void sendChestLockedNotifications(net.minecraft.world.phys.Vec3, net.minecraft.world.entity.player.Player, net.minecraft.network.chat.Component);
    public boolean isLocked();
    protected abstract net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getItems();
    protected abstract void setItems(net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>);
    public boolean isEmpty();
    public net.minecraft.world.item.ItemStack getItem(int);
    public net.minecraft.world.item.ItemStack removeItem(int, int);
    public net.minecraft.world.item.ItemStack removeItemNoUpdate(int);
    public void setItem(int, net.minecraft.world.item.ItemStack);
    public boolean stillValid(net.minecraft.world.entity.player.Player);
    public void clearContent();
    public net.minecraft.world.inventory.AbstractContainerMenu createMenu(int, net.minecraft.world.entity.player.Inventory, net.minecraft.world.entity.player.Player);
    protected abstract net.minecraft.world.inventory.AbstractContainerMenu createMenu(int, net.minecraft.world.entity.player.Inventory);
    protected void applyImplicitComponents(net.minecraft.core.component.DataComponentGetter);
    protected void collectImplicitComponents(net.minecraft.core.component.DataComponentMap$Builder);
    public void removeComponentsFromTag(net.minecraft.world.level.storage.ValueOutput);
    protected net.minecraft.world.level.storage.loot.LootContext getLootContext(net.minecraft.server.level.ServerLevel);
}
```
