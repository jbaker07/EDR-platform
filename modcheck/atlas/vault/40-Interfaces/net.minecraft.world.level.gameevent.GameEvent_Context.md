---
type: "interface"
fqcn: "net.minecraft.world.level.gameevent.GameEvent$Context"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.gameevent.GameEvent$Context

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@57 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/wor` | exact | invokestatic@234 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (2 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final sourceEntity : Lnet/minecraft/world/entity/Entity;
private final affectedState : Lnet/minecraft/world/level/block/state/BlockState;
public <init>(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/level/block/state/BlockState;)V
public static of(Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/level/gameevent/GameEvent$Context;
public static of(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/gameevent/GameEvent$Context;
public static of(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/gameevent/GameEvent$Context;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public sourceEntity()Lnet/minecraft/world/entity/Entity;
public affectedState()Lnet/minecraft/world/level/block/state/BlockState;
```
