---
type: "interface"
fqcn: "net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getClass()Ljava/lang/Class;` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.renderer.state.gui.pip.PictureInPictureRenderState extends net.minecraft.client.renderer.state.gui.ScreenArea {
    public static final org.joml.Matrix3x2fc IDENTITY_POSE;
    public abstract int x0();
    public abstract int x1();
    public abstract int y0();
    public abstract int y1();
    public abstract float scale();
    public default org.joml.Matrix3x2fc pose();
    public abstract net.minecraft.client.gui.navigation.ScreenRectangle scissorArea();
    public static net.minecraft.client.gui.navigation.ScreenRectangle getBounds(int, int, int, int, net.minecraft.client.gui.navigation.ScreenRectangle);
    public static net.minecraft.client.gui.navigation.ScreenRectangle getBounds(int, int, int, int, org.joml.Matrix3x2fc, net.minecraft.client.gui.navigation.ScreenRectangle);
    static {};
}
```
