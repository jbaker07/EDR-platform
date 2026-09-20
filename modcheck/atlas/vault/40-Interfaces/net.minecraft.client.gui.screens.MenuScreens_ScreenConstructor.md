---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.MenuScreens$ScreenConstructor"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.MenuScreens$ScreenConstructor

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `create(Lnet/minecraft/world/inventory/AbstractContainerMenu;Lnet/m` | `` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (2, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
interface net.minecraft.client.gui.screens.MenuScreens$ScreenConstructor<T extends net.minecraft.world.inventory.AbstractContainerMenu, U extends net.minecraft.client.gui.screens.Screen & net.minecraft.client.gui.screens.inventory.MenuAccess<T>> {
    public default void fromPacket(net.minecraft.network.chat.Component, net.minecraft.world.inventory.MenuType<T>, net.minecraft.client.Minecraft, int);
    public abstract U create(T, net.minecraft.world.entity.player.Inventory, net.minecraft.network.chat.Component);
}
```
