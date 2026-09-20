---
type: "interface"
fqcn: "net.minecraft.world.WorldlyContainerHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.WorldlyContainerHolder

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getContainer` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/worl` | exact | invokeinterface@29 in `ItemStorage.lambda$static$2` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getContainer` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/worl` | exact | invokeinterface@41 in `ItemStorage.lambda$static$2` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getContainer(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/WorldlyContainer;
```
