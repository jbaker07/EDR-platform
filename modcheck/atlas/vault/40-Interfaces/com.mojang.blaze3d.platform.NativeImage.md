---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.NativeImage"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.NativeImage

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`class` public final; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(IIZ)V` | exact | invokespecial@88 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `checkAllocated` | `()V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| calls | `close` | `()V` | exact | invokevirtual@176 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@262 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@373 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@383 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@16 in `TestScreenshotComparisonOptionsImpl.lambda$getColorTemplateImage$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@51 in `TestScreenshotComparisonOptionsImpl.lambda$getColorTemplateImage$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@62 in `TestScreenshotComparisonOptionsImpl.lambda$getColorTemplateImage$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@16 in `TestScreenshotComparisonOptionsImpl.lambda$getGrayscaleTemplateImage$ | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@51 in `TestScreenshotComparisonOptionsImpl.lambda$getGrayscaleTemplateImage$ | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@62 in `TestScreenshotComparisonOptionsImpl.lambda$getGrayscaleTemplateImage$ | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| calls | `getHeight` | `()I` | exact | invokevirtual@18 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@58 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@9 in `TestScreenshotComparisonAlgorithms$RawImageImpl.fromGrayscaleNativeIma | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@9 in `TestScreenshotComparisonAlgorithms$RawImageImpl.fromColorNativeImage` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@30 in `TestScreenshotComparisonOptionsImpl.lambda$getColorTemplateImage$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@30 in `TestScreenshotComparisonOptionsImpl.lambda$getGrayscaleTemplateImage$ | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getPixelsABGR` | `()[I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| calls | `getWidth` | `()I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| calls | `getWidth` | `()I` | exact | invokevirtual@14 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@42 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@5 in `TestScreenshotComparisonAlgorithms$RawImageImpl.fromGrayscaleNativeIma | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@5 in `TestScreenshotComparisonAlgorithms$RawImageImpl.fromColorNativeImage` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@26 in `TestScreenshotComparisonOptionsImpl.lambda$getColorTemplateImage$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@26 in `TestScreenshotComparisonOptionsImpl.lambda$getGrayscaleTemplateImage$ | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `read` | `(Ljava/io/InputStream;)Lcom/mojang/blaze3d/platform/NativeImage;` | exact | invokestatic@43 in `TestScreenshotComparisonOptionsImpl.loadNativeImage` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `resizeSubRectTo` | `(IIIILcom/mojang/blaze3d/platform/NativeImage;)V` | exact | invokevirtual@112 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `writeToFile` | `(Ljava/nio/file/Path;)V` | exact | invokevirtual@105 in `ClientGameTestContextImpl.saveScreenshot` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `writeToFile` | `(Ljava/nio/file/Path;)V` | exact | invokevirtual@55 in `ClientGameTestContextImpl.onTemplateImageDoesntExist` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `format` | `Lcom/mojang/blaze3d/platform/NativeImage$Format;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `pixels` | `J` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |

## Declared members (9 fields, 39 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final MEMORY_POOL : Lcom/mojang/jtracy/MemoryPool;
private static final OPEN_OPTIONS : Ljava/util/Set;
private final format : Lcom/mojang/blaze3d/platform/NativeImage$Format;
private final width : I
private final height : I
private final useStbFree : Z
private pixels : J
private final size : J
public <init>(IIZ)V
public <init>(Lcom/mojang/blaze3d/platform/NativeImage$Format;IIZ)V
public <init>(Lcom/mojang/blaze3d/platform/NativeImage$Format;IIZJ)V
public toString()Ljava/lang/String;
private isOutsideBounds(II)Z
public static read(Ljava/io/InputStream;)Lcom/mojang/blaze3d/platform/NativeImage;
public static read([B)Lcom/mojang/blaze3d/platform/NativeImage;
private static putAndRead(Ljava/nio/ByteBuffer;[B)Lcom/mojang/blaze3d/platform/NativeImage;
public static read(Ljava/nio/ByteBuffer;)Lcom/mojang/blaze3d/platform/NativeImage;
private checkAllocated()V
public close()V
public isClosed()Z
public getWidth()I
public getHeight()I
public format()Lcom/mojang/blaze3d/platform/NativeImage$Format;
private getPixelABGR(II)I
public getPixel(II)I
public setPixelABGR(III)V
public setPixel(III)V
public mappedCopy(Ljava/util/function/IntUnaryOperator;)Lcom/mojang/blaze3d/platform/NativeImage;
public getPixelsABGR()[I
public getPixels()[I
public getLuminanceOrAlpha(II)B
public makePixelArray()[I
public writeToFile(Ljava/io/File;)V
public copyFromFont(Lorg/lwjgl/util/freetype/FT_Face;I)Z
public writeToFile(Ljava/nio/file/Path;)V
private writeToChannel(Ljava/nio/channels/WritableByteChannel;)Z
public copyFrom(Lcom/mojang/blaze3d/platform/NativeImage;)V
public fillRect(IIIII)V
public copyRect(IIIIIIZZ)V
public copyRect(Lcom/mojang/blaze3d/platform/NativeImage;IIIIIIZZ)V
public resizeSubRectTo(IIIILcom/mojang/blaze3d/platform/NativeImage;)V
public untrack()V
public getPointer()J
public getPixelBytes()Ljava/nio/ByteBuffer;
public computeTransparency(IIII)Lcom/mojang/blaze3d/platform/Transparency;
public computeTransparency()Lcom/mojang/blaze3d/platform/Transparency;
static <clinit>()V
```
