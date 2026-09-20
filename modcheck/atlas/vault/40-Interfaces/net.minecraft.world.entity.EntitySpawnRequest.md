---
type: "interface"
fqcn: "net.minecraft.world.entity.EntitySpawnRequest"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntitySpawnRequest

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `reason()Lnet/minecraft/world/entity/EntitySpawnReason;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.entity.EntitySpawnRequest extends java.lang.Record {
    private final net.minecraft.world.entity.EntitySpawnReason reason;
    private final boolean ignoreChecks;
    public net.minecraft.world.entity.EntitySpawnRequest(net.minecraft.world.entity.EntitySpawnReason, boolean);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.entity.EntitySpawnReason reason();
    public boolean ignoreChecks();
}
```
