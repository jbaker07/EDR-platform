---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.Hopper"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.Hopper

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getLevelX()D` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLevelY()D` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getLevelZ()D` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.level.block.entity.Hopper extends net.minecraft.world.Container {
    public static final net.minecraft.world.phys.AABB SUCK_AABB;
    public default net.minecraft.world.phys.AABB getSuckAabb();
    public abstract double getLevelX();
    public abstract double getLevelY();
    public abstract double getLevelZ();
    public abstract boolean isGridAligned();
    static {};
}
```
