---
type: "interface"
fqcn: "net.minecraft.world.Container"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.Container

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `canPlaceItem(ILnet/minecraft/world/item/ItemStack;)Z` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getContainerSize()I` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getItem(I)Lnet/minecraft/world/item/ItemStack;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getMaxStackSize()I` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getMaxStackSize(Lnet/minecraft/world/item/ItemStack;)I` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setItem(ILnet/minecraft/world/item/ItemStack;)V` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `toString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (24, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.Container extends net.minecraft.world.Clearable, java.lang.Iterable<net.minecraft.world.item.ItemStack>, net.minecraft.world.entity.SlotProvider {
    public static final float DEFAULT_DISTANCE_BUFFER;
    public abstract int getContainerSize();
    public abstract boolean isEmpty();
    public abstract net.minecraft.world.item.ItemStack getItem(int);
    public abstract net.minecraft.world.item.ItemStack removeItem(int, int);
    public abstract net.minecraft.world.item.ItemStack removeItemNoUpdate(int);
    public abstract void setItem(int, net.minecraft.world.item.ItemStack);
    public default int getMaxStackSize();
    public default int getMaxStackSize(net.minecraft.world.item.ItemStack);
    public abstract void setChanged();
    public abstract boolean stillValid(net.minecraft.world.entity.player.Player);
    public default void startOpen(net.minecraft.world.entity.ContainerUser);
    public default void stopOpen(net.minecraft.world.entity.ContainerUser);
    public default java.util.List<net.minecraft.world.entity.ContainerUser> getEntitiesWithContainerOpen();
    public default boolean canPlaceItem(int, net.minecraft.world.item.ItemStack);
    public default boolean canTakeItem(net.minecraft.world.Container, int, net.minecraft.world.item.ItemStack);
    public default int countItem(net.minecraft.world.item.Item);
    public default boolean hasAnyOf(java.util.Set<net.minecraft.world.item.Item>);
    public default boolean hasAnyMatching(java.util.function.Predicate<net.minecraft.world.item.ItemStack>);
    public static boolean stillValidBlockEntity(net.minecraft.world.level.block.entity.BlockEntity, net.minecraft.world.entity.player.Player);
    public static boolean stillValidBlockEntity(net.minecraft.world.level.block.entity.BlockEntity, net.minecraft.world.entity.player.Player, float);
    public default net.minecraft.world.entity.SlotAccess getSlot(int);
    public default java.util.Iterator<net.minecraft.world.item.ItemStack> iterator();
    private static boolean lambda$hasAnyOf$0(java.util.Set, net.minecraft.world.item.ItemStack);
}
```
