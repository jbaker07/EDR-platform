---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.MenuScreens"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.MenuScreens

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getConstructor(Lnet/minecraft/world/inventory/MenuType;)Lnet/minecraft/cli` | `` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.MenuScreens {
    private static final org.slf4j.Logger LOGGER;
    private static final java.util.Map<net.minecraft.world.inventory.MenuType<?>, net.minecraft.client.gui.screens.MenuScreens$ScreenConstructor<?, ?>> SCREENS;
    public net.minecraft.client.gui.screens.MenuScreens();
    public static <T extends net.minecraft.world.inventory.AbstractContainerMenu> void create(net.minecraft.world.inventory.MenuType<T>, net.minecraft.client.Minecraft, int, net.minecraft.network.chat.Component);
    private static <T extends net.minecraft.world.inventory.AbstractContainerMenu> net.minecraft.client.gui.screens.MenuScreens$ScreenConstructor<T, ?> getConstructor(net.minecraft.world.inventory.MenuType<T>);
    private static <M extends net.minecraft.world.inventory.AbstractContainerMenu, U extends net.minecraft.client.gui.screens.Screen & net.minecraft.client.gui.screens.inventory.MenuAccess<M>> void register(net.minecraft.world.inventory.MenuType<? extends M>, net.minecraft.client.gui.screens.MenuScreens$ScreenConstructor<M, U>);
    public static boolean selfTest();
    static {};
}
```
