---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.ChiseledBookShelfBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/entity/BlockEntity`; implements `net/minecraft/world/level/block/entity/ListBackedContainer`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `updateState` | `(I)V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |
| injects_into | `setItem` | `(ILnet/minecraft/world/item/ItemStack;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `items` | `Lnet/minecraft/core/NonNullList;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |
| reads | `lastInteractedSlot` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |

## Declared members (5 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAX_BOOKS_IN_STORAGE : I
private static final LOGGER : Lorg/slf4j/Logger;
private static final DEFAULT_LAST_INTERACTED_SLOT : I
private final items : Lnet/minecraft/core/NonNullList;
private lastInteractedSlot : I
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
private updateState(I)V
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
public getMaxStackSize()I
public acceptsItemType(Lnet/minecraft/world/item/ItemStack;)Z
public removeItem(II)Lnet/minecraft/world/item/ItemStack;
public setItem(ILnet/minecraft/world/item/ItemStack;)V
public canTakeItem(Lnet/minecraft/world/Container;ILnet/minecraft/world/item/ItemStack;)Z
public getItems()Lnet/minecraft/core/NonNullList;
public stillValid(Lnet/minecraft/world/entity/player/Player;)Z
public getLastInteractedSlot()I
protected applyImplicitComponents(Lnet/minecraft/core/component/DataComponentGetter;)V
protected collectImplicitComponents(Lnet/minecraft/core/component/DataComponentMap$Builder;)V
public removeComponentsFromTag(Lnet/minecraft/world/level/storage/ValueOutput;)V
private static synthetic lambda$canTakeItem$0(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/Container;Lnet/minecraft/world/item/ItemStack;)Z
static <clinit>()V
```
