---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/renderer/state/gui/ScreenArea`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getClass` | `()Ljava/lang/Class;` | inherited_exact | invokeinterface@4 in `PictureInPictureRendererRegistryImpl.createNewRenderer` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (1 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final IDENTITY_POSE : Lorg/joml/Matrix3x2fc;
public abstract x0()I
public abstract x1()I
public abstract y0()I
public abstract y1()I
public abstract scale()F
public pose()Lorg/joml/Matrix3x2fc;
public abstract scissorArea()Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public static getBounds(IIIILnet/minecraft/client/gui/navigation/ScreenRectangle;)Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public static getBounds(IIIILorg/joml/Matrix3x2fc;Lnet/minecraft/client/gui/navigation/ScreenRectangle;)Lnet/minecraft/client/gui/navigation/ScreenRectangle;
static <clinit>()V
```
