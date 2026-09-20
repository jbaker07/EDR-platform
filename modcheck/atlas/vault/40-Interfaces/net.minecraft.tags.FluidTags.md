---
type: "interface"
fqcn: "net.minecraft.tags.FluidTags"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.tags.FluidTags

System: [[20-Systems/net.minecraft.tags|net.minecraft.tags]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `LAVA` | `Lnet/minecraft/tags/TagKey;` | exact | getstatic@13 in `EntityMixin.isInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/tags/TagKey;` | exact | getstatic@1 in `EntityMixin.isInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/tags/TagKey;` | exact | getstatic@17 in `HudStatusBarHeightRegistryImpl.lambda$static$5` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (10 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final WATER : Lnet/minecraft/tags/TagKey;
public static final LAVA : Lnet/minecraft/tags/TagKey;
public static final SUPPORTS_SUGAR_CANE_ADJACENTLY : Lnet/minecraft/tags/TagKey;
public static final SUPPORTS_LILY_PAD : Lnet/minecraft/tags/TagKey;
public static final SUPPORTS_FROGSPAWN : Lnet/minecraft/tags/TagKey;
public static final BUBBLE_COLUMN_CAN_OCCUPY : Lnet/minecraft/tags/TagKey;
public static final AXOLOTL_TRIES_TO_FIND : Lnet/minecraft/tags/TagKey;
public static final DOLPHIN_TRIES_TO_FIND : Lnet/minecraft/tags/TagKey;
public static final FROG_TRIES_TO_FIND_LAND_NEAR : Lnet/minecraft/tags/TagKey;
public static final ENTITY_FLOATABLE : Lnet/minecraft/tags/TagKey;
private <init>()V
private static create(Ljava/lang/String;)Lnet/minecraft/tags/TagKey;
static <clinit>()V
```
