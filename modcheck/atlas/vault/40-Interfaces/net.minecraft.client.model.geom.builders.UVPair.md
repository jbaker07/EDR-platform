---
type: "interface"
fqcn: "net.minecraft.client.model.geom.builders.UVPair"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.model.geom.builders.UVPair

System: [[20-Systems/net.minecraft.client.model|net.minecraft.client.model]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `pack` | `(FF)J` | exact | invokestatic@52 in `QuadView.toBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `pack` | `(FF)J` | exact | invokestatic@71 in `QuadView.toBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `pack` | `(FF)J` | exact | invokestatic@90 in `QuadView.toBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `pack` | `(FF)J` | exact | invokestatic@109 in `QuadView.toBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `unpackU` | `(J)F` | exact | invokestatic@80 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `unpackU` | `(J)F` | exact | invokestatic@95 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `unpackU` | `(J)F` | exact | invokestatic@111 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `unpackU` | `(J)F` | exact | invokestatic@127 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `unpackV` | `(J)F` | exact | invokestatic@84 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `unpackV` | `(J)F` | exact | invokestatic@100 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `unpackV` | `(J)F` | exact | invokestatic@116 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `unpackV` | `(J)F` | exact | invokestatic@132 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (2 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final u : F
private final v : F
public <init>(FF)V
public toString()Ljava/lang/String;
public static pack(FF)J
public static unpackU(J)F
public static unpackV(J)F
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public u()F
public v()F
```
