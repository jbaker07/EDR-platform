---
type: "interface"
fqcn: "net.minecraft.world.inventory.ItemCombinerMenu"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.inventory.ItemCombinerMenu

System: [[20-Systems/net.minecraft.world.inventory|net.minecraft.world.inventory]]

`abstract_class` public abstract; extends `net/minecraft/world/inventory/AbstractContainerMenu`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/inventory/MenuType;ILnet/minecraft/world/entity/` | exact | invokespecial@8 in `AnvilMenuMixin.<init>` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (8 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final INVENTORY_SLOTS_PER_ROW : I
private static final INVENTORY_ROWS : I
private static final INPUT_SLOT_START : I
protected final access : Lnet/minecraft/world/inventory/ContainerLevelAccess;
protected final player : Lnet/minecraft/world/entity/player/Player;
protected final inputSlots : Lnet/minecraft/world/Container;
protected final resultSlots : Lnet/minecraft/world/inventory/ResultContainer;
private final resultSlotIndex : I
protected mayPickup(Lnet/minecraft/world/entity/player/Player;Z)Z
protected abstract onTake(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/ItemStack;)V
protected abstract isValidBlock(Lnet/minecraft/world/level/block/state/BlockState;)Z
public <init>(Lnet/minecraft/world/inventory/MenuType;ILnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/world/inventory/ContainerLevelAccess;Lnet/minecraft/world/inventory/ItemCombinerMenuSlotDefinition;)V
private createInputSlots(Lnet/minecraft/world/inventory/ItemCombinerMenuSlotDefinition;)V
private createResultSlot(Lnet/minecraft/world/inventory/ItemCombinerMenuSlotDefinition;)V
public abstract createResult()V
private createContainer(I)Lnet/minecraft/world/SimpleContainer;
public slotsChanged(Lnet/minecraft/world/Container;)V
public removed(Lnet/minecraft/world/entity/player/Player;)V
public stillValid(Lnet/minecraft/world/entity/player/Player;)Z
public quickMoveStack(Lnet/minecraft/world/entity/player/Player;I)Lnet/minecraft/world/item/ItemStack;
protected canMoveIntoInputSlots(Lnet/minecraft/world/item/ItemStack;)Z
public getResultSlot()I
private getInventorySlotStart()I
private getInventorySlotEnd()I
private getUseRowStart()I
private getUseRowEnd()I
private synthetic lambda$stillValid$0(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Ljava/lang/Boolean;
private synthetic lambda$removed$0(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
```
