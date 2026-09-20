---
type: "interface"
fqcn: "net.minecraft.world.SimpleMenuProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.SimpleMenuProvider

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `menuConstructorLnet/minecraft/world/inventory/MenuConstructor;` | `` | both | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.SimpleMenuProvider implements net.minecraft.world.MenuProvider {
    private final net.minecraft.network.chat.Component title;
    private final net.minecraft.world.inventory.MenuConstructor menuConstructor;
    public net.minecraft.world.SimpleMenuProvider(net.minecraft.world.inventory.MenuConstructor, net.minecraft.network.chat.Component);
    public net.minecraft.network.chat.Component getDisplayName();
    public net.minecraft.world.inventory.AbstractContainerMenu createMenu(int, net.minecraft.world.entity.player.Inventory, net.minecraft.world.entity.player.Player);
}
```
