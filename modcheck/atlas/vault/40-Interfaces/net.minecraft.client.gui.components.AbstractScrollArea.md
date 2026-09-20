---
type: "interface"
fqcn: "net.minecraft.client.gui.components.AbstractScrollArea"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.AbstractScrollArea

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `defaultSettings(I)Lnet/minecraft/client/gui/components/AbstractScrollArea$S` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.gui.components.AbstractScrollArea extends net.minecraft.client.gui.components.AbstractWidget {
    public static final int SCROLLBAR_WIDTH;
    private static final int SCROLLBAR_MIN_HEIGHT;
    private static final net.minecraft.resources.Identifier SCROLLER_SPRITE;
    private static final net.minecraft.resources.Identifier SCROLLER_BACKGROUND_SPRITE;
    private final net.minecraft.client.gui.components.AbstractScrollArea$ScrollbarSettings scrollbarSettings;
    private double scrollAmount;
    private boolean scrolling;
    public net.minecraft.client.gui.components.AbstractScrollArea(int, int, int, int, net.minecraft.network.chat.Component, net.minecraft.client.gui.components.AbstractScrollArea$ScrollbarSettings);
    public boolean mouseScrolled(double, double, double, double);
    public boolean mouseDragged(net.minecraft.client.input.MouseButtonEvent, double, double);
    public void onRelease(net.minecraft.client.input.MouseButtonEvent);
    public double scrollAmount();
    public void setScrollAmount(double);
    public boolean updateScrolling(net.minecraft.client.input.MouseButtonEvent);
    protected boolean isOverScrollbar(double, double);
    public void refreshScrollAmount();
    public int maxScrollAmount();
    protected boolean scrollable();
    public int scrollbarWidth();
    protected int scrollerHeight();
    protected int scrollBarX();
    public int scrollBarY();
    protected void extractScrollbar(net.minecraft.client.gui.GuiGraphicsExtractor, int, int);
    protected abstract int contentHeight();
    protected double scrollRate();
    public static net.minecraft.client.gui.components.AbstractScrollArea$ScrollbarSettings defaultSettings(int);
    static {};
}
```
