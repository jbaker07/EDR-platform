---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: interface

```java
public static net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext forPlayerInteraction(net.minecraft.world.entity.player.Player, net.minecraft.world.InteractionHand)
public static net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext forCreativeInteraction(net.minecraft.world.entity.player.Player, net.minecraft.world.item.ItemStack)
public static net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext ofPlayerHand(net.minecraft.world.entity.player.Player, net.minecraft.world.InteractionHand)
public static net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext ofPlayerCursor(net.minecraft.world.entity.player.Player, net.minecraft.world.inventory.AbstractContainerMenu)
public static net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext ofPlayerSlot(net.minecraft.world.entity.player.Player, net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage<net.fabricmc.fabric.api.transfer.v1.item.ItemVariant>)
public static net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext ofSingleSlot(net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage<net.fabricmc.fabric.api.transfer.v1.item.ItemVariant>)
public static net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext withConstant(net.minecraft.world.item.ItemStack)
public static net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext withConstant(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long)
public default <A> A find(net.fabricmc.fabric.api.lookup.v1.item.ItemApiLookup<A, net.fabricmc.fabric.api.transfer.v1.context.ContainerItemContext>)
public default net.fabricmc.fabric.api.transfer.v1.item.ItemVariant getItemVariant()
public default long getAmount()
public default long insert(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public default long extract(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public default long exchange(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public abstract net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage<net.fabricmc.fabric.api.transfer.v1.item.ItemVariant> getMainSlot()
public abstract long insertOverflow(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public abstract java.util.List<net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage<net.fabricmc.fabric.api.transfer.v1.item.ItemVariant>> getAdditionalSlots()
```
