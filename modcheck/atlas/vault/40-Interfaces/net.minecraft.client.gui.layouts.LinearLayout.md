---
type: "interface"
fqcn: "net.minecraft.client.gui.layouts.LinearLayout"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.layouts.LinearLayout

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `addChild(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minec` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `arrangeElements()V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `spacing(I)Lnet/minecraft/client/gui/layouts/LinearLayout;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `vertical()Lnet/minecraft/client/gui/layouts/LinearLayout;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.layouts.LinearLayout implements net.minecraft.client.gui.layouts.Layout {
    private final net.minecraft.client.gui.layouts.GridLayout wrapped;
    private final net.minecraft.client.gui.layouts.LinearLayout$Orientation orientation;
    private int nextChildIndex;
    private net.minecraft.client.gui.layouts.LinearLayout(net.minecraft.client.gui.layouts.LinearLayout$Orientation);
    public net.minecraft.client.gui.layouts.LinearLayout(int, int, net.minecraft.client.gui.layouts.LinearLayout$Orientation);
    public net.minecraft.client.gui.layouts.LinearLayout spacing(int);
    public net.minecraft.client.gui.layouts.LayoutSettings newCellSettings();
    public net.minecraft.client.gui.layouts.LayoutSettings defaultCellSetting();
    public <T extends net.minecraft.client.gui.layouts.LayoutElement> T addChild(T, net.minecraft.client.gui.layouts.LayoutSettings);
    public <T extends net.minecraft.client.gui.layouts.LayoutElement> T addChild(T);
    public <T extends net.minecraft.client.gui.layouts.LayoutElement> T addChild(T, java.util.function.Consumer<net.minecraft.client.gui.layouts.LayoutSettings>);
    public void visitChildren(java.util.function.Consumer<net.minecraft.client.gui.layouts.LayoutElement>);
    public void removeChildren();
    public void arrangeElements();
    public int getWidth();
    public int getHeight();
    public void setX(int);
    public void setY(int);
    public int getX();
    public int getY();
    public static net.minecraft.client.gui.layouts.LinearLayout vertical();
    public static net.minecraft.client.gui.layouts.LinearLayout horizontal();
}
```
