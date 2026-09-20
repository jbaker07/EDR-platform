---
type: "interface"
fqcn: "net.minecraft.world.inventory.EnchantmentMenu"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.inventory.EnchantmentMenu

System: [[20-Systems/net.minecraft.world.inventory|net.minecraft.world.inventory]]

`class` public; extends `net/minecraft/world/inventory/AbstractContainerMenu`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `lambda$slotsChanged$0` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| wraps | `lambda$slotsChanged$0` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (8 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final EMPTY_SLOT_LAPIS_LAZULI : Lnet/minecraft/resources/Identifier;
private final enchantSlots : Lnet/minecraft/world/Container;
private final access : Lnet/minecraft/world/inventory/ContainerLevelAccess;
private final random : Lnet/minecraft/util/RandomSource;
private final enchantmentSeed : Lnet/minecraft/world/inventory/DataSlot;
public final costs : [I
public final enchantClue : [I
public final levelClue : [I
public <init>(ILnet/minecraft/world/entity/player/Inventory;)V
public <init>(ILnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/world/inventory/ContainerLevelAccess;)V
public slotsChanged(Lnet/minecraft/world/Container;)V
public clickMenuButton(Lnet/minecraft/world/entity/player/Player;I)Z
private getEnchantmentList(Lnet/minecraft/core/RegistryAccess;Lnet/minecraft/world/item/ItemStack;II)Ljava/util/List;
public getGoldCount()I
public getEnchantmentSeed()I
public removed(Lnet/minecraft/world/entity/player/Player;)V
public stillValid(Lnet/minecraft/world/entity/player/Player;)Z
public quickMoveStack(Lnet/minecraft/world/entity/player/Player;I)Lnet/minecraft/world/item/ItemStack;
private synthetic lambda$removed$0(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
private synthetic lambda$clickMenuButton$0(Lnet/minecraft/world/item/ItemStack;ILnet/minecraft/world/entity/player/Player;ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
private synthetic lambda$slotsChanged$0(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
static <clinit>()V
```
