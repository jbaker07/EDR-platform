---
type: "interface"
fqcn: "net.minecraft.world.entity.EntitySpawnRequest"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntitySpawnRequest

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `reason` | `()Lnet/minecraft/world/entity/EntitySpawnReason;` | exact | invokevirtual@20 in `EntityTypeMixin.setSpawnReason` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final reason : Lnet/minecraft/world/entity/EntitySpawnReason;
private final ignoreChecks : Z
public <init>(Lnet/minecraft/world/entity/EntitySpawnReason;Z)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public reason()Lnet/minecraft/world/entity/EntitySpawnReason;
public ignoreChecks()Z
```
