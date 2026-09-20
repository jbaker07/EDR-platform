---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.packs.TransferableSelectionList$PackEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.packs.TransferableSelectionList$PackEntry

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/screens/packs/TransferableSelectionList$Entry`; implements `net/minecraft/client/gui/components/SelectableEntry`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `extractContent` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIZF)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `MAX_DESCRIPTION_WIDTH_PIXELS` | `I` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| reads | `descriptionWidget` | `Lnet/minecraft/client/gui/components/MultiLineTextWidget;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| reads | `nameWidget` | `Lnet/minecraft/client/gui/components/StringWidget;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| reads | `this$0` | `Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |

## Declared members (8 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final MAX_DESCRIPTION_WIDTH_PIXELS : I
public static final ICON_SIZE : I
private final parent : Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;
protected final minecraft : Lnet/minecraft/client/Minecraft;
private final pack : Lnet/minecraft/client/gui/screens/packs/PackSelectionModel$Entry;
private final nameWidget : Lnet/minecraft/client/gui/components/StringWidget;
private final descriptionWidget : Lnet/minecraft/client/gui/components/MultiLineTextWidget;
final synthetic this$0 : Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;
public <init>(Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList;Lnet/minecraft/client/gui/screens/packs/PackSelectionModel$Entry;)V
public getNarration()Lnet/minecraft/network/chat/Component;
public extractContent(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIZF)V
public mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
private showHoverOverlay()Z
public keyboardSelection()V
private keyboardMoveUp()V
private keyboardMoveDown()V
private handlePackSelection()V
public getPackId()Ljava/lang/String;
public shouldTakeFocusAfterInteraction()Z
private synthetic lambda$shouldTakeFocusAfterInteraction$0(Lnet/minecraft/client/gui/screens/packs/TransferableSelectionList$Entry;)Z
private synthetic lambda$handlePackSelection$0(Z)V
```
