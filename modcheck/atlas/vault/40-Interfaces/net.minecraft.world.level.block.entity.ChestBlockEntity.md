---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.ChestBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.ChestBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity`; implements `net/minecraft/world/level/block/entity/LidBlockEntity`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBlockPos` | `()Lnet/minecraft/core/BlockPos;` | inherited_exact | invokevirtual@53 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | inherited_exact | invokevirtual@37 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | inherited_exact | invokevirtual@57 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLevel` | `()Lnet/minecraft/world/level/Level;` | inherited_exact | invokevirtual@68 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (5 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final EVENT_SET_OPEN_COUNT : I
private static final DEFAULT_NAME : Lnet/minecraft/network/chat/Component;
private items : Lnet/minecraft/core/NonNullList;
private final openersCounter : Lnet/minecraft/world/level/block/entity/ContainerOpenersCounter;
private final chestLidController : Lnet/minecraft/world/level/block/entity/ChestLidController;
protected <init>(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public getContainerSize()I
protected getDefaultName()Lnet/minecraft/network/chat/Component;
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
public static lidAnimateTick(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/ChestBlockEntity;)V
private static playSound(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/sounds/SoundEvent;)V
public triggerEvent(II)Z
public startOpen(Lnet/minecraft/world/entity/ContainerUser;)V
public stopOpen(Lnet/minecraft/world/entity/ContainerUser;)V
public getEntitiesWithContainerOpen()Ljava/util/List;
protected getItems()Lnet/minecraft/core/NonNullList;
protected setItems(Lnet/minecraft/core/NonNullList;)V
public getOpenNess(F)F
public static getOpenCount(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)I
public static swapContents(Lnet/minecraft/world/level/block/entity/ChestBlockEntity;Lnet/minecraft/world/level/block/entity/ChestBlockEntity;)V
protected createMenu(ILnet/minecraft/world/entity/player/Inventory;)Lnet/minecraft/world/inventory/AbstractContainerMenu;
public recheckOpen()V
protected signalOpenCount(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;II)V
static <clinit>()V
```
