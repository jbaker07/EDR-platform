---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.goal.FloatGoal"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.goal.FloatGoal

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `net/minecraft/world/entity/ai/goal/Goal`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `canUse` | `()Z` | name_only | @ModifyReturnValue at ['RETURN'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `mob` | `Lnet/minecraft/world/entity/Mob;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final mob : Lnet/minecraft/world/entity/Mob;
protected final fluid : Lnet/minecraft/tags/TagKey;
public <init>(Lnet/minecraft/world/entity/Mob;)V
public <init>(Lnet/minecraft/world/entity/Mob;Lnet/minecraft/tags/TagKey;)V
public canUse()Z
public requiresUpdateEveryTick()Z
public tick()V
```
