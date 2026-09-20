---
type: "interface"
fqcn: "net.minecraft.client.gui.render.pip.GuiEntityRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.render.pip.GuiEntityRenderer

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/render/pip/PictureInPictureRenderer`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/renderer/entity/EntityRenderDispatcher;)V` | exact | invokespecial@13 in `PictureInPictureRendererRegistryImpl.lambda$registerVanillaFactories$ | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (1 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entityRenderDispatcher : Lnet/minecraft/client/renderer/entity/EntityRenderDispatcher;
public <init>(Lnet/minecraft/client/renderer/entity/EntityRenderDispatcher;)V
public getRenderStateClass()Ljava/lang/Class;
protected renderToTexture(Lnet/minecraft/client/renderer/state/gui/pip/GuiEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;)V
protected getTranslateY(II)F
protected getTextureLabel()Ljava/lang/String;
protected synthetic renderToTexture(Lnet/minecraft/client/renderer/state/gui/pip/PictureInPictureRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;)V
```
