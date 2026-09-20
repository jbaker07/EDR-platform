---
type: "interface"
fqcn: "net.minecraft.world.item.TooltipFlag"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.TooltipFlag

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`interface` public abstract; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/item/v1/FabricTooltipFlag`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isAdvanced` | `()Z` | exact | invokeinterface@48 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (2 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NORMAL : Lnet/minecraft/world/item/TooltipFlag$Default;
public static final ADVANCED : Lnet/minecraft/world/item/TooltipFlag$Default;
public abstract isAdvanced()Z
public abstract isCreative()Z
static <clinit>()V
```
