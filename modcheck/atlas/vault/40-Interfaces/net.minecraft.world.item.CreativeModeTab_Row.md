---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTab$Row"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTab$Row

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `BOTTOMLnet/minecraft/world/item/CreativeModeTab$Row;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `TOPLnet/minecraft/world/item/CreativeModeTab$Row;` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.item.CreativeModeTab$Row extends java.lang.Enum<net.minecraft.world.item.CreativeModeTab$Row> {
    public static final net.minecraft.world.item.CreativeModeTab$Row TOP;
    public static final net.minecraft.world.item.CreativeModeTab$Row BOTTOM;
    private static final net.minecraft.world.item.CreativeModeTab$Row[] $VALUES;
    public static net.minecraft.world.item.CreativeModeTab$Row[] values();
    public static net.minecraft.world.item.CreativeModeTab$Row valueOf(java.lang.String);
    private net.minecraft.world.item.CreativeModeTab$Row();
    private static net.minecraft.world.item.CreativeModeTab$Row[] $values();
    static {};
}
```
