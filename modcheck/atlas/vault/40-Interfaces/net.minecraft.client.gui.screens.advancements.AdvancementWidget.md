---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.advancements.AdvancementWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.advancements.AdvancementWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getAdvancement` | `()Lnet/minecraft/advancements/AdvancementHolder;` | exact | invokevirtual@4 in `AdvancementTabMixin.preBackgroundRender` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getAdvancement` | `()Lnet/minecraft/advancements/AdvancementHolder;` | exact | invokevirtual@43 in `AdvancementTabMixin.extractAdvancementBackground` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extractHover` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIFIII)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `advancementNode` | `Lnet/minecraft/advancements/AdvancementNode;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| reads | `progress` | `Lnet/minecraft/advancements/AdvancementProgress;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| wraps | `extractHover` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIFIII)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHover` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIFIII)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHover` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIFIII)V` | name_only | @WrapWithCondition at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHover` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIFIII)V` | name_only | @WrapWithCondition at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHover` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIFIII)V` | name_only | @WrapWithCondition at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHover` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIFIII)V` | name_only | @WrapWithCondition at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (28 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final TITLE_BOX_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HEIGHT : I
private static final BOX_X : I
private static final BOX_WIDTH : I
private static final FRAME_WIDTH : I
private static final ICON_X : I
private static final ICON_Y : I
private static final ICON_WIDTH : I
private static final TITLE_PADDING_LEFT : I
private static final TITLE_PADDING_RIGHT : I
private static final TITLE_X : I
private static final TITLE_PADDING_TOP : I
private static final TITLE_PADDING_BOTTOM : I
private static final TITLE_MAX_WIDTH : I
private static final TITLE_MIN_WIDTH : I
private static final TEST_SPLIT_OFFSETS : [I
private final advancementNode : Lnet/minecraft/advancements/AdvancementNode;
private final display : Lnet/minecraft/advancements/DisplayInfo;
private final icon : Lnet/minecraft/world/item/ItemStack;
private final titleLines : Ljava/util/List;
private final width : I
private final description : Ljava/util/List;
private final minecraft : Lnet/minecraft/client/Minecraft;
private parent : Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;
private final children : Ljava/util/List;
private progress : Lnet/minecraft/advancements/AdvancementProgress;
private final x : I
private final y : I
private <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/advancements/AdvancementNode;Lnet/minecraft/advancements/DisplayInfo;)V
public static createWidget(Lnet/minecraft/client/Minecraft;Lnet/minecraft/advancements/AdvancementNode;)Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;
private getMaxProgressWidth()I
private static getMaxWidth(Lnet/minecraft/client/StringSplitter;Ljava/util/List;)F
private findOptimalLines(Lnet/minecraft/network/chat/Component;I)Ljava/util/List;
private static findFirstVisibleParent(Lnet/minecraft/advancements/AdvancementNode;)Lnet/minecraft/advancements/AdvancementHolder;
public extractConnectivity(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIZ)V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
public getWidth()I
public setProgress(Lnet/minecraft/advancements/AdvancementProgress;)V
public addChild(Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;)V
public extractHover(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIFIII)V
private extractMultilineText(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Ljava/util/List;III)V
public isMouseOver(IIII)Z
public attachToParent(Lnet/minecraft/client/gui/screens/advancements/AdvancementTab;)V
public getY()I
public getX()I
public getAdvancement()Lnet/minecraft/advancements/AdvancementHolder;
public getDisplay()Lnet/minecraft/advancements/DisplayInfo;
static <clinit>()V
```
