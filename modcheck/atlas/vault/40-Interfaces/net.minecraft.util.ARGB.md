---
type: "interface"
fqcn: "net.minecraft.util.ARGB"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.ARGB

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `blue` | `(I)I` | exact | invokestatic@54 in `TestScreenshotComparisonAlgorithms$MeanSquaredDifference.lambda$findCo | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `blue` | `(I)I` | exact | invokestatic@87 in `TestScreenshotComparisonAlgorithms$MeanSquaredDifference.lambda$findCo | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `green` | `(I)I` | exact | invokestatic@47 in `TestScreenshotComparisonAlgorithms$MeanSquaredDifference.lambda$findCo | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `green` | `(I)I` | exact | invokestatic@80 in `TestScreenshotComparisonAlgorithms$MeanSquaredDifference.lambda$findCo | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `multiply` | `(II)I` | exact | invokestatic@10 in `MutableQuadView.multiplyColor` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `multiply` | `(II)I` | exact | invokestatic@29 in `MutableQuadView.multiplyColor` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `multiply` | `(II)I` | exact | invokestatic@48 in `MutableQuadView.multiplyColor` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `multiply` | `(II)I` | exact | invokestatic@67 in `MutableQuadView.multiplyColor` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `multiply` | `(II)I` | exact | invokestatic@42 in `ExtendedBlockModelFeatureRenderer.putQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `multiply` | `(II)I` | exact | invokestatic@107 in `ExtendedBlockModelFeatureRenderer.bufferQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `red` | `(I)I` | exact | invokestatic@40 in `TestScreenshotComparisonAlgorithms$MeanSquaredDifference.lambda$findCo | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `red` | `(I)I` | exact | invokestatic@73 in `TestScreenshotComparisonAlgorithms$MeanSquaredDifference.lambda$findCo | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `redFloat` | `(I)F` | exact | invokestatic@103 in `AoCalculator.calcVanilla` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `scaleRGB` | `(IF)I` | exact | invokestatic@46 in `FlatLighter.applyDirectionalBrightness` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `scaleRGB` | `(IF)I` | exact | invokestatic@135 in `FlatLighter.applyDirectionalBrightness` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `scaleRGB` | `(IF)I` | exact | invokestatic@273 in `FlatLighter.applyDirectionalBrightness` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `scaleRGB` | `(IF)I` | exact | invokestatic@316 in `FlatLighter.applyDirectionalBrightness` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `scaleRGB` | `(IF)I` | exact | invokestatic@369 in `FlatLighter.applyDirectionalBrightness` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `scaleRGB` | `(IF)I` | exact | invokestatic@46 in `AltModelBlockRendererImpl.shadeQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `scaleRGB` | `(IF)I` | exact | invokestatic@100 in `AltModelBlockRendererImpl.shadeQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `vector3fFromRGB24` | `(I)Lorg/joml/Vector3f;` | exact | invokestatic@11 in `BiomeModificationContextImpl$EffectsContextImpl.setFogColor` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `vector3fFromRGB24` | `(I)Lorg/joml/Vector3f;` | exact | invokestatic@11 in `BiomeModificationContextImpl$EffectsContextImpl.setWaterFogColor` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `vector3fFromRGB24` | `(I)Lorg/joml/Vector3f;` | exact | invokestatic@11 in `BiomeModificationContextImpl$EffectsContextImpl.setSkyColor` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (3 fields, 71 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LINEAR_CHANNEL_DEPTH : I
private static final SRGB_TO_LINEAR : [S
private static final LINEAR_TO_SRGB : [B
public <init>()V
private static computeSrgbToLinear(F)F
private static computeLinearToSrgb(F)F
public static srgbToLinearChannel(I)F
public static linearToSrgbChannel(F)I
public static meanLinear(IIII)I
private static linearChannelMean(IIII)I
public static alpha(I)I
public static red(I)I
public static green(I)I
public static blue(I)I
public static color(IIII)I
public static color(III)I
public static color(Lnet/minecraft/world/phys/Vec3;)I
public static multiply(II)I
public static multiply(Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;)Lorg/joml/Vector3fc;
public static multiply(Lorg/joml/Vector4fc;Lorg/joml/Vector4fc;)Lorg/joml/Vector4fc;
public static addRgb(II)I
public static addRgb(Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;)Lorg/joml/Vector3fc;
public static addRgb(Lorg/joml/Vector4fc;Lorg/joml/Vector3fc;)Lorg/joml/Vector4fc;
public static subtractRgb(Ljava/lang/Integer;Ljava/lang/Integer;)Ljava/lang/Integer;
public static subtractRgb(Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;)Lorg/joml/Vector3fc;
public static subtractRgb(Lorg/joml/Vector4fc;Lorg/joml/Vector3fc;)Lorg/joml/Vector4fc;
public static multiplyAlpha(IF)I
public static scaleRGB(IF)I
public static scaleRGB(IFFF)I
public static scaleRGB(Lorg/joml/Vector3fc;F)Lorg/joml/Vector3fc;
public static scaleRGB(Lorg/joml/Vector4fc;F)Lorg/joml/Vector4fc;
public static scaleRGB(Lorg/joml/Vector3fc;FFF)Lorg/joml/Vector3fc;
public static scaleRGB(Lorg/joml/Vector4fc;FFF)Lorg/joml/Vector4fc;
public static scaleRGB(II)I
public static greyscale(I)I
public static greyscale(Lorg/joml/Vector3fc;)Lorg/joml/Vector3fc;
public static greyscale(Lorg/joml/Vector4fc;)Lorg/joml/Vector4fc;
public static alphaBlend(II)I
public static alphaBlend(Lorg/joml/Vector3fc;Lorg/joml/Vector4fc;)Lorg/joml/Vector3fc;
public static alphaBlend(Lorg/joml/Vector4fc;Lorg/joml/Vector4fc;)Lorg/joml/Vector4fc;
private static alphaBlendChannel(IIII)I
private static alphaBlendChannel(FFFF)F
public static srgbLerp(FII)I
public static srgbLerp(FLorg/joml/Vector3fc;Lorg/joml/Vector3fc;)Lorg/joml/Vector3fc;
public static srgbLerp(FLorg/joml/Vector4fc;Lorg/joml/Vector4fc;)Lorg/joml/Vector4fc;
public static linearLerp(FII)I
public static opaque(I)I
public static transparent(I)I
public static color(II)I
public static color(FI)I
public static white(F)I
public static white(I)I
public static black(F)I
public static black(I)I
public static gray(F)I
public static colorFromFloat(FFFF)I
public static colorFromVector3f(Lorg/joml/Vector3fc;)I
public static colorFromVector4f(Lorg/joml/Vector4fc;)I
public static vector3fFromRGB24(I)Lorg/joml/Vector3f;
public static vector4fFromARGB32(I)Lorg/joml/Vector4f;
public static setVector4fFromARGB32(Lorg/joml/Vector4f;I)Lorg/joml/Vector4f;
public static average(II)I
public static as8BitChannel(F)I
public static alphaFloat(I)F
public static redFloat(I)F
public static greenFloat(I)F
public static blueFloat(I)F
private static from8BitChannel(I)F
public static toABGR(I)I
public static fromABGR(I)I
public static setBrightness(IF)I
private static synthetic lambda$static$1([B)V
private static synthetic lambda$static$0([S)V
static <clinit>()V
```
