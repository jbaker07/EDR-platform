---
type: "interface"
fqcn: "net.minecraft.world.SimpleMenuProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.SimpleMenuProvider

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`class` public final; extends `java/lang/Object`; implements `net/minecraft/world/MenuProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `menuConstructor` | `Lnet/minecraft/world/inventory/MenuConstructor;` | exact | getfield@22 in `ServerPlayerMixin.fabric_storeOpenedMenu` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| reads | `menuConstructor` | `Lnet/minecraft/world/inventory/MenuConstructor;` | exact | getfield@15 in `ServerPlayerMixin.fabric_replaceVanillaScreenPacket` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (2 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final title : Lnet/minecraft/network/chat/Component;
private final menuConstructor : Lnet/minecraft/world/inventory/MenuConstructor;
public <init>(Lnet/minecraft/world/inventory/MenuConstructor;Lnet/minecraft/network/chat/Component;)V
public getDisplayName()Lnet/minecraft/network/chat/Component;
public createMenu(ILnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/inventory/AbstractContainerMenu;
```
