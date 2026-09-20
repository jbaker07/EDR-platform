---
type: "interface"
fqcn: "net.minecraft.client.gui.layouts.LayoutElement"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.layouts.LayoutElement

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `visitWidgets(Ljava/util/function/Consumer;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.gui.layouts.LayoutElement {
    public abstract void setX(int);
    public abstract void setY(int);
    public abstract int getX();
    public abstract int getY();
    public abstract int getWidth();
    public abstract int getHeight();
    public default net.minecraft.client.gui.navigation.ScreenRectangle getRectangle();
    public default void setPosition(int, int);
    public abstract void visitWidgets(java.util.function.Consumer<net.minecraft.client.gui.components.AbstractWidget>);
}
```
