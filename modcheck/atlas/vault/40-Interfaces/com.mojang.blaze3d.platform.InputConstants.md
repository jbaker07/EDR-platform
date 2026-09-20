---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.InputConstants"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.InputConstants

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `grabMouse` | `(Lcom/mojang/blaze3d/platform/Window;DD)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `isKeyDown` | `(I)Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `releaseMouse` | `(Lcom/mojang/blaze3d/platform/Window;DD)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `UNKNOWN` | `Lcom/mojang/blaze3d/platform/InputConstants$Key;` | exact | getstatic@11 in `TestInputImpl.getBoundKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (169 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final KEY_0 : I
public static final KEY_1 : I
public static final KEY_2 : I
public static final KEY_3 : I
public static final KEY_4 : I
public static final KEY_5 : I
public static final KEY_6 : I
public static final KEY_7 : I
public static final KEY_8 : I
public static final KEY_9 : I
public static final KEY_A : I
public static final KEY_B : I
public static final KEY_C : I
public static final KEY_D : I
public static final KEY_E : I
public static final KEY_F : I
public static final KEY_G : I
public static final KEY_H : I
public static final KEY_I : I
public static final KEY_J : I
public static final KEY_K : I
public static final KEY_L : I
public static final KEY_M : I
public static final KEY_N : I
public static final KEY_O : I
public static final KEY_P : I
public static final KEY_Q : I
public static final KEY_R : I
public static final KEY_S : I
public static final KEY_T : I
public static final KEY_U : I
public static final KEY_V : I
public static final KEY_W : I
public static final KEY_X : I
public static final KEY_Y : I
public static final KEY_Z : I
public static final KEY_F1 : I
public static final KEY_F2 : I
public static final KEY_F3 : I
public static final KEY_F4 : I
public static final KEY_F5 : I
public static final KEY_F6 : I
public static final KEY_F7 : I
public static final KEY_F8 : I
public static final KEY_F9 : I
public static final KEY_F10 : I
public static final KEY_F11 : I
public static final KEY_F12 : I
public static final KEY_F13 : I
public static final KEY_F14 : I
public static final KEY_F15 : I
public static final KEY_F16 : I
public static final KEY_F17 : I
public static final KEY_F18 : I
public static final KEY_F19 : I
public static final KEY_F20 : I
public static final KEY_F21 : I
public static final KEY_F22 : I
public static final KEY_F23 : I
public static final KEY_F24 : I
public static final KEY_NUMLOCK : I
public static final KEY_NUMPAD0 : I
public static final KEY_NUMPAD1 : I
public static final KEY_NUMPAD2 : I
public static final KEY_NUMPAD3 : I
public static final KEY_NUMPAD4 : I
public static final KEY_NUMPAD5 : I
public static final KEY_NUMPAD6 : I
public static final KEY_NUMPAD7 : I
public static final KEY_NUMPAD8 : I
public static final KEY_NUMPAD9 : I
public static final KEY_NUMPADCOMMA : I
public static final KEY_NUMPADENTER : I
public static final KEY_NUMPADEQUALS : I
public static final KEY_DOWN : I
public static final KEY_LEFT : I
public static final KEY_RIGHT : I
public static final KEY_UP : I
public static final KEY_ADD : I
public static final KEY_APOSTROPHE : I
public static final KEY_BACKSLASH : I
public static final KEY_COMMA : I
public static final KEY_EQUALS : I
public static final KEY_GRAVE : I
public static final KEY_LBRACKET : I
public static final KEY_MINUS : I
public static final KEY_MULTIPLY : I
public static final KEY_PERIOD : I
public static final KEY_RBRACKET : I
public static final KEY_SEMICOLON : I
public static final KEY_SLASH : I
public static final KEY_SPACE : I
public static final KEY_TAB : I
public static final KEY_LALT : I
public static final KEY_LCONTROL : I
public static final KEY_LSHIFT : I
public static final KEY_LGUI : I
public static final KEY_RALT : I
public static final KEY_RCONTROL : I
public static final KEY_RSHIFT : I
public static final KEY_RGUI : I
public static final KEY_RETURN : I
public static final KEY_ESCAPE : I
public static final KEY_BACKSPACE : I
public static final KEY_DELETE : I
public static final KEY_END : I
public static final KEY_HOME : I
public static final KEY_INSERT : I
public static final KEY_PAGEDOWN : I
public static final KEY_PAGEUP : I
public static final KEY_CAPSLOCK : I
public static final KEY_PAUSE : I
public static final KEY_SCROLLLOCK : I
public static final KEY_PRINTSCREEN : I
public static final PRESS : I
public static final RELEASE : I
public static final REPEAT : I
public static final MOUSE_BUTTON_LEFT : I
public static final MOUSE_BUTTON_MIDDLE : I
public static final MOUSE_BUTTON_RIGHT : I
public static final MOUSE_BUTTON_4 : I
public static final MOUSE_BUTTON_5 : I
public static final MOUSE_BUTTON_6 : I
public static final MOUSE_BUTTON_7 : I
public static final MOUSE_BUTTON_8 : I
public static final MOD_SHIFT : I
public static final MOD_CONTROL : I
public static final MOD_ALT : I
public static final MOD_SUPER : I
public static final MOD_CAPS_LOCK : I
public static final MOD_NUM_LOCK : I
public static final KEYCODE_A : I
public static final KEYCODE_B : I
public static final KEYCODE_C : I
public static final KEYCODE_E : I
public static final KEYCODE_F : I
public static final KEYCODE_L : I
public static final KEYCODE_M : I
public static final KEYCODE_O : I
public static final KEYCODE_R : I
public static final KEYCODE_U : I
public static final KEYCODE_V : I
public static final KEYCODE_W : I
public static final KEYCODE_X : I
public static final KEYCODE_Y : I
public static final KEYCODE_Z : I
public static final KEYCODE_RETURN : I
public static final KEYCODE_NUMPADENTER : I
public static final KEYCODE_PAGEUP : I
public static final KEYCODE_PAGEDOWN : I
public static final KEYCODE_BACKSPACE : I
public static final KEYCODE_UP : I
public static final KEYCODE_DOWN : I
public static final KEYCODE_FORWARD : I
public static final KEYCODE_BACKWARD : I
public static final KEYCODE_LEFT : I
public static final KEYCODE_RIGHT : I
public static final KEYCODE_NUMPAD9 : I
public static final KEYCODE_NUMPAD3 : I
public static final KEYCODE_DELETE : I
public static final KEYCODE_HOME : I
public static final KEYCODE_END : I
public static final KEYCODE_F5 : I
public static final KEYCODE_TAB : I
public static final KEYCODE_LCONTROL : I
public static final KEYCODE_RCONTROL : I
public static final KEYCODE_SPACE : I
public static final UNKNOWN : Lcom/mojang/blaze3d/platform/InputConstants$Key;
public <init>()V
public static getKey(Lnet/minecraft/client/input/KeyEvent;)Lcom/mojang/blaze3d/platform/InputConstants$Key;
public static getKey(Ljava/lang/String;)Lcom/mojang/blaze3d/platform/InputConstants$Key;
public static isKeyDown(I)Z
public static grabMouse(Lcom/mojang/blaze3d/platform/Window;DD)V
public static releaseMouse(Lcom/mojang/blaze3d/platform/Window;DD)V
static <clinit>()V
```
