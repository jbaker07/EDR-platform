---
type: "interface"
fqcn: "net.minecraft.client.KeyboardHandler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.KeyboardHandler

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `charTyped` | `(JLnet/minecraft/client/input/CharacterEvent;)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| wraps | `keyPress` | `(JILnet/minecraft/client/input/KeyEvent;)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| wraps | `keyPress` | `(JILnet/minecraft/client/input/KeyEvent;)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |

## Declared members (9 fields, 28 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final DEBUG_CRASH_TIME : I
private final minecraft : Lnet/minecraft/client/Minecraft;
private final clipboardManager : Lcom/mojang/blaze3d/platform/ClipboardManager;
private debugCrashKeyTime : J
private debugCrashKeyReportedTime : J
private debugCrashKeyReportedCount : J
private usedDebugKeyAsModifier : Z
private lastPreeditEvent : Lnet/minecraft/client/input/PreeditEvent;
public <init>(Lnet/minecraft/client/Minecraft;)V
private handleChunkDebugKeys(Lnet/minecraft/client/input/KeyEvent;)Z
private debugFeedbackEnabledStatus(Ljava/lang/String;Z)V
private static decorateDebugComponent(Lnet/minecraft/ChatFormatting;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
private debugWarningComponent(Lnet/minecraft/network/chat/Component;)V
private debugFeedbackComponent(Lnet/minecraft/network/chat/Component;)V
private debugFeedbackTranslated(Ljava/lang/String;)V
private debugFeedback(Ljava/lang/String;)V
private handleDebugKeys(Lnet/minecraft/client/input/KeyEvent;)Z
private copyRecreateCommand(ZZ)V
private copyCreateBlockCommand(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/nbt/CompoundTag;)V
private copyCreateEntityCommand(Lnet/minecraft/resources/Identifier;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/nbt/CompoundTag;)V
public keyPress(JILnet/minecraft/client/input/KeyEvent;)V
public charTyped(JLnet/minecraft/client/input/CharacterEvent;)V
public textInput(JLjava/lang/String;)V
public textEditing(JLnet/minecraft/client/input/PreeditEvent;)V
public resubmitLastPreeditEvent(Lnet/minecraft/client/gui/components/events/GuiEventListener;)V
public static submitPreeditEvent(Lnet/minecraft/client/gui/components/events/GuiEventListener;Lnet/minecraft/client/input/PreeditEvent;)V
public getClipboard()Ljava/lang/String;
public setClipboard(Ljava/lang/String;)V
public tick()V
private static synthetic lambda$submitPreeditEvent$0(Lnet/minecraft/client/input/PreeditEvent;)Ljava/lang/String;
private synthetic lambda$textInput$0(JI)V
private synthetic lambda$keyPress$0(I)V
private synthetic lambda$copyRecreateCommand$1(Lnet/minecraft/resources/Identifier;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/nbt/CompoundTag;)V
private synthetic lambda$copyRecreateCommand$0(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/nbt/CompoundTag;)V
private static synthetic lambda$handleDebugKeys$0(Ljava/nio/file/Path;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
static <clinit>()V
```
