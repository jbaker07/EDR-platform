---
type: "interface"
fqcn: "net.minecraft.world.entity.player.Player$BedSleepingProblem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.player.Player$BedSleepingProblem

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `OTHER_PROBLEM` | `Lnet/minecraft/world/entity/player/Player$BedSleepingProblem;` | exact | getstatic@69 in `ServerPlayerMixin.redirectSleepDirection` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (5 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final message : Lnet/minecraft/network/chat/Component;
public static final TOO_FAR_AWAY : Lnet/minecraft/world/entity/player/Player$BedSleepingProblem;
public static final OBSTRUCTED : Lnet/minecraft/world/entity/player/Player$BedSleepingProblem;
public static final OTHER_PROBLEM : Lnet/minecraft/world/entity/player/Player$BedSleepingProblem;
public static final NOT_SAFE : Lnet/minecraft/world/entity/player/Player$BedSleepingProblem;
public <init>(Lnet/minecraft/network/chat/Component;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public message()Lnet/minecraft/network/chat/Component;
static <clinit>()V
```
