---
type: "interface"
fqcn: "net.minecraft.client.gui.components.CycleButton"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.CycleButton

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/components/AbstractButton`; implements `net/minecraft/client/gui/components/ResettableOptionWidget`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `onPress` | `(Lnet/minecraft/client/input/InputWithModifiers;)V` | exact | invokevirtual@84 in `ClientGameTestContextImpl.pressMatchingButton` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (13 fields, 27 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DEFAULT_ALT_LIST_SELECTOR : Ljava/util/function/BooleanSupplier;
private static final BOOLEAN_OPTIONS : Ljava/util/List;
private final defaultValueSupplier : Ljava/util/function/Supplier;
private final name : Lnet/minecraft/network/chat/Component;
private index : I
private value : Ljava/lang/Object;
private final values : Lnet/minecraft/client/gui/components/CycleButton$ValueListSupplier;
private final valueStringifier : Ljava/util/function/Function;
private final narrationProvider : Ljava/util/function/Function;
private final onValueChange : Lnet/minecraft/client/gui/components/CycleButton$OnValueChange;
private final displayState : Lnet/minecraft/client/gui/components/CycleButton$DisplayState;
private final tooltipSupplier : Lnet/minecraft/client/OptionInstance$TooltipSupplier;
private final spriteSupplier : Lnet/minecraft/client/gui/components/CycleButton$SpriteSupplier;
private <init>(IIIILnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;ILjava/lang/Object;Ljava/util/function/Supplier;Lnet/minecraft/client/gui/components/CycleButton$ValueListSupplier;Ljava/util/function/Function;Ljava/util/function/Function;Lnet/minecraft/client/gui/components/CycleButton$OnValueChange;Lnet/minecraft/client/OptionInstance$TooltipSupplier;Lnet/minecraft/client/gui/components/CycleButton$DisplayState;Lnet/minecraft/client/gui/components/CycleButton$SpriteSupplier;)V
protected extractContents(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
private updateTooltip()V
public onPress(Lnet/minecraft/client/input/InputWithModifiers;)V
private cycleValue(I)V
private getCycledValue(I)Ljava/lang/Object;
public mouseScrolled(DDDD)Z
public setValue(Ljava/lang/Object;)V
public resetValue()V
private updateValue(Ljava/lang/Object;)V
private createLabelForValue(Ljava/lang/Object;)Lnet/minecraft/network/chat/Component;
private createFullName(Ljava/lang/Object;)Lnet/minecraft/network/chat/MutableComponent;
public getValue()Ljava/lang/Object;
protected createNarrationMessage()Lnet/minecraft/network/chat/MutableComponent;
public updateWidgetNarration(Lnet/minecraft/client/gui/narration/NarrationElementOutput;)V
public createDefaultNarrationMessage()Lnet/minecraft/network/chat/MutableComponent;
public static builder(Ljava/util/function/Function;Ljava/util/function/Supplier;)Lnet/minecraft/client/gui/components/CycleButton$Builder;
public static builder(Ljava/util/function/Function;Ljava/lang/Object;)Lnet/minecraft/client/gui/components/CycleButton$Builder;
public static booleanBuilder(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Z)Lnet/minecraft/client/gui/components/CycleButton$Builder;
public static onOffBuilder(Z)Lnet/minecraft/client/gui/components/CycleButton$Builder;
private static synthetic lambda$onOffBuilder$1(Z)Ljava/lang/Boolean;
private static synthetic lambda$onOffBuilder$0(Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$booleanBuilder$1(Z)Ljava/lang/Boolean;
private static synthetic lambda$booleanBuilder$0(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$builder$0(Ljava/lang/Object;)Ljava/lang/Object;
private static synthetic lambda$static$0()Z
static <clinit>()V
```
