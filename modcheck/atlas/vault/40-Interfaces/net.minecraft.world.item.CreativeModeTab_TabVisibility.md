---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTab$TabVisibility"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTab$TabVisibility

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`enum` public final; extends `java/lang/Enum`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `FabricCreativeModeTabOutput$1.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `FabricCreativeModeTabOutput$1.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@45 in `FabricCreativeModeTabOutput$1.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@16 in `FabricCreativeModeTabOutput.accept` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@16 in `FabricCreativeModeTabOutput.prepend` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@20 in `FabricCreativeModeTabOutput.insertAfter` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@20 in `FabricCreativeModeTabOutput.insertAfter` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@20 in `FabricCreativeModeTabOutput.insertAfter` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@20 in `FabricCreativeModeTabOutput.insertBefore` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@20 in `FabricCreativeModeTabOutput.insertBefore` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@20 in `FabricCreativeModeTabOutput.insertBefore` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | invokestatic@0 in `FabricCreativeModeTabOutput$1.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `PARENT_AND_SEARCH_TABS` | `Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | getstatic@12 in `FabricCreativeModeTabOutput$1.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `PARENT_AND_SEARCH_TABS` | `Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | getstatic@2 in `FabricCreativeModeTabOutput.prepend` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `PARENT_AND_SEARCH_TABS` | `Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | getstatic@2 in `FabricCreativeModeTabOutput.prepend` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `PARENT_AND_SEARCH_TABS` | `Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | getstatic@3 in `FabricCreativeModeTabOutput.insertAfter` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `PARENT_AND_SEARCH_TABS` | `Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | getstatic@3 in `FabricCreativeModeTabOutput.insertAfter` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `PARENT_AND_SEARCH_TABS` | `Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | getstatic@3 in `FabricCreativeModeTabOutput.insertBefore` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `PARENT_AND_SEARCH_TABS` | `Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | getstatic@3 in `FabricCreativeModeTabOutput.insertBefore` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `PARENT_TAB_ONLY` | `Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | getstatic@27 in `FabricCreativeModeTabOutput$1.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| reads | `SEARCH_TAB_ONLY` | `Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;` | exact | getstatic@42 in `FabricCreativeModeTabOutput$1.<clinit>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (4 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final PARENT_AND_SEARCH_TABS : Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;
public static final PARENT_TAB_ONLY : Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;
public static final SEARCH_TAB_ONLY : Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;
private static final synthetic $VALUES : [Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;
public static values()[Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;
private <init>(Ljava/lang/String;I)V
private static synthetic $values()[Lnet/minecraft/world/item/CreativeModeTab$TabVisibility;
static <clinit>()V
```
