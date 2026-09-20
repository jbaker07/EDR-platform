---
type: "interface"
fqcn: "net.minecraft.world.phys.EntityHitResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.phys.EntityHitResult

System: [[20-Systems/net.minecraft.world.phys|net.minecraft.world.phys]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phy` | `` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getLocation()Lnet/minecraft/world/phys/Vec3;` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.phys.EntityHitResult extends net.minecraft.world.phys.HitResult {
    private final net.minecraft.world.entity.Entity entity;
    public net.minecraft.world.phys.EntityHitResult(net.minecraft.world.entity.Entity);
    public net.minecraft.world.phys.EntityHitResult(net.minecraft.world.entity.Entity, net.minecraft.world.phys.Vec3);
    public net.minecraft.world.entity.Entity getEntity();
    public net.minecraft.world.phys.HitResult$Type getType();
}
```
