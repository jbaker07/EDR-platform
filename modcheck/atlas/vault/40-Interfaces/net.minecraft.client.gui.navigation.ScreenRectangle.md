---
type: "interface"
fqcn: "net.minecraft.client.gui.navigation.ScreenRectangle"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.navigation.ScreenRectangle

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(IIII)V` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (32, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.gui.navigation.ScreenRectangle extends java.lang.Record {
    private final net.minecraft.client.gui.navigation.ScreenPosition position;
    private final int width;
    private final int height;
    private static final net.minecraft.client.gui.navigation.ScreenRectangle EMPTY;
    public net.minecraft.client.gui.navigation.ScreenRectangle(int, int, int, int);
    public net.minecraft.client.gui.navigation.ScreenRectangle(net.minecraft.client.gui.navigation.ScreenPosition, int, int);
    public static net.minecraft.client.gui.navigation.ScreenRectangle empty();
    public static net.minecraft.client.gui.navigation.ScreenRectangle of(net.minecraft.client.gui.navigation.ScreenAxis, int, int, int, int);
    public net.minecraft.client.gui.navigation.ScreenRectangle step(net.minecraft.client.gui.navigation.ScreenDirection);
    public int getLength(net.minecraft.client.gui.navigation.ScreenAxis);
    public int getBoundInDirection(net.minecraft.client.gui.navigation.ScreenDirection);
    public net.minecraft.client.gui.navigation.ScreenRectangle getBorder(net.minecraft.client.gui.navigation.ScreenDirection);
    public boolean overlaps(net.minecraft.client.gui.navigation.ScreenRectangle);
    public boolean overlapsInAxis(net.minecraft.client.gui.navigation.ScreenRectangle, net.minecraft.client.gui.navigation.ScreenAxis);
    public int getCenterInAxis(net.minecraft.client.gui.navigation.ScreenAxis);
    public net.minecraft.client.gui.navigation.ScreenRectangle intersection(net.minecraft.client.gui.navigation.ScreenRectangle);
    public boolean intersects(net.minecraft.client.gui.navigation.ScreenRectangle);
    public boolean encompasses(net.minecraft.client.gui.navigation.ScreenRectangle);
    public int top();
    public int bottom();
    public int left();
    public int right();
    public boolean containsPoint(int, int);
    public net.minecraft.client.gui.navigation.ScreenRectangle transformAxisAligned(org.joml.Matrix3x2fc);
    public net.minecraft.client.gui.navigation.ScreenRectangle transformMaxBounds(org.joml.Matrix3x2fc);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.client.gui.navigation.ScreenPosition position();
    public int width();
    public int height();
    static {};
}
```
