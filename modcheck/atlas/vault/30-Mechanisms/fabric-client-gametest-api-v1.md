---
type: "mechanism"
module: "fabric-client-gametest-api-v1"
version: "6.0.7+4be74c3f5d"
sha256: "09d7d48475eef47a347c12c51bd32c6cffc065a6785f3023f5aa6be4b815cd38"
lifecycle: "experimental"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-client-gametest-api-v1

**Version** `6.0.7+4be74c3f5d` -- **artifact sha256** `09d7d48475eef47a347c12c51bd32c6cffc065a6785f3023f5aa6be4b815cd38`

## Declared (fabric.mod.json)

- environment: `client`
- depends: `{"fabricloader": ">=0.19.3", "fabric-resource-loader-v1": "*", "fabric-networking-api-v1": "*"}`
- entrypoints: `{"client": ["net.fabricmc.fabric.impl.client.gametest.FabricClientGameTestImpl"]}`
- mixin configs: `["fabric-client-gametest-api-v1.mixins.json"]`
- access widener: `fabric-client-gametest-api-v1.classtweaker`

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/com.mojang.blaze3d.platform.InputConstants|InputConstants]] | `grabMouse` | injects_into `@Inject at HEAD` | both | `InputConstantsMixin.disableCursorGrabbing` |
| [[40-Interfaces/com.mojang.blaze3d.platform.InputConstants|InputConstants]] | `isKeyDown` | injects_into `@Inject at HEAD` | both | `InputConstantsMixin.useGameTestInputForKeyDown` |
| [[40-Interfaces/com.mojang.blaze3d.platform.InputConstants|InputConstants]] | `releaseMouse` | injects_into `@Inject at HEAD` | both | `InputConstantsMixin.disableCursorGrabbing` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]] | `handleDropBeginEvent` | injects_into `@Inject at HEAD` | both | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]] | `handleDropCompleteEvent` | injects_into `@Inject at HEAD` | both | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]] | `handleDropFileEvent` | injects_into `@Inject at HEAD` | both | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]] | `handleKeyEvent` | injects_into `@Inject at HEAD` | both | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]] | `handleMouseButtonEvent` | injects_into `@Inject at HEAD` | both | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]] | `handleMouseMotionEvent` | injects_into `@Inject at HEAD` | both | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]] | `handleMouseWheelEvent` | injects_into `@Inject at HEAD` | both | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]] | `handleTextEditingEvent` | injects_into `@Inject at HEAD` | both | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.SDLEventHandler|SDLEventHandler]] | `handleTextInputEvent` | injects_into `@Inject at HEAD` | both | `SDLEventHandlerMixin.disableRealInput` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]] | `<init>(Lcom/mojang/blaze3d/platform/WindowEventHandler;Lcom/mojang/blaze3d/platform/DisplayData;Ljava/lang/String;ZLjava/lang/String;Lcom/mojang/blaze3d/platform/MonitorManager;Lcom/mojang/renderpearl/api/device/GpuBackend;I)V` | injects_into `@Inject at RETURN` | both | `WindowMixin.onInit` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]] | `handleEvent` | injects_into `@Inject at HEAD` | both | `WindowMixin.cancelDirectEvents` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]] | `onFocus` | injects_into `@Inject at HEAD` | both | `WindowMixin.cancelEvents` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]] | `onFramebufferResize` | injects_into `@Inject at HEAD` | both | `WindowMixin.cancelFramebufferResize` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]] | `onIconified` | injects_into `@Inject at HEAD` | both | `WindowMixin.cancelEvents` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]] | `onResize` | injects_into `@Inject at HEAD` | both | `WindowMixin.cancelResize` |
| [[40-Interfaces/com.mojang.blaze3d.platform.Window|Window]] | `setWindowed` | injects_into `@Inject at HEAD` | both | `WindowMixin.setWindowedSize` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `disconnect(Lnet/minecraft/client/gui/screens/Screen;Z)V` | injects_into `@Inject at HEAD` | both | `MinecraftMixin.deferDisconnect` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `disconnect(Lnet/minecraft/client/gui/screens/Screen;ZZ)V` | injects_into `@Inject at INVOKE Lnet/minecraft/client/Minecraft;renderFrame(Z)V` | both | `MinecraftMixin.onDisconnectBusyWait` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `doWorldLoad` | injects_into `@Inject at HEAD` | both | `MinecraftMixin.deferStartIntegratedServer` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `doWorldLoad` | injects_into `@Inject at INVOKE Lnet/minecraft/client/Minecraft;managedBlock(Ljava/util/function/BooleanSupplier;)V` | both | `MinecraftMixin.onStartIntegratedServerBusyWait` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `emergencySave` | injects_into `@Inject at HEAD` | both | `MinecraftMixin.deregisterAfterCrash` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `getInstance` | injects_into `@Inject at HEAD` | both | `MinecraftMixin.checkThreadOnGetInstance` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `runTick` | injects_into `@Inject at INVOKE Lnet/minecraft/client/Minecraft;runAllTasks()V` | both | `MinecraftMixin.postRunTasksHook` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `tick` | injects_into `@Inject at HEAD` | both | `MinecraftMixin.onTick` |
| [[40-Interfaces/net.minecraft.client.Options|Options]] | `<init>` | injects_into `@Inject at RETURN` | both | `OptionsMixin.onCreateGameOptions` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]] | `extractPanorama` | injects_into `@Inject at HEAD` | both | `ScreenMixin.disableRotatingPanoramaForClientGameTests` |
| [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]] | `onCreate` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/worldselection/WorldOpenFlows;confirmWorldCreation(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Lcom/mojang/serialization/Lifecycle;Ljava/lang/Runnable;Z)V` | both | `CreateWorldScreenMixin.createLevelDataForServers` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `runServer` | injects_into `@Inject at INVOKE Lnet/minecraft/server/MinecraftServer;onServerCrash(Lnet/minecraft/CrashReport;)V` | both | `MinecraftServerMixin.onCrash` |
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `runServer` | injects_into `@Inject at INVOKE Lnet/minecraft/server/MinecraftServer;waitUntilNextTick()V` | both | `MinecraftServerMixin.postRunTasks` |
| [[40-Interfaces/net.minecraft.server.dedicated.DedicatedServer|DedicatedServer]] | `initServer` | injects_into `@Inject at INVOKE Lnet/minecraft/server/dedicated/DedicatedServer;loadLevel()V` | both | `DedicatedServerMixin.captureServerInstance` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.FabricClientGameTest|FabricClientGameTest]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.TestInput|TestInput]] (interface, 35 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.ClientGameTestContext|ClientGameTestContext]] (interface, 21 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestDedicatedServerConnection|TestDedicatedServerConnection]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestDedicatedServerContext|TestDedicatedServerContext]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestServerConnection|TestServerConnection]] (interface, 14 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestServerContext|TestServerContext]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.context.TestSingleplayerContext|TestSingleplayerContext]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotCommonOptions|TestScreenshotCommonOptions]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm|TestScreenshotComparisonAlgorithm]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonOptions|TestScreenshotComparisonOptions]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotOptions|TestScreenshotOptions]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.world.TestWorldBuilder|TestWorldBuilder]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.gametest.v1.world.TestWorldSave|TestWorldSave]] (interface, 2 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
