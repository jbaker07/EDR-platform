---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.advancements.AdvancementTab"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.advancements.AdvancementTab

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getRootAdvancement` | `()Lnet/minecraft/advancements/AdvancementHolder;` | exact | invokevirtual@1 in `AdvancementsScreenMixin.wrapDrawIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `isMouseOver` | `(IIDD)Z` | exact | invokevirtual@27 in `AdvancementsScreenMixin.wrapDrawIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extractContents` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V` | name_only | @ModifyExpressionValue at ['CONSTANT'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `extractContents` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `root` | `Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| reads | `scrollX` | `D` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| reads | `scrollY` | `D` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| wraps | `extractContents` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractIcon` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (18 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final minecraft : Lnet/minecraft/client/Minecraft;
private final screen : Lnet/minecraft/client/gui/screens/advancements/AdvancementsScreen;
private final type : Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
private final index : I
private final background : Lnet/minecraft/resources/Identifier;
private final icon : Lnet/minecraft/world/item/ItemStack;
private final title : Lnet/minecraft/network/chat/Component;
private final root : Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;
private final widgets : Ljava/util/Map;
private scrollX : D
private scrollY : D
private minX : I
private minY : I
private maxX : I
private maxY : I
private fade : F
private centered : Z
private hovered : Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/advancements/AdvancementsScreen;Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;ILnet/minecraft/client/gui/screens/advancements/AdvancementWidget;Lnet/minecraft/world/item/ItemStackTemplate;Lnet/minecraft/network/chat/Component;Lnet/minecraft/resources/Identifier;)V
public copyPosition(Lnet/minecraft/client/gui/screens/advancements/AdvancementTab;)V
public getType()Lnet/minecraft/client/gui/screens/advancements/AdvancementTabType;
public getIndex()I
public getRootAdvancement()Lnet/minecraft/advancements/AdvancementHolder;
public getTitle()Lnet/minecraft/network/chat/Component;
public tick(II)V
public extractTab(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIIIZ)V
public extractIcon(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
public extractContents(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
public extractTooltips(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
public isMouseOver(IIDD)Z
public static create(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/advancements/AdvancementsScreen;ILnet/minecraft/advancements/AdvancementNode;)Lnet/minecraft/client/gui/screens/advancements/AdvancementTab;
public scroll(DD)V
public canScrollHorizontally()Z
public canScrollVertically()Z
public addAdvancement(Lnet/minecraft/advancements/AdvancementNode;)V
private addWidget(Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;)V
public getWidget(Lnet/minecraft/advancements/AdvancementHolder;)Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;
public getScreen()Lnet/minecraft/client/gui/screens/advancements/AdvancementsScreen;
```
