---
type: "interface"
fqcn: "net.minecraft.client.model.geom.builders.UVPair"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.model.geom.builders.UVPair

System: [[20-Systems/net.minecraft.client.model|net.minecraft.client.model]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `unpackU(J)F` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `unpackV(J)F` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.model.geom.builders.UVPair extends java.lang.Record {
    private final float u;
    private final float v;
    public net.minecraft.client.model.geom.builders.UVPair(float, float);
    public java.lang.String toString();
    public static long pack(float, float);
    public static float unpackU(long);
    public static float unpackV(long);
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public float u();
    public float v();
}
```
