---
type: "interface"
fqcn: "net.minecraft.client.resources.model.geometry.BakedQuad"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.geometry.BakedQuad

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;Lorg/joml/Vector3fc` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `direction()Lnet/minecraft/core/Direction;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `materialInfo()Lnet/minecraft/client/resources/model/geometry/BakedQuad$M` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `materialInfo()Lnet/minecraft/client/resources/model/geometry/BakedQuad$M` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `packedUV0()J` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `packedUV1()J` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `packedUV2()J` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `packedUV3()J` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `position0()Lorg/joml/Vector3fc;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `position1()Lorg/joml/Vector3fc;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `position2()Lorg/joml/Vector3fc;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `position3()Lorg/joml/Vector3fc;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (29, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.resources.model.geometry.BakedQuad extends java.lang.Record {
    private final org.joml.Vector3fc position0;
    private final org.joml.Vector3fc position1;
    private final org.joml.Vector3fc position2;
    private final org.joml.Vector3fc position3;
    private final long packedUV0;
    private final long packedUV1;
    private final long packedUV2;
    private final long packedUV3;
    private final net.minecraft.core.Direction direction;
    private final net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo materialInfo;
    public static final int VERTEX_COUNT;
    public static final int FLAG_TRANSLUCENT;
    public static final int FLAG_ANIMATED;
    public net.minecraft.client.resources.model.geometry.BakedQuad(org.joml.Vector3fc, org.joml.Vector3fc, org.joml.Vector3fc, org.joml.Vector3fc, long, long, long, long, net.minecraft.core.Direction, net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo);
    public org.joml.Vector3fc position(int);
    public long packedUV(int);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public org.joml.Vector3fc position0();
    public org.joml.Vector3fc position1();
    public org.joml.Vector3fc position2();
    public org.joml.Vector3fc position3();
    public long packedUV0();
    public long packedUV1();
    public long packedUV2();
    public long packedUV3();
    public net.minecraft.core.Direction direction();
    public net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo materialInfo();
}
```
