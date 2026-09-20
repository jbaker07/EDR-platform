---
type: "interface"
fqcn: "net.minecraft.client.gui.layouts.HeaderAndFooterLayout"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.layouts.HeaderAndFooterLayout

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/client/gui/screens/Screen;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `addTitleHeader(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `addToContents(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minec` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `addToFooter(Lnet/minecraft/client/gui/layouts/LayoutElement;)Lnet/minec` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `arrangeElements()V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getContentHeight()I` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getHeaderHeight()I` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `visitWidgets(Ljava/util/function/Consumer;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (33, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.layouts.HeaderAndFooterLayout implements net.minecraft.client.gui.layouts.Layout {
    public static final int MAGIC_PADDING;
    public static final int DEFAULT_HEADER_AND_FOOTER_HEIGHT;
    private static final int CONTENT_MARGIN_TOP;
    private final net.minecraft.client.gui.layouts.FrameLayout headerFrame;
    private final net.minecraft.client.gui.layouts.FrameLayout footerFrame;
    private final net.minecraft.client.gui.layouts.FrameLayout contentsFrame;
    private final net.minecraft.client.gui.screens.Screen screen;
    private int headerHeight;
    private int footerHeight;
    public net.minecraft.client.gui.layouts.HeaderAndFooterLayout(net.minecraft.client.gui.screens.Screen);
    public net.minecraft.client.gui.layouts.HeaderAndFooterLayout(net.minecraft.client.gui.screens.Screen, int);
    public net.minecraft.client.gui.layouts.HeaderAndFooterLayout(net.minecraft.client.gui.screens.Screen, int, int);
    public void setX(int);
    public void setY(int);
    public int getX();
    public int getY();
    public int getWidth();
    public int getHeight();
    public int getFooterHeight();
    public void setFooterHeight(int);
    public void setHeaderHeight(int);
    public int getHeaderHeight();
    public int getContentHeight();
    public void visitChildren(java.util.function.Consumer<net.minecraft.client.gui.layouts.LayoutElement>);
    public void removeChildren();
    public void arrangeElements();
    public <T extends net.minecraft.client.gui.layouts.LayoutElement> T addToHeader(T);
    public <T extends net.minecraft.client.gui.layouts.LayoutElement> T addToHeader(T, java.util.function.Consumer<net.minecraft.client.gui.layouts.LayoutSettings>);
    public void addTitleHeader(net.minecraft.network.chat.Component, net.minecraft.client.gui.Font);
    public <T extends net.minecraft.client.gui.layouts.LayoutElement> T addToFooter(T);
    public <T extends net.minecraft.client.gui.layouts.LayoutElement> T addToFooter(T, java.util.function.Consumer<net.minecraft.client.gui.layouts.LayoutSettings>);
    public <T extends net.minecraft.client.gui.layouts.LayoutElement> T addToContents(T);
    public <T extends net.minecraft.client.gui.layouts.LayoutElement> T addToContents(T, java.util.function.Consumer<net.minecraft.client.gui.layouts.LayoutSettings>);
}
```
