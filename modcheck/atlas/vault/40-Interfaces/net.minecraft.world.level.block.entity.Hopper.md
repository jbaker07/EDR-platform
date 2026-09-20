---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.Hopper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.Hopper

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/Container`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getLevelX` | `()D` | exact | invokeinterface@6 in `HopperBlockEntityMixin.hookExtract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLevelY` | `()D` | exact | invokeinterface@12 in `HopperBlockEntityMixin.hookExtract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLevelZ` | `()D` | exact | invokeinterface@20 in `HopperBlockEntityMixin.hookExtract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (1 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SUCK_AABB : Lnet/minecraft/world/phys/AABB;
public getSuckAabb()Lnet/minecraft/world/phys/AABB;
public abstract getLevelX()D
public abstract getLevelY()D
public abstract getLevelZ()D
public abstract isGridAligned()Z
static <clinit>()V
```
