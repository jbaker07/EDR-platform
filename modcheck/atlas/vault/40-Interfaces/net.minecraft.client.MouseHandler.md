---
type: "interface"
fqcn: "net.minecraft.client.MouseHandler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.MouseHandler

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `xpos` | `()D` | exact | invokevirtual@5 in `TestInputImpl.lambda$moveCursor$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `xpos` | `()D` | exact | invokevirtual@24 in `TestInputImpl.lambda$setCursorPos$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ypos` | `()D` | exact | invokevirtual@17 in `TestInputImpl.lambda$moveCursor$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ypos` | `()D` | exact | invokevirtual@34 in `TestInputImpl.lambda$setCursorPos$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| wraps | `handleAccumulatedMovement` | `()V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| wraps | `onButton` | `(JLnet/minecraft/client/input/MouseButtonInfo;I)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| wraps | `onButton` | `(JLnet/minecraft/client/input/MouseButtonInfo;I)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| wraps | `onScroll` | `(JDD)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| wraps | `onScroll` | `(JDD)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |

## Declared members (20 fields, 27 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final DOUBLE_CLICK_THRESHOLD_MS : J
private final minecraft : Lnet/minecraft/client/Minecraft;
private isLeftPressed : Z
private isMiddlePressed : Z
private isRightPressed : Z
private xpos : D
private ypos : D
private lastClick : Lnet/minecraft/client/MouseHandler$LastClick;
protected lastClickButton : I
private activeButton : Lnet/minecraft/client/input/MouseButtonInfo;
private ignoreFirstMove : Z
private mousePressedTime : D
private final smoothTurnX : Lnet/minecraft/util/SmoothDouble;
private final smoothTurnY : Lnet/minecraft/util/SmoothDouble;
private accumulatedDX : D
private accumulatedDY : D
private final scrollWheelHandler : Lnet/minecraft/client/ScrollWheelHandler;
private lastHandleMovementTime : D
private mouseGrabbed : Z
public <init>(Lnet/minecraft/client/Minecraft;)V
public onButton(JLnet/minecraft/client/input/MouseButtonInfo;I)V
public fillMousePositionDetails(Lnet/minecraft/CrashReportCategory;Lcom/mojang/blaze3d/platform/Window;)V
public onScroll(JDD)V
public onDrop(JLjava/util/List;)V
public onMove(JDDDD)V
public handleAccumulatedMovement()V
public static getScaledXPos(Lcom/mojang/blaze3d/platform/Window;D)D
public getScaledXPos(Lcom/mojang/blaze3d/platform/Window;)D
public static getScaledYPos(Lcom/mojang/blaze3d/platform/Window;D)D
public getScaledYPos(Lcom/mojang/blaze3d/platform/Window;)D
private turnPlayer(D)V
public isLeftPressed()Z
public isMiddlePressed()Z
public isRightPressed()Z
public xpos()D
public ypos()D
public setIgnoreFirstMove()V
public resyncMousePosition()V
public isMouseGrabbed()Z
public grabMouse()V
public releaseMouse()V
public cursorEntered()V
public drawDebugMouseInfo(Lnet/minecraft/client/gui/Font;Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
private static synthetic lambda$fillMousePositionDetails$1(Lcom/mojang/blaze3d/platform/Window;)Ljava/lang/String;
private synthetic lambda$fillMousePositionDetails$0(Lcom/mojang/blaze3d/platform/Window;)Ljava/lang/String;
static <clinit>()V
```
