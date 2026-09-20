---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.Window"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.Window

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`class` public final; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getHeight` | `()I` | exact | invokevirtual@12 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getScreenHeight` | `()I` | exact | invokevirtual@4 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$2` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getScreenHeight` | `()I` | exact | invokevirtual@15 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getScreenWidth` | `()I` | exact | invokevirtual@4 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getScreenWidth` | `()I` | exact | invokevirtual@8 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@4 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `handle` | `()J` | exact | invokevirtual@47 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `handle` | `()J` | exact | invokevirtual@91 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `handle` | `()J` | exact | invokevirtual@37 in `TestInputImpl.lambda$moveCursor$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `handle` | `()J` | exact | invokevirtual@13 in `TestInputImpl.lambda$setCursorPos$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `handle` | `()J` | exact | invokevirtual@13 in `TestInputImpl.lambda$scroll$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `handle` | `()J` | exact | invokevirtual@11 in `TestInputImpl.lambda$typeChars$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `handle` | `()J` | exact | invokevirtual@11 in `TestInputImpl.lambda$typeChar$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setHeight` | `(I)V` | exact | invokevirtual@19 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$4` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setHeight` | `(I)V` | exact | invokevirtual@51 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setMode` | `()V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| calls | `setWidth` | `(I)V` | exact | invokevirtual@8 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$4` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setWidth` | `(I)V` | exact | invokevirtual@40 in `ClientGameTestContextImpl.lambda$doTakeScreenshot$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lcom/mojang/blaze3d/platform/WindowEventHandler;Lcom/mojang/blaze3d/p` | exact | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleEvent` | `(Lorg/lwjgl/sdl/SDL_Event;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `onFocus` | `(Z)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `onFramebufferResize` | `(II)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `onIconified` | `(Z)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `onResize` | `(II)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `setWindowed` | `(II)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `eventHandler` | `Lcom/mojang/blaze3d/platform/WindowEventHandler;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `focused` | `Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `framebufferHeight` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `framebufferWidth` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `fullscreen` | `Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `height` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `monitorManager` | `Lcom/mojang/blaze3d/platform/MonitorManager;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `preferredFullscreenVideoMode` | `Ljava/util/Optional;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `width` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `windowedHeight` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `windowedWidth` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `windowedX` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `windowedY` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `x` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| reads | `y` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | declared |
| wraps | `setMode` | `()V` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (36 fields, 77 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final MIN_WINDOW_WIDTH : I
public static final MIN_WINDOW_HEIGHT : I
public static final BASE_WIDTH : I
public static final BASE_HEIGHT : I
private static final BORDERLESS_FULLSCREEN_PADDING : I
private final eventHandler : Lcom/mojang/blaze3d/platform/WindowEventHandler;
private final monitorManager : Lcom/mojang/blaze3d/platform/MonitorManager;
private final handle : J
private windowedX : I
private windowedY : I
private windowedWidth : I
private windowedHeight : I
private preferredFullscreenVideoMode : Ljava/util/Optional;
private fullscreenRequested : Z
private fullscreen : Z
private x : I
private y : I
private width : I
private height : I
private framebufferWidth : I
private framebufferHeight : I
private guiScaledWidth : I
private guiScaledHeight : I
private guiScale : I
private errorSection : Ljava/lang/String;
private dirty : Z
private iconified : Z
private focused : Z
private shouldClose : Z
private closeCallback : Ljava/lang/Runnable;
private allowCursorChanges : Z
private quitShortcuts : Z
private currentCursor : Lcom/mojang/blaze3d/platform/cursor/CursorType;
private exclusiveFullscreen : Z
private borderlessFullscreen : Z
public <init>(Lcom/mojang/blaze3d/platform/WindowEventHandler;Lcom/mojang/blaze3d/platform/DisplayData;Ljava/lang/String;ZLjava/lang/String;Lcom/mojang/blaze3d/platform/MonitorManager;Lcom/mojang/renderpearl/api/device/GpuBackend;)V
public <init>(Lcom/mojang/blaze3d/platform/WindowEventHandler;Lcom/mojang/blaze3d/platform/DisplayData;Ljava/lang/String;ZLjava/lang/String;Lcom/mojang/blaze3d/platform/MonitorManager;Lcom/mojang/renderpearl/api/device/GpuBackend;I)V
public static getPlatform()Ljava/lang/String;
private static createIconSurface(Lcom/mojang/blaze3d/platform/NativeImage;)Lorg/lwjgl/sdl/SDL_Surface;
private createWindow(Lcom/mojang/renderpearl/api/device/GpuBackend;IILjava/lang/String;)J
public getActiveVideoMode()Lcom/mojang/blaze3d/platform/VideoMode;
private getActiveDisplayMode()Lorg/lwjgl/sdl/SDL_DisplayMode;
public shouldClose()Z
public handleEvent(Lorg/lwjgl/sdl/SDL_Event;)V
private onDisplayModeChanged(I)V
private onQuitRequested()V
private updateFullscreenState()V
private requestClose()V
public setIcon(Lnet/minecraft/server/packs/PackMetadataResources;Lcom/mojang/blaze3d/platform/IconSet;)V
private setIcon(Ljava/util/List;)V
private static createIconSurface(Lnet/minecraft/server/packs/resources/IoSupplier;Ljava/util/List;)Lorg/lwjgl/sdl/SDL_Surface;
public getErrorSection()Ljava/lang/String;
public setErrorSection(Ljava/lang/String;)V
public close()V
private onMove(II)V
private onResize(II)V
private onFramebufferResize(II)V
private refreshFramebufferSize()V
private framebufferWidthPadding()I
public queryFramebufferSize()Lcom/mojang/blaze3d/platform/Window$FramebufferSize;
private onFocus(Z)V
private onIconified(Z)V
public updateFullscreenIfChanged()V
public getPreferredFullscreenVideoMode()Ljava/util/Optional;
public setPreferredFullscreenVideoMode(Ljava/util/Optional;)V
public changeFullscreenVideoMode()V
private setMode()V
private updateWindowMouseGrab()V
private isWindowFullscreen()Z
public isExclusiveFullscreen()Z
private applyFullscreen()Z
private useBorderlessFullscreenWindow()Z
private applySdlBorderlessFullscreen()Z
private leaveBorderlessFullscreenWindow()V
private applyFullscreenMode()Z
private applyExclusiveFullscreen()Z
private applyWindowed()Z
private applyBorderlessFullscreenWindow()Z
private restoreWindow()V
private syncWindow()V
private setWindowSizeAndPosition(IIII)Z
private applyBorderlessFullscreen()Z
public setExclusiveFullscreen(Z)V
public setWindowed(II)V
public calculateScale(IZ)I
public setTitle(Ljava/lang/String;)V
public setWindowMaxSize(II)V
public handle()J
public setFullscreen(Z)V
public isIconified()Z
public isFocused()Z
public getWidth()I
public setWidth(I)V
public getHeight()I
public setHeight(I)V
public getScreenWidth()I
public getScreenHeight()I
public getGuiScaledWidth()I
public getGuiScaledHeight()I
public getX()I
public getY()I
public getGuiScale()I
public setGuiScale(I)V
public getPixelDensity()F
public findBestMonitor()Lcom/mojang/blaze3d/platform/Monitor;
public setWindowCloseCallback(Ljava/lang/Runnable;)V
public setAllowCursorChanges(Z)V
public setQuitShortcuts(Z)V
public selectCursor(Lcom/mojang/blaze3d/platform/cursor/CursorType;)V
public getAppropriateLineWidth()F
private static allowedWindowMinSize(II)I
static <clinit>()V
```
