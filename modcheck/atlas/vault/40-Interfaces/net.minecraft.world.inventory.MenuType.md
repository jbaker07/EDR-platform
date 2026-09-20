---
type: "interface"
fqcn: "net.minecraft.world.inventory.MenuType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.inventory.MenuType

System: [[20-Systems/net.minecraft.world.inventory|net.minecraft.world.inventory]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/flag/FeatureElement`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/inventory/MenuType$MenuSupplier;Lnet/minecraft/w` | exact | invokespecial@5 in `ExtendedMenuType.<init>` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (27 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final GENERIC_9x1 : Lnet/minecraft/world/inventory/MenuType;
public static final GENERIC_9x2 : Lnet/minecraft/world/inventory/MenuType;
public static final GENERIC_9x3 : Lnet/minecraft/world/inventory/MenuType;
public static final GENERIC_9x4 : Lnet/minecraft/world/inventory/MenuType;
public static final GENERIC_9x5 : Lnet/minecraft/world/inventory/MenuType;
public static final GENERIC_9x6 : Lnet/minecraft/world/inventory/MenuType;
public static final GENERIC_3x3 : Lnet/minecraft/world/inventory/MenuType;
public static final CRAFTER_3x3 : Lnet/minecraft/world/inventory/MenuType;
public static final ANVIL : Lnet/minecraft/world/inventory/MenuType;
public static final BEACON : Lnet/minecraft/world/inventory/MenuType;
public static final BLAST_FURNACE : Lnet/minecraft/world/inventory/MenuType;
public static final BREWING_STAND : Lnet/minecraft/world/inventory/MenuType;
public static final CRAFTING : Lnet/minecraft/world/inventory/MenuType;
public static final ENCHANTMENT : Lnet/minecraft/world/inventory/MenuType;
public static final FURNACE : Lnet/minecraft/world/inventory/MenuType;
public static final GRINDSTONE : Lnet/minecraft/world/inventory/MenuType;
public static final HOPPER : Lnet/minecraft/world/inventory/MenuType;
public static final LECTERN : Lnet/minecraft/world/inventory/MenuType;
public static final LOOM : Lnet/minecraft/world/inventory/MenuType;
public static final MERCHANT : Lnet/minecraft/world/inventory/MenuType;
public static final SHULKER_BOX : Lnet/minecraft/world/inventory/MenuType;
public static final SMITHING : Lnet/minecraft/world/inventory/MenuType;
public static final SMOKER : Lnet/minecraft/world/inventory/MenuType;
public static final CARTOGRAPHY_TABLE : Lnet/minecraft/world/inventory/MenuType;
public static final STONECUTTER : Lnet/minecraft/world/inventory/MenuType;
private final requiredFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private final constructor : Lnet/minecraft/world/inventory/MenuType$MenuSupplier;
private static register(Ljava/lang/String;Lnet/minecraft/world/inventory/MenuType$MenuSupplier;)Lnet/minecraft/world/inventory/MenuType;
private static register(Ljava/lang/String;Lnet/minecraft/world/inventory/MenuType$MenuSupplier;[Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/world/inventory/MenuType;
public <init>(Lnet/minecraft/world/inventory/MenuType$MenuSupplier;Lnet/minecraft/world/flag/FeatureFlagSet;)V
public create(ILnet/minecraft/world/entity/player/Inventory;)Lnet/minecraft/world/inventory/AbstractContainerMenu;
public requiredFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
private static synthetic lambda$static$0(ILnet/minecraft/world/entity/player/Inventory;)Lnet/minecraft/world/inventory/LecternMenu;
static <clinit>()V
```
