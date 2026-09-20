---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.SDLEventHandler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.SDLEventHandler

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `handleDropBeginEvent` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleDropCompleteEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleDropFileEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleKeyEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleMouseButtonEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleMouseMotionEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleMouseWheelEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleTextEditingEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleTextInputEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (3 fields, 24 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final dropFiles : Ljava/util/List;
private final minecraft : Lnet/minecraft/client/Minecraft;
private final window : Lcom/mojang/blaze3d/platform/Window;
public <init>(Lnet/minecraft/client/Minecraft;Lcom/mojang/blaze3d/platform/Window;)V
private static getWindowHandle(Lorg/lwjgl/sdl/SDL_Event;)J
public pollEvents()V
public pumpEvents()V
public flushInputEvents()V
private handleKeymapChangedEvent()V
private handleKeyEvent(Lorg/lwjgl/sdl/SDL_Event;)V
private handleTextInputEvent(Lorg/lwjgl/sdl/SDL_Event;)V
private handleTextEditingEvent(Lorg/lwjgl/sdl/SDL_Event;)V
private handleMouseMotionEvent(Lorg/lwjgl/sdl/SDL_Event;)V
private handleMouseButtonEvent(Lorg/lwjgl/sdl/SDL_Event;)V
private handleDropFileEvent(Lorg/lwjgl/sdl/SDL_Event;)V
private handleDropBeginEvent()V
private handleMouseWheelEvent(Lorg/lwjgl/sdl/SDL_Event;)V
private static isShiftInvertedScroll(DD)Z
private handleDropCompleteEvent(Lorg/lwjgl/sdl/SDL_Event;)V
private synthetic lambda$handleDropCompleteEvent$0(JLjava/util/List;)V
private synthetic lambda$handleMouseWheelEvent$0(Ljava/lang/Long;DD)V
private synthetic lambda$handleMouseButtonEvent$0(JLnet/minecraft/client/input/MouseButtonInfo;I)V
private synthetic lambda$handleMouseMotionEvent$0(JLorg/lwjgl/sdl/SDL_MouseMotionEvent;)V
private synthetic lambda$handleTextEditingEvent$0(JLnet/minecraft/client/input/PreeditEvent;)V
private synthetic lambda$handleTextInputEvent$0(JLjava/lang/String;)V
private synthetic lambda$handleKeyEvent$0(Lorg/lwjgl/sdl/SDL_Event;ILnet/minecraft/client/input/KeyEvent;)V
private synthetic lambda$handleKeymapChangedEvent$0()V
```
