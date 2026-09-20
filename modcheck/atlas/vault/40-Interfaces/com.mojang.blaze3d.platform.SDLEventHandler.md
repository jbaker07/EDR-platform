---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.SDLEventHandler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.SDLEventHandler

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `handleDropBeginEvent` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleDropCompleteEvent` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleDropFileEvent` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleKeyEvent` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleMouseButtonEvent` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleMouseMotionEvent` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleMouseWheelEvent` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleTextEditingEvent` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleTextInputEvent` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class com.mojang.blaze3d.platform.SDLEventHandler {
    private final java.util.List<java.lang.String> dropFiles;
    private final net.minecraft.client.Minecraft minecraft;
    private final com.mojang.blaze3d.platform.Window window;
    public com.mojang.blaze3d.platform.SDLEventHandler(net.minecraft.client.Minecraft, com.mojang.blaze3d.platform.Window);
    private static long getWindowHandle(org.lwjgl.sdl.SDL_Event);
    public void pollEvents();
    public void pumpEvents();
    public void flushInputEvents();
    private void handleKeymapChangedEvent();
    private void handleKeyEvent(org.lwjgl.sdl.SDL_Event);
    private void handleTextInputEvent(org.lwjgl.sdl.SDL_Event);
    private void handleTextEditingEvent(org.lwjgl.sdl.SDL_Event);
    private void handleMouseMotionEvent(org.lwjgl.sdl.SDL_Event);
    private void handleMouseButtonEvent(org.lwjgl.sdl.SDL_Event);
    private void handleDropFileEvent(org.lwjgl.sdl.SDL_Event);
    private void handleDropBeginEvent();
    private void handleMouseWheelEvent(org.lwjgl.sdl.SDL_Event);
    private static boolean isShiftInvertedScroll(double, double);
    private void handleDropCompleteEvent(org.lwjgl.sdl.SDL_Event);
    private void lambda$handleDropCompleteEvent$0(long, java.util.List);
    private void lambda$handleMouseWheelEvent$0(java.lang.Long, double, double);
    private void lambda$handleMouseButtonEvent$0(long, net.minecraft.client.input.MouseButtonInfo, int);
    private void lambda$handleMouseMotionEvent$0(long, org.lwjgl.sdl.SDL_MouseMotionEvent);
    private void lambda$handleTextEditingEvent$0(long, net.minecraft.client.input.PreeditEvent);
    private void lambda$handleTextInputEvent$0(long, java.lang.String);
    private void lambda$handleKeyEvent$0(org.lwjgl.sdl.SDL_Event, int, net.minecraft.client.input.KeyEvent);
    private void lambda$handleKeymapChangedEvent$0();
}
```
