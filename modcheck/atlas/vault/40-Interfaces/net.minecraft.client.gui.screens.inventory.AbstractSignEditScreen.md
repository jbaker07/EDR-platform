---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `net/minecraft/client/gui/screens/Screen`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraf` | exact | invokespecial@4 in `HangingSignEditScreenMixin.<init>` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraf` | exact | invokespecial@4 in `SignEditScreenMixin.<init>` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (12 fields, 24 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LINE_COUNT : I
protected final sign : Lnet/minecraft/world/level/block/entity/SignBlockEntity;
private final text : Lnet/minecraft/world/level/block/entity/SignText$Mutable;
private final messages : [Ljava/lang/String;
private final slot : Lnet/minecraft/world/level/block/entity/SignTextSlot;
private final textColor : I
protected final woodType : Lnet/minecraft/world/level/block/state/properties/WoodType;
private cursorBlinkStartTime : J
private line : I
private final signField : Lnet/minecraft/client/gui/font/TextFieldHelper;
private preeditOverlay : Lnet/minecraft/client/gui/components/IMEPreeditOverlay;
private final cursorPosScratch : Lorg/joml/Vector2f;
public <init>(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraft/world/level/block/entity/SignTextSlot;Z)V
public <init>(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lnet/minecraft/world/level/block/entity/SignTextSlot;ZLnet/minecraft/network/chat/Component;)V
protected init()V
public tick()V
private isValid()Z
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public isInputCaptured()Z
public charTyped(Lnet/minecraft/client/input/CharacterEvent;)Z
public preeditUpdated(Lnet/minecraft/client/input/PreeditEvent;)Z
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
public onClose()V
public removed()V
public isPauseScreen()Z
public isInGameUi()Z
protected abstract extractSignBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
protected abstract getSignTextScale()Lorg/joml/Vector3fc;
protected abstract getSignYOffset()F
private extractSign(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
private extractSignText(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lorg/joml/Vector2f;)V
private setMessage(Ljava/lang/String;)V
private onDone()V
private synthetic lambda$init$0(Lnet/minecraft/client/gui/components/Button;)V
private synthetic lambda$new$1(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Ljava/lang/String;)Z
private synthetic lambda$new$0()Ljava/lang/String;
```
