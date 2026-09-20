---
type: "interface"
fqcn: "net.minecraft.world.entity.MoverType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.MoverType

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `SELF` | `Lnet/minecraft/world/entity/MoverType;` | exact | getstatic@8 in `FluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `SELF` | `Lnet/minecraft/world/entity/MoverType;` | exact | getstatic@20 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `SELF` | `Lnet/minecraft/world/entity/MoverType;` | exact | getstatic@20 in `SimpleConfiguredFluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (6 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SELF : Lnet/minecraft/world/entity/MoverType;
public static final PLAYER : Lnet/minecraft/world/entity/MoverType;
public static final PISTON : Lnet/minecraft/world/entity/MoverType;
public static final SHULKER_BOX : Lnet/minecraft/world/entity/MoverType;
public static final SHULKER : Lnet/minecraft/world/entity/MoverType;
private static final synthetic $VALUES : [Lnet/minecraft/world/entity/MoverType;
public static values()[Lnet/minecraft/world/entity/MoverType;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/world/entity/MoverType;
private <init>(Ljava/lang/String;I)V
public isServerAndClientSimulated()Z
private static synthetic $values()[Lnet/minecraft/world/entity/MoverType;
static <clinit>()V
```
