---
type: "interface"
fqcn: "net.fabricmc.fabric.api.transfer.v1.item.PlayerInventoryStorage"
module: "fabric-transfer-api-v1"
sha256: "599f69de9e7e693b4b8ca2f2792f129d8bd2e17fced9ae7b66f7e20b5a674db6"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.transfer.v1.item.PlayerInventoryStorage

Module: [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] -- kind: interface

```java
public static net.fabricmc.fabric.api.transfer.v1.item.PlayerInventoryStorage of(net.minecraft.world.entity.player.Player)
public static net.fabricmc.fabric.api.transfer.v1.item.PlayerInventoryStorage of(net.minecraft.world.entity.player.Inventory)
public static net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage getCursorStorage(net.minecraft.world.inventory.AbstractContainerMenu)
public abstract long insert(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public void offerOrDrop(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public abstract long offer(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public abstract void drop(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, boolean, boolean, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public void drop(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, boolean, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public void drop(net.fabricmc.fabric.api.transfer.v1.item.ItemVariant, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
public abstract net.fabricmc.fabric.api.transfer.v1.storage.base.SingleSlotStorage getHandSlot(net.minecraft.world.InteractionHand)
public long insert(java.lang.Object, long, net.fabricmc.fabric.api.transfer.v1.transaction.TransactionContext)
```
