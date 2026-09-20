---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.MenuScreens"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.MenuScreens

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getConstructor` | `(Lnet/minecraft/world/inventory/MenuType;)Lnet/minecraft/client/gui/sc` | exact | invokestatic@76 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final SCREENS : Ljava/util/Map;
public <init>()V
public static create(Lnet/minecraft/world/inventory/MenuType;Lnet/minecraft/client/Minecraft;ILnet/minecraft/network/chat/Component;)V
private static getConstructor(Lnet/minecraft/world/inventory/MenuType;)Lnet/minecraft/client/gui/screens/MenuScreens$ScreenConstructor;
public static register(Lnet/minecraft/world/inventory/MenuType;Lnet/minecraft/client/gui/screens/MenuScreens$ScreenConstructor;)V
public static selfTest()Z
static <clinit>()V
```
