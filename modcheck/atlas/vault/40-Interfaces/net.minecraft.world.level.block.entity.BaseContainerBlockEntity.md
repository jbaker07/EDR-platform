---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.BaseContainerBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.BaseContainerBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `net/minecraft/world/level/block/entity/BlockEntity`; implements `net/minecraft/world/Container`, `net/minecraft/world/MenuProvider`, `net/minecraft/world/Nameable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraf` | exact | invokespecial@4 in `AbstractFurnaceBlockEntityMixin.<init>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| wraps | `setItem` | `(ILnet/minecraft/world/item/ItemStack;)V` | exact | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (2 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private lockKey : Lnet/minecraft/world/LockCode;
private name : Lnet/minecraft/network/chat/Component;
protected <init>(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
public getName()Lnet/minecraft/network/chat/Component;
public getDisplayName()Lnet/minecraft/network/chat/Component;
public getCustomName()Lnet/minecraft/network/chat/Component;
protected abstract getDefaultName()Lnet/minecraft/network/chat/Component;
public canOpen(Lnet/minecraft/world/entity/player/Player;)Z
public static sendChestLockedNotifications(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/network/chat/Component;)V
public isLocked()Z
protected abstract getItems()Lnet/minecraft/core/NonNullList;
protected abstract setItems(Lnet/minecraft/core/NonNullList;)V
public isEmpty()Z
public getItem(I)Lnet/minecraft/world/item/ItemStack;
public removeItem(II)Lnet/minecraft/world/item/ItemStack;
public removeItemNoUpdate(I)Lnet/minecraft/world/item/ItemStack;
public setItem(ILnet/minecraft/world/item/ItemStack;)V
public stillValid(Lnet/minecraft/world/entity/player/Player;)Z
public clearContent()V
public createMenu(ILnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/inventory/AbstractContainerMenu;
protected abstract createMenu(ILnet/minecraft/world/entity/player/Inventory;)Lnet/minecraft/world/inventory/AbstractContainerMenu;
protected applyImplicitComponents(Lnet/minecraft/core/component/DataComponentGetter;)V
protected collectImplicitComponents(Lnet/minecraft/core/component/DataComponentMap$Builder;)V
public removeComponentsFromTag(Lnet/minecraft/world/level/storage/ValueOutput;)V
protected getLootContext(Lnet/minecraft/server/level/ServerLevel;)Lnet/minecraft/world/level/storage/loot/LootContext;
```
