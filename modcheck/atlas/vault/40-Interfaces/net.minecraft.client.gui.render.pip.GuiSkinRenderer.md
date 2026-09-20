---
type: "interface"
fqcn: "net.minecraft.client.gui.render.pip.GuiSkinRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.render.pip.GuiSkinRenderer

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.render.pip.GuiSkinRenderer extends net.minecraft.client.gui.render.pip.PictureInPictureRenderer<net.minecraft.client.renderer.state.gui.pip.GuiSkinRenderState> {
    public net.minecraft.client.gui.render.pip.GuiSkinRenderer();
    public java.lang.Class<net.minecraft.client.renderer.state.gui.pip.GuiSkinRenderState> getRenderStateClass();
    protected void renderToTexture(net.minecraft.client.renderer.state.gui.pip.GuiSkinRenderState, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector);
    protected java.lang.String getTextureLabel();
    protected void renderToTexture(net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector);
}
```
