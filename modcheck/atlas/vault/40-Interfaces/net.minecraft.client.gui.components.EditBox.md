---
type: "interface"
fqcn: "net.minecraft.client.gui.components.EditBox"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.EditBox

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/components/AbstractWidget`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/gui/Font;IIIILnet/minecraft/network/chat/Compon` | exact | invokespecial@59 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | inherited_exact | invokevirtual@47 in `DoubleRuleEntry.extractContent` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setResponder` | `(Ljava/util/function/Consumer;)V` | exact | invokevirtual@98 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setTextColor` | `(I)V` | exact | invokevirtual@23 in `DoubleRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setTextColor` | `(I)V` | exact | invokevirtual@63 in `DoubleRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setValue` | `(Ljava/lang/String;)V` | exact | invokevirtual@81 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setX` | `(I)V` | exact | invokevirtual@24 in `DoubleRuleEntry.extractContent` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setY` | `(I)V` | exact | invokevirtual@35 in `DoubleRuleEntry.extractContent` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (28 fields, 65 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final SPRITES : Lnet/minecraft/client/gui/components/WidgetSprites;
public static final BACKWARDS : I
public static final FORWARDS : I
public static final DEFAULT_TEXT_COLOR : I
public static final DEFAULT_HINT_STYLE : Lnet/minecraft/network/chat/Style;
public static final SEARCH_HINT_STYLE : Lnet/minecraft/network/chat/Style;
private final font : Lnet/minecraft/client/gui/Font;
private value : Ljava/lang/String;
private maxLength : I
private bordered : Z
private canLoseFocus : Z
private isEditable : Z
private centered : Z
private textShadow : Z
private invertHighlightedTextColor : Z
private displayPos : I
private cursorPos : I
private highlightPos : I
private textColor : I
private textColorUneditable : I
private suggestion : Ljava/lang/String;
private responder : Ljava/util/function/Consumer;
private final formatters : Ljava/util/List;
private hint : Lnet/minecraft/network/chat/Component;
private preeditOverlay : Lnet/minecraft/client/gui/components/IMEPreeditOverlay;
private focusedTime : J
private textX : I
private textY : I
public <init>(Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;)V
public <init>(Lnet/minecraft/client/gui/Font;IILnet/minecraft/network/chat/Component;)V
public <init>(Lnet/minecraft/client/gui/Font;IIIILnet/minecraft/network/chat/Component;)V
public <init>(Lnet/minecraft/client/gui/Font;IIIILnet/minecraft/client/gui/components/EditBox;Lnet/minecraft/network/chat/Component;)V
public setResponder(Ljava/util/function/Consumer;)V
public addFormatter(Lnet/minecraft/client/gui/components/EditBox$TextFormatter;)V
protected createNarrationMessage()Lnet/minecraft/network/chat/MutableComponent;
public setValue(Ljava/lang/String;)V
public getValue()Ljava/lang/String;
public getHighlighted()Ljava/lang/String;
public setX(I)V
public setY(I)V
public insertText(Ljava/lang/String;)V
private onValueChange(Ljava/lang/String;)V
private deleteText(IZ)V
public deleteWords(I)V
public deleteChars(I)V
public deleteCharsToPos(I)V
public getWordPosition(I)I
private getWordPosition(II)I
private getWordPosition(IIZ)I
public moveCursor(IZ)V
private getCursorPos(I)I
public moveCursorTo(IZ)V
public setCursorPosition(I)V
public moveCursorToStart(Z)V
public moveCursorToEnd(Z)V
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public canConsumeInput()Z
public capturesInput()Z
public charTyped(Lnet/minecraft/client/input/CharacterEvent;)Z
public preeditUpdated(Lnet/minecraft/client/input/PreeditEvent;)Z
private findClickedPositionInText(Lnet/minecraft/client/input/MouseButtonEvent;)I
private selectWord(Lnet/minecraft/client/input/MouseButtonEvent;)V
public onClick(Lnet/minecraft/client/input/MouseButtonEvent;Z)V
protected onDrag(Lnet/minecraft/client/input/MouseButtonEvent;DD)V
public playDownSound(Lnet/minecraft/client/sounds/SoundManager;)V
public extractWidgetRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
private applyFormat(Ljava/lang/String;I)Lnet/minecraft/util/FormattedCharSequence;
private updateTextPosition()V
public setMaxLength(I)V
private getMaxLength()I
public getCursorPosition()I
public isBordered()Z
public setBordered(Z)V
public setTextColor(I)V
public setTextColorUneditable(I)V
public setFocused(Z)V
private isEditable()Z
public setEditable(Z)V
private isCentered()Z
public setCentered(Z)V
public setTextShadow(Z)V
public setInvertHighlightedTextColor(Z)V
public getInnerWidth()I
public setHighlightPos(I)V
private scrollTo(I)V
public setCanLoseFocus(Z)V
public isVisible()Z
public setVisible(Z)V
public setSuggestion(Ljava/lang/String;)V
public getScreenX(I)I
public updateWidgetNarration(Lnet/minecraft/client/gui/narration/NarrationElementOutput;)V
public setHint(Lnet/minecraft/network/chat/Component;)V
static <clinit>()V
```
