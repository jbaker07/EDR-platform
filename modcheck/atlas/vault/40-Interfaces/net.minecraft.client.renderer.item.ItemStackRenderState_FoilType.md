---
type: "interface"
fqcn: "net.minecraft.client.renderer.item.ItemStackRenderState$FoilType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.item.ItemStackRenderState$FoilType

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `ordinal()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values()[Lnet/minecraft/client/renderer/item/ItemStackRenderState$` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `NONELnet/minecraft/client/renderer/item/ItemStackRenderState$Foi` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.item.ItemStackRenderState$FoilType extends java.lang.Enum<net.minecraft.client.renderer.item.ItemStackRenderState$FoilType> {
    public static final net.minecraft.client.renderer.item.ItemStackRenderState$FoilType NONE;
    public static final net.minecraft.client.renderer.item.ItemStackRenderState$FoilType STANDARD;
    public static final net.minecraft.client.renderer.item.ItemStackRenderState$FoilType SPECIAL;
    private static final net.minecraft.client.renderer.item.ItemStackRenderState$FoilType[] $VALUES;
    public static net.minecraft.client.renderer.item.ItemStackRenderState$FoilType[] values();
    public static net.minecraft.client.renderer.item.ItemStackRenderState$FoilType valueOf(java.lang.String);
    private net.minecraft.client.renderer.item.ItemStackRenderState$FoilType();
    private static net.minecraft.client.renderer.item.ItemStackRenderState$FoilType[] $values();
    static {};
}
```
