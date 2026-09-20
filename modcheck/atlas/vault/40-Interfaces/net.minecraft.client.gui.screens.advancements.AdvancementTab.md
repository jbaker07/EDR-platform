---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.advancements.AdvancementTab"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.advancements.AdvancementTab

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getRootAdvancement()Lnet/minecraft/advancements/AdvancementHolder;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `isMouseOver(IIDD)Z` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extractContents` | `@Inject at INVOKE Lnet/minecraft/client/gui/screens/advancements/AdvancementWidg` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (38, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.advancements.AdvancementTab {
    private final net.minecraft.client.Minecraft minecraft;
    private final net.minecraft.client.gui.screens.advancements.AdvancementsScreen screen;
    private final net.minecraft.client.gui.screens.advancements.AdvancementTabType type;
    private final int index;
    private final net.minecraft.resources.Identifier background;
    private final net.minecraft.world.item.ItemStack icon;
    private final net.minecraft.network.chat.Component title;
    private final net.minecraft.client.gui.screens.advancements.AdvancementWidget root;
    private final java.util.Map<net.minecraft.advancements.AdvancementHolder, net.minecraft.client.gui.screens.advancements.AdvancementWidget> widgets;
    private double scrollX;
    private double scrollY;
    private int minX;
    private int minY;
    private int maxX;
    private int maxY;
    private float fade;
    private boolean centered;
    private net.minecraft.client.gui.screens.advancements.AdvancementWidget hovered;
    public net.minecraft.client.gui.screens.advancements.AdvancementTab(net.minecraft.client.Minecraft, net.minecraft.client.gui.screens.advancements.AdvancementsScreen, net.minecraft.client.gui.screens.advancements.AdvancementTabType, int, net.minecraft.client.gui.screens.advancements.AdvancementWidget, net.minecraft.world.item.ItemStackTemplate, net.minecraft.network.chat.Component, net.minecraft.resources.Identifier);
    public void copyPosition(net.minecraft.client.gui.screens.advancements.AdvancementTab);
    public net.minecraft.client.gui.screens.advancements.AdvancementTabType getType();
    public int getIndex();
    public net.minecraft.advancements.AdvancementHolder getRootAdvancement();
    public net.minecraft.network.chat.Component getTitle();
    public void tick(int, int);
    public void extractTab(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, int, int, boolean);
    public void extractIcon(net.minecraft.client.gui.GuiGraphicsExtractor, int, int);
    public void extractContents(net.minecraft.client.gui.GuiGraphicsExtractor, int, int);
    public void extractTooltips(net.minecraft.client.gui.GuiGraphicsExtractor, int, int);
    public boolean isMouseOver(int, int, double, double);
    public static net.minecraft.client.gui.screens.advancements.AdvancementTab create(net.minecraft.client.Minecraft, net.minecraft.client.gui.screens.advancements.AdvancementsScreen, int, net.minecraft.advancements.AdvancementNode);
    public void scroll(double, double);
    public boolean canScrollHorizontally();
    public boolean canScrollVertically();
    public void addAdvancement(net.minecraft.advancements.AdvancementNode);
    private void addWidget(net.minecraft.client.gui.screens.advancements.AdvancementWidget);
    public net.minecraft.client.gui.screens.advancements.AdvancementWidget getWidget(net.minecraft.advancements.AdvancementHolder);
    public net.minecraft.client.gui.screens.advancements.AdvancementsScreen getScreen();
}
```
