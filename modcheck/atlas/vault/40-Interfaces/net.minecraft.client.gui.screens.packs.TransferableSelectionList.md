---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.packs.TransferableSelectionList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.packs.TransferableSelectionList

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `maxScrollAmount()I` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.packs.TransferableSelectionList extends net.minecraft.client.gui.components.ObjectSelectionList<net.minecraft.client.gui.screens.packs.TransferableSelectionList$Entry> {
    private static final net.minecraft.resources.Identifier SELECT_HIGHLIGHTED_SPRITE;
    private static final net.minecraft.resources.Identifier SELECT_SPRITE;
    private static final net.minecraft.resources.Identifier UNSELECT_HIGHLIGHTED_SPRITE;
    private static final net.minecraft.resources.Identifier UNSELECT_SPRITE;
    private static final net.minecraft.resources.Identifier MOVE_UP_HIGHLIGHTED_SPRITE;
    private static final net.minecraft.resources.Identifier MOVE_UP_SPRITE;
    private static final net.minecraft.resources.Identifier MOVE_DOWN_HIGHLIGHTED_SPRITE;
    private static final net.minecraft.resources.Identifier MOVE_DOWN_SPRITE;
    private static final net.minecraft.network.chat.Component INCOMPATIBLE_TITLE;
    private static final net.minecraft.network.chat.Component INCOMPATIBLE_CONFIRM_TITLE;
    private static final int ENTRY_PADDING;
    private final net.minecraft.network.chat.Component title;
    private final net.minecraft.client.gui.screens.packs.PackSelectionScreen screen;
    public net.minecraft.client.gui.screens.packs.TransferableSelectionList(net.minecraft.client.Minecraft, net.minecraft.client.gui.screens.packs.PackSelectionScreen, int, int, net.minecraft.network.chat.Component);
    public int getRowWidth();
    protected int scrollBarX();
    public boolean keyPressed(net.minecraft.client.input.KeyEvent);
    public void updateList(java.util.stream.Stream<net.minecraft.client.gui.screens.packs.PackSelectionModel$Entry>, net.minecraft.client.gui.screens.packs.PackSelectionModel$EntryBase);
    static boolean access$000(net.minecraft.client.gui.screens.packs.TransferableSelectionList);
    static void access$100(net.minecraft.client.gui.screens.packs.TransferableSelectionList, net.minecraft.client.gui.GuiGraphicsExtractor);
    static void access$200(net.minecraft.client.gui.screens.packs.TransferableSelectionList, net.minecraft.client.gui.GuiGraphicsExtractor);
    static void access$300(net.minecraft.client.gui.screens.packs.TransferableSelectionList, net.minecraft.client.gui.GuiGraphicsExtractor);
    static void access$400(net.minecraft.client.gui.screens.packs.TransferableSelectionList, net.minecraft.client.gui.GuiGraphicsExtractor);
    static boolean access$500(net.minecraft.client.gui.screens.packs.TransferableSelectionList);
    static boolean access$600(net.minecraft.client.gui.screens.packs.TransferableSelectionList);
    private void lambda$updateList$0(net.minecraft.client.gui.screens.packs.PackSelectionModel$EntryBase, net.minecraft.client.gui.screens.packs.PackSelectionModel$Entry);
    static {};
}
```
