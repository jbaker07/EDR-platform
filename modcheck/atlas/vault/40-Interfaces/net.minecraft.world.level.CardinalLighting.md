---
type: "interface"
fqcn: "net.minecraft.world.level.CardinalLighting"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.CardinalLighting

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `byFace(Lnet/minecraft/core/Direction;)F` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace(Lnet/minecraft/core/Direction;)F` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.CardinalLighting extends java.lang.Record {
    private final float down;
    private final float up;
    private final float north;
    private final float south;
    private final float west;
    private final float east;
    public static final net.minecraft.world.level.CardinalLighting DEFAULT;
    public static final net.minecraft.world.level.CardinalLighting NETHER;
    public net.minecraft.world.level.CardinalLighting(float, float, float, float, float, float);
    public float byFace(net.minecraft.core.Direction);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public float down();
    public float up();
    public float north();
    public float south();
    public float west();
    public float east();
    static {};
}
```
