---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.DispenserBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.DispenserBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | inherited_exact | invokevirtual@12 in `DropperBlockMixin.hookDispense` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getRandomSlot` | `(Lnet/minecraft/util/RandomSource;)I` | exact | invokevirtual@67 in `DropperBlockMixin.hookDispense` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (3 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CONTAINER_SIZE : I
private static final DEFAULT_NAME : Lnet/minecraft/network/chat/Component;
private items : Lnet/minecraft/core/NonNullList;
protected <init>(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public getContainerSize()I
public getRandomSlot(Lnet/minecraft/util/RandomSource;)I
public insertItem(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;
protected getDefaultName()Lnet/minecraft/network/chat/Component;
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
protected getItems()Lnet/minecraft/core/NonNullList;
protected setItems(Lnet/minecraft/core/NonNullList;)V
protected createMenu(ILnet/minecraft/world/entity/player/Inventory;)Lnet/minecraft/world/inventory/AbstractContainerMenu;
static <clinit>()V
```
