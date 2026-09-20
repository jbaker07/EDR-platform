---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.packs.TransferableSelectionList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.packs.TransferableSelectionList

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/components/ObjectSelectionList`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `maxScrollAmount` | `()I` | inherited_exact | invokevirtual@73 in `TransferableSelectionListPackEntryMixin.onExtractContent` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (13 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final SELECT_HIGHLIGHTED_SPRITE : Lnet/minecraft/resources/Identifier;
private static final SELECT_SPRITE : Lnet/minecraft/resources/Identifier;
private static final UNSELECT_HIGHLIGHTED_SPRITE : Lnet/minecraft/resources/Identifier;
private static final UNSELECT_SPRITE : Lnet/minecraft/resources/Identifier;
private static final MOVE_UP_HIGHLIGHTED_SPRITE : Lnet/minecraft/resources/Identifier;
private static final MOVE_UP_SPRITE : Lnet/minecraft/resources/Identifier;
private static final MOVE_DOWN_HIGHLIGHTED_SPRITE : Lnet/minecraft/resources/Identifier;
private static final MOVE_DOWN_SPRITE : Lnet/minecraft/resources/Identifier;
private static final INCOMPATIBLE_TITLE : Lnet/minecraft/network/chat/Component;
private static final INCOMPATIBLE_CONFIRM_TITLE : Lnet/minecraft/network/chat/Component;
private static final ENTRY_PADDING : I
private final title : Lnet/minecraft/network/chat/Component;
private final screen : Lnet/minecraft/client/gui/screens/packs/PackSelectionScreen;
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/packs/PackSelectionScreen;IILnet/minecraft/network/chat/Component;)V
public getRowWidth()I
protected scrollBarX()I
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public updateList(Ljava/util/stream/Stream;Lnet/minecraft/client/gui/screens/packs/PackSelectionModel$EntryBase;)V
static synthetic access$000(Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;)Z
static synthetic access$100(Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
static synthetic access$200(Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
static synthetic access$300(Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
static synthetic access$400(Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
static synthetic access$500(Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;)Z
static synthetic access$600(Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;)Z
private synthetic lambda$updateList$0(Lnet/minecraft/client/gui/screens/packs/PackSelectionModel$EntryBase;Lnet/minecraft/client/gui/screens/packs/PackSelectionModel$Entry;)V
static <clinit>()V
```
