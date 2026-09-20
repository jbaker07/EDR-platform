---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.packs.TransferableSelectionList$PackEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.packs.TransferableSelectionList$PackEntry

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `extractContent` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.packs.TransferableSelectionList$PackEntry extends net.minecraft.client.gui.screens.packs.TransferableSelectionList$Entry implements net.minecraft.client.gui.components.SelectableEntry {
    private static final int MAX_DESCRIPTION_WIDTH_PIXELS;
    public static final int ICON_SIZE;
    private final net.minecraft.client.gui.screens.packs.TransferableSelectionList parent;
    protected final net.minecraft.client.Minecraft minecraft;
    private final net.minecraft.client.gui.screens.packs.PackSelectionModel$Entry pack;
    private final net.minecraft.client.gui.components.StringWidget nameWidget;
    private final net.minecraft.client.gui.components.MultiLineTextWidget descriptionWidget;
    final net.minecraft.client.gui.screens.packs.TransferableSelectionList this$0;
    public net.minecraft.client.gui.screens.packs.TransferableSelectionList$PackEntry(net.minecraft.client.gui.screens.packs.TransferableSelectionList, net.minecraft.client.Minecraft, net.minecraft.client.gui.screens.packs.TransferableSelectionList, net.minecraft.client.gui.screens.packs.PackSelectionModel$Entry);
    public net.minecraft.network.chat.Component getNarration();
    public void extractContent(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, boolean, float);
    public boolean mouseClicked(net.minecraft.client.input.MouseButtonEvent, boolean);
    public boolean keyPressed(net.minecraft.client.input.KeyEvent);
    private boolean showHoverOverlay();
    public void keyboardSelection();
    private void keyboardMoveUp();
    private void keyboardMoveDown();
    private void handlePackSelection();
    public java.lang.String getPackId();
    public boolean shouldTakeFocusAfterInteraction();
    private boolean lambda$shouldTakeFocusAfterInteraction$0(net.minecraft.client.gui.screens.packs.TransferableSelectionList$Entry);
    private void lambda$handlePackSelection$0(boolean);
}
```
