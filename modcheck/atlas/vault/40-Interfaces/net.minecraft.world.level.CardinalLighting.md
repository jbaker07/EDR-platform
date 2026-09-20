---
type: "interface"
fqcn: "net.minecraft.world.level.CardinalLighting"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.CardinalLighting

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@906 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@14 in `FlatLighter.applyDirectionalBrightness` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@165 in `FlatLighter.applyDirectionalBrightness` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@337 in `FlatLighter.applyDirectionalBrightness` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@19 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@49 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@75 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@105 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@131 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `byFace` | `(Lnet/minecraft/core/Direction;)F` | exact | invokevirtual@161 in `FlatLighter.normalShade` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (8 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final down : F
private final up : F
private final north : F
private final south : F
private final west : F
private final east : F
public static final DEFAULT : Lnet/minecraft/world/level/CardinalLighting;
public static final NETHER : Lnet/minecraft/world/level/CardinalLighting;
public <init>(FFFFFF)V
public byFace(Lnet/minecraft/core/Direction;)F
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public down()F
public up()F
public north()F
public south()F
public west()F
public east()F
static <clinit>()V
```
