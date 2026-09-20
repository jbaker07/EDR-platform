---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.NativeImage$Format"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.NativeImage$Format

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `alphaOffset` | `()I` | exact | invokevirtual@26 in `NativeImageMixin.fabric_isFullyOpaque` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `components` | `()I` | exact | invokevirtual@49 in `NativeImageMixin.fabric_isFullyOpaque` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `hasAlpha` | `()Z` | exact | invokevirtual@4 in `NativeImageMixin.fabric_isFullyOpaque` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `NativeImageMixin$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `NativeImageMixin$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@45 in `NativeImageMixin$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@60 in `NativeImageMixin$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@23 in `NativeImageMixin.fabric_copyPixelsLuminance` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@11 in `NativeImageMixin.fabric_copyPixelsRgb` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `values` | `()[Lcom/mojang/blaze3d/platform/NativeImage$Format;` | exact | invokestatic@0 in `NativeImageMixin$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `LUMINANCE` | `Lcom/mojang/blaze3d/platform/NativeImage$Format;` | exact | getstatic@57 in `NativeImageMixin$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `LUMINANCE_ALPHA` | `Lcom/mojang/blaze3d/platform/NativeImage$Format;` | exact | getstatic@42 in `NativeImageMixin$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `RGB` | `Lcom/mojang/blaze3d/platform/NativeImage$Format;` | exact | getstatic@27 in `NativeImageMixin$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `RGBA` | `Lcom/mojang/blaze3d/platform/NativeImage$Format;` | exact | getstatic@12 in `NativeImageMixin$1.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (16 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final RGBA : Lcom/mojang/blaze3d/platform/NativeImage$Format;
public static final RGB : Lcom/mojang/blaze3d/platform/NativeImage$Format;
public static final LUMINANCE_ALPHA : Lcom/mojang/blaze3d/platform/NativeImage$Format;
public static final LUMINANCE : Lcom/mojang/blaze3d/platform/NativeImage$Format;
private final components : I
private final hasRed : Z
private final hasGreen : Z
private final hasBlue : Z
private final hasLuminance : Z
private final hasAlpha : Z
private final redOffset : I
private final greenOffset : I
private final blueOffset : I
private final luminanceOffset : I
private final alphaOffset : I
private static final synthetic $VALUES : [Lcom/mojang/blaze3d/platform/NativeImage$Format;
public static values()[Lcom/mojang/blaze3d/platform/NativeImage$Format;
public static valueOf(Ljava/lang/String;)Lcom/mojang/blaze3d/platform/NativeImage$Format;
private <init>(Ljava/lang/String;IIZZZZZIIIII)V
public components()I
public hasRed()Z
public hasGreen()Z
public hasBlue()Z
public hasLuminance()Z
public hasAlpha()Z
public redOffset()I
public greenOffset()I
public blueOffset()I
public luminanceOffset()I
public alphaOffset()I
public hasLuminanceOrRed()Z
public hasLuminanceOrGreen()Z
public hasLuminanceOrBlue()Z
public hasLuminanceOrAlpha()Z
public luminanceOrRedOffset()I
public luminanceOrGreenOffset()I
public luminanceOrBlueOffset()I
public luminanceOrAlphaOffset()I
private static getStbFormat(I)Lcom/mojang/blaze3d/platform/NativeImage$Format;
private static synthetic $values()[Lcom/mojang/blaze3d/platform/NativeImage$Format;
static <clinit>()V
```
