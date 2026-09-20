---
type: "interface"
fqcn: "net.minecraft.client.gui.components.FocusableTextWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.FocusableTextWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/components/MultiLineTextWidget`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `builder` | `(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;` | exact | invokestatic@50 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `builder` | `(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;` | exact | invokestatic@125 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setCentered` | `(Z)Lnet/minecraft/client/gui/components/MultiLineTextWidget;` | inherited_exact | invokevirtual@77 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setCentered` | `(Z)Lnet/minecraft/client/gui/components/MultiLineTextWidget;` | inherited_exact | invokevirtual@152 in `DetailsScreen.addContents` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (8 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DEFAULT_PADDING : I
private final padding : I
private final maxWidth : I
private final alwaysShowBorder : Z
private final backgroundFill : Lnet/minecraft/client/gui/components/FocusableTextWidget$BackgroundFill;
private narrateMessage : Z
private focusedUsageNarration : Lnet/minecraft/network/chat/Component;
private hoveredUsageNarration : Lnet/minecraft/network/chat/Component;
private <init>(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;IILnet/minecraft/client/gui/components/FocusableTextWidget$BackgroundFill;Z)V
protected updateWidgetNarration(Lnet/minecraft/client/gui/narration/NarrationElementOutput;)V
public setNarrateMessage(Z)V
public setUsageNarration(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;)V
public extractWidgetRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
protected getTextX()I
protected getTextY()I
public setMaxWidth(I)Lnet/minecraft/client/gui/components/MultiLineTextWidget;
public getWidth()I
public getHeight()I
public getPadding()I
public updateWidth()V
public updateHeight()V
public setMessage(Lnet/minecraft/network/chat/Component;)V
public playDownSound(Lnet/minecraft/client/sounds/SoundManager;)V
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public static builder(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;
public static builder(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/Font;I)Lnet/minecraft/client/gui/components/FocusableTextWidget$Builder;
private static synthetic lambda$keyPressed$0(Lnet/minecraft/network/chat/Style;Ljava/lang/String;)Ljava/util/Optional;
```
