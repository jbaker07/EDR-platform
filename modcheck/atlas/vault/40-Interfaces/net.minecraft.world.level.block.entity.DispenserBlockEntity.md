---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.DispenserBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.DispenserBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getBlockState()Lnet/minecraft/world/level/block/state/BlockState;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getRandomSlot(Lnet/minecraft/util/RandomSource;)I` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (15, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.entity.DispenserBlockEntity extends net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity {
    public static final int CONTAINER_SIZE;
    private static final net.minecraft.network.chat.Component DEFAULT_NAME;
    private net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> items;
    protected net.minecraft.world.level.block.entity.DispenserBlockEntity(net.minecraft.world.level.block.entity.BlockEntityType<?>, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.level.block.entity.DispenserBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public int getContainerSize();
    public int getRandomSlot(net.minecraft.util.RandomSource);
    public net.minecraft.world.item.ItemStack insertItem(net.minecraft.world.item.ItemStack);
    protected net.minecraft.network.chat.Component getDefaultName();
    protected void loadAdditional(net.minecraft.world.level.storage.ValueInput);
    protected void saveAdditional(net.minecraft.world.level.storage.ValueOutput);
    protected net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getItems();
    protected void setItems(net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>);
    protected net.minecraft.world.inventory.AbstractContainerMenu createMenu(int, net.minecraft.world.entity.player.Inventory);
    static {};
}
```
