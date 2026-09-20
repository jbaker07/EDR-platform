---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.MenuScreens$ScreenConstructor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.MenuScreens$ScreenConstructor

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `create` | `(Lnet/minecraft/world/inventory/AbstractContainerMenu;Lnet/minecraft/w` | exact | invokeinterface@125 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (0 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public fromPacket(Lnet/minecraft/network/chat/Component;Lnet/minecraft/world/inventory/MenuType;Lnet/minecraft/client/Minecraft;I)V
public abstract create(Lnet/minecraft/world/inventory/AbstractContainerMenu;Lnet/minecraft/world/entity/player/Inventory;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/client/gui/screens/Screen;
```
