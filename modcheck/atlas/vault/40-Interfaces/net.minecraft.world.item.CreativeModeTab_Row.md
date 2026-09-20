---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTab$Row"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTab$Row

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `BOTTOM` | `Lnet/minecraft/world/item/CreativeModeTab$Row;` | exact | getstatic@241 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `TOP` | `Lnet/minecraft/world/item/CreativeModeTab$Row;` | exact | getstatic@235 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `TOP` | `Lnet/minecraft/world/item/CreativeModeTab$Row;` | exact | getstatic@259 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (3 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final TOP : Lnet/minecraft/world/item/CreativeModeTab$Row;
public static final BOTTOM : Lnet/minecraft/world/item/CreativeModeTab$Row;
private static final synthetic $VALUES : [Lnet/minecraft/world/item/CreativeModeTab$Row;
public static values()[Lnet/minecraft/world/item/CreativeModeTab$Row;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/item/CreativeModeTab$Row;
private <init>(Ljava/lang/String;I)V
private static synthetic $values()[Lnet/minecraft/world/item/CreativeModeTab$Row;
static <clinit>()V
```
