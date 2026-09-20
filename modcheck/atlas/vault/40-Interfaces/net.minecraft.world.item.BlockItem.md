---
type: "interface"
fqcn: "net.minecraft.world.item.BlockItem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.BlockItem

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `net/minecraft/world/item/Item`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBlock` | `()Lnet/minecraft/world/level/block/Block;` | exact | invokevirtual@19 in `ModelProviderItemInfoCollectorMixin.filterItemsForProcessingMod` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `registerBlocks` | `(Ljava/util/Map;Lnet/minecraft/world/item/Item;)V` | exact | invokevirtual@15 in `BlockItemTracker.onEntryAdded` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (1 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final block : Lnet/minecraft/world/level/block/Block;
public <init>(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/item/Item$Properties;)V
public useOn(Lnet/minecraft/world/item/context/UseOnContext;)Lnet/minecraft/world/InteractionResult;
public place(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/InteractionResult;
protected getPlaceSound(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/sounds/SoundEvent;
public updatePlacementContext(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/item/context/BlockPlaceContext;
private static updateBlockEntityComponents(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/item/ItemStack;)V
protected getPlacementState(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/level/block/state/BlockState;
private static updateBlockStateFromTag(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;
protected canPlace(Lnet/minecraft/world/item/context/BlockPlaceContext;Lnet/minecraft/world/level/block/state/BlockState;)Z
protected mustSurvive()Z
protected placeBlock(Lnet/minecraft/world/item/context/BlockPlaceContext;Lnet/minecraft/world/level/block/state/BlockState;)Z
public static updateCustomBlockEntityTag(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/item/ItemStack;)Z
public shouldPrintOpWarning(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/player/Player;)Z
public getBlock()Lnet/minecraft/world/level/block/Block;
public registerBlocks(Ljava/util/Map;Lnet/minecraft/world/item/Item;)V
public canFitInsideContainerItems()Z
public onDestroyed(Lnet/minecraft/world/entity/item/ItemEntity;)V
public static setBlockEntityData(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/world/level/storage/TagValueOutput;)V
```
