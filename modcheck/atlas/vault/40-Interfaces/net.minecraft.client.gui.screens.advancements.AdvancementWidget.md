---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.advancements.AdvancementWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.advancements.AdvancementWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getAdvancement()Lnet/minecraft/advancements/AdvancementHolder;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extractHover` | `@Inject at INVOKE Ljava/util/List;isEmpty()Z` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (48, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.advancements.AdvancementWidget {
    private static final net.minecraft.resources.Identifier TITLE_BOX_SPRITE;
    private static final int HEIGHT;
    private static final int BOX_X;
    private static final int BOX_WIDTH;
    private static final int FRAME_WIDTH;
    private static final int ICON_X;
    private static final int ICON_Y;
    private static final int ICON_WIDTH;
    private static final int TITLE_PADDING_LEFT;
    private static final int TITLE_PADDING_RIGHT;
    private static final int TITLE_X;
    private static final int TITLE_PADDING_TOP;
    private static final int TITLE_PADDING_BOTTOM;
    private static final int TITLE_MAX_WIDTH;
    private static final int TITLE_MIN_WIDTH;
    private static final int[] TEST_SPLIT_OFFSETS;
    private final net.minecraft.advancements.AdvancementNode advancementNode;
    private final net.minecraft.advancements.DisplayInfo display;
    private final net.minecraft.world.item.ItemStack icon;
    private final java.util.List<net.minecraft.util.FormattedCharSequence> titleLines;
    private final int width;
    private final java.util.List<net.minecraft.util.FormattedCharSequence> description;
    private final net.minecraft.client.Minecraft minecraft;
    private net.minecraft.client.gui.screens.advancements.AdvancementWidget parent;
    private final java.util.List<net.minecraft.client.gui.screens.advancements.AdvancementWidget> children;
    private net.minecraft.advancements.AdvancementProgress progress;
    private final int x;
    private final int y;
    private net.minecraft.client.gui.screens.advancements.AdvancementWidget(net.minecraft.client.Minecraft, net.minecraft.advancements.AdvancementNode, net.minecraft.advancements.DisplayInfo);
    public static net.minecraft.client.gui.screens.advancements.AdvancementWidget createWidget(net.minecraft.client.Minecraft, net.minecraft.advancements.AdvancementNode);
    private int getMaxProgressWidth();
    private static float getMaxWidth(net.minecraft.client.StringSplitter, java.util.List<net.minecraft.network.chat.FormattedText>);
    private java.util.List<net.minecraft.network.chat.FormattedText> findOptimalLines(net.minecraft.network.chat.Component, int);
    private static net.minecraft.advancements.AdvancementHolder findFirstVisibleParent(net.minecraft.advancements.AdvancementNode);
    public void extractConnectivity(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, boolean);
    public void extractRenderState(net.minecraft.client.gui.GuiGraphicsExtractor, int, int);
    public int getWidth();
    public void setProgress(net.minecraft.advancements.AdvancementProgress);
    public void addChild(net.minecraft.client.gui.screens.advancements.AdvancementWidget);
    public void extractHover(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, float, int, int, int);
    private void extractMultilineText(net.minecraft.client.gui.GuiGraphicsExtractor, java.util.List<net.minecraft.util.FormattedCharSequence>, int, int, int);
    public boolean isMouseOver(int, int, int, int);
    public void attachToParent(net.minecraft.client.gui.screens.advancements.AdvancementTab);
    public int getY();
    public int getX();
    public net.minecraft.advancements.AdvancementHolder getAdvancement();
    public net.minecraft.advancements.DisplayInfo getDisplay();
    static {};
}
```
