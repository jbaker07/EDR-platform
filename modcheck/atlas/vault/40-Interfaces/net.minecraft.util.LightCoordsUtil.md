---
type: "interface"
fqcn: "net.minecraft.util.LightCoordsUtil"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.LightCoordsUtil

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `block` | `(I)I` | exact | invokestatic@159 in `QuadView.toBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `max` | `(II)I` | exact | invokestatic@101 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `pack` | `(II)I` | exact | invokestatic@149 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `sky` | `(I)I` | exact | invokestatic@166 in `QuadView.toBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `smoothBlock` | `(I)I` | exact | invokestatic@1 in `ExtraLightCoordsUtil.smoothMax` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `smoothBlock` | `(I)I` | exact | invokestatic@6 in `ExtraLightCoordsUtil.smoothMax` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `smoothPack` | `(II)I` | exact | invokestatic@34 in `ExtraLightCoordsUtil.smoothMax` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `smoothSky` | `(I)I` | exact | invokestatic@11 in `ExtraLightCoordsUtil.smoothMax` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `smoothSky` | `(I)I` | exact | invokestatic@17 in `ExtraLightCoordsUtil.smoothMax` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (3 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final FULL_BRIGHT : I
public static final FULL_SKY : I
private static final MAX_SMOOTH_LIGHT_LEVEL : I
public <init>()V
public static pack(II)I
public static block(I)I
public static sky(I)I
public static withBlock(II)I
public static smoothPack(II)I
public static smoothBlock(I)I
public static smoothSky(I)I
public static addSmoothBlockEmission(IF)I
public static max(II)I
public static lightCoordsWithEmission(II)I
public static smoothBlend(IIII)I
public static smoothWeightedBlend(IIIIFFFF)I
public static getLightCoords(Lnet/minecraft/world/level/BlockAndLightGetter;Lnet/minecraft/core/BlockPos;)I
public static getLightCoords(Lnet/minecraft/util/LightCoordsUtil$BrightnessGetter;Lnet/minecraft/world/level/BlockAndLightGetter;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)I
```
