---
type: "interface"
fqcn: "net.minecraft.client.gui.components.FocusableTextWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.FocusableTextWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `builder(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setCentered(Z)Lnet/minecraft/client/gui/components/MultiLineTextWidget;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.components.FocusableTextWidget extends net.minecraft.client.gui.components.MultiLineTextWidget {
    public static final int DEFAULT_PADDING;
    private final int padding;
    private final int maxWidth;
    private final boolean alwaysShowBorder;
    private final net.minecraft.client.gui.components.FocusableTextWidget$BackgroundFill backgroundFill;
    private boolean narrateMessage;
    private net.minecraft.network.chat.Component focusedUsageNarration;
    private net.minecraft.network.chat.Component hoveredUsageNarration;
    private net.minecraft.client.gui.components.FocusableTextWidget(net.minecraft.network.chat.Component, net.minecraft.client.gui.Font, int, int, net.minecraft.client.gui.components.FocusableTextWidget$BackgroundFill, boolean);
    protected void updateWidgetNarration(net.minecraft.client.gui.narration.NarrationElementOutput);
    public void setNarrateMessage(boolean);
    public void setUsageNarration(net.minecraft.network.chat.Component, net.minecraft.network.chat.Component);
    public void extractWidgetRenderState(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, float);
    protected int getTextX();
    protected int getTextY();
    public net.minecraft.client.gui.components.MultiLineTextWidget setMaxWidth(int);
    public int getWidth();
    public int getHeight();
    public int getPadding();
    public void updateWidth();
    public void updateHeight();
    public void setMessage(net.minecraft.network.chat.Component);
    public void playDownSound(net.minecraft.client.sounds.SoundManager);
    public boolean keyPressed(net.minecraft.client.input.KeyEvent);
    public static net.minecraft.client.gui.components.FocusableTextWidget$Builder builder(net.minecraft.network.chat.Component, net.minecraft.client.gui.Font);
    public static net.minecraft.client.gui.components.FocusableTextWidget$Builder builder(net.minecraft.network.chat.Component, net.minecraft.client.gui.Font, int);
    private static java.util.Optional lambda$keyPressed$0(net.minecraft.network.chat.Style, java.lang.String);
}
```
