---
type: "interface"
fqcn: "net.minecraft.client.Screenshot"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.Screenshot

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `takeScreenshot(Lcom/mojang/blaze3d/pipeline/RenderTarget;Ljava/util/functi` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.Screenshot {
    private static final org.slf4j.Logger LOGGER;
    public static final java.lang.String SCREENSHOT_DIR;
    public net.minecraft.client.Screenshot();
    public static void grab(java.io.File, com.mojang.blaze3d.pipeline.RenderTarget, java.util.function.Consumer<net.minecraft.network.chat.Component>);
    public static void grab(net.minecraft.client.Minecraft, boolean);
    public static void grab(java.io.File, java.lang.String, com.mojang.blaze3d.pipeline.RenderTarget, int, java.util.function.Consumer<net.minecraft.network.chat.Component>);
    public static void takeScreenshot(com.mojang.blaze3d.pipeline.RenderTarget, java.util.function.Consumer<com.mojang.blaze3d.platform.NativeImage>);
    public static void takeScreenshot(com.mojang.blaze3d.pipeline.RenderTarget, int, java.util.function.Consumer<com.mojang.blaze3d.platform.NativeImage>);
    private static java.io.File getFile(java.io.File);
    private static void lambda$takeScreenshot$1(com.mojang.renderpearl.api.buffers.GpuBuffer, int, int, int, com.mojang.renderpearl.api.textures.GpuTexture, java.util.function.Consumer);
    private static java.lang.String lambda$takeScreenshot$0();
    private static void lambda$grab$2(java.io.File, java.lang.String, java.util.function.Consumer, com.mojang.blaze3d.platform.NativeImage);
    private static void lambda$grab$3(com.mojang.blaze3d.platform.NativeImage, java.io.File, java.util.function.Consumer);
    private static net.minecraft.network.chat.Style lambda$grab$4(java.io.File, net.minecraft.network.chat.Style);
    private static void lambda$grab$0(net.minecraft.client.Minecraft, net.minecraft.network.chat.Component);
    private static void lambda$grab$1(net.minecraft.client.Minecraft, net.minecraft.network.chat.Component);
    static {};
}
```
