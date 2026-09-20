---
type: "interface"
fqcn: "net.minecraft.world.inventory.AnvilMenu"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.inventory.AnvilMenu

System: [[20-Systems/net.minecraft.world.inventory|net.minecraft.world.inventory]]

`class` public; extends `net/minecraft/world/inventory/ItemCombinerMenu`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `createResult` | `()V` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (21 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final INPUT_SLOT : I
public static final ADDITIONAL_SLOT : I
public static final RESULT_SLOT : I
private static final LOGGER : Lorg/slf4j/Logger;
private static final DEBUG_COST : Z
public static final MAX_NAME_LENGTH : I
private repairItemCountCost : I
private itemName : Ljava/lang/String;
private final cost : Lnet/minecraft/world/inventory/DataSlot;
private onlyRenaming : Z
private static final COST_FAIL : I
private static final COST_BASE : I
private static final COST_ADDED_BASE : I
private static final COST_REPAIR_MATERIAL : I
private static final COST_REPAIR_SACRIFICE : I
private static final COST_INCOMPATIBLE_PENALTY : I
private static final COST_RENAME : I
private static final INPUT_SLOT_X_PLACEMENT : I
private static final ADDITIONAL_SLOT_X_PLACEMENT : I
private static final RESULT_SLOT_X_PLACEMENT : I
private static final SLOT_Y_PLACEMENT : I
public <init>(ILnet/minecraft/world/entity/player/Inventory;)V
public <init>(ILnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/world/inventory/ContainerLevelAccess;)V
private static createInputSlotDefinitions()Lnet/minecraft/world/inventory/ItemCombinerMenuSlotDefinition;
protected isValidBlock(Lnet/minecraft/world/level/block/state/BlockState;)Z
protected mayPickup(Lnet/minecraft/world/entity/player/Player;Z)Z
protected onTake(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/ItemStack;)V
public createResult()V
public static calculateIncreasedRepairCost(I)I
public setItemName(Ljava/lang/String;)Z
private static validateName(Ljava/lang/String;)Ljava/lang/String;
public getCost()I
private static synthetic lambda$onTake$0(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
private static synthetic lambda$createInputSlotDefinitions$1(Lnet/minecraft/world/item/ItemStack;)Z
private static synthetic lambda$createInputSlotDefinitions$0(Lnet/minecraft/world/item/ItemStack;)Z
static <clinit>()V
```
