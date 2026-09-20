---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.MenuAccess"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.MenuAccess

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getMenu()Lnet/minecraft/world/inventory/AbstractContainerMenu;` | `` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (1, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.gui.screens.inventory.MenuAccess<T extends net.minecraft.world.inventory.AbstractContainerMenu> {
    public abstract T getMenu();
}
```
