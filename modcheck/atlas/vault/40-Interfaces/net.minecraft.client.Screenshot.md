---
type: "interface"
fqcn: "net.minecraft.client.Screenshot"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.Screenshot

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `takeScreenshot` | `(Lcom/mojang/blaze3d/pipeline/RenderTarget;Ljava/util/function/Consume` | exact | invokestatic@69 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (2 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final SCREENSHOT_DIR : Ljava/lang/String;
public <init>()V
public static grab(Ljava/io/File;Lcom/mojang/blaze3d/pipeline/RenderTarget;Ljava/util/function/Consumer;)V
public static grab(Lnet/minecraft/client/Minecraft;Z)V
public static grab(Ljava/io/File;Ljava/lang/String;Lcom/mojang/blaze3d/pipeline/RenderTarget;ILjava/util/function/Consumer;)V
public static takeScreenshot(Lcom/mojang/blaze3d/pipeline/RenderTarget;Ljava/util/function/Consumer;)V
public static takeScreenshot(Lcom/mojang/blaze3d/pipeline/RenderTarget;ILjava/util/function/Consumer;)V
private static getFile(Ljava/io/File;)Ljava/io/File;
private static synthetic lambda$takeScreenshot$1(Lcom/mojang/renderpearl/api/buffers/GpuBuffer;IIILcom/mojang/renderpearl/api/textures/GpuTexture;Ljava/util/function/Consumer;)V
private static synthetic lambda$takeScreenshot$0()Ljava/lang/String;
private static synthetic lambda$grab$2(Ljava/io/File;Ljava/lang/String;Ljava/util/function/Consumer;Lcom/mojang/blaze3d/platform/NativeImage;)V
private static synthetic lambda$grab$3(Lcom/mojang/blaze3d/platform/NativeImage;Ljava/io/File;Ljava/util/function/Consumer;)V
private static synthetic lambda$grab$4(Ljava/io/File;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
private static synthetic lambda$grab$0(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/chat/Component;)V
private static synthetic lambda$grab$1(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/chat/Component;)V
static <clinit>()V
```
