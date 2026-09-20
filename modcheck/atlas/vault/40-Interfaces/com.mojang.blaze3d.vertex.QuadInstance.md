---
type: "interface"
fqcn: "com.mojang.blaze3d.vertex.QuadInstance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.vertex.QuadInstance

System: [[20-Systems/com.mojang.blaze3d.vertex|com.mojang.blaze3d.vertex]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@85 in `AoCalculator.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@9 in `ExtendedBlockModelFeatureRenderer.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getColor` | `(I)I` | exact | invokevirtual@100 in `AoCalculator.calcVanilla` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(I)I` | exact | invokevirtual@116 in `AoCalculator.calcVanilla` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setColor` | `(I)V` | exact | invokevirtual@49 in `ExtendedBlockModelFeatureRenderer.putQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setLightCoords` | `(I)V` | exact | invokevirtual@64 in `ExtendedBlockModelFeatureRenderer.buildGroup` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setOverlayCoords` | `(I)V` | exact | invokevirtual@76 in `ExtendedBlockModelFeatureRenderer.buildGroup` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (9 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private color0 : I
private color1 : I
private color2 : I
private color3 : I
private lightCoords0 : I
private lightCoords1 : I
private lightCoords2 : I
private lightCoords3 : I
private overlayCoords : I
public <init>()V
public getColor(I)I
public getLightCoords(I)I
public getLightCoordsWithEmission(II)I
public overlayCoords()I
public setColor(II)V
public setLightCoords(II)V
public setColor(I)V
public setLightCoords(I)V
public setOverlayCoords(I)V
public multiplyColor(I)V
public scaleColor(F)V
```
